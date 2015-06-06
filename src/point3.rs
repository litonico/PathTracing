use std::ops::{Sub};
use ::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Sub for Point3 {
    type Output = Vec3;
    fn sub(self, other: Point3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Point3 {
    pub fn new(x:f64, y:f64, z:f64) -> Vec3 {
        Vec3{x:x, y:y, z:z}
    }
}
