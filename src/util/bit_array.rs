use alloc::vec;
use alloc::vec::Vec;
use core::mem;

use lazy_static::lazy_static;

lazy_static! {
    static ref W_SIZE: usize = mem::size_of::<usize>() * 8;
    static ref W_WIDTH: usize = {
        let mut a = 0;
        let mut b = *W_SIZE;
        loop {
            match b {
                1 => return a,
                0 => panic!("invalid word size"),
                _ => a += 1,
            };
            b = b >> 1;
        }
    };
}

pub struct BitArray {
    inner: Vec<usize>,
    size: usize,
}

impl BitArray {
    pub fn new(s: usize) -> Self {
        BitArray {
            inner: vec![0 as usize; length(s)],
            size: s,
        }
    }

    pub fn set_on(&mut self, i: usize) {
        self.check_bound(i);
        let (a, b) = pos(i);
        self.inner[a] |= 1 << (b - 1);
    }

    pub fn set_off(&mut self, i: usize) {
        self.check_bound(i);
        let (a, b) = pos(i);
        self.inner[a] &= !(1 << (b - 1));
    }

    pub fn get(&self, i: usize) -> usize {
        self.check_bound(i);
        let (a, b) = pos(i);
        (self.inner[a] & (1 << (b - 1))) >> (b - 1)
    }

    fn check_bound(&self, i: usize) {
        if i > self.size {
            panic!("Index out bound");
        }
    }
}

fn pos(i: usize) -> (usize, usize) {
    (i >> *W_WIDTH - 1, i - (i >> *W_WIDTH << *W_WIDTH))
}

fn length(s: usize) -> usize {
    (s + *W_SIZE - 1) >> *W_WIDTH
}
