# How to build Snake in Rust with Tetra
## Step 2 - Setup the screen for snake
In the tiny snake game we want to have a screen that is square (well honestly you could probably do with an arbitrary size but this is most common). We are also going to leverage an integer based grid to position the snake and the apple to be eaten, and the size of the blocks to represent the objects is also integer based. Let's put these parameters into some constants.

```rust
const SPRITE_SIZE: i32 = 20;
const SCREEN_SIZE: i32 = 20;
```
The cornflower blue color is not doing it for me, let's change it to something darker, let's say pitch black:
```
graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));
```
Now we calculate the size of the game screen to be "the grid size" x "the sprite size" and update the main method as follows:
```rust
fn main() -> tetra::Result {
    let width = (SPRITE_SIZE * SCREEN_SIZE) as i32;
    let height = (SPRITE_SIZE * SCREEN_SIZE) as i32;

    ContextBuilder::new("Snake!", width, height)
        .quit_on_escape(true) // can be used to exit the game
        .buid()?
        .run(|_| Ok(GameState))
}
```
In the [next step](step_3.md) we will add the apple and the snake to the source files.
