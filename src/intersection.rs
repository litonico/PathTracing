use ::math::{Vec3, Point3};

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub distance: f64,
    pub point: Point3,
    pub normal: Vec3,
}
