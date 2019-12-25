# How to build Snake in Rust with Tetra
## Step 9 - The snake eating apples
When the snake moves over the apple we want to increase the lenght of the tail and also move the apple to a new position. The first logic we need to make this happen is a method in the snake instance to allow us to check for any collisions with the current position. Here is a small collision checking method that iterates over all the body parts in the queue and checks if the passed position collides with any part.
```rust
fn check_collision(&mut self, position: Vec2<i32>) -> bool {
    for pos in &self.trail {
        if *pos == position {
            return true;
        }
    }
    false
}
```
When the snake eats the apple we want the apple moved to a new position. Instead of handpicking these positions we are going to use and external crate called rand that includes a random number generator that serves our purpose. Update the cargo.toml file with the following dependency:
```toml
rand = "*"
```
And now we add a method within the GameState implementation to generate the apple. The method below leverages a loop to check the proposed randomly generated position, and make sure it doesn't collide with the existing snake. If there's a collision we generate a new position, and keep doing this until there's no collision (meaning the position is safe to move the apple to), and break out of the loop.
```rust
fn generate_apple(&mut self) {
    loop {
        let position = Vec2::new(
            rand::thread_rng().gen_range(0, SCREEN_SIZE),
            rand::thread_rng().gen_range(0, SCREEN_SIZE),
        );
        if !self.snake.check_collision(position) {
            self.apple.position = position;
            break;
        }
    }
}
```
Now we add a couple of lines of code in the GameState update method, after we have moved the snake, to check if the snake collides with the apple, and then update the snakes length before we move the apple as mentioned above.
```rust
self.snake.update();
if self.snake.check_collision(self.apple.position) {
    self.snake.tail += 1;
    self.generate_apple();
}
```
Now we can eat apples, lots of them actually cause the snake can pretty much grow indefinetly since there's nothing that keeps us from growing to far. We need to [change that now](step_10.md), by adding collision handling of the snake itself.