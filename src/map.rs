use macroquad::prelude::*;

pub struct TileMap {
    tiles: Vec<Vec<u8>>,
    bomb_texture: Texture2D,
}

impl TileMap {
    pub fn new() -> Self {

        // 0 — пусто, 1 — платформа, 2 — ловушка
        let tiles = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 1, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];


        Self { tiles, bomb_texture: Texture2D::empty(), }
    }
    
    pub async fn load(&mut self) {
        self.bomb_texture = load_texture("assets/bomb.png").await.unwrap();
        self.bomb_texture.set_filter(FilterMode::Nearest);
    }

    pub fn draw(&self) {

        for (y, row) in self.tiles.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
               // if tile == 1 {
                    // draw_rectangle((x as f32) * 32.0, (y as f32) * 32.0, 32.0, 32.0, DARKGREEN);

                    match tile {
                        1 => draw_rectangle((x as f32) * 32.0, (y as f32) * 32.0, 32.0, 32.0, DARKGREEN),
                        //      1 => draw_texture(&ground_texture,(x as f32) * 32.0, (y as f32) * 32.0, WHITE);
                        2 => draw_texture(&self.bomb_texture, (x as f32) * 30.0, (y as f32) * 30.0, WHITE),

                        _ => {}
                    }
              //  }
            }
        }
    }

    pub fn is_solid(&self, x: f32, y: f32) -> bool {
        let tile_x = (x / 32.0) as usize;
        let tile_y = (y / 32.0) as usize;

        if tile_y >= self.tiles.len() || tile_x >= self.tiles[0].len() {
            return false;
        }

        self.tiles[tile_y][tile_x] == 1
    }

    pub fn is_hurt_tile(&self, x: usize, y: usize) -> bool {
        self.tiles[y][x] == 2
    }

    pub fn is_bomb_tile(&self, x: usize, y: usize) -> bool {
        self.tiles[y][x] == 2
    }
}
