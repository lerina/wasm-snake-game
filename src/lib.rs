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
    
    pub fn size(&self) -> usize {
        self.width * self.height
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }
    
    pub fn change_snake_dir(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }
    
    fn set_snake_head(&mut self, idx: usize) {
        self.snake.body[0].0 = idx;
    }

    fn index_to_cell(&self, idx: usize) -> (usize, usize) {
        //  COLUMN       ,     ROW
        (idx / self.width, idx % self.height)
    }

    fn cell_to_index(&self, row: usize, col: usize) -> usize {
        // WHOLE ROWS + remaining COLUMNS
        (row * self.width) + col
    }

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        let (row, col) = self.index_to_cell(snake_idx);

        let (new_row, new_col) = match self.snake.direction {
            Direction::Right => {
                (row, (col + 1) % self.width)
            },
            Direction::Left => {
                (row, (col - 1) % self.width)
            },
            Direction::Up => {
                ((row - 1) % self.width, col)
            },
            Direction::Down => {
                ((row +1) % self.width, col)
            },
        };

        let next_idx = self.cell_to_index(new_row, new_col);
        self.set_snake_head(next_idx);
    } //^-- update()
} //^-- impl World



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
