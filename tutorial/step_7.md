# How to build Snake in Rust with Tetra
## Step 7 - The tail of the snake
The logic for the tail of the snake will be implemented by using a simple queue. Each frame we add the current position to the head of the queue and based on how long we want the snake to be, keep the queue growing, but removing the tail of the queue each frame.

Lets use a standard rust collection called VecDeque which is the recommended structure when leveraging a queue.
```rust
use std::collections::VecDeque;
```
Initially we want the snake to be five parts long, lets add this a constant.
```rust
const INITIAL_TAIL: usize = 5
```
The actual instance of the queue will be organized within the snake. Add these two fields to the snake structure, notice that the queue will be a generic queue with the `Vec2<i32>` as type parameter. The tail field will be used to keep track of the length of the tail.
```rust
trail: VecDeque<Vec2<i32>>,
tail: usize,
```
Now initialize these values in the Snake constructor.
```rust
trail: VecDeque::new(),
tail: INITIAL_TAIL,
```
Now we need to update the rendering of the snake to not just only draw the actual current position, but only iterate through all of the elements in the queue and render them in their respective positions.
```rust
fn draw(&self, ctx: &mut Context) {
    for element in &self.trail {
        graphics::draw(
            ctx,
            &self.texture,
            DrawParams::new()
                .position(Vec2::new(
                    (element.x * SPRITE_SIZE) as f32,
                    (element.y * SPRITE_SIZE) as f32,
                ))                   
                .scale(Vec2::new(
                    (SPRITE_SIZE as f32) * 0.95,
                    (SPRITE_SIZE as f32) * 0.95,
                )),
        );
    }
}
```
The last thing we need to to is to modify the update method in the snake to push the current position to the head of the queue and the use a loop to pop from the back of the queue until we have the, current length of the snake, elements left in the queue.
```rust
fn update(&mut self) {
    let position = Vec2::new(
        (self.position.x + SCREEN_SIZE + self.direction.x) % SCREEN_SIZE,
        (self.position.y + SCREEN_SIZE + self.direction.y) % SCREEN_SIZE,
    );
    self.position = position;

    self.trail.push_front(self.position);
    loop {
        if self.trail.len() <= self.tail {
            break;
        }
        self.trail.pop_back();
    }
}
```
If we run the code now, we will have a snake with five body parts moving across the screen. [Now we're going](step_8.md) to implement the interaction, allowing us to steer the snake as we want.
