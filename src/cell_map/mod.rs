use macroquad::prelude::*;

use macroquad::rand::gen_range;


const CELLS_COLOR: Color = Color::new(255.0, 0.0, 0.0, 1.0);

#[derive(Clone, Copy, PartialEq)] // Ajout des traits Clone, Copy et PartialEq
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

                        let red_intensity = (1.0 - 0.01 * (y as f32) + 0.5).clamp(0.0, 1.0);  // La composante rouge varie entre 1.0 et 0.0

                        let red_color: Color = Color::new(
                            red_intensity,  // Intensity de rouge dans la plage [0.0, 1.0]
                            0.0,            // Vert (pas de vert)
                            0.0,            // Bleu (pas de bleu)
                            1.0,            // Opacité
                        );
                        draw_rectangle(
                            x as f32 * self.cell_width as f32,
                            y as f32 * self.cell_width as f32,
                            self.cell_width as f32,
                            self.cell_width as f32,
                            red_color
                        );
                    }
                    CellState::EMPTY => {}
                }
            }
        }
    }

    pub fn process(&mut self) {
        // Stocker les changements à appliquer après le parcours de la grille
        let mut changes: Vec<((usize, usize), (usize, usize))> = Vec::new();

        // Parcourir la grille de bas en haut
        for y in (0..self.cells.len()).rev() {
            for x in 0..self.cells[y].len() {
                match self.cells[y][x] {
                    CellState::FILLED => {
                        // Vérifier si on peut tomber directement
                        if y + 1 < self.cells.len() && self.cells[y + 1][x] == CellState::EMPTY {
                            changes.push(((x, y), (x, y + 1)));
                        } else if y + 1 < self.cells.len() {
                            let stay_there = gen_range(0, 2);
                            if stay_there == 0 {
                                continue;
                            }

                            // Générer aléatoirement un choix entre aller à gauche ou à droite
                            let direction = gen_range(0, 2); // 0 pour gauche, 1 pour droite
                            let mut moved = false;

                            if direction == 0 {
                                // Vérifier d'abord à gauche
                                if x > 0 && self.cells[y + 1][x - 1] == CellState::EMPTY {
                                    changes.push(((x, y), (x - 1, y + 1)));
                                    moved = true;
                                }
                            }

                            if direction == 1 || !moved {
                                // Vérifier à droite (ou si gauche a échoué)
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

        // Appliquer les changements en une seule passe
        for ((x1, y1), (x2, y2)) in changes {
            self.cells[y1][x1] = CellState::EMPTY;
            self.cells[y2][x2] = CellState::FILLED;
        }
    }

    pub fn fill_circle(&mut self, center_x: u32, center_y: u32, radius: u32) {
        let radius_squared = (radius * radius) as i32;

        // Parcourir les cellules dans la zone carrée entourant le cercle
        for y in (center_y as i32 - radius as i32)..=(center_y as i32 + radius as i32) {
            for x in (center_x as i32 - radius as i32)..=(center_x as i32 + radius as i32) {
                // Vérifier que la position est bien dans les limites du tableau

           
                    // Calculer la distance au centre du cercle
                    let dx = x - center_x as i32;
                    let dy = y - center_y as i32;

                    // Vérifier si le point est dans le cercle
                    if dx * dx + dy * dy <= radius_squared {
                        self.add_cell(x as u32, y as u32);}
           
            }
        }
    }

    fn look_around_cell(&self, x: usize, y: usize) -> u32 {
        let mut count = 0;
    
        // Rayon de la brume autour de chaque cellule
        let radius = 3;
    
        for y_offset in 0..(radius * 2 + 1) {
            for x_offset in 0..(radius * 2 + 1) {
                let x_adj = x as i32 + x_offset - radius as i32;
                let y_adj = y as i32 + y_offset - radius as i32;
    
                // Vérifier les limites de la grille
                if x_adj < 0 || y_adj < 0 || x_adj >= self.cells[0].len() as i32 || y_adj >= self.cells.len() as i32 {
                    continue;
                }
    
                // Calculer la distance à la cellule centrale
                let distance = (((x_offset - radius) as f32).powi(2) + ((y_offset - radius) as f32).powi(2)).sqrt();
    
                // Appliquer une fonction de réduction en fonction de la distance
                let max_distance = radius as f32;
                let intensity = (1.0 - distance / max_distance).clamp(0.0, 1.0);
    
                // Si la cellule adjacente est vide, appliquer l'effet de brume/contraste
                if self.cells[y_adj as usize][x_adj as usize] == CellState::EMPTY {
                    let red_intensity = (1.0 - 0.01 * (y_adj as f32) * intensity).clamp(0.0, 1.0);  // La composante rouge varie avec l'intensité
    
                    let red_color: Color = Color::new(
                        red_intensity,  // Intensité de rouge basée sur la distance
                        0.0,            // Pas de vert
                        0.0,            // Pas de bleu
                        intensity * 0.3,  // Transparence, plus opaque près de la cellule centrale
                    );
                    draw_rectangle(
                        x_adj as f32 * self.cell_width as f32,
                        y_adj as f32 * self.cell_width as f32,
                        self.cell_width as f32,
                        self.cell_width as f32,
                        red_color
                    );
                }
            }
        }
    
        count
    }
    
    pub fn add_light(&mut self) {
        for y in (0..self.cells.len()).rev() {
            for x in 0..self.cells[y].len() {
                if self.cells[y][x] == CellState::EMPTY {
                    continue;
                }
                self.look_around_cell(x as usize, y as usize);
            }
        }
    }
    

}
