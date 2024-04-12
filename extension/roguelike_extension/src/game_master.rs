use godot::prelude::*;
use crate::player::Direction;
use crate::item::GameItem;
use crate::item::HealthPotion;
use crate::item::DroppedItem;
use crate::mob::GameMob;
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
        let n_position_candidates = self.static_map_manager.room_params.len();
        if n_position_candidates == 0 {
            return;
        } else if n_position_candidates == 1 {
            let param = &self.static_map_manager.room_params[0];
            self.dynamic_map_manager.player.position = 
                (param.room_center_x, param.room_center_y);
            return;
        } else {
            let position_idx = (rand::random::<f32>() * (n_position_candidates - 1) as f32) as usize;
            let param = &self.static_map_manager.room_params[position_idx];
            self.dynamic_map_manager.player.position = 
                (param.room_center_x, param.room_center_y);
        }

        // アイテムの初期位置を設定
        let item_max = 10;
        // 小部屋ごとに均一になるようにアイテムを配置したい
        // アイテムの総数/小部屋の数で小部屋ごとの配置数を決める
        // 端数が出るので、あえて+1している
        let item_per_room = (item_max / self.static_map_manager.room_params.len()) + 1;
        let mut item_count = 0;
        for param in &self.static_map_manager.room_params {
            for _ in 0..item_per_room {
                if item_count >= item_max {
                    break;
                }
                let x = param.x + (rand::random::<f32>() * param.width as f32) as i32;
                let y = param.y + (rand::random::<f32>() * param.height as f32) as i32;
                // 床である場所にのみアイテムを配置
                if (self.static_map_manager.dungeon_map_2d[x as usize][y as usize] == 0) {
                    let item = GameItem::HealthPotion(HealthPotion {heal_amount: 10});
                    self.dynamic_map_manager.item_list.push(DroppedItem {position: (x, y), item: item});
                    item_count += 1;
                }
                // 無限ループを避け、かつアイテム数にランダム性を持たせるため厳密にmaxを狙わない
            }
        }
        godot_print!("{} items generated (max: {})", item_count, item_max);

        // 敵の初期位置を設定
        let mob_max = 10;
        // アイテムと同様の生成方法とする。
        let mob_per_room = (mob_max / self.static_map_manager.room_params.len()) + 1;
        let mut mob_count = 0;
        for param in &self.static_map_manager.room_params {
            for _ in 0..mob_per_room {
                if mob_count >= mob_max {
                    break;
                }
                let x = param.x + (rand::random::<f32>() * param.width as f32) as i32;
                let y = param.y + (rand::random::<f32>() * param.height as f32) as i32;
                // 床である場所にのみモブを配置
                if (self.static_map_manager.dungeon_map_2d[x as usize][y as usize] == 0) {
                    let mob = GameMob::new(x, y);
                    self.dynamic_map_manager.mob_list.push(mob);
                    mob_count += 1;
                }
            }
        }
        godot_print!("{} mobs generated (max: {})", mob_count, mob_max);
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

    // 落ちているアイテムの情報を取得
    #[func]
    fn get_dropped_item_positions(&self) -> Array<Vector2i> {
        let mut positions = array![];
        for item in &self.dynamic_map_manager.item_list {
            positions.push(Vector2i::new(item.position.0, item.position.1));
        }
        positions
    }

    // 敵の情報を取得
    #[func]
    fn get_mob_positions(&self) -> Array<Vector2i> {
        let mut positions = array![];
        for mob in &self.dynamic_map_manager.mob_list {
            positions.push(Vector2i::new(mob.position.0, mob.position.1));
        }
        positions
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