use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/rnd.ts")]
extern "C" {
    fn rnd(max: usize) -> usize;
}

#[wasm_bindgen]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}


struct SnakeCell(usize);

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

#[wasm_bindgen]
pub struct World {
    width: usize,
    height: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, height: usize, snake_start_idx: usize) -> Self {
        World { 
            width,
            height,
            snake: Snake::new(snake_start_idx),
        }
    }


    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize { 
        self.height 
    }
    
    pub fn dim(&self) -> usize {
        self.width * self.height
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }
    
    pub fn change_snake_dir(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }
    
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
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
