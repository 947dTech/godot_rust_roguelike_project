//! アイテムを定義するモジュール

use std::rc::Rc;
use std::cell::RefCell;

/// アイテム管理用クラス
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameItem {
    /// 無
    Null,
    /// 回復薬
    HealthPotion(HealthPotion),
    /// 武器
    Sword(Sword),
    /// 防具
    Shield(Shield),
}

/// 回復薬
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HealthPotion {
    pub heal_amount: i32,
}

/// 武器
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sword {
    pub attack_bonus: i32,
}

/// 防具
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shield {
    pub defense_bonus: i32,
}

/// アイテムの効果が自分以外に及ぶ場合のクラス
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SideEffect {
    /// アイテムの使用に失敗
    Fault,
    /// アイテムを使ったが、自分以外に効果を及ぼさない
    None,
}


/// アイテムに座標系と管理IDを割り付けたクラス
pub struct DroppedItem {
    pub id: i32,
    pub position: (i32, i32),
    pub item: RefCell<GameItem>,
}
