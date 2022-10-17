use super::utils::TextureManager;
use sdl2::video::WindowContext;
use std::sync::Arc;
use sdl2::rect::Rect;
use vector2d::Vector2D;

pub struct Tilesheet {
    pub tilesheet_path: String,
    pub tileset: Arc<tiled::Tileset>,
    pub size: Vector2D<u32>,
}

impl Tilesheet {
    pub fn from_tileset(tileset: Arc<tiled::Tileset>, texture_manager: &mut TextureManager<WindowContext>) -> Self {
        let tileset_image = tileset.image.as_ref().unwrap();

        let texture = {
            let texture_path = &tileset_image
                .source
                .to_str()
                .expect("obtaining valid UTF-8 path");
            texture_manager.load(texture_path).unwrap()
        };
        let tilesheet_path = tileset_image.source.to_str().unwrap().to_string();

        Tilesheet { tilesheet_path, tileset, size: Vector2D { x: texture.query().width, y: texture.query().height } }
    } 

    pub fn tile_rect(&self, id: u32) -> sdl2::rect::Rect {
        let tile_width = self.tileset.tile_width;
        let tile_height = self.tileset.tile_height;
        let spacing = self.tileset.spacing;
        let margin = self.tileset.margin;
        let tiles_per_row = (self.size.x - margin + spacing) / (tile_width + spacing);
        let x = id % tiles_per_row * tile_width;
        let y = id / tiles_per_row * tile_height;

        Rect::new(x as i32, y as i32, tile_width, tile_height)

    }
}

