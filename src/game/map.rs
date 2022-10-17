use vector2d::Vector2D;
use sdl2::video::WindowContext;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use super::tilesheet::Tilesheet;
use crate::utils::TextureManager;


pub struct Layer {
    width: u32,
    height: u32,
    tiles_id: Vec<u32>,
    tiles: Vec<(Rect, Rect)>
}

impl Layer {
    pub fn new(width: u32, height: u32, tiles_id: Vec<u32>, tiles: Vec<(Rect, Rect)>) -> Self {
        Layer {
            width,
            height,
            tiles_id,
            tiles,
        }
    } 

    pub fn draw(&self, canvas: &mut WindowCanvas, tex: &Texture) -> Result<(), String> {
        for (src, dest) in self.tiles.clone() {
            canvas.copy_ex(&tex, src, dest, 0.0, None, false, false)?;
        } 
        Ok(())
    }
}

pub struct Map {
    map_path: String,
    size: Vector2D<u32>,
    tile_size: Vector2D<u32>,
    tilesheet: Tilesheet,
    layers: Vec<Layer> 
}


impl Map {
    pub fn new(map_path: String, texture_manager: &mut TextureManager<WindowContext>) -> Self {
        let mut loader = tiled::Loader::new();
        let map = loader
            .load_tmx_map(map_path.clone())
            .map_err(|e| e.to_string()).unwrap();

        let tilesheet = Tilesheet::from_tileset(map.tilesets()[0].clone(), texture_manager);
        let mut layers: Vec<Layer> = vec![];

        for layer in map.layers() {
            match layer.layer_type() {
                tiled::LayerType::TileLayer(layer) => match layer {
                    tiled::TileLayer::Finite(data) => {
                        let (width, height) = (data.width() as usize, data.height() as usize);
                        let mut tiles_id: Vec<u32> = vec![];
                        let mut tiles: Vec<(Rect, Rect)> = vec![];
                        for x in 0..width as i32 {
                            for y in 0..height as i32 {
                                if let Some(tile) = data.get_tile(x, y) {
                                    tiles_id.push(tile.id());
                                    let src = tilesheet.tile_rect(tile.id());
                                    let dest = Rect::new(
                                        x * src.width() as i32,
                                        y * src.height() as i32,
                                        src.width(), 
                                        src.height()
                                    );

                                    tiles.push((src, dest));
                                }
                            }
                        }
                        layers.push(Layer::new(data.width(), data.height(), tiles_id, tiles));
                    },
                    _ => {}
                }
                _ => {}
            }
        }

        Map { 
            map_path,
            size: Vector2D { x: map.width, y: map.height },
            tile_size: Vector2D { x: map.tile_width, y: map.tile_height },
            tilesheet,
            layers,
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, texture_manager: &mut TextureManager<WindowContext>) -> Result<(), String> {
        let tex = texture_manager.load(self.tilesheet.tilesheet_path.as_str())?;
        for layer in self.layers.iter() {
            layer.draw(canvas, &tex)?;
        }
        Ok(())
    }
}
