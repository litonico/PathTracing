pub mod math;
use math::Point3;
pub mod intersection;
pub mod geometry;
use geometry::Sphere;
pub mod material;
pub mod scene;
use scene::{Scene, Object, Camera, ImagePlane};
pub mod renderer;
pub mod color;

fn init_test_scene () -> Scene {
    let mut objs = Vec::new();


    let origin = Point3::new(0.0,0.0,0.0);
    let sphere_surface = Box::new(Sphere::new(origin, 1.0));
    let sphere = Object::new(sphere_surface);
    objs.push(sphere);

    let camera = Camera {
        position: origin,
        focus_point: Point3::new(1.0,0.0,0.0),
        image_plane: ImagePlane {
            width: 500,
            height: 500,
            pixel_density: 500,
        },
    };

    Scene {
        objects: objs,
        camera: camera,
    }
}

fn main() {
    let scene = init_test_scene();
    //let image = Renderer::render(scene);
}
