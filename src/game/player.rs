use vector2d::Vector2D;
use sdl2::render::WindowCanvas;
use sdl2::video::WindowContext;
use crate::utils::TextureManager;

pub struct Player {
    texture_path: String, 
    position: Vector2D<i32>,
    size: Vector2D<u32>,
    velocity: Vector2D<i32>,
}

impl Player {
    pub fn new(texture_path: String, position: Vector2D<i32>, size: Vector2D<u32>) -> Self {
        Player {
            texture_path,
            position,
            size,
            velocity: Vector2D { x: 10, y: 10 },
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, texture_manager: &mut TextureManager<WindowContext>) -> Result<(), String> {
        let texture = texture_manager.load(self.texture_path.as_str())?;
        canvas.copy(&texture, None, sdl2::rect::Rect::new(self.position.x, self.position.y, self.size.x, self.size.y))?;
        Ok(())

    }

    pub fn texture_path(&self) -> String {
        self.texture_path.clone()
    }

    pub fn x(&self) -> i32 {
        self.position.x
    }

    pub fn y(&self) -> i32 {
        self.position.y
    }

    pub fn width(&self) -> u32 {
        self.size.x
    }

    pub fn height(&self) -> u32 {
        self.size.y
    }

    pub fn move_to(&mut self, dir: Vector2D<i32>) {
        self.position.x += dir.x * self.velocity.x;
        self.position.y += dir.y * self.velocity.y;
    }

}
