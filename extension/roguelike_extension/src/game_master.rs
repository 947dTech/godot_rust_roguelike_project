use godot::prelude::*;
use crate::static_map::StaticMapManager;
use crate::dynamic_map::DynamicMapManager;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct GameMaster {
    static_map_manager: StaticMapManager,
    dynamic_map_manager: DynamicMapManager,

    #[export]
    dungeon_width: i32,
    #[export]
    dungeon_height: i32,
    #[export]
    dungeon_map_1d: Array<i32>,

    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for GameMaster {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            dungeon_width: 100,
            dungeon_height: 100,
            dungeon_map_1d: Array::new(),
            static_map_manager: StaticMapManager::new(100, 100),
            dynamic_map_manager: DynamicMapManager::new(),
            base,
        }
    }

    fn to_string(&self) -> GString {
        "GameMaster".into()
    }
}


#[godot_api]
impl GameMaster {
    #[func]
    fn new() -> Gd<Self> {
        Gd::from_init_fn(|base| {
            Self {
                dungeon_width: 100,
                dungeon_height: 100,
                dungeon_map_1d: Array::new(),
                static_map_manager: StaticMapManager::new(100, 100),
                dynamic_map_manager: DynamicMapManager::new(),
                base,
            }
        })
    }

    #[func]
    fn initialize_level(&mut self, width: i32, height: i32) {
        // 静的マップの生成
        self.static_map_manager.generate_dungeon(width, height);
        self.copy_from_static_map_manager();

        // プレイヤーの初期位置を候補からランダムに選択
        let n_position_candidates = self.static_map_manager.room_centers.len();
        if n_position_candidates == 0 {
            return;
        } else if n_position_candidates == 1 {
            self.dynamic_map_manager.player_position = self.static_map_manager.room_centers[0];
            return;
        } else {
            let position_idx = (rand::random::<f32>() * (n_position_candidates - 1) as f32) as usize;
            self.dynamic_map_manager.player_position = self.static_map_manager.room_centers[position_idx];
        }

        // アイテムの初期位置を設定


        // 敵の初期位置を設定

    }

    #[func]
    fn get_player_position(&self) -> Vector2 {
        let mut position = Vector2::ZERO;
        let (x, y) = self.dynamic_map_manager.player_position;
        position.x = x as f32;
        position.y = y as f32;
        position
    }

    // StaticMapManagerのdungeon_map_2dをコピーしてGodotからアクセスできるdungeon_map_1dにセットする
    // これは一度作成したら変わらないので、exportした変数にアクセスしてもらう
    fn set_tile(&mut self, x: i32, y: i32, tile: i32) {
        self.dungeon_map_1d.set(
            (y * self.dungeon_width + x).try_into().unwrap(),
            tile);
    }

    fn copy_from_static_map_manager(&mut self) {
        self.dungeon_width = self.static_map_manager.dungeon_width;
        self.dungeon_height = self.static_map_manager.dungeon_height;
        self.dungeon_map_1d.resize((self.dungeon_width * self.dungeon_height).try_into().unwrap(), &0);
        for y in 0..self.dungeon_height {
            for x in 0..self.dungeon_width {
                self.set_tile(x, y, self.static_map_manager.dungeon_map_2d[x as usize][y as usize]);
            }
        }
    }

}