use std::time::Duration;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::event::Event;
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use vector2d::Vector2D;
use specs::prelude::*;

/// mod components
mod components;
mod systems;
mod tilesheet;
mod map;

use super::utils::TextureManager; 
use components::*;
use systems::*;
use map::Map;

/// consts
const TITLE: &'static str = "RPG";
const SCREEN_HEIGHT: u32 = 736;
const SCREEN_WIDTH: u32 = 900;
const BACKGROUND_COLOR: Color = Color::RGB(100, 100, 100);
const PLAYER_SIZE: Vector2D<u32> = Vector2D { x: 64, y: 64 };
const PLAYER_POSITION: Vector2D<f64> = Vector2D { x: 450.0, y: 350.0 };
const PLAYER_MOVEMENT_SPEED: i32 = 10;

#[derive(Clone, Copy)]
pub enum MovementCommand {
    Stop,
    Move(components::Direction),
}

fn get_path() -> String {
    format!("{}{}", std::env::var("CARGO_MANIFEST_DIR").unwrap(), "/assets/")
}

fn render (
    texture_manager: &mut TextureManager<WindowContext>,
    canvas: &mut WindowCanvas,
    world: &World,
    map: &Map,
) -> Result<(), String> {
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();
    map.draw(canvas, texture_manager)?;

    // render components
    
    let positions = world.read_storage::<Position>();
    let renderables = world.read_storage::<Renderable>();

    for (pos, rend) in (&positions, &renderables).join() {
        let src = Rect::new(0, 0, rend.src_width, rend.src_height);
        let dest = Rect::new(
            pos.x as i32,
            pos.y as i32,
            rend.dest_width,
            rend.dest_height,
        );
        let tex = texture_manager.load(rend.texture_name.as_str())?;
        canvas.copy(&tex, src, dest)?;

    }


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
    let mut texture_manager = TextureManager::new(&texture_creator);

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Velocity>();
    world.register::<KeyboardControlled>();

    let mut dispatcher = DispatcherBuilder::new()
        .with(Keyboard, "Keyboard", &[])
        .with(Physics, "Physics", &["Keyboard"])
        .build();

    dispatcher.setup(&mut world);

    let mut movement_command: Option<MovementCommand> = None;

    world.insert(movement_command);

    // create player with renderable and keyboard bindings
    world.create_entity()
        .with(KeyboardControlled)
        .with(Renderable {
            texture_name: format!("{}{}", get_path(), "sprites/player_sprite.png"),
            src_width: PLAYER_SIZE.x,
            src_height: PLAYER_SIZE.y,
            dest_width: PLAYER_SIZE.x,
            dest_height: PLAYER_SIZE.y,
            frame: 1,
            total_frames: 1,
            rot: 0.0,
        })
        .with(Position::new(PLAYER_POSITION))
        .with(Velocity { speed: 0, dir: Direction::Right })
        .build();


    let map = Map::new(format!("{}{}", get_path(), "map/map.tmx"), &mut texture_manager);
    let mut event_pump = sdl_context.event_pump()?;
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
                    movement_command = Some(MovementCommand::Move(Direction::Left));
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Down));
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Right));
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Up));
                },
                Event::KeyUp { keycode: Some(Keycode::A), .. } |
                Event::KeyUp { keycode: Some(Keycode::S), .. } |
                Event::KeyUp { keycode: Some(Keycode::D), .. } |
                Event::KeyUp { keycode: Some(Keycode::W), .. } => {
                    movement_command = Some(MovementCommand::Stop);
                }

                _ => {}
            }
        }
        *world.write_resource() = movement_command;
        dispatcher.dispatch(&mut world);
        world.maintain();
        count += 1;
        render(&mut texture_manager, &mut canvas, &world, &map)?;
        
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
