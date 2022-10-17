extern crate sdl2;
use std::time::Duration;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::event::Event;
use sdl2::video::WindowContext;
use vector2d::Vector2D;

pub mod utils;
pub mod player;
pub mod tilesheet;
pub mod map;

use utils::TextureManager;
use player::Player;
use map::Map;

/// consts
const TITLE: &'static str = "RPG";
const SCREEN_HEIGHT: u32 = 736;
const SCREEN_WIDTH: u32 = 900;
const BACKGROUND_COLOR: Color = Color::RGB(100, 100, 100);
const PLAYER_SIZE: Vector2D<u32> = Vector2D { x: 64, y: 64 };
const PLAYER_POSITION: Vector2D<i32> = Vector2D { x: 450, y: 350 };


fn get_path() -> String {
    format!("{}{}", std::env::var("CARGO_MANIFEST_DIR").unwrap(), "/assets/")
}

fn render(
    texture_manager: &mut TextureManager<WindowContext>,
    canvas: &mut WindowCanvas,
    player: &player::Player,
    map: &Map,
) -> Result<(), String> {
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();
    map.draw(canvas, texture_manager)?;
    player.draw(canvas, texture_manager)?;

    canvas.present();
    Ok(())
}

/// run sdl2
pub fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys
        .window(TITLE, SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
        
    let mut canvas = window
        .into_canvas()
        .build().map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let mut texture_manager = utils::TextureManager::new(&texture_creator);
    let mut player = Player::new(
        format!("{}{}", get_path(), "sprites/player_sprite.png"),
        PLAYER_POSITION,
        PLAYER_SIZE
    );

    let map = Map::new(format!("{}{}", get_path(), "map/map.tmx"), &mut texture_manager);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut count = 0;

    'running: loop {
        println!("{}", count);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    player.move_to(Vector2D { x: -1, y: 0 });
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    player.move_to(Vector2D { x: 0, y: 1 });
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    player.move_to(Vector2D { x: 1, y: 0 });
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    player.move_to(Vector2D { x: 0, y: -1 });
                },
                _ => {}
            }
        }
        count += 1;
        render(&mut texture_manager, &mut canvas, &player, &map)?;
        
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
