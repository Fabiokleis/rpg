pub mod utils;
pub mod game;

fn main() -> Result<(), String> {
    game::run()
}
