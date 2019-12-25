# How to build Snake in Rust with Tetra
## Step 4 - Rendering the apple
Before we make any calls to render graphics, we need to include some paths to structures in Tetra.
```rust
use tetra::graphics::{self, Color, Texture, DrawParams};
use tetra::math::Vec2;
```
The apple structure will be rendered at a specific x and y coordinate, let's leverage the generic Vec2 structure with i32 (integers) as type parameter. We also need a texture to represent the apple graphically and in this case a simple 1x1 [red pixel] image will be sufficient.
```rust
struct Apple {
    position: Vec2<i32>,
    texture: Texture,
}
```
Now we update the constructor with some defaults for the position and the texture. I chose to place the texture in a "resources" directory. The texture is loaded by Tetras constructor of the Texture structure, and requires the Context as a parameter, this also means we have to update the signature of the Apple::new constructor to get this Context parameters passed from the instatiting structure. We will update the calling code later in this step.
```rust
impl Apple {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            position: Vec2::new(3, 3),
            texture: Texture::new(ctx, "./resources/red.png")?,
        })
    }
}
```
For the actual rendering of the apple, let's add a draw-method to the implementation of the apple as follows. The parameters to the graphis::draw call is the Context, a reference to the texture, and a DrawParams structure with which we can position and scale the texture to take into consideration the specified SPRITE_SIZE. When we scale the texture, we tone it down a notch (95%) which will add a thin border to the apple, giving a neat pixelized effect.
```rust
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
```
Now let's make sure to pass the Context as a parameter to the constructor of the apple.
```rust
fn new(ctx: &mut Context) -> tetra::Result<Self> {
    Ok(Self {
        apple: Apple::new(ctx)?,
        snake: Snake::new()?
    })
}
```
And lastly let's make sure to actually call the draw method in the draw function which will be called each frame by Tetra.
```rust
fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
    graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));

    self.apple.draw(ctx);

    Ok(())
}
```
In the [next step](step_5.md) we will render the snake, or at least some part of it.

[red pixel]: https://github.com/programmeramera/snake-in-tetra/blob/master/resources/red.png
