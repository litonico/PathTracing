use std::ops::{Add,Sub,Mul};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Vec3 {
    pub fn new(x:f64, y:f64, z:f64) -> Vec3 {
        Vec3{x:x, y:y, z:z}
    }

    // Fancy vectors-as-zipWithable: I can't be bothered
    // pub fn zip_with(self, other: Vec3, op: |f64, f64| -> f64) -> Vec3 {
    //     Vec3 {
    //         x: op(self.x, other.x),
    //         y: op(self.y, other.y),
    //         z: op(self.y, other.z),
    //     }
    // }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z
    }
    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y*other.z - self.z*other.y,
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x,
        }
    }
    pub fn scale(self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn magnitude(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        let magnitude = self.magnitude();
        if magnitude == 0.0 {
            self
        } else {
            Vec3 {
                x: self.x/magnitude,
                y: self.y/magnitude,
                z: self.z/magnitude,
            }
        }
    }
}
