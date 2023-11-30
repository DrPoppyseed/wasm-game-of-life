mod utils;

use fixedbitset::FixedBitSet;
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;

static UNIVERSE_START_STATE: Lazy<FixedBitSet> = Lazy::new(|| FixedBitSet::with_capacity(64 * 64));

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
    past1: FixedBitSet,
    past2: FixedBitSet,
}

impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    pub fn get_cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        cells.iter().for_each(|(row, col)| {
            let idx = self.get_index(*row, *col);
            self.cells.set(idx, true);
        });
    }

    fn store_and_compare(&mut self) -> bool {
        if self.cells.is_subset(&self.past2) {
            return true;
        }

        let past1 = self.past1.clone();
        self.past1 = self.cells.clone();
        self.past2 = past1;
        false
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
        let width = 64;
        let height = 64;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, js_sys::Math::random() < 0.5)
        }

        Self {
            width,
            height,
            cells,
            past1: UNIVERSE_START_STATE.clone(),
            past2: UNIVERSE_START_STATE.clone(),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells.set_range(.., false);
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells.set_range(.., false);
    }

    pub fn tick(&mut self) -> bool {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell: bool = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                next.set(
                    idx,
                    match (cell, live_neighbors) {
                        // Rule 1: Any live cell with fewer than two live neighbors
                        // dies, as if caused by underpopulation
                        (true, x) if x < 2 => false,
                        // Rule 2: Any liv ecell with two or three live neighbors
                        // lives on to the next generation
                        (true, 2) | (true, 3) => true,
                        // Rule 3: Any live cell with more than three live neighbors
                        // dies, as if by overpopulation
                        (true, x) if x > 3 => false,
                        // Rule 4: Any dead cell with exactly three live dead neighbors
                        // becomes a live cell, as if by reproduction
                        (false, 3) => true,
                        // All other cells remain in the same state
                        (otherwise, _) => otherwise,
                    },
                );
            }
        }

        self.cells = next;
        self.store_and_compare()
    }
}
