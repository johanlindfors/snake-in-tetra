# How to build Snake in Rust with Tetra
## Step 10 - Let's make some noise
All good games have sound in various forms, lets add some effects to this snake game as well. We're going to add a chomping sound when the snake eats the apple, and a somewhat squashing sound when the snake collides with itself. Let's start with the chomping sound and reap the benefits of using Tetra.

I'm going to use a [chomp sound] and a [splat sound] that I found on the website [freesoundeffect.net](http://freesoundeffect.net).


For this simple exercise all we need is the path to the Sound structure that Tetra exposes. 
```rust
use tetra::audio::Sound;
```
Then in the GameState, we add a variable that will hold the sound effect.
```rust
struct SnakeGame {
    ...
    eat: Sound,
}
```
And let's not forget to initialize this variable in the constructor.
```rust
impl SnakeGame {
    fn new(ctx: &mut Context) -> tetra::Result<SnakeGame> {
        Ok(SnakeGame {
            ...
            eat: Sound::new("./resources/chomp.mp3")?,
        })
    }
```
To play this [chomp sound] we simply need to call the play method on the instance. Let's add this call in the update method on the GameState as follows:
```rust
if self.snake.check_collision(self.apple.position) {
    self.eat.play(ctx)?;
    ...
}
```
Adding the [splat sound] is a bit more work, which is kind of interesting cause it also displays how the initial object oriented structure of this sample has worked nicely until now, but requires us to slightly refactor some of the existing code. Nothing major but still something to take into consideration. I would like to keep the object oriented approach moving forward, encapsulating some of the snake's logic within it's implementation, and playing it's collision sound is clearly one of those tasks. Let's begin by adding the sound as a variable to the snake structure:
```rust
struct Snake {
    ...
    die: Sound,
}
```
And initialize it appropriately:
```rust
impl Snake {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            ...
            die: Sound::new("./resources/splat.mp3")?,
        })
    }
```
This sound is going to be played when the snake collides with itself, and hence I need to update the update method in the snake. Notice that the changes includes passing the Context as a parameter to the update method (breaking the interface for this structure), and that we also need to check if the snake is static or actually moving before checkin for collision (since when the game starts, the snake doesn't have a direction in which it moves). The play method returns a Result but with a simple call to `.unwrap()` I explicitly say that I feel safe about no errors being returned from this call. Should probably have a better error handling but it's out of this tutorials scope. 
```rust
fn update(&mut self, ctx: &mut Context) {
    ...
    if self.direction != Vec2::zero() &&
        self.check_collision(position) 
    {
        self.die.play(ctx).unwrap();
        ...
    }
    ...
}
```
Since I broke the interface for the snake, I also have to pass the Context as a parameter from the update method in the GameState as follows:
```rust
fn update(&mut self, ctx: &mut Context) -> tetra::Result {
    ...
    self.snake.update(ctx);
    ...
}
```
This should be sufficient to be able to build and run the game and now have some neat sound effects making it a bit more appealing, I hope.

Now you should have a small snake game running, built with the language rust and leveraging the tetra-framework.

[chomp sound]: https://github.com/programmeramera/snake-in-tetra/blob/master/resources/chomp.mp3
[splat sound]: https://github.com/programmeramera/snake-in-tetra/blob/master/resources/splat.mp3