use macroquad::prelude::*;
use std::rc::Rc;

pub struct Enemy {
    pub pos: Vec2,
    direction: f32,
    speed: f32,
    texture: Rc<Texture2D>,
    frame: usize,
    frame_timer: f32,
}

impl Enemy {
    pub fn new(pos: Vec2, texture: Rc<Texture2D>) -> Self {
        Self {
            pos,
            direction: 1.0,
            speed: 50.0,
            texture,
            frame: 0,
            frame_timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.pos.x += self.direction * self.speed * dt;

        // Простое отражение направления при выходе за границы
        if self.pos.x < 32.0 || self.pos.x > 600.0 {
            self.direction *= -1.0;
        }

        self.frame_timer += dt;
        if self.frame_timer > 0.2 {
            self.frame = (self.frame + 1) % 2;
            self.frame_timer = 0.0;
        }
    }

    pub fn draw(&self) {
        let src = Rect::new(self.frame as f32 * 45.0, 0.0, 38.0, 64.0);
        draw_texture_ex(
            &self.texture,
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

    pub fn collides_with(&self, player_pos: Vec2) -> bool {
        let rect = Rect::new(self.pos.x, self.pos.y, 64.0, 64.0);
        rect.contains(player_pos)
    }


}
