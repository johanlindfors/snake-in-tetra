# How to build Snake in Rust with Tetra
## Step 8 - Steering the snake
Tetra has some builtin features to handle keyboard interaction, lets reference those paths and structure needed:
```rust
use tetra::input::{self, Key};
```
The overall logic for the keyboard interation is that we're going to allow changing the horizontal direction only if we're currently isn't moving horizontally, and vice versa for vertical movement. The direction we choose is simply the vector with appropriate x and y values to increase or decrease x or y positions each frame. The method called handle_input is added to the GameState implementation as follows:
```rust
fn handle_input(&mut self, ctx: &mut Context) {
    if input::is_key_pressed(ctx, Key::Left) && self.snake.direction.x == 0 {
        self.snake.direction = Vec2::new(-1, 0);
    } else if input::is_key_pressed(ctx, Key::Right) && self.snake.direction.x == 0 {
        self.snake.direction = Vec2::new(1, 0);
    } else if input::is_key_pressed(ctx, Key::Up) && self.snake.direction.y == 0 {
        self.snake.direction = Vec2::new(0, -1);
    } else if input::is_key_pressed(ctx, Key::Down) && self.snake.direction.y == 0 {
        self.snake.direction = Vec2::new(0, 1);
    }
}
```
Then we also make sure to call the handle_input method in the update method called in the GameState game loop.
```rust
fn update(&mut self, ctx: &mut Context) -> tetra::Result {
    self.handle_input(ctx);
    self.snake.update();

    Ok(())
}
```
Now we are really getting somewhere, we can steer the snake and move across the screen, but we still can't eat the apple, let's [fix that now].

[fix that now]: (step_9.md)