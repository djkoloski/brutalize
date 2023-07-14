use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const RIGHT: Self = Self { x: 1, y: 0 };
    pub const LEFT: Self = Self { x: -1, y: 0 };
    pub const UP: Self = Self { x: 0, y: 1 };
    pub const DOWN: Self = Self { x: 0, y: -1 };

    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn abs(self) -> Self {
        Self { x: self.x.abs(), y: self.y.abs() }
    }
}

impl Add for Vec2 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul<i32> for Vec2 {
    type Output = Self;

    #[inline]
    fn mul(self, other: i32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl MulAssign<i32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, other: i32) {
        *self = *self * other;
    }
}
