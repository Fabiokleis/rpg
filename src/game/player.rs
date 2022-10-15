use vector2d::Vector2D;
use sdl2::video::WindowContext;
use sdl2::render::WindowCanvas;
use sdl2::render::TextureCreator;
use sdl2::image::LoadTexture;
pub struct Player {
    texture_path: String,
    position: Vector2D<i32>,
    size: Vector2D<u32>,
    shape: sdl2::rect::Rect,
    velocity: Vector2D<i32>,
}

impl Player {
    pub fn new(texture_path: String, position: Vector2D<i32>, size: Vector2D<u32>) -> Self {
        Player { 
            texture_path: texture_path,
            position: position,
            size: size,
            shape:  sdl2::rect::Rect::new(position.x, position.y, size.x, size.y),
            velocity: Vector2D { x: 32, y: 32 }
        }
    }

    pub fn move_to(&mut self, dir: Vector2D<i32>) {
        self.position.x += dir.x * self.velocity.x;
        self.position.y += dir.y * self.velocity.y;
        self.shape.set_x(self.position.x);
        self.shape.set_y(self.position.y);
    }

    pub fn draw(&self, texture_creator: &TextureCreator<WindowContext>, canvas: &mut WindowCanvas) -> Result<(), String> {
        let player_texture = texture_creator.load_texture(self.texture_path.as_str())?;
        canvas.copy(&player_texture, None, self.shape)?;
        Ok(())
    }

    pub fn shape(&self) -> sdl2::rect::Rect {
        self.shape
    }
}
