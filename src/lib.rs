// Traits? Swizzels
use std::ops::{Mul,Div,Add,Sub};

macro_rules! vectors {
    ($Vec:ident, $dim:expr, $(($axis_fn:ident, $axis:ident => $index:expr)),*) => {
        #[derive(PartialEq, Debug)]
        pub struct $Vec([f32;$dim]);
        impl $Vec {
            $(
                pub fn $axis(&self) -> f32 {
                    self.0[$index]
                }
                pub fn $axis_fn() -> Self {
                    let mut arr = [0.0; $dim];
                    arr[$index] = 1.0;
                    $Vec(arr)
                }
            )*
            pub fn new() -> Self {
                $Vec([0.0; $dim])
            }
            pub fn normalise(&self) -> Self {
                let magnitude = self.magnitude();
                // chose not to panic, a normalised empty vector is just an empty vector
                if magnitude == 0.0 {
                    return Self::new();
                }
                
                self.scalar_mult(1.0/magnitude)
            }
            fn scalar_mult(&self, scale: f32) -> Self {
                $Vec(self.0.map(|val| val * scale))
            }
            fn scalar_add(&self, num: f32) -> Self {
                $Vec(self.0.map(|val| val + num))
            }
            fn scalar_div(&self, scale: f32) -> Self {
                $Vec(self.0.map(|val| val / scale))
            }
            fn scalar_sub(&self, num: f32) -> Self {
                $Vec(self.0.map(|val| val - num))
            }
            pub fn dot(&self, other: &Self) -> f32 {
                self.0.iter().zip(other.0.iter()).map(|(a, b)| a * b).sum()
            }
            pub fn magnitude(&self) -> f32 {
                self.0.iter().fold(0.0, |sum, val| sum + val * val).sqrt()
            }
            pub fn cos(&self, other: &Self) -> f32 {
                // chose to panic since there is no meaning in cos(angle) of a vector with zero magnitude
                assert!(self.magnitude() != 0.0 && other.magnitude() != 0.0, "Magnitude of one of the vectors is zero");
                self.dot(other)/(self.magnitude()*other.magnitude())
            }
        }
        impl Mul<f32> for $Vec {
            type Output = $Vec;
            fn mul(self, scale: f32) -> Self::Output {
               self.scalar_mult(scale)
            }
        }
        impl Mul<$Vec> for f32 {
            type Output = $Vec;
            fn mul(self, vector: $Vec) -> Self::Output {
                vector.scalar_mult(self)
            }
        }
        impl Add<f32> for $Vec {
            type Output = $Vec;
            fn add(self, num: f32) -> Self::Output {
                self.scalar_add(num)
            }
        }
        impl Add<$Vec> for f32 {
            type Output = $Vec;
            fn add(self, vector: $Vec) -> Self::Output {
                vector.scalar_add(self)
            }
        }
        impl Div<f32> for $Vec {
            type Output = $Vec;
            fn div(self, num: f32) -> Self::Output {
                self.scalar_div(num)
            }
        }
        impl Sub<f32> for $Vec {
            type Output = $Vec;
            fn sub(self, num: f32) -> Self::Output {
                self.scalar_sub(num)
            }
        }
    };
}

vectors!(Vec2, 2, (x_axis, x => 0), (y_axis, y => 1));
vectors!(Vec3, 3, (x_axis, x => 0), (y_axis, y => 1), (z_axis, z => 2));
vectors!(Vec4, 4, (x_axis, x => 0), (y_axis, y => 1), (z_axis, z => 2), (w_axis, w => 3));

impl Vec3 {
    pub fn cross(&self, other: &Self) -> Self {
        Vec3([
            self.0[1] * other.0[2] - self.0[2] * other.0[1],
            other.0[0] * self.0[2] - self.0[0] * other.0[2],
            self.0[0] * other.0[1] - other.0[0] * self.0[1],
        ])
    }
    pub fn sin(&self, other: &Self) -> f32 {
        // chose to panic since there is no meaning in sin(angle) of a vector with zero magnitude
        assert!(self.magnitude() != 0.0 && other.magnitude() != 0.0, "Magnitude of one of the vectors is zero");
        self.cross(other).magnitude()/(self.magnitude()*other.magnitude())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_axis_identifier() {
        let v = Vec4([1.0, 2.0, 3.0, 4.0]);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
        assert_eq!(v.w(), 4.0);
    }
    #[test]
    fn vector_axis() {
        assert_eq!(Vec4([1.0, 0.0, 0.0, 0.0]), Vec4::x_axis());
        assert_eq!(Vec4([0.0, 1.0, 0.0, 0.0]), Vec4::y_axis());
        assert_eq!(Vec4([0.0, 0.0, 1.0, 0.0]), Vec4::z_axis());
        assert_eq!(Vec4([0.0, 0.0, 0.0, 1.0]), Vec4::w_axis());
    }
    #[test]
    fn vector_normalise() {
        assert_eq!(Vec2([3.0, 4.0]).normalise(), Vec2([0.6, 0.8]));
        assert_eq!(Vec2::new().normalise(), Vec2::new());
    }
    #[test]
    fn vector_scalar_mult() {
        assert_eq!(Vec2([3.0, 4.0]) * 2.0, Vec2([6.0, 8.0]));
        assert_eq!(2.0 * Vec2([3.0, 4.0]), Vec2([6.0, 8.0]));
    }
    #[test]
    fn vector_scalar_div() {
        assert_eq!(Vec2([2.0, 3.0]) / 2.0, Vec2([1.0, 1.5]));
    }
    #[test]
    fn vector_scalar_add() {
        assert_eq!(Vec2([3.0, 4.0]) + 2.0, Vec2([5.0, 6.0]));
        assert_eq!(2.0 + Vec2([3.0, 4.0]), Vec2([5.0, 6.0]));
    }
    #[test]
    fn vector_scalar_sub() {
        assert_eq!(Vec2([3.0, 4.0]) - 2.0, Vec2([1.0, 2.0]));
    }
    #[test]
    fn vector_magnitude() {
        assert_eq!(Vec2([3.0, 4.0]).magnitude(), 5.0);
    }
    #[test]
    fn vector_dot() {
        let v = Vec4([1.0, 2.0, 3.0, 4.0]);
        let v2 = Vec4([4.0, 3.0, 2.0, 1.0]);
        assert_eq!(v.dot(&v2), 20.0);
    }
    #[test]
    fn vector_cross() {
        let v = Vec3([1.0, 2.0, 3.0]);
        let v2 = Vec3([3.0, 2.0, 1.0]);
        assert_eq!(v.cross(&v2), Vec3([-4.0, 8.0, -4.0]));
    }
    #[test]
    fn vector_cos() {
        let v = Vec3([1.0, 2.0, 3.0]);
        let v2 = Vec3([3.0, 2.0, 1.0]);
        assert_eq!(v.cos(&v2), 0.7142857)
    }
    #[test]
    #[should_panic = "Magnitude of one of the vectors is zero"]
    fn vector_cos_zero() {
        let v = Vec3([0.0; 3]);
        let v2 = Vec3([3.0, 2.0, 1.0]);
        v.cos(&v2);
    }
    #[test]
    fn vector_sin() {
        let v = Vec3([1.0, 2.0, 3.0]);
        let v2 = Vec3([3.0, 2.0, 1.0]);
        assert_eq!(v.sin(&v2), 0.6998542)
    }
    #[test]
    #[should_panic = "Magnitude of one of the vectors is zero"]
    fn vector_sin_zero() {
        let v = Vec3([0.0; 3]);
        let v2 = Vec3([3.0, 2.0, 1.0]);
        v.sin(&v2);
    }
}
