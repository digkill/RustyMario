use macroquad::prelude::*;

mod player;
mod map;
mod enemy;
mod build;
mod explosion;

use enemy::Enemy;
use explosion::Explosion;
use std::rc::Rc;
use macroquad::audio::play_sound_once;
use macroquad::audio::load_sound;
use macroquad::audio::Sound;

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


    let explosion_sound = load_sound("assets/explosion.wav").await.unwrap();
    // Загружаем текстуру врага
    let enemy_texture: Rc<Texture2D> = Rc::new(load_texture("assets/enemy_64x64.png").await.unwrap());
    enemy_texture.set_filter(FilterMode::Nearest);

    // Загружаем текстуру взрыва
    let explosion_texture = load_texture("assets/explosion_64x64.png").await.unwrap();
    explosion_texture.set_filter(FilterMode::Nearest);

    let mut explosions: Vec<Explosion> = vec![];

    let mut enemies = vec![
        Enemy::new(vec2(150.0, 350.0), enemy_texture.clone()),
        Enemy::new(vec2(250.0, 350.0), enemy_texture.clone()),
        Enemy::new(vec2(350.0, 350.0), enemy_texture.clone()),
    ];

    // Загружаем игрока
    let mut player = player::Player::new();
    player.load().await;

    // Загружаем карту
    let tilemap = map::TileMap::new();


    // Проверка "прыжка на врага"


    loop {
        clear_background(SKYBLUE);
        check_enemy_stomp(&mut player, &mut enemies, &mut explosions, explosion_sound.clone());
        // Рисуем карту
        tilemap.draw();

        // Обновляем и рисуем игрока
        player.update(&tilemap);
        player.draw();
        if player.hurt_timer > 0.0 {
            draw_text("😵 Ouch!", player.pos.x - 10.0, player.pos.y - 20.0, 32.0, RED);
        }



        // Обновляем и рисуем врагов
        for enemy in enemies.iter_mut() {
            enemy.update(get_frame_time());
            enemy.draw();

            if enemy.collides_with(player.pos) {
                let player_bottom = player.pos.y + 64.0;
                let enemy_top = enemy.pos.y;

                if player.get_velocity().y <= 0.0 || player_bottom > enemy_top + 20.0 {
                    player.hurt_timer = 1.0;
                }
                draw_text("💢 GLASSES MAN CAME FROM HABR! upyachka's!", 200.0, 50.0, 32.0, RED);
            }

        }

        // Обновляем и рисуем взрывы
        let dt = get_frame_time();
        for exp in explosions.iter_mut() {
            exp.update(dt);
        }
        explosions.retain(|e| !e.finished);
        for exp in explosions.iter() {
            exp.draw(&explosion_texture);
        }

        // UI
        draw_text("< > move, _ jump", 10.0, 20.0, 24.0, DARKGRAY);

        next_frame().await;
    }
}
