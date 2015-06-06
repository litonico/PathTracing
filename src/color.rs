#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u16,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r:f64, g:f64, b:f64) -> Color {
        Color{r:r, g:g, b:b}
    }
}
