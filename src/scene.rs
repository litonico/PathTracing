use ::math::{Point3, Vec3, Ray};
use ::geometry::Surface;
use ::intersection::Intersection;

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
    pub fov: u16,
    pub image_plane: ImagePlane,
}

impl Camera {
    pub fn look_at(&mut self, p:Point3) {
        self.focus_point = p;
    }

    pub fn move_to(&mut self, p:Point3) {
        self.position = p;
    }

    pub fn get_ray(&self, image_x:u32, image_y:u32) -> Ray {
        // TODO: FOV, however that works
        // TODO: Pixel Density, however that works
        // Get primary directions from the perpsective of the camera
        let forward : Vec3 = (self.focus_point - self.position).normalize();
        // TODO: I can never remember which sign this should have, so
        // check if the camera is upside-down
        let horizontal = Vec3 {x: -forward.z, y: 0.0, z: forward.x}.normalize();
        let vertical = forward.cross(horizontal);
        // Amount of offset from the left
        let x_offset = image_x as f64 / self.image_plane.width as f64;
        // Amount of offset from the top
        let y_offset = image_y as f64 / self.image_plane.height as f64;
        // Move the direction to point to the top-left of the image plane
        let top_left = forward -
                       horizontal.scale(image_x as f64 * x_offset / 2.0) +
                       vertical.scale(image_x as f64 * x_offset / 2.0);
        Ray {
            origin: self.position,
            direction: (top_left +
                        horizontal.scale(x_offset) -
                        vertical.scale(y_offset)).normalize()
        }
    }
}
#[test]
fn test_getting_a_ray_from_camera_oriented_x() {
    let image_plane = ImagePlane {
        width: 10,
        height: 10,
        pixel_density: 1,
    };
    let camera = Camera {
        position: Point3::new(0.0,0.0,0.0),
        focus_point: Point3::new(1.0,0.0,0.0),
        fov: 45,
        image_plane: image_plane,
    };
    // Get a ray from the center
    assert_eq!(
        camera.get_ray(5,5),
        Ray::new(Point3::new(0.0,0.0,0.0), Vec3::new(1.0,0.0,0.0))
    );

    // More thorough check
    assert_eq!(true, false)
}
//#[test]
fn test_getting_a_ray_from_camera_understands_pixel_density() {
    assert_eq!(true, false)
}
//#[test]
fn test_getting_a_ray_from_camera_understands_FOV() {
    assert_eq!(true, false)
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
