type Float = f64;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Vec3 {
    fn new(x: Float, y: Float, z: Float) -> Vec3 {
        return Vec3 { x, y, z };
    }

    fn zero() -> Vec3 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    #[inline]
    fn length(&self) -> Float {
        return self.length_squared().sqrt();
    }

    #[inline]
    fn length_squared(&self) -> Float {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    #[inline]
    fn dot(&self, other: &Vec3) -> Float {
        //
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    #[inline]
    fn unit(&self) -> Vec3 {
        *self / self.length()
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = Float;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Out of bounds on vec3!"),
        }
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Out of bounds on vec3!"),
        }
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhz: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x + rhz.x,
            y: self.y + rhz.y,
            z: self.z + rhz.z,
        };
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhz: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x - rhz.x,
            y: self.y - rhz.y,
            z: self.z - rhz.z,
        };
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl std::ops::Mul<Float> for Vec3 {
    type Output = Vec3;
    fn mul(self, scalar: Float) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl std::ops::Mul<Vec3> for Float {
    type Output = Vec3;
    fn mul(self, scalar: Vec3) -> Vec3 {
        scalar * self
    }
}

impl std::ops::Div for Vec3 {
    type Output = Vec3;
    fn div(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl std::ops::Div<Float> for Vec3 {
    type Output = Vec3;
    fn div(self, scalar: Float) -> Vec3 {
        self * (1.0 / scalar)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        return Vec3::new(-self.x, -self.y, -self.z);
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::MulAssign<Float> for Vec3 {
    fn mul_assign(&mut self, scalar: Float) {
        *self = Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl std::ops::DivAssign<Float> for Vec3 {
    fn div_assign(&mut self, scalar: Float) {
        *self *= 1.0 / scalar
    }
}

#[test]
fn test_vectors_addition() {
    // Addition with copy.

    let a = Vec3::new(1.0, 10.0, 100.0);
    let b = Vec3::new(2.0, 3.0, 4.0);

    let c = a + b;

    assert_eq!(c, Vec3::new(3.0, 13.0, 104.0));

    // Addition with mutation.

    let mut base = Vec3::zero();
    assert_eq!(base, Vec3::new(0.0, 0.0, 0.0));

    base += b;
    assert_eq!(base, Vec3::new(2.0, 3.0, 4.0));

    base += b;
    assert_eq!(base, Vec3::new(4.0, 6.0, 8.0));
}
