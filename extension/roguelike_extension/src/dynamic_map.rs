use crate::player::GamePlayer;
use crate::mob::GameMob;
use crate::item::DroppedItem;

pub struct DynamicMapManager {
    pub player: GamePlayer,
    pub item_list: Vec<DroppedItem>,
    pub mob_list: Vec<GameMob>,
}

impl DynamicMapManager {
    pub fn new() -> Self {
        Self {
            player: GamePlayer::new(),
            item_list: vec![],
            mob_list: vec![],
        }
    }
}
