use ::scene::Scene;
use ::math::Ray;
use ::color::{Color, WHITE, BLACK};

pub struct Image {
    image: Vec<u8>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let ix = x as usize;
        let iy = y as usize;
        let w = self.width as usize * 3;
        self.image[ix*3 + iy*w + 0] = color.r;
        self.image[ix*3 + iy*w + 1] = color.g;
        self.image[ix*3 + iy*w + 2] = color.b;
    }
    // TODO: pub fn save
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

pub fn render(scene: &Scene) -> Image {
    let ref p = scene.camera.image_plane;
    let mut image = Image { image: Vec::new(),
                            width: p.width, //* p.pixel_density
                            height: p.height, //* p.pixel_density
                          };
    for x in 0..p.width {
        for y in 0..p.height {
            let ray = scene.camera.get_ray(x, y);
            let color = render_ray(scene, &ray, 10);
            image.set_pixel(x, y, color);
        }
    }
    image
}
