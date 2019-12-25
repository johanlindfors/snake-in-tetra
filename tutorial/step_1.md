# How to build Snake in Rust with Tetra
## Step 1 - Initial setup
Before we begin this tutorial on how to use Tetra, we will leverage the minimal code sample already available in the readme file on the mainpage of [Tetra's repository].

If you want to skip navigating back and forth you can use the following contents to copy and paste into your own rust solution, after having created a new directory, preferrably with the command `cargo new snake --bin`.

Update the generated cargo.toml and main.rs to the following:

cargo.toml:

```toml
[package]
name = "snake"
version = "0.1.0"
authors = ["Johan Lindfors <johan.lindfors@coderox.se>"]
edition = "2018"

[dependencies]
tetra = "0.3"
```

main.rs:

```rust
use tetra::graphics::{self, Color};
use tetra::{Context, ContextBuilder, State};

struct GameState;

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Cornflower blue, as is tradition
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Hello, world!", 1280, 720)
        .build()?
        .run(|_| Ok(GameState))
}
```
Now to the [next step] in which we will modify this scaffold to fit our purpose with the tiny snake game.

[next step]: (step_2.md)
[Tetra's repository]: https://github.com/17cupsofcoffee/tetra