use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    pub const ONE: Vec2 = Vec2 { x: 1.0, y: 1.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn normalized(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            Self::ZERO
        }
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn xy(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }
}

/// Address is a packed 32-bit integer encoding (x, y) coordinates.
/// x is stored in the upper 16 bits, y in the lower 16 bits.
/// Supports coordinates from -32768 to 32767.
pub type Address = i32;

pub fn encode_address(x: i32, y: i32) -> Address {
    (x << 16) | (y & 0xFFFF)
}

pub fn decode_address(addr: Address) -> (i32, i32) {
    let x = addr >> 16;
    let y = (addr << 16) >> 16;
    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_encoding() {
        assert_eq!(encode_address(0, 0), 0);
        assert_eq!(decode_address(0), (0, 0));

        let addr = encode_address(10, 20);
        assert_eq!(decode_address(addr), (10, 20));

        let addr = encode_address(-5, -10);
        assert_eq!(decode_address(addr), (-5, -10));

        let addr = encode_address(100, -50);
        assert_eq!(decode_address(addr), (100, -50));
    }
}
