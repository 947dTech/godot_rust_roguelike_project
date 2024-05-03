use crate::item::GameItem;
use crate::item::SideEffect;

use std::rc::Rc;
use std::cell::RefCell;

pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

pub struct GamePlayer {
    pub position: (i32, i32),
    pub direction: Direction,
    pub max_hp: i32,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub items: Vec<RefCell<GameItem>>,
    pub active_item_index: usize,
}

impl GamePlayer {
    pub fn new() -> Self {
        let mut obj = Self {
            position: (0, 0),
            direction: Direction::Up,
            max_hp: 100,
            hp: 100,
            attack: 10,
            defense: 5,
            items: vec![],
            active_item_index: 0,
        };
        obj.init_items(8);
        obj
    }

    // TODO: プレイヤー側もDroppedItemを使うようにする
    // アイテムスロットの初期化、アイテム所持数は固定とする。
    pub fn init_items(&mut self, length: usize) {
        self.items.clear();
        self.items.reserve(length);
        for _ in 0..length {
            self.items.push(RefCell::new(GameItem::Null));
        }
    }

    // アイテムを追加する、空きがなければ失敗する。
    pub fn add_item(&mut self, item: &RefCell<GameItem>) -> bool {
        for i in 0..self.items.len() {
            if *self.items[i].borrow() == GameItem::Null {
                self.items[i] = item.clone();
                return true;
            }
        }
        false
    }

    // アイテムを選択する
    pub fn select_item(&mut self, index: usize) {
        if index < self.items.len() {
            self.active_item_index = index;
        }
    }

    // アイテムを使用する
    // TODO: 自分以外に効果があるアイテム使用時は、
    // SideEffectをgamemasterに返すようにする
    pub fn use_item(&mut self) -> SideEffect {
        // 使ったけど失敗したということを通知したい
        let mut result = SideEffect::Fault;
        if self.active_item_index < self.items.len() {
            let mut item_used = false;
            // アイテムの種類によって処理を変える
            match *self.items[self.active_item_index].borrow() {
                GameItem::HealthPotion(potion) => {
                    self.hp += potion.heal_amount;
                    if self.hp > self.max_hp {
                        self.hp = self.max_hp;
                    }
                    result = SideEffect::None;
                    item_used = true;
                }
                _ => {}
            }
            if item_used {
                self.items[self.active_item_index] = RefCell::new(GameItem::Null);
            }
        }
        result
    }

    // 攻撃を行った場合、ダメージとそれを与える座標をセットにして、リストで返す。
    pub fn attack(&self, result: &mut Vec<(i32, i32, i32)>) {
        let (mut x, mut y) = self.position;
        let mut damage = self.attack;
        if self.active_item_index < self.items.len() {
            match *self.items[self.active_item_index].borrow() {
                GameItem::Sword(sword) => {
                    damage += sword.attack_bonus;
                }
                _ => {}
            }
        }
        // Directionに応じて座標を変更
        match self.direction {
            Direction::Up => y -= 1,
            Direction::UpRight => {
                x += 1;
                y -= 1;
            }
            Direction::Right => x += 1,
            Direction::DownRight => {
                x += 1;
                y += 1;
            }
            Direction::Down => y += 1,
            Direction::DownLeft => {
                x -= 1;
                y += 1;
            }
            Direction::Left => x -= 1,
            Direction::UpLeft => {
                x -= 1;
                y -= 1;
            }
        }
        result.push((x, y, damage));
    }
}
