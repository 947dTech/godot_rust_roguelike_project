use std::rc::Rc;
use std::cell::RefCell;

// アイテム管理用クラス
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameItem {
    Null,
    HealthPotion(HealthPotion),
    Sword(Sword),
    Shield(Shield),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HealthPotion {
    pub heal_amount: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sword {
    pub attack_bonus: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shield {
    pub defense_bonus: i32,
}

// アイテムの効果が自分以外に及ぶ場合のクラス
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SideEffect {
    Fault,  // アイテムの使用に失敗
    None,  // アイテムを使ったが、自分以外に効果を及ぼさない
}


// アイテムに座標系と管理IDを割り付けたクラス
pub struct DroppedItem {
    pub id: i32,
    pub position: (i32, i32),
    pub item: RefCell<GameItem>,
}
