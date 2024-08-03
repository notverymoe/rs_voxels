// Copyright 2024 Natalie Baker // AGPLv3 //

use core::ops::{BitAnd, BitOr, BitXor, Not};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitPlane(u64);

impl BitOr for BitPlane {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitAnd for BitPlane {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitXor for BitPlane {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl Not for BitPlane {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl BitPlane {
    #[must_use]
    pub const fn to_raw(self) -> u64 {
        self.0
    }
}

impl BitPlane {
    pub fn push(&mut self) {
        self.0 = (self.0 << 1) | 1;
    }

    pub fn skip(&mut self) {
        self.0 <<= 1;
    }
}

impl BitPlane {

    pub const DEFAULT: Self = Self(0);

    pub fn set(&mut self, idx: u32, state: bool) {
        debug_assert!(Self::is_idx_valid(idx));
        if state {
            self.mark(idx);
        } else {
            self.clear(idx);
        }
    }

    pub fn mark(&mut self, idx: u32) {
        debug_assert!(Self::is_idx_valid(idx));
        self.0 |= 1 << idx;
    }

    pub fn clear(&mut self, idx: u32) {
        debug_assert!(Self::is_idx_valid(idx));
        self.0 &= !(1 << idx);
    }

    #[must_use]
    pub fn get(self, idx: u32) -> bool {
        debug_assert!(Self::is_idx_valid(idx));
        (self.0 & (1 << idx)) != 0
    }

}

impl BitPlane {
    pub fn mark_at(&mut self, x: u32, y: u32) {
        self.mark(Self::idx_from_pos(x, y));
    }

    pub fn clear_at(&mut self, x: u32, y: u32) {
        self.clear(Self::idx_from_pos(x, y));
    }

    #[must_use]
    pub fn get_at(self, x: u32, y: u32) -> bool {
        self.get(Self::idx_from_pos(x, y))
    }
}

impl BitPlane {
    #[must_use]
    pub const fn idx_from_pos(x: u32, y: u32) -> u32 {
        x | (y << 3)
    }

    #[must_use]
    pub const fn pos_from_idx(idx: u32) -> [u32; 2] {
        [idx & 0x07, idx >> 3]
    }

    #[must_use]
    pub const fn is_idx_valid(idx: u32) -> bool {
        idx < 64
    }
}