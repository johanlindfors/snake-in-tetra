# How to build Snake in Rust with Tetra
## Step 6 - Rendering the snake
Well the snake will actually not be a single square, but instead a continously moving snake of body parts that we can guide by pressing arrow keys on our keyboard in the direction we want the snake to trail to. To implement the movement, we add a field to the snake structure called direction.
```rust
struct Snake {
    position: Vec2<i32>,
    direction: Vec2<i32>,
    texture: Texture,
}
```
This field will have either it's x or y coordinate set to 1 or -1 based on which direction we want the snake to trail to. In the constructor of the snake we give the direction initially a positive x value meaning the snake will move to the right of the screen, one step each frame.
```rust
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            position: Vec2::new(10, 10),
            direction: Vec2::new(1, 0),
            texture: Texture::new(ctx, "./resources/green.png")?
        })
    }
```
But if we run the code now, we still have no movement, since we have to implement an actual update of the position. In classic game development style, we add a method called update to the snake implementation. In this method we move the position by the direction vector, but also taking into consideration the boundaries of the screen. A simple trick is to continously add the actual screen size to the movement, but then also leverage the mod operator to make sure our coordinates gets clamped to the size of the window.
```rust
fn update(&mut self) {
    let position = Vec2::new(
        (self.position.x + SCREEN_SIZE + self.direction.x) % SCREEN_SIZE,
        (self.position.y + SCREEN_SIZE + self.direction.y) % SCREEN_SIZE,
    );
    self.position = position;
}
```
To make sure the update gets called from the GameState we add an update method to the GameState implementation as well. It's signature is that it's and instance method that also takes the Context as a parameter, returning a result enumeration if all went well.
```rust
fn update(&mut self, ctx: &mut Context) -> tetra::Result {
    self.snake.update();

    Ok(())
}
```
Running the code now will make it apparent that the snake moves silly fast and there's no way we will be able to actually play the game at all. Let's add constant that we can play around with in finding out an appropriate fram rate. I've found that 15.0 works quite nicely. This gets added to the previous constants:
```rust
const FRAMES_PER_SECOND: f64 = 15.0;
```
And then we add the following update to the construction of the Context by specifying the timestep for the game loop.
```rust
ContextBuilder::new("Snake!", width, height)
    .quit_on_escape(true)
    .timestep(tetra::time::Timestep::Fixed(FRAMES_PER_SECOND))
    .build()?
    .run(GameState::new)
```
Now the game should run quite smoothly but we still only have one single body element of the snake. Lets change that in the [next step].

[next step]: (step_7.md)