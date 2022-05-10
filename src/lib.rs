use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/rnd.ts")]
extern "C" {
    fn rnd(max: usize) -> usize;
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

// Clone is a supertrait of Copy, so everything which is Copy must also implement Clone.
#[derive(Copy, Clone)]
pub struct SnakeCell(usize);


struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Snake {
        let mut body = vec!();

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }

        Snake {
            body,
            direction: Direction::Right,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    reward_cell: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> World {
        World {
            width,
            size: width * width,
            snake: Snake::new(snake_idx, 3),
            next_cell: None,
            reward_cell: 10,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn reward_cell(&self) -> usize {
        self.reward_cell
    }

    pub fn snake_head_idx(&self) -> usize {
       self.snake.body[0].0
    }

    pub fn change_snake_dir(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&direction);

        // can't 180
        if self.snake.body[1].0 == next_cell.0 { return;}
        
        // Its ok to change dir    
        self.next_cell = Some(next_cell);
        self.snake.direction = direction;
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }
    // helper method
    fn index_to_xy(&self, idx: usize) -> (usize, usize) {
        // get 2d x,y coordinates from an index in 1d 
        //  x            ,     y
        (idx % self.width, idx / self.width)
    }

    fn xy_to_index(&self, x: usize, y: usize) -> usize {
        // get index in 1d from a 2d x,y coordinates
        // WHOLE ROWS + remaining COLUMNS
        (y * self.width) + x
    }

    //NOTE: can't return ref to Js (borrow can't be checked so not allowed)
    //cannot return a borrowed ref with #[wasm_bindgen]
    //pub fn snake_body(&self) -> &Vec<SnakeCell> {
    //  &self.snake.body
    //}
    // Solution is to use a raw pointer (*const) to the first element of our vector
    // Borrow checker will not apply the rules.
    pub fn snake_cells_ptr(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }


    pub fn step(&mut self) {        // because of the Mutable Reference
        let tmp = self.snake.body.clone();

        match self.next_cell {      // Copy trait on SnakeCell is needed here
            Some(next_cell) => { //keypress occured with valid dir
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

    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;

         match direction {
            Direction::Right => {
                let right_bound = (row + 1) * self.width;
                if snake_idx + 1 == right_bound {
                    SnakeCell(right_bound - self.width)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            },
          Direction::Left => {
                let left_bound = row * self.width;
                if snake_idx == left_bound {
                    SnakeCell(left_bound + (self.width - 1))
                } else {
                    SnakeCell(snake_idx - 1)
                }
            },
            Direction::Up => {
                let upper_bound = snake_idx - (row * self.width);
                if snake_idx == upper_bound {
                    SnakeCell((self.size - self.width) + upper_bound)
                } else {
                    SnakeCell(snake_idx - self.width)
                }
            },
            Direction::Down => {
                let lower_bound = snake_idx + ((self.width - row) * self.width);
                if snake_idx + self.width == lower_bound {
                    SnakeCell(lower_bound - ((row + 1) * self.width))
                } else {
                    SnakeCell(snake_idx + self.width)
                }
            }
        }
    }

}

// wasm-pack build --target web
