use tetra::graphics::{self, Color};
use tetra::{Context, ContextBuilder, State};

//const FRAMES_PER_SECOND: f64 = 15.0;
const SPRITE_SIZE: i32 = 20;
const SCREEN_SIZE: i32 = 20;
//const INITIAL_TAIL: usize = 5;

struct GameState;

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Cornflower blue, as is tradition
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));
        Ok(())
    }
}

fn main() -> tetra::Result {
    let width = (SPRITE_SIZE * SCREEN_SIZE) as i32;
    let height = (SPRITE_SIZE * SCREEN_SIZE) as i32;

    ContextBuilder::new("Snake!", width, height)
        .quit_on_escape(true)
        .build()?
        .run(|_| Ok(GameState))
}