use macroquad::prelude::*;

use macroquad::rand::gen_range;

const CELLS_COLOR: Color = Color::new(255.0, 0.0, 0.0, 1.0);

#[derive(Clone, Copy, PartialEq)] 
pub enum CellState {
    EMPTY,
    FILLED,
}

pub type CellGrid = Vec<Vec<CellState>>;

pub struct CellMap {
    cells: CellGrid,
    cell_width: u32,
}

impl CellMap {
    pub fn new() -> Self {
        let cell_width: u32 = 5;
        let n_cell_width: u32 = screen_width() as u32 / cell_width;
        let n_cell_height: u32 = screen_height() as u32 / cell_width;

        let cell_map: CellGrid =
            vec![vec![CellState::EMPTY; n_cell_width as usize]; n_cell_height as usize];

        CellMap {
            cells: cell_map,
            cell_width: cell_width,
        }
    }

    pub fn add_cell(&mut self, x: u32, y: u32) {
        let x: usize = x as usize / self.cell_width as usize;
        let y: usize = y as usize / self.cell_width as usize;

        if x >= self.cells[0].len() || y >= self.cells.len() {
            return;
        }
        if self.cells[y][x] == CellState::FILLED {
            return;
        }

        self.cells[y][x] = CellState::FILLED;
    }

    pub fn draw_cells(&self) {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    CellState::FILLED => {
                        let red_intensity = (1.0 - 0.01 * (y as f32) + 0.5).clamp(0.0, 1.0); 

                        let red_color: Color = Color::new(
                            red_intensity, 
                            0.0,           
                            0.0,           
                            1.0,           
                        );
                        draw_rectangle(
                            x as f32 * self.cell_width as f32,
                            y as f32 * self.cell_width as f32,
                            self.cell_width as f32,
                            self.cell_width as f32,
                            red_color,
                        );
                    }
                    CellState::EMPTY => {}
                }
            }
        }
    }

    pub fn process(&mut self) {
        let mut changes: Vec<((usize, usize), (usize, usize))> = Vec::new();

        for y in (0..self.cells.len()).rev() {
            for x in 0..self.cells[y].len() {
                match self.cells[y][x] {
                    CellState::FILLED => {
                        if y + 1 < self.cells.len() && self.cells[y + 1][x] == CellState::EMPTY {
                            changes.push(((x, y), (x, y + 1)));
                        } else if y + 1 < self.cells.len() {
                            let stay_there = gen_range(0, 2);
                            if stay_there == 0 {
                                continue;
                            }

                            let direction = gen_range(0, 2); // 0 pour gauche, 1 pour droite
                            let mut moved = false;

                            if direction == 0 {
                                if x > 0 && self.cells[y + 1][x - 1] == CellState::EMPTY {
                                    changes.push(((x, y), (x - 1, y + 1)));
                                    moved = true;
                                }
                            }

                            if direction == 1 || !moved {
                                if x + 1 < self.cells[y].len()
                                    && self.cells[y + 1][x + 1] == CellState::EMPTY
                                {
                                    changes.push(((x, y), (x + 1, y + 1)));
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        for ((x1, y1), (x2, y2)) in changes {
            self.cells[y1][x1] = CellState::EMPTY;
            self.cells[y2][x2] = CellState::FILLED;
        }
    }

    pub fn fill_circle(&mut self, center_x: u32, center_y: u32, radius: u32) {
        let radius_squared: i32 = (radius * radius) as i32;

        for y in (center_y as i32 - radius as i32)..=(center_y as i32 + radius as i32) {
            for x in (center_x as i32 - radius as i32)..=(center_x as i32 + radius as i32) {
                let dx = x - center_x as i32;
                let dy = y - center_y as i32;

                if dx * dx + dy * dy <= radius_squared {
                    self.add_cell(x as u32, y as u32);
                }
            }
        }
    }
}
