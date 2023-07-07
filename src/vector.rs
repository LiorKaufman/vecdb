use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl fmt::Display for Vector3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
impl Vector3D {
    // Addition
    pub fn add(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    // Subtraction
    pub fn subtract(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    // Scalar multiplication
    pub fn multiply_scalar(&self, scalar: f64) -> Vector3D {
        Vector3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    // Dot product
    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // Cross product
    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    // Magnitude
    pub fn magnitude(&self) -> f64 {
        (self.x.exp2() + self.y.exp2() + self.z.exp2()).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vector3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vector3D {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let result = v1.add(&v2);
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, 9.0);
    }

    #[test]
    fn test_subtract() {
        let v1 = Vector3D {
            x: 5.0,
            y: 7.0,
            z: 9.0,
        };
        let v2 = Vector3D {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let result = v1.subtract(&v2);
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 3.0);
    }

    #[test]
    fn test_multiply_scalar() {
        let v1 = Vector3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let result = v1.multiply_scalar(2.0);
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 4.0);
        assert_eq!(result.z, 6.0);
    }

    #[test]
    fn test_dot() {
        let v1 = Vector3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vector3D {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let result = v1.dot(&v2);
        assert_eq!(result, 32.0);
    }
    #[test]
    fn test_cross() {
        let v1 = Vector3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vector3D {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let result = v1.cross(&v2);
        assert_eq!(result.x, -3.0);
        assert_eq!(result.y, 6.0);
        assert_eq!(result.z, -3.0);
    }
}
