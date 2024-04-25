use crate::player::GamePlayer;
use crate::mob::GameMob;
use crate::item::DroppedItem;

use std::rc::Rc;
use std::cell::RefCell;

pub struct DynamicMapManager {
    pub player: GamePlayer,
    pub item_list: Vec<Rc<DroppedItem>>,
    pub mob_list: Vec<RefCell<GameMob>>,

    pub defeated_mob_id: Vec<i32>,
}

impl DynamicMapManager {
    pub fn new() -> Self {
        Self {
            player: GamePlayer::new(),
            item_list: vec![],
            mob_list: vec![],
            defeated_mob_id: vec![],
        }
    }
}
