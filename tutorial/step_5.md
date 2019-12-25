# How to build Snake in Rust with Tetra
## Step 5 - Rendering the snake
Let's mimic the logic of rendering the apple in the snake structure and implementation, but naturally us a different texture with a [green pixel] for representing the snake.
```rust
struct Snake {
    position: Vec2<i32>,
    texture: Texture,
}

impl Snake {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            position: Vec2::new(10, 10),
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
```
Now update the GameState constructor to also pass the Context parameter to the snake constructor.
```rust
fn new(ctx: &mut Context) -> tetra::Result<Self> {
    Ok(Self{
        apple: Apple::new(ctx)?,
        snake: Snake::new(ctx)?
    })
}
```
And obviously we need to call the draw-method as well in the game loop.
```rust
fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
    graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));

    self.apple.draw(ctx);
    self.snake.draw(ctx);

    Ok(())
}
```
While this will actually render the snake as a single piece on the screen, we have no tail or actual interation of the snake. The movement of the snake and also rendering it's tail will be done in the [next step].

[green pixel]: https://github.com/programmeramera/snake-in-tetra/blob/master/resources/green.png
[next step]: (step_6.md)