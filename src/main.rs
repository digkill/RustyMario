use macroquad::prelude::*;

mod build;
mod enemy;
mod explosion;
mod map;
mod player;

use enemy::Enemy;
use explosion::Explosion;
use macroquad::audio::load_sound;
use macroquad::audio::play_sound_once;
use macroquad::audio::Sound;
use std::rc::Rc;

fn check_enemy_stomp(
    player: &mut player::Player,
    enemies: &mut Vec<Enemy>,
    explosions: &mut Vec<Explosion>,
    explosion_sound: Sound,
) {
    let mut to_remove = vec![];

    for (i, enemy) in enemies.iter().enumerate() {
        let enemy_rect = Rect::new(enemy.pos.x, enemy.pos.y, 64.0, 64.0);
        let player_rect = Rect::new(player.pos.x, player.pos.y, 40.0, 64.0);

        if player_rect.overlaps(&enemy_rect) {
            let player_bottom = player.pos.y + 64.0;
            let enemy_top = enemy.pos.y;

            if player.get_velocity().y > 0.0 && player_bottom <= enemy_top + 20.0 {
                to_remove.push(i);
                player.vel.y = -250.0;
                explosions.push(Explosion::new(enemy.pos));
                play_sound_once(&explosion_sound);
            }
        }
    }

    for i in to_remove.iter().rev() {
        enemies.remove(*i);
    }
}

#[macroquad::main("Rusty Mario")]
async fn main() {
    let mut screen_shake: f32 = 0.0;
    let explosion_sound = load_sound("assets/explosion.wav").await.unwrap();
    // Загружаем текстуру врага
    let enemy_texture: Rc<Texture2D> =
        Rc::new(load_texture("assets/enemy_64x64.png").await.unwrap());
    enemy_texture.set_filter(FilterMode::Nearest);

    // Загружаем текстуру взрыва
    let explosion_texture = load_texture("assets/explosion_64x64.png").await.unwrap();
    explosion_texture.set_filter(FilterMode::Nearest);

    let mut explosions: Vec<Explosion> = vec![];

    let mut enemies = vec![
        Enemy::new(vec2(150.0, 350.0), enemy_texture.clone()),
        Enemy::new(vec2(250.0, 150.0), enemy_texture.clone()),
        Enemy::new(vec2(350.0, 250.0), enemy_texture.clone()),
    ];

    // Загружаем игрока
    let mut player = player::Player::new();
    player.load().await;

    // Загружаем карту
    let mut tilemap = map::TileMap::new();
    tilemap.load().await;

    loop {
        let shake_offset = if screen_shake > 0.0 {
            screen_shake -= get_frame_time() * 50.0;
            vec2(rand::gen_range(-2.0, 2.0), rand::gen_range(-2.0, 2.0))
        } else {
            vec2(0.0, 0.0)
        };
        clear_background(SKYBLUE);
        check_enemy_stomp(
            &mut player,
            &mut enemies,
            &mut explosions,
            explosion_sound.clone(),
        );
        // Рисуем карту
        tilemap.draw();

        // Обновляем и рисуем игрока
        player.update(&tilemap);
        player.draw();

        let tile_x = (player.pos.x / 30.0) as usize;
        let tile_y = (player.pos.y / 30.0) as usize;

        if tilemap.is_bomb_tile(tile_x, tile_y) {
            explosions.push(Explosion::new(player.pos));
            play_sound_once(&explosion_sound);
            player.health = 0;
            screen_shake = 10.0;

            for exp in explosions.iter_mut() {
                exp.update(get_frame_time());
            }
            explosions.retain(|e| !e.finished);

            for exp in explosions.iter() {
                exp.draw(&explosion_texture, shake_offset);
            }
        }
        if player.health <= 0 {
            draw_text(
                "Game Over",
                screen_width() / 2.0 - 100.0,
                screen_height() / 2.0,
                48.0,
                RED,
            );
            next_frame().await;
            continue;
        }

        if player.hurt_timer > 0.0 {
            draw_text("Ouch!", player.pos.x - 10.0, player.pos.y - 20.0, 32.0, RED);
        }

        // Обновляем и рисуем врагов
        for enemy in enemies.iter_mut() {
            enemy.update(get_frame_time());
            enemy.draw();

            if enemy.collides_with(player.pos) {
                let player_bottom = player.pos.y + 64.0;
                let enemy_top = enemy.pos.y;

                if player.get_velocity().y <= 0.0 || player_bottom > enemy_top + 20.0 {
                    player.damage();
                }

                draw_text(
                    "GLASSES MAN CAME FROM HABR! upyachka's!",
                    200.0,
                    50.0,
                    32.0,
                    RED,
                );
            } 
        }

        // Обновляем и рисуем взрывы
        let dt = get_frame_time();
        for exp in explosions.iter_mut() {
            exp.update(dt);
        }
        explosions.retain(|e| !e.finished);
        for exp in explosions.iter() {
            exp.draw(&explosion_texture, shake_offset);
        }

        // UI
        draw_text("< > move, _ jump", 10.0, 20.0, 24.0, DARKGRAY);

        // UI: отрисовка полосы здоровья
        let bar_width = 200.0;
        let health_percent = player.health as f32 / 3.0;
        draw_rectangle(10.0, 40.0, bar_width, 20.0, GRAY);
        draw_rectangle(10.0, 40.0, bar_width * health_percent, 20.0, RED);
        draw_text("HP", 10.0, 38.0, 20.0, BLACK);

        next_frame().await;
    }
}
