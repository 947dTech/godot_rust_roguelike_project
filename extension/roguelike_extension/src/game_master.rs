use godot::prelude::*;
use crate::player::Direction;
use crate::item::GameItem;
use crate::item::HealthPotion;
use crate::item::DroppedItem;
use crate::item::SideEffect;
use crate::mob::GameMob;
use crate::static_map::StaticMapManager;
use crate::dynamic_map::DynamicMapManager;

use std::rc::Rc;
use std::cell::RefCell;

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

    player_attack_info: Vec<(i32, i32, i32)>,
    player_side_effect_info: Vec<SideEffect>,

    mob_attack_info: Vec<(i32, i32, i32)>,
    mob_side_effect_info: Vec<SideEffect>,

    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for GameMaster {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            dungeon_width: 100,
            dungeon_height: 100,
            dungeon_map_1d: Array::new(),
            player_attack_info: vec![],
            player_side_effect_info: vec![],
            mob_attack_info: vec![],
            mob_side_effect_info: vec![],
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
                player_attack_info: vec![],
                player_side_effect_info: vec![],
                mob_attack_info: vec![],
                mob_side_effect_info: vec![],
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
                    let ditem = DroppedItem {
                        id: item_count as i32,
                        position: (x, y),
                        item: item
                    };
                    self.dynamic_map_manager.item_list.push(Rc::new(ditem));
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
                    let mob = GameMob::new(mob_count as i32, x, y);
                    self.dynamic_map_manager.mob_list.push(RefCell::new(mob));
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
        // まず移動先がstatic_map上でfreeであることを確認
        if (self.static_map_manager.dungeon_map_2d[next_position.x as usize][next_position.y as usize] == 0) {
            // 次に移動先にmobがいないことを確認
            let mut mob_exist = false;
            for mob in &self.dynamic_map_manager.mob_list {
                if mob.borrow().position == (next_position.x, next_position.y) {
                    mob_exist = true;
                    break;
                }
            }
            if !mob_exist {
                self.dynamic_map_manager.player.position = (next_position.x, next_position.y);
                result = true;
            }
        }
        result
    }

    // playerに攻撃を指示
    #[func]
    fn player_attack(&mut self) {
        self.player_attack_info.clear();
        self.dynamic_map_manager.player.attack(&mut self.player_attack_info);
        // プレイヤーから帰ってきた攻撃情報を保存
        for (x, y, damage) in &self.player_attack_info {
            godot_print!("Player Attack: x: {}, y: {}, damage: {}", x, y, damage);
        }
    }

    // playerにアイテムを拾うよう指示
    #[func]
    fn player_pickup_item(&mut self) {
        let (x, y) = self.dynamic_map_manager.player.position;
        let mut item_idx = None;
        for (idx, item) in self.dynamic_map_manager.item_list.iter().enumerate() {
            if item.position == (x, y) {
                item_idx = Some(idx);
                break;
            }
        }
        if let Some(idx) = item_idx {
            let item = self.dynamic_map_manager.item_list.remove(idx);
            self.dynamic_map_manager.player.add_item(item.item);
        }
    }

    // playerにアイテムを使うよう指示
    #[func]
    fn player_use_item(&mut self, item_idx: i32) {
        self.player_side_effect_info.clear();
        self.dynamic_map_manager.player.select_item(item_idx as usize);
        self.player_side_effect_info.push(self.dynamic_map_manager.player.use_item());
    }

    // playerのアイテム使用時のsideeffectの反映
    fn applyPlayerSideEffect(&mut self) {
        for (idx, side_effect) in self.player_side_effect_info.iter().enumerate() {
            match side_effect {
                SideEffect::Fault => {
                    godot_print!("Item {} use failed", idx);
                },
                SideEffect::None => {
                    godot_print!("Item {} use success", idx);
                },
            }
        }
    }

    // playerのattack_infoの反映
    fn applyPlayerAttackInfo(&mut self) {
        self.dynamic_map_manager.defeated_mob_id.clear();
        for (x, y, damage) in &self.player_attack_info {
            // モブの位置と一致するものがあればダメージを与える
            let mut mob_idx = None;
            for (idx, mob) in self.dynamic_map_manager.mob_list.iter().enumerate() {
                if mob.borrow().position == (*x, *y) {
                    mob_idx = Some(idx);
                    break;
                }
            }
            if let Some(idx) = mob_idx {
                let id = self.dynamic_map_manager.mob_list[idx].borrow().id;
                godot_print!("Mob {} damaged: {}", id, damage);
                self.dynamic_map_manager.mob_list[idx].borrow_mut().hp -= damage;
                if self.dynamic_map_manager.mob_list[idx].borrow().hp <= 0 {
                    self.dynamic_map_manager.mob_list.remove(idx);
                    self.dynamic_map_manager.defeated_mob_id.push(id);
                    godot_print!("Mob {} defeated.", id);
                }
            }
        }
    }

    // mobの行動を決定
    // TODO: もっと複雑なAIを実装する
    fn decideMobAction(&mut self) {
        let mut mob_next_positions = vec![];
        self.mob_attack_info.clear();

        for mob_rc in &mut self.dynamic_map_manager.mob_list {
            // TODO: 同じ部屋に入ったモブだけがアクティブになるようにする

            let mut mob = mob_rc.borrow_mut();
            // プレイヤーの位置との距離を計算
            let (px, py) = self.dynamic_map_manager.player.position;
            let (mx, my) = mob.position;
            let dx = px - mx;
            let dy = py - my;
            let abs_dx = dx.abs();
            let abs_dy = dy.abs();
            // プレイヤーに隣接している場合は攻撃
            if abs_dx <= 1 && abs_dy <= 1 {
                let mut attack_info = vec![];
                // mobのdirectionをプレイヤーに向ける
                if dx > 0 {
                    if dy > 0 {
                        mob.direction = Direction::DownRight;
                    } else if dy < 0 {
                        mob.direction = Direction::UpRight;
                    } else {
                        mob.direction = Direction::Right;
                    }
                } else if dx < 0 {
                    if dy > 0 {
                        mob.direction = Direction::DownLeft;
                    } else if dy < 0 {
                        mob.direction = Direction::UpLeft;
                    } else {
                        mob.direction = Direction::Left;
                    }
                } else {
                    if dy > 0 {
                        mob.direction = Direction::Down;
                    } else if dy < 0 {
                        mob.direction = Direction::Up;
                    }
                }
                mob.attack(&mut attack_info);
                for (x, y, damage) in &attack_info {
                    self.mob_attack_info.push((*x, *y, *damage));
                }
            } else {
                // そうでなければプレイヤーの方向に移動
                // 移動したい位置を決めておいて、そのあとで実際移動できるかどうかを確認
                let mut next_position = (mx, my);
                if abs_dx > abs_dy {
                    if dx > 0 {
                        next_position = (mx + 1, my);
                    } else {
                        next_position = (mx - 1, my);
                    }
                } else {
                    if dy > 0 {
                        next_position = (mx, my + 1);
                    } else {
                        next_position = (mx, my - 1);
                    }
                }
                // static_map上で空きがあれば移動候補に入れる
                if self.static_map_manager.dungeon_map_2d[next_position.0 as usize][next_position.1 as usize] == 0 {
                    mob_next_positions.push((mob.id, next_position));
                }
            }
        }

        for (id, next_position) in &mob_next_positions {
            // mob_listの中のmobを全部読みだして
            // mob.idが一致するものは自分なので一度無視
            // それ以外のmobは、next_positionと一致しないかどうかを確認
            // 一致するものがあれば移動しない
            let mut can_move = true;
            for mob_rc in &self.dynamic_map_manager.mob_list {
                let mob = mob_rc.borrow_mut();
                if mob.id == *id {
                    continue;
                }
                if mob.position == *next_position {
                    can_move = false;
                    break;
                }
            }
            if can_move {
                for mob_rc in &mut self.dynamic_map_manager.mob_list {
                    let mut mob = mob_rc.borrow_mut();
                    if mob.id == *id {
                        mob.position = *next_position;
                        break;
                    }
                }
            }
        }
    }

    // mobのアイテム使用時のsideeffectの反映
    fn applyMobSideEffect(&mut self) {
        for (idx, side_effect) in self.mob_side_effect_info.iter().enumerate() {
            match side_effect {
                SideEffect::Fault => {
                    godot_print!("Mob {} item use failed", idx);
                },
                SideEffect::None => {
                    godot_print!("Mob {} item use success", idx);
                },
            }
        }
    }

    // mobのattack_infoの反映
    fn applyMobAttackInfo(&mut self) {
        for (x, y, damage) in &self.mob_attack_info {
            // プレイヤーの位置と一致するものがあればダメージを与える
            if self.dynamic_map_manager.player.position == (*x, *y) {
                self.dynamic_map_manager.player.hp -= damage;
                if self.dynamic_map_manager.player.hp <= 0 {
                    // ゲームオーバー
                    godot_print!("Game Over");
                }
            }
        }
    }

    // 1ターンを定義、godot側から進めるかどうかを決めて呼び出す。
    #[func]
    fn process(&mut self) {
        // プレイヤーの行動はすでに反映された状態を起点とする。
        // プレイヤーのアイテム使用時のsideeffectの反映
        self.applyPlayerSideEffect();
        // プレイヤーのattack_infoの反映
        self.applyPlayerAttackInfo();

        // TODO: モブの行動を決定
        self.decideMobAction();

        // モブのアイテム使用時のsideeffectの反映
        self.applyMobSideEffect();
        // モブのattack_infoの反映
        self.applyMobAttackInfo();

        // TODO: ターンの処理の結果、起きた結果をgodotに通知
        // プレイヤーのHP<=0でゲーム終了
        // モブのHP<=0でそのモブは削除
        // プレイヤーがアイテムを拾った場合、そのアイテムはマップ上からは削除
        // etc...
        match self.dynamic_map_manager.player.direction {
            Direction::Up => godot_print!("Player Direcction: up"),
            Direction::UpRight => godot_print!("Player Direcction: up right"),
            Direction::Right => godot_print!("Player Direcction: right"),
            Direction::DownRight => godot_print!("Player Direcction: down right"),
            Direction::Down => godot_print!("Player Direcction: down"),
            Direction::DownLeft => godot_print!("Player Direcction: down left"),
            Direction::Left => godot_print!("Player Direcction: left"),
            Direction::UpLeft => godot_print!("Player Direcction: up left"),
            _ => godot_print!("Player Direcction: up"),
        }
        godot_print!("Player HP: {}", self.dynamic_map_manager.player.hp);
        for mob_rc in &self.dynamic_map_manager.mob_list {
            let mob = mob_rc.borrow();
            godot_print!("Mob {} HP: {}", mob.id, mob.hp);
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

    // 敵の情報を取得する関数群
    // 敵の位置を取得
    #[func]
    fn get_mob_positions(&self) -> Array<Vector2i> {
        let mut positions = array![];
        for mob_rc in &self.dynamic_map_manager.mob_list {
            let mob = mob_rc.borrow();
            positions.push(Vector2i::new(mob.position.0, mob.position.1));
        }
        positions
    }

    // 敵のIDを取得
    #[func]
    fn get_mob_ids(&self) -> Array<i32> {
        let mut ids = array![];
        for mob_rc in &self.dynamic_map_manager.mob_list {
            let mob = mob_rc.borrow();
            ids.push(mob.id);
        }
        ids
    }

    // このターンに倒された敵のIDを取得
    #[func]
    fn get_defeated_mob_ids(&self) -> Array<i32> {
        let mut ids = array![];
        for id in &self.dynamic_map_manager.defeated_mob_id {
            ids.push(*id);
        }
        ids
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