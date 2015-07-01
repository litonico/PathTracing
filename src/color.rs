#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub const WHITE : Color = Color {r:255, g:255, b:255};
pub const BLACK : Color = Color {r:0, g:0, b:0};
