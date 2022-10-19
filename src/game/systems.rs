use super::components::{
    Velocity,
    Position,
    KeyboardControlled,
};
use specs::{System, ReadExpect, WriteStorage, ReadStorage, Join};
use crate::game::PLAYER_MOVEMENT_SPEED;
use crate::game::MovementCommand;

pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        ReadExpect<'a, Option<MovementCommand>>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        // only moves if had a command todo 
        let movement_command = match &*data.0 {
            Some(movement_command) => movement_command,
            None => return,
        };

        for (_, vel) in (&data.1, &mut data.2).join() {
            match movement_command {
                &MovementCommand::Move(direction) => {
                    vel.speed = PLAYER_MOVEMENT_SPEED;
                    vel.dir = direction;
                },
                MovementCommand::Stop => vel.speed = 0,
            }
        }
    }
}

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, mut data: Self::SystemData) {
        use super::Direction::*;
        for (pos, vel) in (&mut data.0, &data.1).join() {
            match vel.dir {
                Left => {
                    pos.x = pos.x - vel.speed as f64;
                },
                Right => {
                    pos.x = pos.x + vel.speed as f64;
                },
                Up => {
                    pos.y = pos.y - vel.speed as f64;
                },
                Down => {
                    pos.y = pos.y + vel.speed as f64;
                },
            }
        }
    }
}


