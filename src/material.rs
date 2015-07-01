use ::math::Ray;
use ::intersection::Intersection;

pub trait Material {
    fn reflect(incoming_ray: Ray, intersection: Intersection) -> Ray;
}

// TODO:
// pub trait Emitter {
//     fn emit() -> f32;
// }

