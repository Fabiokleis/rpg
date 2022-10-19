use specs::{Component, VecStorage, NullStorage};
use vector2d::Vector2D;

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}
impl Position {
    pub fn new(pos: Vector2D<f64>) -> Self {
        Position {
            x: pos.x,
            y: pos.y,
        }
    }
}

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub texture_name: String,
    pub src_width: u32,
    pub src_height: u32,
    pub dest_width: u32,
    pub dest_height: u32,
    pub frame: u32,
    pub total_frames: u32,
    pub rot: f64,
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct KeyboardControlled;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Velocity {
    pub speed: i32,
    pub dir: Direction,
}
