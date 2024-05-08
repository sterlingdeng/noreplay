mod bigint;
mod dequeue;
mod errors;

use crate::bigint::Bigint;
use crate::errors::{ReplayError, Result};

pub trait Checker {
    fn check_and_accept(&mut self, seq: usize) -> Result<bool>;
}

/// Mask provides a mask to detect if a seq number has been used.
/// Implementations can include a Bigint, vector, or a dequeue.
pub trait Mask {
    fn bit(&self, pos: usize) -> bool;
    fn set_bit(&mut self, n: usize);
    fn shl(&mut self, n: usize);
}

pub struct NoWrapReplayDetector {
    sliding_window: Box<dyn Mask>,
    max_seq: usize,
    latest_seq: usize,
    window_size: usize,
}

pub struct DetectorConfig {
    // if mask is not specified, the Bigint implementation is used
    mask: Option<Box<dyn Mask>>,
    max_seq: usize,
    window_size: usize,
}

impl NoWrapReplayDetector {
    pub fn new(cfg: DetectorConfig) -> Self {
        let sliding_window: Box<dyn Mask>;
        match cfg.mask {
            Some(mask) => sliding_window = mask,
            None => sliding_window = Box::new(Bigint::new(cfg.window_size)),
        }

        NoWrapReplayDetector {
            sliding_window,
            max_seq: cfg.max_seq,
            latest_seq: 0,
            window_size: cfg.window_size,
        }
    }

    pub fn check(&self, seq: usize) -> Result<()> {
        if seq > self.max_seq {
            return Err(ReplayError::OutsideWindow(seq));
        }
        if seq <= self.latest_seq {
            // seq is outside the lower end of the window
            if self.latest_seq >= self.window_size + seq {
                return Err(ReplayError::OutsideWindow(seq));
            }
            // seq is duplicated
            if self.sliding_window.bit(self.latest_seq - seq) {
                return Err(ReplayError::Duplicated(seq));
            }
        }
        Ok(())
    }
}

impl Checker for NoWrapReplayDetector {
    fn check_and_accept(&mut self, seq: usize) -> Result<bool> {
        self.check(seq)?;

        let mut latest = self.latest_seq == 0;
        // slide the window if a newer seq number arrived
        if seq > self.latest_seq {
            self.sliding_window.shl(seq - self.latest_seq);
            self.latest_seq = seq;
            latest = true;
        }
        let diff = self.latest_seq - seq;
        self.sliding_window.set_bit(diff);
        Ok(latest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    mod nowrap {
        use crate::dequeue::Dequeue;

        use super::*;

        #[test]
        fn happy_path() {
            let cfg = DetectorConfig {
                mask: None,
                max_seq: 128,
                window_size: 32,
            };

            let mut detector = NoWrapReplayDetector::new(cfg);
            for i in 0..128 {
                match detector.check_and_accept(i) {
                    Ok(latest) => {
                        assert!(latest);
                    }
                    Err(e) => {
                        assert!(false, "unexpected error {}", e.to_string())
                    }
                }
            }
        }

        #[test]
        fn duplicated_value() {
            let cfg = DetectorConfig {
                mask: None,
                max_seq: 128,
                window_size: 32,
            };

            let mut detector = NoWrapReplayDetector::new(cfg);
            detector.check_and_accept(10).unwrap();
            detector.check_and_accept(12).unwrap();
            match detector.check_and_accept(10) {
                Ok(_) => {
                    assert!(false, "expected error")
                }
                Err(e) => {
                    assert!(e == ReplayError::Duplicated(10));
                }
            }
        }

        #[test]
        fn seq_too_low() {
            let cfg = DetectorConfig {
                mask: None,
                max_seq: (1 << 32) - 1,
                window_size: 64,
            };
            let mut detector = NoWrapReplayDetector::new(cfg);
            detector.check_and_accept(1000).unwrap();
            assert!(detector.check_and_accept(1000 - 64).is_err());
        }

        #[test]
        fn big_valid_jump() {
            let cfg = DetectorConfig {
                mask: None,
                max_seq: (1 << 32) - 1,
                window_size: 0xFF,
            };
            let mut detector = NoWrapReplayDetector::new(cfg);
            detector.check_and_accept(1).unwrap();
            detector.check_and_accept(2).unwrap();
            detector.check_and_accept(3).unwrap();
            detector.check_and_accept(0xFF4).unwrap();
            assert!(detector.check_and_accept(4).is_err());
        }

        #[test]
        fn invalid_huge_jump() {
            let cfg = DetectorConfig {
                mask: None,
                max_seq: (1 << 32) - 1,
                window_size: 0xFF,
            };
            let mut detector = NoWrapReplayDetector::new(cfg);
            detector.check_and_accept(1).unwrap();
            detector.check_and_accept(2).unwrap();
            detector.check_and_accept(3).unwrap();
            assert!(detector.check_and_accept((1 << 33) - 1).is_err());
        }

        #[test]
        fn dequeue_invalid_huge_jump() {
            let cfg = DetectorConfig {
                mask: Option::Some(Box::new(Dequeue::new(0xFF))),
                max_seq: (1 << 32) - 1,
                window_size: 0xFF,
            };
            let mut detector = NoWrapReplayDetector::new(cfg);
            detector.check_and_accept(1).unwrap();
            detector.check_and_accept(2).unwrap();
            detector.check_and_accept(3).unwrap();
            assert!(detector.check_and_accept((1 << 33) - 1).is_err());
        }
    }
}
