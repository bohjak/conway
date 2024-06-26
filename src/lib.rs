use std::fmt;

use wasm_bindgen::prelude::*;

// TODO: read more about macros
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
macro_rules! err {
    ( $( $t:tt )* ) => {
        web_sys::console::error_1(&format!( $( $t )* ).into());
    }
}

fn now() -> f64 {
    web_sys::window()
        .expect("running on server")
        .performance()
        .expect("no Performance")
        .now()
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        }
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Self {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
        log!("Let there be light!");
        let cells = (0..width * height).map(|_| Cell::Dead).collect();
        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn scramble(&mut self) {
        let cells = (0..self.width * self.height)
            .map(|_| {
                if js_sys::Math::random() > 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        self.cells = cells;
    }

    pub fn clear(&mut self) {
        let cells = (0..self.width * self.height).map(|_| Cell::Dead).collect();

        self.cells = cells;
    }

    pub fn toggle_cell(&mut self, x: u32, y: u32) {
        let idx = self.get_index(x, y);
        self.cells[idx].toggle();
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, w: u32) {
        self.width = w;
        self.cells = (0..w * self.height).map(|_| Cell::Dead).collect();
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, h: u32) {
        self.height = h;
        self.cells = (0..self.width * h).map(|_| Cell::Dead).collect();
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        (x + self.width * y) as usize
    }

    fn live_neighbor_count(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;

        // TODO: figure out how this works
        for dx in [self.height - 1, 0, 1].iter().cloned() {
            for dy in [self.width - 1, 0, 1].iter().cloned() {
                if dx == 0 && dy == 0 {
                    continue;
                }

                // TODO: justify why this works for the first row and column
                let nx = (x + dx) % self.width;
                let ny = (y + dy) % self.height;
                count += self.cells[self.get_index(nx, ny)] as u8;
            }
        }

        count
    }

    pub fn next(&mut self) {
        let mut next = self.cells.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.get_index(x, y);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(x, y);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }
}

impl Universe {
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (y, x) in cells.iter().cloned() {
            let idx = self.get_index(x, y);
            self.cells[idx] = Cell::Alive;
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Alive { '◼' } else { '◻' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
