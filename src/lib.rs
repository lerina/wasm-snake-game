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

    fn index_to_xy(&self, idx: usize) -> (usize, usize) {
        //  x            ,     y
        (idx % self.width, idx / self.width)
    }

    fn xy_to_index(&self, x: usize, y: usize) -> usize {
        // WHOLE ROWS + remaining COLUMNS
        (y * self.width) + x
    }

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        let (x, y) = self.index_to_xy(snake_idx);

        let (new_x, new_y) = match self.snake.direction {
            Direction::Right => {
                ((x + 1) % self.width, y)
            },
            Direction::Left => {
                ((x - 1) % self.width, y)
            },
            Direction::Up => {
                (x, (y - 1) % self.width)
            },
            Direction::Down => {
                (x, (y +1) % self.width)
            },
        };

        let next_idx = self.xy_to_index(new_x, new_y);
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
