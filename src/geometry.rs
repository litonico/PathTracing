use std::option;
use ::intersection::Intersection;
use ::math::{Ray, Point3};


pub trait Surface {
    fn intersect(&self, ray: Ray) -> Option<Intersection>;
}

pub struct Sphere {
    pub position: Point3,
    pub radius: f64,
}

// impl Surface for Sphere {
//     fn intersect(&self, ray: Ray) -> Option<Intersection> { }
// }
