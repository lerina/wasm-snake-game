# wasm-snake-game: Slide 13

## Changing the snake head direction.

- Make Direction visible to javascript

```rust
#[wasm_bindgen]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

```

- update the snake to have a direction with a default going down for now

```rust
...
struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize) -> Self {
        Snake{
            body: vec!(SnakeCell(spawn_index)),
            direction: Direction::Down,
        }
    }
}

...
```

- Give Js the ability to change the snake direction via the World
 
```rust
#[wasm_bindgen]
impl World {
... 

    pub fn change_snake_dir(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }
       
```

- let the World know about direction changes

```rust
#[wasm_bindgen]
impl World {
... 

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.height;
        let col = snake_idx % self.height;

        match self.snake.direction {
            Direction::Up => {
                let next_row = (row - 1) % self.height;
                self.snake.body[0].0 = (next_row * self.height) + col;
            },
            Direction::Right => {
                let next_col = (col + 1) % self.width;
                self.snake.body[0].0 = (row * self.width) + next_col;
            },
            Direction::Down => {
                let next_row = (row + 1) % self.height;
                self.snake.body[0].0 = (next_row * self.height) + col;
            },
            Direction::Left => {
                let next_col = (col - 1) % self.width;
                self.snake.body[0].0 = (row * self.width) + next_col;
            },
        }
...
```

