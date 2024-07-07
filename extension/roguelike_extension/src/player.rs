//! プレイヤーの定義を行うモジュール

use crate::item::GameItem;
use crate::item::SideEffect;

use std::rc::Rc;
use std::cell::RefCell;

/// プレイヤーの向きを定義する列挙型
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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


/// プレイヤーのステータス
// TODO: 経験値とレベルの概念を追加する
// TODO: 装備品の概念を追加する
//  item.rsのSwrod, Shieldを保持できるようにする。
pub struct GamePlayer {
    pub position: (i32, i32),
    pub direction: Direction,
    pub max_hp: i32,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub items: Vec<RefCell<GameItem>>,
    pub active_item_index: usize,
    pub exp_point: i32,
    pub is_heal_when_level_up: bool,
    pub level: i32,
}

impl GamePlayer {
    /// プレイヤーの初期化
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
            exp_point: 0,
            is_heal_when_level_up: false,
            level: 1,
        };
        obj.init_items(8);
        obj
    }

    // TODO: プレイヤー側もDroppedItemを使うようにする
    /// アイテムスロットの初期化、アイテム所持数は固定とする。
    ///
    /// # Arguments
    ///
    /// * `length` - アイテム所持数
    pub fn init_items(&mut self, length: usize) {
        self.items.clear();
        self.items.reserve(length);
        for _ in 0..length {
            self.items.push(RefCell::new(GameItem::Null));
        }
    }

    /// アイテムを追加する、空きがなければ失敗する。
    ///
    /// # Arguments
    ///
    /// * `item` - 追加するアイテム
    ///
    /// # Returns
    ///
    /// * 成功した場合はtrue、失敗した場合はfalse
    pub fn add_item(&mut self, item: &RefCell<GameItem>) -> bool {
        for i in 0..self.items.len() {
            if *self.items[i].borrow() == GameItem::Null {
                self.items[i] = item.clone();
                return true;
            }
        }
        false
    }

    /// アイテムを選択する
    pub fn select_item(&mut self, index: usize) {
        if index < self.items.len() {
            self.active_item_index = index;
        }
    }

    /// アイテムを使用する
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

    /// 攻撃を行った場合、ダメージとそれを与える座標をセットにして、リストで返す。
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

    /// レベルアップ判定を行う
    pub fn check_level_up(&mut self) -> bool {
        // TODO: ここは調整が必要
        let is_level_up = self.exp_point >= self.level * 3;
        if is_level_up {
            self.exp_point = 0;
            self.level += 1;
            self.max_hp += 10;
            if self.is_heal_when_level_up {
                self.hp = self.max_hp;
            }
            self.attack += 2;
            self.defense += 1;
        }
        is_level_up
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::item::HealthPotion;

    #[test]
    fn test_new() {
        let player = GamePlayer::new();
        assert_eq!(player.position, (0, 0));
        assert_eq!(player.direction, Direction::Up);
        assert_eq!(player.max_hp, 100);
        assert_eq!(player.hp, 100);
        assert_eq!(player.attack, 10);
        assert_eq!(player.defense, 5);
        assert_eq!(player.items.len(), 8);
        assert_eq!(player.active_item_index, 0);
        assert_eq!(player.exp_point, 0);
        assert_eq!(player.level, 1);
    }

    #[test]
    fn test_init_items() {
        let mut player = GamePlayer::new();
        player.init_items(8);
        assert_eq!(player.items.len(), 8);
        for i in 0..8 {
            assert_eq!(*player.items[i].borrow(), GameItem::Null);
        }
    }

    #[test]
    fn test_add_item() {
        let mut player = GamePlayer::new();
        player.init_items(8);
        let item = RefCell::new(GameItem::HealthPotion(HealthPotion { heal_amount: 10 }));
        assert_eq!(player.add_item(&item), true);
        assert_eq!(*player.items[0].borrow(), GameItem::HealthPotion(HealthPotion { heal_amount: 10 }));
        let item = RefCell::new(GameItem::HealthPotion(HealthPotion { heal_amount: 10 }));
        assert_eq!(player.add_item(&item), true);
        assert_eq!(*player.items[1].borrow(), GameItem::HealthPotion(HealthPotion { heal_amount: 10 }));
        let item = RefCell::new(GameItem::HealthPotion(HealthPotion { heal_amount: 10 }));
        assert_eq!(player.add_item(&item), true);
        assert_eq!(*player.items[2].borrow(), GameItem::HealthPotion(HealthPotion { heal_amount: 10 }));
        let item = RefCell::new(GameItem::HealthPotion(HealthPotion { heal_amount: 10 }));
        assert_eq!(player.add_item(&item), true);
        assert_eq!(*player.items[3].borrow(), GameItem::HealthPotion(HealthPotion { heal_amount: 10 }));
    }

    #[test]
    fn test_select_item() {
        let mut player = GamePlayer::new();
        player.init_items(8);
        player.select_item(0);
        assert_eq!(player.active_item_index, 0);
        player.select_item(1);
        assert_eq!(player.active_item_index, 1);
        player.select_item(2);
        assert_eq!(player.active_item_index, 2);
        player.select_item(3);
        assert_eq!(player.active_item_index, 3);
    }

    #[test]
    fn test_use_item() {
        let mut player = GamePlayer::new();
        player.init_items(8);

        // 回復薬を与えて選択し使うと、消費されてサイドエフェクトはNone
        let item = RefCell::new(GameItem::HealthPotion(HealthPotion { heal_amount: 10 }));
        player.add_item(&item);
        player.select_item(0);
        assert_eq!(player.use_item(), SideEffect::None);
        assert_eq!(*player.items[0].borrow(), GameItem::Null);

        // 何もない状態で使うとサイドエフェクトはFault
        player.select_item(0);
        assert_eq!(player.use_item(), SideEffect::Fault);
    }

    #[test]
    fn test_attack() {
        let mut player = GamePlayer::new();
        player.position = (5, 5);
        let mut result = vec![];
        player.attack(&mut result);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], (5, 4, 10));
    }

    #[test]
    fn test_check_level_up() {
        let mut player = GamePlayer::new();
        player.exp_point = 0;
        player.level = 1;
        assert_eq!(player.check_level_up(), false);
        player.exp_point = 3;
        player.level = 1;
        assert_eq!(player.check_level_up(), true);
        assert_eq!(player.exp_point, 0);
        assert_eq!(player.level, 2);
        assert_eq!(player.max_hp, 110);
        assert_eq!(player.hp, 100);
        assert_eq!(player.attack, 12);
        assert_eq!(player.defense, 6);
    }

    #[test]
    fn test_check_level_up_with_heal() {
        let mut player = GamePlayer::new();
        player.is_heal_when_level_up = true;
        player.exp_point = 3;
        player.level = 1;
        assert_eq!(player.check_level_up(), true);
        assert_eq!(player.exp_point, 0);
        assert_eq!(player.level, 2);
        assert_eq!(player.max_hp, 110);
        assert_eq!(player.hp, 110);
        assert_eq!(player.attack, 12);
        assert_eq!(player.defense, 6);
    }
}
