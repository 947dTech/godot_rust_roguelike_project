//! ゲーム全体を管理するモジュール、Godot側からはこのモジュールが呼び出される

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
    /// 静的マップ
    pub static_map_manager: StaticMapManager,
    /// 動的マップ
    pub dynamic_map_manager: DynamicMapManager,

    /// 現在の階層
    #[export]
    pub current_level: i32,

    /// マップの幅
    #[export]
    pub dungeon_width: i32,
    /// マップの高さ
    #[export]
    pub dungeon_height: i32,
    /// 1次元配列でのマップ情報
    #[export]
    pub dungeon_map_1d: Array<i32>,

    /// そのターンに発行されたメッセージ
    #[export]
    pub message: Array<GString>,

    /// 初期配置するアイテムの数
    #[export]
    pub initial_item_count: i32,

    /// 初期配置する敵の数
    #[export]
    pub initial_mob_count: i32,

    /// 敵がアイテムを落とす確率
    #[export]
    pub mob_drop_item_probability: f32,

    /// そのターンにプレイヤーが行った攻撃情報
    pub player_attack_info: Vec<(i32, i32, i32)>,
    /// そのターンにプレイヤーが行ったアイテム使用情報
    pub player_side_effect_info: Vec<SideEffect>,

    /// そのターンに敵が行った攻撃情報
    pub mob_attack_info: Vec<(i32, i32, i32, i32)>,
    /// そのターンに敵が行ったアイテム使用情報
    pub mob_side_effect_info: Vec<SideEffect>,

    /// 現在落ちているアイテムのIDの最大値
    pub current_item_id_max: i32,
    /// そのターンにマップ上に追加されたアイテムのID
    pub dropped_item_added_ids: Vec<i32>,
    /// そのターンにマップ上に削除されたアイテムのID
    pub dropped_item_removed_ids: Vec<i32>,

    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for GameMaster {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            current_level: 1,
            initial_item_count: 10,
            initial_mob_count: 10,
            mob_drop_item_probability: 0.5,
            dungeon_width: 100,
            dungeon_height: 100,
            dungeon_map_1d: Array::new(),
            message: Array::new(),
            player_attack_info: vec![],
            player_side_effect_info: vec![],
            mob_attack_info: vec![],
            mob_side_effect_info: vec![],
            dropped_item_added_ids: vec![],
            dropped_item_removed_ids: vec![],
            current_item_id_max: 0,
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
    /// インスタンスを生成
    #[func]
    pub fn new() -> Gd<Self> {
        Gd::from_init_fn(|base| {
            Self {
                current_level: 1,
                initial_item_count: 10,
                initial_mob_count: 10,
                mob_drop_item_probability: 0.5,
                dungeon_width: 100,
                dungeon_height: 100,
                dungeon_map_1d: Array::new(),
                message: Array::new(),
                player_attack_info: vec![],
                player_side_effect_info: vec![],
                mob_attack_info: vec![],
                mob_side_effect_info: vec![],
                dropped_item_added_ids: vec![],
                dropped_item_removed_ids: vec![],
                current_item_id_max: 0,
                static_map_manager: StaticMapManager::new(100, 100),
                dynamic_map_manager: DynamicMapManager::new(),
                base,
            }
        })
    }

    /// 次の階層へ移動する際に、現在の階層を一つ進める
    #[func]
    pub fn next_level(&mut self) {
        self.current_level += 1;
    }

    /// 一番最初にマップ生成を行う関数
    #[func]
    pub fn initialize_level(&mut self, width: i32, height: i32) {
        // 静的マップの生成
        self.static_map_manager.generate_dungeon(width, height);
        self.copy_from_static_map_manager();

        // 動的マップの初期化
        self.dynamic_map_manager.clear();

        // プレイヤーの初期位置とゴールを候補からランダムに選択
        let n_position_candidates = self.static_map_manager.room_params.len();
        if n_position_candidates == 0 {
            return;
        } else if n_position_candidates == 1 {
            let param = &self.static_map_manager.room_params[0];
            self.dynamic_map_manager.player.position = 
                (param.room_center_x, param.room_center_y);
            self.dynamic_map_manager.goal_position = 
                (param.room_center_x, param.room_center_y);
            return;
        } else {
            let position_idx = (rand::random::<f32>() * (n_position_candidates - 1) as f32) as usize;
            let param = &self.static_map_manager.room_params[position_idx];
            self.dynamic_map_manager.player.position = 
                (param.room_center_x, param.room_center_y);
            let position_idx = (rand::random::<f32>() * (n_position_candidates - 1) as f32) as usize;
            let param = &self.static_map_manager.room_params[position_idx];
            self.dynamic_map_manager.goal_position = 
                (param.room_center_x, param.room_center_y);
        }

        // アイテムの初期位置を設定
        // 小部屋ごとに均一になるようにアイテムを配置したい
        // アイテムの総数/小部屋の数で小部屋ごとの配置数を決める
        // 端数が出るので、あえて+1している
        let item_per_room = ((self.initial_item_count as usize) / self.static_map_manager.room_params.len()) + 1;
        let mut item_count = 0;
        for param in &self.static_map_manager.room_params {
            for _ in 0..item_per_room {
                if item_count >= self.initial_item_count {
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
                        item: RefCell::new(item)
                    };
                    self.dynamic_map_manager.item_list.push(RefCell::new(ditem));
                    item_count += 1;
                }
                // 無限ループを避け、かつアイテム数にランダム性を持たせるため厳密にmaxを狙わない
            }
        }
        godot_print!("{} items generated (max: {})", item_count, self.initial_item_count);
        self.current_item_id_max = item_count as i32;

        // 敵の初期位置を設定
        // アイテムと同様の生成方法とする。
        let mob_per_room = ((self.initial_mob_count as usize) / self.static_map_manager.room_params.len()) + 1;
        let mut mob_count = 0;
        for param in &self.static_map_manager.room_params {
            for _ in 0..mob_per_room {
                if mob_count >= self.initial_mob_count {
                    break;
                }
                let x = param.x + (rand::random::<f32>() * param.width as f32) as i32;
                let y = param.y + (rand::random::<f32>() * param.height as f32) as i32;
                // 床である場所にのみモブを配置
                if (self.static_map_manager.dungeon_map_2d[x as usize][y as usize] == 0) {
                    let mob = GameMob::new_from_level(mob_count as i32, x, y, self.current_level);
                    self.dynamic_map_manager.mob_list.push(RefCell::new(mob));
                    mob_count += 1;
                }
            }
        }
        godot_print!("{} mobs generated (max: {})", mob_count, self.initial_mob_count);
    }

    /// メッセージをクリア、godot側から呼び出される
    #[func]
    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    /// goal_positionをgodotに渡す
    ///
    /// # Returns
    /// goal_positionをVector2iにして返す
    #[func]
    pub fn get_goal_position(&self) -> Vector2i {
        let (x, y) = self.dynamic_map_manager.goal_position;
        Vector2i::new(x, y)
    }

    /// gamemasterはplayerに関する情報をgodotに渡す
    ///
    /// # Returns
    /// playerのステータスをGStringにして返す
    #[func]
    pub fn get_player_status(&self) -> GString {
        let player = &self.dynamic_map_manager.player;
        format!("Level: {}\nHP: {} /{}\nAttack: {}\nDefense: {}\nexp: {}",
            player.level, player.hp, player.max_hp, player.attack, player.defense, player.exp_point).into()
    }

    /// playerのアイテムリストをGStringのArrayにしてgodotに渡す
    ///
    /// # Returns
    /// playerのアイテムリストをGStringのArrayにして返す
    #[func]
    pub fn get_player_items(&self) -> Array<GString> {
        let mut items = Array::new();
        for item in &self.dynamic_map_manager.player.items {
            let item_str = match *item.borrow() {
                GameItem::HealthPotion(potion) => {
                    format!("Health Potion: {}", potion.heal_amount)
                }
                GameItem::Sword(sword) => {
                    format!("Sword: {}", sword.attack_bonus)
                }
                GameItem::Shield(shield) => {
                    format!("Shield: {}", shield.defense_bonus)
                }
                _ => "-".into()
            };
            items.push(item_str.into());
        }
        items
    }

    /// playerの位置
    #[func]
    pub fn get_player_position(&self) -> Vector2i {
        let mut position = Vector2i::ZERO;
        let (x, y) = self.dynamic_map_manager.player.position;
        position.x = x;
        position.y = y;
        position
    }

    /// playerの向き
    #[func]
    pub fn get_player_direction(&self) -> i32 {
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

    /// playerに向きを指示、ターンを消費しない
    #[func]
    pub fn player_turn(&mut self, direction: i32) {
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

    /// playerに移動を指示、ターンを消費する
    ///
    /// # Arguments
    /// * `next_position` - 移動先の座標
    ///
    /// # Returns
    /// 移動が成功したかどうか、成功した場合はtrueを返す、失敗した場合はfalseを返す。
    #[func]
    pub fn player_move(&mut self, next_position: Vector2i) -> bool {
        let mut result = false;
        // ターンの最初にアイテムの差分をクリア
        self.dropped_item_added_ids.clear();
        self.dropped_item_removed_ids.clear();

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
            // 移動先にmobがいない場合のみ移動、移動できない場合は移動できなかったことを通知
            if !mob_exist {
                self.dynamic_map_manager.player.position = (next_position.x, next_position.y);
                // TODO: プレイヤーが移動した先にアイテムがある場合、それを自動的に拾うかどうか
                //  たとえば、特定のキーと同時に移動をした場合拾わないという選択もありうる。
                //  また、アイテム所持上限に達している場合は拾えない。
                
                // まず、移動先にアイテムがあるかどうかを確認
                let mut item_idx = None;
                for (idx, item_rc) in self.dynamic_map_manager.item_list.iter().enumerate() {
                    let item = item_rc.borrow();
                    if item.position == (next_position.x, next_position.y) {
                        item_idx = Some(idx);
                        break;
                    }
                }
                // 移動先にアイテムがあった場合
                if let Some(idx) = item_idx {
                    // アイテム所持上限に達していないことを確認
                    let ditem_rc = &self.dynamic_map_manager.item_list[idx];
                    let got_item = self.dynamic_map_manager.player.add_item(&ditem_rc.borrow().item);
                    if got_item {
                        // 拾った場合、アイテムリストから削除して、削除したことを削除リストに追加
                        self.dropped_item_removed_ids.push(ditem_rc.borrow().id);
                        self.dynamic_map_manager.item_list.remove(idx);
                        self.message.push("アイテムを拾った。".into());
                    } else {
                        // 拾えなかった場合、メッセージを表示
                        self.message.push("持ち物がいっぱいです。".into());
                    }
                }
                result = true;
            }
        }
        result
    }

    /// playerに攻撃を指示、ターンを消費する
    #[func]
    pub fn player_attack(&mut self) {
        // ターンの最初にアイテムの差分をクリア
        self.dropped_item_added_ids.clear();
        self.dropped_item_removed_ids.clear();
        self.player_attack_info.clear();
        self.dynamic_map_manager.player.attack(&mut self.player_attack_info);
        // プレイヤーから帰ってきた攻撃情報を保存
        for (x, y, damage) in &self.player_attack_info {
            godot_print!("Player Attack: x: {}, y: {}, damage: {}", x, y, damage);
        }
    }

    /// playerにアイテムを拾うよう指示、ターンを消費する
    #[func]
    pub fn player_pickup_item(&mut self) {
        // ターンの最初にアイテムの差分をクリア
        self.dropped_item_added_ids.clear();
        self.dropped_item_removed_ids.clear();
        let (x, y) = self.dynamic_map_manager.player.position;
        let mut item_idx = None;
        for (idx, item) in self.dynamic_map_manager.item_list.iter().enumerate() {
            if item.borrow().position == (x, y) {
                item_idx = Some(idx);
                break;
            }
        }
        // アイテムを拾った場合の処理
        if let Some(idx) = item_idx {
            let item = self.dynamic_map_manager.item_list.remove(idx);
            self.dropped_item_removed_ids.push(item.borrow().id);
            self.dynamic_map_manager.player.add_item(&item.borrow().item);
        }
    }

    /// playerが現在所持しているアイテムが使えるかどうかを確認
    #[func]
    pub fn player_can_use_item(&self, item_idx: i32) -> bool {
        if item_idx < 0 || item_idx as usize >= self.dynamic_map_manager.player.items.len() {
            return false;
        }
        let item = &self.dynamic_map_manager.player.items[item_idx as usize];
        match *item.borrow() {
            GameItem::HealthPotion(_) => true,
            _ => false,
        }
    }

    /// playerにアイテムを使うよう指示、ターンを消費する
    #[func]
    pub fn player_use_item(&mut self, item_idx: i32) {
        self.player_side_effect_info.clear();
        self.dynamic_map_manager.player.select_item(item_idx as usize);
        self.player_side_effect_info.push(self.dynamic_map_manager.player.use_item());
        self.message.push("HPが回復した。".into());
    }

    /// playerのアイテム使用時のsideeffectの反映
    pub fn applyPlayerSideEffect(&mut self) {
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

    /// playerのattack_infoの反映
    pub fn applyPlayerAttackInfo(&mut self) {
        self.dynamic_map_manager.defeated_mob_id.clear();
        let mut fumbled = true;
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
                self.message.push(format!("ID{}に{}ダメージを与えた。", id, damage).into());
                self.dynamic_map_manager.mob_list[idx].borrow_mut().hp -= damage;
                // モブのHPが0以下になった場合、リストから削除
                if self.dynamic_map_manager.mob_list[idx].borrow().hp <= 0 {
                    // モブの最終位置を確認
                    let (x, y) = self.dynamic_map_manager.mob_list[idx].borrow().position;

                    // モブを倒したら一定確率でアイテムをドロップするようにする
                    if (rand::random::<f32>() < self.mob_drop_item_probability) {
                        // モブの最終位置にアイテムをドロップ
                        let item = GameItem::HealthPotion(HealthPotion {heal_amount: 10});
                        let item_id = self.current_item_id_max;
                        let ditem = DroppedItem {
                            id: item_id,
                            position: (x, y),
                            item: RefCell::new(item)
                        };
                        self.dynamic_map_manager.item_list.push(RefCell::new(ditem));
                        self.current_item_id_max += 1;
                        self.dropped_item_added_ids.push(item_id);
                    }

                    // モブの持っていたexp_pointをプレイヤーに加算
                    self.dynamic_map_manager.player.exp_point +=
                        self.dynamic_map_manager.mob_list[idx].borrow().exp_point;
                    // モブをリストから削除
                    self.dynamic_map_manager.mob_list.remove(idx);
                    self.dynamic_map_manager.defeated_mob_id.push(id);
                    godot_print!("Mob {} defeated.", id);
                    self.message.push(format!("ID{}を倒した。", id).into());
                }
                fumbled = false;
            }
        }
        // 攻撃を外したらメッセージを表示
        if fumbled && !self.player_attack_info.is_empty() {
            godot_print!("Player Attack Fumbled");
            self.message.push("攻撃が外れた。".into());
        }
        self.player_attack_info.clear();
    }

    /// mobの行動を決定
    // TODO: もっと複雑なAIを実装する
    pub fn decideMobAction(&mut self) {
        let mut mob_next_positions = vec![];
        self.mob_attack_info.clear();

        // プレイヤーの位置はこの関数を呼び出している間は不変なので、ループの外で取得
        let (px, py) = self.dynamic_map_manager.player.position;

        for mob_rc in &mut self.dynamic_map_manager.mob_list {
            let mut mob = mob_rc.borrow_mut();
            // 同じ部屋に入ったモブだけがアクティブになるようにする
            let (mx, my) = mob.position;
            // 部屋にいるかどうかの判定は、BSPNodeParamsのx, y, width, heightから計算を行う。
            // これを、px, pyとmx, myが同じ部屋にいるかどうかで判定する。
            let mut in_same_room = false;
            for param in &self.static_map_manager.room_params {
                if px >= param.x && px < param.x + param.width &&
                    py >= param.y && py < param.y + param.height &&
                    mx >= param.x && mx < param.x + param.width &&
                    my >= param.y && my < param.y + param.height {
                    in_same_room = true;
                    break;
                }
            }
            // 同じ部屋にいる場合だけ行動
            if !in_same_room {
                continue;
            }

            // プレイヤーの位置との距離を計算
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
                    self.mob_attack_info.push((*x, *y, *damage, mob.id));
                }
            } else {
                // そうでなければプレイヤーの方向に移動
                // 移動したい位置を決めておいて、そのあとで実際移動できるかどうかを確認
                let mut next_position = (mx, my);
                if abs_dx > abs_dy {
                    if dx > 0 {
                        next_position = (mx + 1, my);
                        mob.direction = Direction::Right;
                    } else {
                        next_position = (mx - 1, my);
                        mob.direction = Direction::Left;
                    }
                } else {
                    if dy > 0 {
                        next_position = (mx, my + 1);
                        mob.direction = Direction::Down;
                    } else {
                        next_position = (mx, my - 1);
                        mob.direction = Direction::Up;
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

    /// mobのアイテム使用時のsideeffectの反映
    pub fn applyMobSideEffect(&mut self) {
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

    /// mobのattack_infoの反映
    pub fn applyMobAttackInfo(&mut self) {
        for (x, y, damage, mob_id) in &self.mob_attack_info {
            // プレイヤーの位置と一致するものがあればダメージを与える
            if self.dynamic_map_manager.player.position == (*x, *y) {
                self.message.push(format!("プレイヤーはID{}から{}ダメージを受けた。", mob_id, damage).into());
                self.dynamic_map_manager.player.hp -= damage;
                if self.dynamic_map_manager.player.hp <= 0 {
                    // ゲームオーバー
                    godot_print!("Game Over");
                    self.message.push("力尽きた。".into());
                }
            }
        }
        self.mob_attack_info.clear();
    }

    /// 1ターンを定義、godot側から進めるかどうかを決めて呼び出す。
    #[func]
    pub fn process(&mut self) {
        // プレイヤーの行動はすでに反映された状態を起点とする。
        // プレイヤーのアイテム使用時のsideeffectの反映
        self.applyPlayerSideEffect();
        // プレイヤーのattack_infoの反映
        self.applyPlayerAttackInfo();

        // TODO: プレイヤーのレベルアップ判定
        if self.dynamic_map_manager.player.check_level_up() {
            self.message.push("レベルアップした。".into());
        }

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

    /// デバッグ用、プレイヤーのステータスを表示
    #[func]
    pub fn print_player_status(&self) {
        godot_print!(
            "Player Status: HP: {} / {}, Attack: {}, Defense: {}",
            self.dynamic_map_manager.player.hp,
            self.dynamic_map_manager.player.max_hp,
            self.dynamic_map_manager.player.attack,
            self.dynamic_map_manager.player.defense);
    }

    /// デバッグ用、プレイヤーの所持品を表示
    #[func]
    pub fn print_player_items(&self) {
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
        let potion = RefCell::new(GameItem::HealthPotion(HealthPotion {heal_amount: 10}));
        self.dynamic_map_manager.player.add_item(&potion);
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
    /// 落ちているアイテムの位置を取得
    #[func]
    pub fn get_dropped_item_positions(&self) -> Array<Vector2i> {
        let mut positions = array![];
        for item_rc in &self.dynamic_map_manager.item_list {
            let item = item_rc.borrow();
            positions.push(Vector2i::new(item.position.0, item.position.1));
        }
        positions
    }

    /// 落ちているアイテムのIDを取得
    #[func]
    pub fn get_dropped_item_ids(&self) -> Array<i32> {
        let mut ids = array![];
        for item_rc in &self.dynamic_map_manager.item_list {
            let item = item_rc.borrow();
            ids.push(item.id);
        }
        ids
    }

    /// 拾われたアイテムのIDを取得
    #[func]
    pub fn get_dropped_item_removed_ids(&self) -> Array<i32> {
        let mut ids = array![];
        for id in &self.dropped_item_removed_ids {
            ids.push(*id);
        }
        ids
    }

    /// 落とされたアイテムのIDを取得
    #[func]
    pub fn get_dropped_item_added_ids(&self) -> Array<i32> {
        let mut ids = array![];
        for id in &self.dropped_item_added_ids {
            ids.push(*id);
        }
        ids
    }

    // 敵の情報を取得する関数群
    /// 敵の位置を取得
    #[func]
    pub fn get_mob_positions(&self) -> Array<Vector2i> {
        let mut positions = array![];
        for mob_rc in &self.dynamic_map_manager.mob_list {
            let mob = mob_rc.borrow();
            positions.push(Vector2i::new(mob.position.0, mob.position.1));
        }
        positions
    }

    /// 敵の向きを取得
    #[func]
    pub fn get_mob_directions(&self) -> Array<i32> {
        let mut directions = array![];
        for mob_rc in &self.dynamic_map_manager.mob_list {
            let mob = mob_rc.borrow();
            let dir = match mob.direction {
                Direction::Up => 0,
                Direction::UpRight => 1,
                Direction::Right => 2,
                Direction::DownRight => 3,
                Direction::Down => 4,
                Direction::DownLeft => 5,
                Direction::Left => 6,
                Direction::UpLeft => 7,
                _ => 0,
            };
            directions.push(dir);
        }
        directions
    }

    /// 敵のIDを取得
    #[func]
    pub fn get_mob_ids(&self) -> Array<i32> {
        let mut ids = array![];
        for mob_rc in &self.dynamic_map_manager.mob_list {
            let mob = mob_rc.borrow();
            ids.push(mob.id);
        }
        ids
    }

    /// このターンに倒された敵のIDを取得
    #[func]
    pub fn get_defeated_mob_ids(&self) -> Array<i32> {
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