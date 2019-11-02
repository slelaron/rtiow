use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn squared_length(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn dot(&self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(mut self, rhs: Vec3) -> Vec3 {
        self += rhs;
        self
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(mut self, rhs: Vec3) -> Vec3 {
        self -= rhs;
        self
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(mut self, rhs: Vec3) -> Vec3 {
        self *= rhs;
        self
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(mut self, rhs: f32) -> Vec3 {
        self *= rhs;
        self
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(mut self, rhs: f32) -> Vec3 {
        self /= rhs;
        self
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(0., 0., 0.) - self
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3;
    use vec3::Vec3;

    #[test]
    fn test_cross_product() {
        let i = Vec3::new(1.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 1.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 1.0);

        assert_eq!(i.cross(j), k);
        assert_eq!(j.cross(k), i);
        assert_eq!(k.cross(i), j);

        assert_eq!(j.cross(i), -k);
        assert_eq!(k.cross(j), -i);
        assert_eq!(i.cross(k), -j);

        assert_eq!((i * 2.0).cross(j * 2.0), k * 4.0);
        assert_eq!((j * 2.0).cross(k * 2.0), i * 4.0);
        assert_eq!((k * 2.0).cross(i * 2.0), j * 4.0);
    }

    #[test]
    fn test_dot_product() {
        let i = Vec3::new(1.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 1.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 1.0);

        assert_eq!(i.dot(i * 4.0 + j * 2.0 + k), 4.0);
        assert_eq!(i.dot(i * (-4.0) + j * 10.0 + k), -4.0);
        assert_eq!((i + j + k).dot(i + j + k), 3.0);
        assert_eq!((i + j - k * 2.0).dot(i + j + k), 0.0);
    }

    #[test]
    fn test_arithmetic() {
        let i = Vec3::new(1.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 1.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 1.0);

        assert_eq!(-(-i), i);

        assert_eq!(i + j, j + i);
        assert_eq!(i + k, k + i);
        assert_eq!(k + j, j + k);

        assert_eq!(i - j, -(j - i));
        assert_eq!(i - k, -(k - i));
        assert_eq!(k - j, -(j - k));

        assert_eq!((i + j) + k, i + (j + k));
        assert_eq!(i / 2.0 * 2.0, i * 2.0 / 2.0);
        assert_eq!(i * 0.5 * 2.0, i * 0.5 * 2.0);
    }
}
