use vector2d::Vector2D;
use tiled::Loader;


pub struct Tilesheet {}
pub struct Map {
    map_path: String,
    size: Vector2D<u32>,
    tile_size: Vector2D<u32>,
    tilesheet: Tilesheet,
}


impl Map {
    pub fn new(map_path: String) -> Self {
        Map { 
            map_path: map_path,
            size: Vector2D { x: 0, y: 0 },
            tile_size: Vector2D { x: 0, y: 0 },
            tilesheet: Tilesheet {  },
        }
    }
    
    pub fn load_map(&mut self) -> Result<(), String> {
        let mut loader = tiled::Loader::new();
        let map = loader
            .load_tmx_map(self.map_path.clone())
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}