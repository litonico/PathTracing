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

#[derive(Debug)]
pub struct ImagePlane {
    pub width: u32,  // px
    pub height: u32, // px
    pub pixel_density: u16, // dpi
}


#[derive(Debug)]
pub struct Camera {
    pub position: Point3,
    pub focus_point: Point3,
    pub fov: u16, // TODO: Make SURE (with the type system?) this is in degrees
    pub image_plane: ImagePlane,
}

impl Camera {
    pub fn look_at(&mut self, p:Point3) {
        self.focus_point = p;
    }

    pub fn move_to(&mut self, p:Point3) {
        self.position = p;
    }

    // TODO(Lito): Can move a lot of this out into a recompute() function
    // called on each move_to or look_at
    pub fn get_ray(&self, image_x:u32, image_y:u32) -> Ray {
        let p = &self.image_plane;
        let image_width = p.width as f64 * p.pixel_density as f64;
        let image_height = p.height as f64 * p.pixel_density as f64;
        // Get primary directions from the perpsective of the camera
        let forward : Vec3 = (self.focus_point - self.position).normalize();
        let horizontal = Vec3 {x: forward.z, y: 0.0, z: -forward.x}.normalize();
        let vertical = forward.cross(horizontal);
        // Amount of offset from the left
        let x_offset = 1.0 / image_width;
        // Amount of offset from the top
        let y_offset = 1.0 / image_width;
        // With a wider FOV, the image is closer to the camera
        let distance_to_image = 1.0 / (self.fov as f64 * 0.5).to_radians().tan();
        // Move the direction to point to the top-left of the image plane
        let top_left = forward.scale(distance_to_image) -
                       horizontal.scale(image_width*x_offset/2.0) +
                       vertical.scale(image_height*y_offset/2.0);
        // Amount of offset from the top
        Ray {
            origin: self.position,
            direction: (top_left +
                        horizontal.scale(image_x as f64 * x_offset) -
                        vertical.scale(image_y as f64 * y_offset)).normalize()
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

macro_rules! assert_ray_approx_eq(
    ($given: expr, $expected: expr) => ({
        let (given_val, expected_val) = (&($given), &($expected));
        let eps = 0.000001;
        if given_val.origin.x - expected_val.origin.x > eps ||
           given_val.origin.y - expected_val.origin.y > eps ||
           given_val.origin.z - expected_val.origin.z > eps ||
           given_val.direction.x - expected_val.direction.x > eps ||
           given_val.direction.y - expected_val.direction.y > eps ||
           given_val.direction.z - expected_val.direction.z > eps {
           panic!("assertion failed: `left ≈ right` \
                   (left: `{:?}`, right: `{:?}`, tolerance: `{:?}`)",
                *given_val, *expected_val, eps
            )
        }
    })
);

#[test]
fn test_getting_a_ray_from_camera_oriented_x() {
    let image_plane = ImagePlane {
        width: 2,
        height: 2,
        pixel_density: 1,
    };
    let camera = Camera {
        position: Point3::new(0.0,0.0,0.0),
        focus_point: Point3::new(1.0,0.0,0.0),
        fov: 90,
        image_plane: image_plane,
    };
    // Get a ray from the center
    assert_ray_approx_eq!(
        camera.get_ray(0,0),
        Ray::new(Point3::new(0.0,0.0,0.0), Vec3::new(1.0,0.5,0.5).normalize())
    );
    assert_ray_approx_eq!(
        camera.get_ray(1,1),
        Ray::new(Point3::new(0.0,0.0,0.0), Vec3::new(1.0,0.0,0.0))
    );
    assert_ray_approx_eq!(
        camera.get_ray(2,2),
        Ray::new(Point3::new(0.0,0.0,0.0), Vec3::new(1.0,-0.5,-0.5).normalize())
    );
}
// #[test] TODO(Lito)
fn test_getting_a_ray_from_camera_oriented_xz() {
    let image_plane = ImagePlane {
        width: 2,
        height: 2,
        pixel_density: 1,
    };
    let camera = Camera {
        position: Point3::new(0.0,0.0,0.0),
        focus_point: Point3::new(1.0,0.0,1.0),
        fov: 90,
        image_plane: image_plane,
    };
    // Get a ray from the center
    assert_ray_approx_eq!(
        camera.get_ray(0,0),
        Ray::new(Point3::new(0.0,0.0,0.0), Vec3::new(1.0,0.5,0.5).normalize())
    );
    assert_ray_approx_eq!(
        camera.get_ray(1,1),
        Ray::new(Point3::new(0.0,0.0,0.0), Vec3::new(1.0,0.0,0.0))
    );
    assert_ray_approx_eq!(
        camera.get_ray(2,2),
        Ray::new(Point3::new(0.0,0.0,0.0), Vec3::new(1.0,-0.5,-0.5).normalize())
    );
}

#[test]
fn test_getting_a_ray_from_camera_understands_FOV() {
    let image_plane = ImagePlane {
        width: 2,
        height: 2,
        pixel_density: 1,
    };
    let camera = Camera {
        position: Point3::new(0.0,0.0,0.0),
        focus_point: Point3::new(1.0,0.0,0.0),
        fov: 45,
        image_plane: image_plane,
    };
    // Get a ray from the center
    assert_ray_approx_eq!(
        camera.get_ray(0,0),
        Ray::new(Point3::new(0.0,0.0,0.0), Vec3::new(sqrt(2.0),0.5,0.5).normalize())
    );
    assert_ray_approx_eq!(
        camera.get_ray(2,2),
        Ray::new(Point3::new(0.0,0.0,0.0), Vec3::new(sqrt(2.0),-0.5,-0.5).normalize())
    );
}

//#[test]
fn test_getting_a_ray_from_camera_understands_pixel_density() {
    let image_plane = ImagePlane {
        width: 2,
        height: 2,
        pixel_density: 2,
    };
    let camera = Camera {
        position: Point3::new(0.0,0.0,0.0),
        focus_point: Point3::new(1.0,0.0,0.0),
        fov: 45,
        image_plane: image_plane,
    };
    // Get a ray from the center
    assert_eq!(
        camera.get_ray(0,0),
        Ray::new(Point3::new(0.0,0.0,0.0), Vec3::new(1.0,0.5,0.5).normalize())
    );
    assert_eq!(
        camera.get_ray(1,1),
        Ray::new(Point3::new(0.0,0.0,0.0), Vec3::new(1.0,0.0,0.0))
    );
    assert_eq!(
        camera.get_ray(2,2),
        Ray::new(Point3::new(0.0,0.0,0.0), Vec3::new(1.0,-0.5,-0.5).normalize())
    );
}


