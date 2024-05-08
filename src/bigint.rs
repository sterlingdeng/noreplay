use core::fmt;
use std::fmt::Write;

use crate::Mask;

pub struct Bigint {
    /// sz is the integer size.
    sz: usize,
    /// segments is the representation of the integer with size sz.
    /// The most significant bits are positioned to the left of the vector
    /// and least significant bits are at the end of the vector.
    segments: Vec<usize>,
    /// msb_mask is the most significant bits mask.
    msb_mask: usize,
}

impl Bigint {
    pub fn new(sz: usize) -> Self {
        if sz == 0 {
            panic!("size must be greater than 0")
        }

        // 1 << (64 - sz %64) - 1
        let msb_mask = 1_usize
            .checked_shl((64 - sz % 64) as u32)
            .unwrap_or(0)
            .wrapping_sub(1);

        Bigint {
            sz,
            segments: vec![0; (sz + 63) / 64],
            msb_mask,
        }
    }
}

impl Mask for Bigint {
    /// bit returns 1 if the bit at position n is 1, 0 otherwise.
    fn bit(&self, n: usize) -> bool {
        if n > self.sz {
            return false;
        }
        let segment: usize = self.segments.len() - (n / 64) - 1;
        let pos = n % 64;

        self.segments[segment] & (1 << pos) != 0
    }

    /// set_bit sets the bit at position n to 1
    fn set_bit(&mut self, n: usize) {
        if n > self.sz {
            return;
        }
        let i: usize = self.segments.len() - (n / 64) - 1;
        let pos = n % 64;
        self.segments[i] |= 1 << pos;
    }

    /// shl performs the left shift operation
    fn shl(&mut self, n: usize) {
        if n == 0 {
            return;
        }
        let len = self.segments.len();
        let pos = n % 64;
        let seg = n / 64;

        for i in 0..len {
            let mut carry: usize = 0;
            if i + seg < len {
                carry |= self.segments[i + seg] << pos
            }
            // if there is a valid segment to the right, we right shift to remove all the bits we
            // don't care about.
            if i + seg + 1 < len {
                carry |= self.segments[i + seg + 1] >> (64 - pos)
            }
            self.segments[i] = self.segments[i] << pos | carry
        }

        // mask any bits that shifted outside the boundary
        self.segments[0] &= self.msb_mask
    }
}

impl fmt::Display for Bigint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for seg in self.segments.clone() {
            s.write_str(format!("{:#016X} ", seg).as_str())?;
        }
        f.write_str(s.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn large_int() {
        let mut bi = Bigint::new(2048);
        bi.set_bit(2000);
        bi.set_bit(1000);

        assert!(bi.bit(2000));
        assert!(bi.bit(1000));

        bi.shl(25);

        assert!(bi.bit(2025));
        assert!(bi.bit(1025));
    }

    #[test]
    fn shift_outside() {
        let mut bi = Bigint::new(4);
        bi.set_bit(3);
        bi.shl(1);
        assert!(bi.bit(4));
        bi.shl(1);
        assert!(!bi.bit(4));
    }

    #[test]
    fn shifting_through_segments() {
        let mut bi = Bigint::new(130);
        bi.set_bit(63);
        assert!(bi.bit(63));
        bi.shl(1);
        assert!(bi.bit(64));

        bi.set_bit(127);
        bi.shl(1);
        assert!(bi.bit(128));
        bi.shl(1);
        assert!(bi.bit(129));
    }

    #[test]
    fn bits_outside_range() {
        let mut bi = Bigint::new(4);
        bi.set_bit(5);
        for i in 0..5 {
            assert!(!bi.bit(i));
        }
    }
}
