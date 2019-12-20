use tetra::graphics::{self, Color};
use tetra::{Context, ContextBuilder, State};
// use std::collections::VecDeque;
// use tetra::math::Vec2;

//const FRAMES_PER_SECOND: f64 = 15.0;
const SPRITE_SIZE: i32 = 20;
const SCREEN_SIZE: i32 = 20;
//const INITIAL_TAIL: usize = 5;

struct Apple {
    // position: Vec2<i32>,
}

impl Apple {
    fn new() -> tetra::Result<Self> {
        Ok(Self {
            // position: Vec2::new(0,0)
        })
    }
}

struct Snake {
    // position: Vec2<i32>,
    // direction: Vec2<i32>,
    // trail: VecDeque<Vec2<i32>>,
    // tail: usize,
}

impl Snake {
    fn new() -> tetra::Result<Self> {
        Ok(Self {
            // position: Vec2::new(0,0),
            // direction: Vec2::new(0,0),
            // trail: VecDeque::new(),
            // tail: INITIAL_TAIL
        })
    }
}

struct GameState {
    apple: Apple,
    snake: Snake,
}

impl GameState {
    fn new(_ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self{
            apple: Apple::new()?,
            snake: Snake::new()?
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
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
        .run(GameState::new)
}