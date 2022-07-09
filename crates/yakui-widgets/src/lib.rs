#![allow(
    clippy::let_unit_value, // When implementing components, we want to spell
                            // out our Response type even if it's unit.
)]

mod align;
mod button;
mod colored_box;
mod image;
mod list;
mod padding;
mod util;

pub use align::*;
pub use button::*;
pub use colored_box::*;
pub use image::*;
pub use list::*;
pub use padding::*;

use yakui_core::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Down,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Alignment {
    x: f32,
    y: f32,
}

impl Alignment {
    const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub const TOP_LEFT: Self = Self::new(0.0, 0.0);
    pub const TOP_CENTER: Self = Self::new(0.5, 0.0);
    pub const TOP_RIGHT: Self = Self::new(1.0, 0.0);

    pub const CENTER_LEFT: Self = Self::new(0.0, 0.5);
    pub const CENTER: Self = Self::new(0.5, 0.5);
    pub const CENTER_RIGHT: Self = Self::new(1.0, 0.5);

    pub const BOTTOM_LEFT: Self = Self::new(0.0, 1.0);
    pub const BOTTOM_CENTER: Self = Self::new(0.5, 1.0);
    pub const BOTTOM_RIGHT: Self = Self::new(1.0, 1.0);
}
