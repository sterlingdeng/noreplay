use crate::Mask;
use std::collections::VecDeque;

pub struct Dequeue {
    vec: VecDeque<usize>,
    size: usize,
}

impl Dequeue {
    pub fn new(size: usize) -> Self {
        let mut vec = VecDeque::with_capacity(size);
        vec.resize(size, 0);
        Dequeue { vec, size }
    }
}

impl Mask for Dequeue {
    fn bit(&self, n: usize) -> bool {
        if n > self.size {
            return false;
        }
        self.vec[self.size - n - 1] == 1
    }
    fn set_bit(&mut self, n: usize) {
        if n > self.size {
            return;
        }
        println!("{}, {}", n, self.size);
        self.vec[self.size - n - 1] = 1;
    }
    fn shl(&mut self, n: usize) {
        if n > self.size {
            self.vec.clear();
            return;
        }
        for _ in 0..n {
            self.vec.push_back(0);
        }
    }
}
