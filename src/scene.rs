use ::math::{Point3, Ray};
use ::geometry::Surface;
use ::color::Color;
use ::intersection::Intersection;
use ::material::Material;

pub struct Object {
    surface: Box<Surface>,
    // TODO: material: Box<Material>,
}

impl Object {
    pub fn new(s: Box<Surface>) -> Object {
        Object {
            surface: s
        }
    }

    pub fn intersect(&self, ray:&Ray) -> Option<Intersection>{
        self.surface.intersect(ray)
    }
}

pub struct ImagePlane {
    pub width: u32,  // px
    pub height: u32, // px
    pub pixel_density: u32, // dpi
}


pub struct Camera {
    pub position: Point3,
    pub focus_point: Point3,
    pub image_plane: ImagePlane,
}

impl Camera {
    pub fn look_at(&mut self, p:Point3) {
        self.focus_point = p;
    }

    pub fn move_to(&mut self, p:Point3) {
        self.position = p;
    }
}

pub struct Scene {
    pub objects: Vec<Object>,
    pub camera: Camera,
    // TODO: pub background_color: Color,
}

impl Scene {
    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // Find the closest intersection of the ray with the scene
        self.objects.iter().fold(None, |so_far, obj| -> Option<Intersection> {
            let candidate = obj.intersect(ray);
            // Compare the closest intersection so far against the intersection
            // with the current object
            match (so_far, candidate) {
                (None, None) => None,
                (None, Some(new)) => Some(new),
                (Some(old), None) => Some(old),
                (Some(old), Some(new)) => {
                    if new.distance < old.distance {
                        Some(new)
                    } else {
                        Some(old)
                    }
                }
            }
        })
    }
}
