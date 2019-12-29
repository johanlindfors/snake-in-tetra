# How to build Snake in Rust with Tetra
## Step 10 - The snake must not run over itself
The overall rule of snake game is to now collide with yourself. We can leverage the already written logic, check_collision of the snake. Simply update the following lines to make sure the position is mutable and then use the check_collision logic to check the new position (before we push it into the queue). If we have a collision, we reset the length of the queue to it's initial length specified by the constant and move the head of the snake to the middle of the screen. Let's also clear the direction, making the snake stand perfectly still on a single position.
```rust
fn update(&mut self) {
    let mut position = Vec2::new(
        (self.position.x + SCREEN_SIZE + self.direction.x) % SCREEN_SIZE,
        (self.position.y + SCREEN_SIZE + self.direction.y) % SCREEN_SIZE,
    );

    if self.check_collision(position) {
        self.tail = INITIAL_TAIL;
        position.x = 10;
        position.y = 10;
        self.direction = Vec2::zero()
    }
    
    ...
```
In the [next step](step_11.md) we add some sound effects to make the game a bit more fun to play.

[GitHub repository]: https://github.com/programmeramera/snake-in-tetra.git