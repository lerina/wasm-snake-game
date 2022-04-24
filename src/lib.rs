use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

const WORLD_WIDTH: usize = 8;

#[wasm_bindgen]
pub struct World {
    pub width: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new() -> Self {
        World { width: WORLD_WIDTH }
    }

    pub fn width(&self) -> usize {
        self.width
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
