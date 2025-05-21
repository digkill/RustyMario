use macroquad::prelude::*;

mod player;
mod map;
mod enemy;
mod build;

use enemy::Enemy;
use std::rc::Rc;

#[macroquad::main("Rusty Platformer")]
async fn main() {
    // Загружаем текстуру врага
    let enemy_texture: Rc<Texture2D> = Rc::new(load_texture("assets/enemy_64x64.png").await.unwrap());
    enemy_texture.set_filter(FilterMode::Nearest);

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

    loop {
        clear_background(SKYBLUE);

        // Рисуем карту
        tilemap.draw();

        // Обновляем и рисуем игрока
        player.update(&tilemap);
        player.draw();

        // Обновляем и рисуем врагов
        for enemy in enemies.iter_mut() {
            enemy.update(get_frame_time());
            enemy.draw();

            if enemy.collides_with(player.pos) {
                draw_text("💢 GLASSES MAN CAME FROM HABR! upyachka's!", 200.0, 50.0, 32.0, RED);
            }
        }

        // UI
        draw_text("< > move, _ jump", 10.0, 20.0, 24.0, DARKGRAY);

        next_frame().await;
    }
}
