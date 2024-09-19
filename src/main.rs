mod grid;
mod cell_map;
mod physic;

use grid::*;
use macroquad::prelude::*;
use cell_map::*;


fn window_conf() -> Conf {
    Conf {
        window_title: "Cell Physic".to_owned(),
        window_width: 1400,
        window_height: 700,
        ..Default::default()
    }
}

fn draw_back(grid: &Grid) {
    clear_background(BLACK);
    draw_texture(grid.get_texture(), 0.0, 0.0, WHITE);
    let fps: String = format!("FPS: {}", get_fps());
    draw_text(&fps, 20.0, 20.0, 30.0, BLUE);
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut last_height: u32 = screen_height() as u32;
    let mut last_width: u32 = screen_width() as u32;
    next_frame().await;

    let mut last_update_time = get_time();
    let update_interval = 1.0 / 60.0; // 5 fois par seconde



    let mut grid: Grid = Grid::new();
    grid.init();

    let mut cell_map: CellMap = CellMap::new();



    loop {
        if (screen_height() as u32 != last_height) || (screen_width() as u32 != last_width) {
            grid.update();
            last_height = screen_height() as u32;
            last_width = screen_width() as u32;
        }


        draw_back(&grid);

        cell_map.draw_cells();



        if get_time() - last_update_time >= update_interval {
            // cell_map.add_light();

            if is_mouse_button_down(MouseButton::Left) {
                let (x, y) = mouse_position();
                cell_map.fill_circle(x as u32, y as u32, 100);  // Ajout du point-virgule ici

            }





            cell_map.process();
            last_update_time = get_time();
        }

        next_frame().await;
    }
}
