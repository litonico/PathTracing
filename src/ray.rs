use ::vec3::Vec3;
use ::point3::Point3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(o: Point3, d: Vec3) -> Ray {
        Ray {origin: o, direction: d.normalize()}
    }

    pub fn position(self, distance: f64) -> Point3 {
        Point3 {
            x: self.origin.x + (self.direction.x * distance),
            y: self.origin.y + (self.direction.y * distance),
            z: self.origin.z + (self.direction.z * distance),
        }
    }
}
