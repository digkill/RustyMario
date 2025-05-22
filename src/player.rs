use macroquad::prelude::*;
use crate::map::TileMap;

pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    pub on_ground: bool,
    texture: Texture2D,
    frame: usize,
    frame_timer: f32,
    pub hurt_timer: f32,
    pub health: i32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: vec2(64.0, 100.0),
            vel: vec2(0.0, 0.0),
            on_ground: false,
            texture: Texture2D::empty(),
            frame: 0,
            frame_timer: 0.0,
            hurt_timer: 0.0,
            health: 3,
        }
    }

    pub fn damage(&mut self) {
        if self.hurt_timer <= 0.0 {
            self.health -= 1;
            self.hurt_timer = 1.0;
        }
    }

    pub async fn load(&mut self) {
        self.texture = load_texture("assets/player_64x64.png").await.unwrap();
        self.texture.set_filter(FilterMode::Nearest);
    }

    pub fn update(&mut self, map: &TileMap) {
        let dt = get_frame_time();
        let gravity = 600.0;
        let jump_force = -350.0;
        let speed = 200.0;

        let mut moving = false;
        self.vel.x = 0.0;

        if is_key_down(KeyCode::Left) {
            self.vel.x -= speed;
            moving = true;
        }
        if is_key_down(KeyCode::Right) {
            self.vel.x += speed;
            moving = true;
        }

        if is_key_pressed(KeyCode::Space) && self.on_ground {
            self.vel.y = jump_force;
            self.on_ground = false;
        }

        self.vel.y += gravity * dt;
        self.pos += self.vel * dt;

        // Коллизия с полом
        if map.is_solid(self.pos.x + 16.0, self.pos.y + 32.0) {
            self.pos.y = (self.pos.y / 32.0).floor() * 32.0;
            self.vel.y = 0.0;
            self.on_ground = true;
        } else {
            self.on_ground = false;
        }

        // Анимация
        if moving {
            self.frame_timer += dt;
            if self.frame_timer >= 0.1 {
                self.frame = (self.frame + 1) % 4;
                self.frame_timer = 0.0;
            }
        } else {
            self.frame = 0;
        }

        let tile_x = (self.pos.x / 64.0) as usize;
        let tile_y = (self.pos.y / 64.0) as usize;

        if map.is_hurt_tile(tile_x, tile_y) {
            self.damage();
        }
    }

    pub fn draw(&self) {
        let src = Rect::new(self.frame as f32 * 40.0, 0.0, 40.0, 64.0);
        draw_texture_ex(
            &self.texture,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(64.0, 64.0)),
                source: Some(src),
                ..Default::default()
            },
        );
    }

    pub fn get_velocity(&self) -> Vec2 {
        self.vel
    }
    

}

