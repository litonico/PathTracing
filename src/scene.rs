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
    pub fov: u8,
    pub image_plane: ImagePlane,
}

impl Camera {
    pub fn look_at(&mut self, p:Point3) {
        self.focus_point = p;
    }

    pub fn move_to(&mut self, p:Point3) {
        self.position = p;
    }

    pub fn get_ray(&self, x:u32, y:u32) -> Ray {
        // TODO: FOV, however that works
        // Get primary directions from the perpsective of the camera
        let forward : Vec3 = (self.focus_point - self.position).normalize();
        // TODO: I can never remember which sign this should have.
        // Check if the camera is upside-down:
        let horizontal = Vec3 {x: -forward.z, y: 0.0, z: forward.x}.normalize();
        let vertical = forward.cross(horizontal);
        // Move the direction to point to the top-left of the iamge plane
        let dir = dir_center; // TODO
        // Amount of offset from the left
        let x_offset = x as f64 / self.imagePlane.width as f64
        // Amount of offset from the top
        let y_offset = y as f64 / self.imagePlane.height as f64

        Ray {
            origin: self.position,
            direction: self.dir + horizontal.scale(x) + vertical.scale(y)
        }
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
