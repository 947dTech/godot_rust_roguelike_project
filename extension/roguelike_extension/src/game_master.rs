use godot::prelude::*;
use crate::player::Direction;
use crate::item::GameItem;
use crate::item::HealthPotion;
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
            self.dynamic_map_manager.player.position = self.static_map_manager.room_centers[0];
            return;
        } else {
            let position_idx = (rand::random::<f32>() * (n_position_candidates - 1) as f32) as usize;
            self.dynamic_map_manager.player.position = self.static_map_manager.room_centers[position_idx];
        }

        // アイテムの初期位置を設定


        // 敵の初期位置を設定

    }

    // gamemasterはplayerに関する情報をgodotに渡す
    // playerの位置
    #[func]
    fn get_player_position(&self) -> Vector2i {
        let mut position = Vector2i::ZERO;
        let (x, y) = self.dynamic_map_manager.player.position;
        position.x = x;
        position.y = y;
        position
    }

    // playerの向き
    #[func]
    fn get_player_direction(&self) -> i32 {
         match self.dynamic_map_manager.player.direction {
            Direction::Up => 0,
            Direction::UpRight => 1,
            Direction::Right => 2,
            Direction::DownRight => 3,
            Direction::Down => 4,
            Direction::DownLeft => 5,
            Direction::Left => 6,
            Direction::UpLeft => 7,
            _ => 0,
        }
    }

    // playerに向きを指示
    #[func]
    fn player_turn(&mut self, direction: i32) {
        let player_dir = match direction {
            0 => Direction::Up,
            1 => Direction::UpRight,
            2 => Direction::Right,
            3 => Direction::DownRight,
            4 => Direction::Down,
            5 => Direction::DownLeft,
            6 => Direction::Left,
            7 => Direction::UpLeft,
            _ => Direction::Up,
        };
        self.dynamic_map_manager.player.direction = player_dir;
    }

    // playerに移動を指示
    #[func]
    fn player_move(&mut self, next_position: Vector2i) -> bool {
        let mut result = false;
        if (self.static_map_manager.dungeon_map_2d[next_position.x as usize][next_position.y as usize] == 0) {
            self.dynamic_map_manager.player.position = (next_position.x, next_position.y);
            result = true;
        }
        result
    }

    // playerに攻撃を指示
    #[func]
    fn player_attack(&mut self) {
        let attack_info = self.dynamic_map_manager.player.attack();
        // TODO: プレイヤーから帰ってきた攻撃情報をモブに反映
        for (x, y, damage) in attack_info {
            godot_print!("Player Attack: x: {}, y: {}, damage: {}", x, y, damage);
        }
    }

    // デバッグ用、プレイヤーのステータスを表示
    #[func]
    fn print_player_status(&self) {
        godot_print!(
            "Player Status: HP: {} / {}, Attack: {}, Defense: {}",
            self.dynamic_map_manager.player.hp,
            self.dynamic_map_manager.player.max_hp,
            self.dynamic_map_manager.player.attack,
            self.dynamic_map_manager.player.defense);
    }

    // デバッグ用、プレイヤーの所持品を表示
    #[func]
    fn print_player_items(&self) {
        godot_print!("Player Items:");
        for item in &self.dynamic_map_manager.player.items {
            godot_print!("{:?}", item);
        }
    }

    // デバッグ用、プレイヤーに回復ポーションをわたし、
    // それを使ってHPを回復させる
    #[func]
    fn give_health_potion_to_player(&mut self) {
        godot_print!("Give Health Potion to Player");
        let potion = GameItem::HealthPotion(HealthPotion {heal_amount: 10});
        self.dynamic_map_manager.player.add_item(potion);
        self.print_player_items();
        self.print_player_status();
        godot_print!("Select item index 0");
        self.dynamic_map_manager.player.select_item(0);
        godot_print!("Use item");
        self.dynamic_map_manager.player.use_item();
        godot_print!("After using item");
        self.print_player_items();
        self.print_player_status();
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