# wasm-snake-game: Slide 19 

## improve snake dir

We will need to copy 

```rust
// Clone is a supertrait of Copy, 
// so everything which is Copy must also implement Clone.
#[derive(Copy, Clone)]
pub struct SnakeCell(usize);

```

- store next_cell in the world

```rust
#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
}
```

- update next_cell when dir change is ok

```rust
    pub fn change_snake_dir(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&direction);

        // can't 180
        if self.snake.body[1].0 == next_cell.0 { return;}
        
        // Its ok to change dir    
        self.next_cell = Some(next_cell); 
        self.snake.direction = direction;
    }
```

- update step

```rust
    pub fn step(&mut self) {        // because of the Mutable Reference
        let tmp = self.snake.body.clone();

        match self.next_cell {      // Copy trait on SnakeCell is needed here
            Some(next_cell) => { // keypress occured with valid dir
                self.snake.body[0] = next_cell;
                self.next_cell = None;
            },
            None => {            // no keypress . keep moving
                self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction);
            }
        }

        let len = self.snake.body.len();

        for i in 1..len {
            self.snake.body[i] = SnakeCell(tmp[i - 1].0);
        }
    }
```
