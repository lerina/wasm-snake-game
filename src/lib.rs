use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/rnd.ts")]
extern "C" {
    fn rnd(max: usize) -> usize;
}

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
}

impl Snake {
    fn new(spawn_index: usize) -> Self {
        Snake{
            body: vec!(SnakeCell(spawn_index)),
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

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        self.snake.body[0].0 = (snake_idx + 1) % self.dim();
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
