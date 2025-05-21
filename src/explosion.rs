use macroquad::prelude::*;

pub struct Explosion {
    pub pos: Vec2,
    pub timer: f32,
    pub frame: usize,
    pub finished: bool,
}

impl Explosion {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            timer: 0.0,
            frame: 0,
            finished: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.timer += dt;
        if self.timer > 0.1 {
            self.frame += 1;
            self.timer = 0.0;

            if self.frame >= 5 {
                self.finished = true;
            }
        }
    }

    pub fn draw(&self, texture: &Texture2D) {
        let src = Rect::new(self.frame as f32 * 64.0, 0.0, 64.0, 64.0);
        draw_texture_ex(
            texture,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(src),
                dest_size: Some(vec2(64.0, 64.0)),
                ..Default::default()
            },
        );
    }
}
