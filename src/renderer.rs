use ::scene::Scene;
use ::math::Ray;
use ::color::{Color, WHITE, BLACK};

pub struct Image {
    image: Vec<u8>,
    width: u32,
    height: u32,
}

fn render_ray(scene: &Scene, ray: &Ray, bounces: u8) -> Color {
    // TODO: for bounce in bounces...
    render_single_bounce(scene, ray)
}

fn render_single_bounce(scene: &Scene, ray: &Ray) -> Color {
    match scene.intersect(ray) {
        None => BLACK,
        Some(intersection) => WHITE, // TODO
    }
}

// pub fn render(scene:Scene) -> Image {
//     let p = scene.camera.ImagePlane
//     for x in 0..p.width {
//         for y in 0..p.height {
//             let ray = scene.camera.get_ray(x, y);
//             render_ray(ray);
//         }
//     }
// }
