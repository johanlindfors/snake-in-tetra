# How to build Snake in Rust with Tetra
## Step 3 - The apple and the snake
The apple and the snake will be two separate structures with their own logic encapsulated. Let's add the structs and their placeholder implementations to the source file. The implementations will be updated in later steps.

```rust
struct Apple;

impl Apple {
    fn new() -> tetra::Result<Self> {
        Ok(Self {})
    }
}

struct Snake;

impl Snake {
    fn new() -> tetra::Result<Self> {
        Ok(Self {})
    }
}
```
Now we also need to create instances of these structures in the actual GameState. To be a bit prepared for later steps, I also added the Context as a parameter.

```rust
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
```
Since we now have a custom "constructor" for the GameState, we need to update the main method as well to make sure we call the instantiator.
```rust
ContextBuilder::new("Snake!", width, height)
    .quit_on_escape(true)
    .build()?
    .run(GameState::new)
```
In the [next step](step_4.md) we will begin rendering the apple by leveraging the functionality of tetra.
