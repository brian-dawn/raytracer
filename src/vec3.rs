use crate::utils::{random, random_range};
type Float = f64;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

pub type Color = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: Float, y: Float, z: Float) -> Vec3 {
        return Vec3 { x, y, z };
    }

    pub fn zero() -> Vec3 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    pub fn ones() -> Vec3 {
        return Vec3::new(1.0, 1.0, 1.0);
    }

    pub fn random() -> Vec3 {
        let x = random();
        let y = random();
        let z = random();

        return Vec3::new(x, y, z);
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        let x = random_range(min, max);
        let y = random_range(min, max);
        let z = random_range(min, max);

        return Vec3::new(x, y, z);
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        return loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                break p;
            }
        };
    }

    pub fn random_unit_vector() -> Vec3 {
        let a = random_range(0.0, 2.0 * std::f64::consts::PI);
        let z = random_range(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Vec3::new(r * a.cos(), r * a.sin(), z)
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(&normal) > 0.0 {
            // In the same hemisphere as the normal.
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        return *self - 2.0 * self.dot(normal) * *normal;
    }

    #[inline]
    /// I would have called this `magnitude` but this is what the book
    /// I am following is using.
    pub fn length(&self) -> Float {
        return self.length_squared().sqrt();
    }

    #[inline]
    pub fn length_squared(&self) -> Float {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    #[inline]
    pub fn dot(&self, other: &Vec3) -> Float {
        //
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    #[inline]
    /// I would have called this `normalize` but this is what the book
    /// I am following is using.
    pub fn unit(&self) -> Vec3 {
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

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
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

    base -= b;
    assert_eq!(base, Vec3::new(2.0, 3.0, 4.0));
}

#[test]
fn test_vectors_multiplication() {
    let a = Vec3::new(1.0, 10.0, 100.0);
    let b = Vec3::new(2.0, 3.0, 4.0);

    let c = a * b;

    assert_eq!(c, Vec3::new(2.0, 30.0, 400.0));

    let d = c / b;

    assert_eq!(d, a);

    let mut base = Vec3::new(1.0, 2.0, 3.0);
    base *= 2.0;

    assert_eq!(base, Vec3::new(2.0, 4.0, 6.0));

    base /= 2.0;

    assert_eq!(base, Vec3::new(1.0, 2.0, 3.0));
}

#[test]
fn test_vectors_dot_product() {
    let a = Vec3::new(2.0, 3.0, 4.0);
    let b = Vec3::new(5.0, 6.0, 7.0);
    assert_eq!(a.dot(&b), 56.0);
}

#[test]
fn test_vectors_cross_product() {
    let a = Vec3::new(2.0, 3.0, 4.0);
    let b = Vec3::new(5.0, 6.0, 7.0);
    assert_eq!(a.cross(&b), Vec3::new(-3.0, 6.0, -3.0));
}

#[test]
fn test_vectors_unit() {
    let a = Vec3::new(1.0, 2.0, 3.0).unit();
    assert_eq!(a.length(), 1.0);
    assert_eq!(
        a,
        Vec3::new(0.2672612419124244, 0.5345224838248488, 0.8017837257372732)
    );
}

#[test]
fn test_index() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(1.0, a[0]);
    assert_eq!(2.0, a[1]);
    assert_eq!(3.0, a[2]);

    let mut b = a;
    b[0] = 9.0;
    b[1] = 10.0;
    b[2] = 11.0;
    assert_eq!(b, Vec3::new(9.0, 10.0, 11.0));
}

#[test]
#[should_panic]
fn test_index_out_of_bounds() {
    let a = Vec3::zero();
    let _ = a[4];
}

#[test]
fn test_vectors_length() {
    let a = Vec3::new(5.0, 0.0, 0.0);
    assert_eq!(a.length(), 5.0);

    let a = Vec3::new(0.0, 6.0, 0.0);
    assert_eq!(a.length(), 6.0);

    let a = Vec3::new(0.0, 0.0, 7.0);
    assert_eq!(a.length(), 7.0);

    let a = Vec3::new(2.0, 1.0, 2.0);
    assert_eq!(a.length(), 3.0);
}
