use tetra::graphics::{self, Color, Texture, DrawParams};
use tetra::{Context, ContextBuilder, State};
// use std::collections::VecDeque;
use tetra::math::Vec2;

//const FRAMES_PER_SECOND: f64 = 15.0;
const SPRITE_SIZE: i32 = 20;
const SCREEN_SIZE: i32 = 20;
//const INITIAL_TAIL: usize = 5;

struct Apple {
    position: Vec2<i32>,
    texture: Texture,
}

impl Apple {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            position: Vec2::new(3, 3),
            texture: Texture::new(ctx, "./resources/red.png")?,
        })
    }

    fn draw(&mut self, ctx: &mut Context) {
        graphics::draw(
            ctx,
            &self.texture,
            DrawParams::new()
                .position(Vec2::new(
                    (self.position.x * SPRITE_SIZE) as f32,
                    (self.position.y * SPRITE_SIZE) as f32,
                ))                   
                .scale(Vec2::new(
                    (SPRITE_SIZE as f32) * 0.95,
                    (SPRITE_SIZE as f32) * 0.95,
                )),
        );
    }
}

struct Snake {
    position: Vec2<i32>,
    direction: Vec2<i32>,
    // trail: VecDeque<Vec2<i32>>,
    // tail: usize,
    texture: Texture,
}

impl Snake {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            position: Vec2::new(10, 10),
            direction: Vec2::new(0, 0),
            // trail: VecDeque::new(),
            // tail: INITIAL_TAIL,
            texture: Texture::new(ctx, "./resources/green.png")?
        })
    }

    fn draw(&mut self, ctx: &mut Context) {
        graphics::draw(
            ctx,
            &self.texture,
            DrawParams::new()
                .position(Vec2::new(
                    (self.position.x * SPRITE_SIZE) as f32,
                    (self.position.y * SPRITE_SIZE) as f32,
                ))                   
                .scale(Vec2::new(
                    (SPRITE_SIZE as f32) * 0.95,
                    (SPRITE_SIZE as f32) * 0.95,
                )),
        );
    }
}

struct GameState {
    apple: Apple,
    snake: Snake,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self{
            apple: Apple::new(ctx)?,
            snake: Snake::new(ctx)?
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));

        self.apple.draw(ctx);
        self.snake.draw(ctx);

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