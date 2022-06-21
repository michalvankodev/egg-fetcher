use bevy::prelude::*;

pub struct Colors;

impl Colors {
    pub const PRIMARY: Color = Color::Rgba {
        red: 255.0 / u8::MAX as f32,
        green: 33.0 / u8::MAX as f32,
        blue: 33.0 / u8::MAX as f32,
        alpha: 1.0,
    };
    pub const LIGHT: Color = Color::Rgba {
        red: 227.0 / u8::MAX as f32,
        green: 227.0 / u8::MAX as f32,
        blue: 227.0 / u8::MAX as f32,
        alpha: 1.0,
    };
    pub const DARK: Color = Color::Rgba {
        red: 28.0 / u8::MAX as f32,
        green: 28.0 / u8::MAX as f32,
        blue: 28.0 / u8::MAX as f32,
        alpha: 1.0,
    };
    pub const TRANSPARENT: Color = Color::Rgba {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
        alpha: 0.0,
    };
}
