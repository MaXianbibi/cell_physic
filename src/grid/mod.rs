use macroquad::prelude::*;



const BG_COLOR: Color = Color::new(0.0, 0.0, 0.0, 0.0);
const GRID_COLOR: Color = Color::new(169.0, 169.0, 169.0, 0.5);


pub struct Grid {
    width: u32,
    height: u32,
    repetition: u32,
    render_target: RenderTarget,
}


impl Grid {
    pub fn new() -> Self {
        let render_target = render_target(screen_width() as u32, screen_height() as u32);
        let width = render_target.texture.width() as u32;
        let height = render_target.texture.height() as u32;
        let repetition = 5;

        Grid {
            width,
            height,
            repetition,
            render_target,
        }
    }

    pub fn update(&mut self) {
        self.render_target = render_target(screen_width() as u32, screen_height() as u32);
        self.width = self.render_target.texture.width() as u32;
        self.height = self.render_target.texture.height() as u32;

        self.init();
    }

    fn draw(&self) {
        set_camera(&Camera2D {
            zoom: vec2(2.0 / self.width as f32, -2.0 / self.height as f32),
            target: vec2(self.width as f32 / 2.0, self.height as f32 / 2.0),
            render_target: Some(self.render_target.clone()),
            ..Default::default()
        });

        clear_background(BG_COLOR);

        for i in (self.repetition..self.width).step_by(self.repetition as usize) {
            draw_line(i as f32, 0.0, i as f32, self.height as f32, 1.0, GRID_COLOR);
        }

        for j in (self.repetition..self.height).step_by(self.repetition as usize) {
            draw_line(0.0, j as f32, self.width as f32, j as f32, 1.0, GRID_COLOR);
        }

        set_default_camera();
    }

    pub fn init(&self) {
        self.draw();
    }

    pub fn get_texture(&self) -> &Texture2D {
        &self.render_target.texture
    }

  


}

