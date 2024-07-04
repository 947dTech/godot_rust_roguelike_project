//! 動的マップ(キャラクターなど頻繁に位置が変動するもの)を管理するモジュール

use crate::player::GamePlayer;
use crate::mob::GameMob;
use crate::item::DroppedItem;

use std::rc::Rc;
use std::cell::RefCell;

/// 動的マップを管理するクラス
pub struct DynamicMapManager {
    /// プレイヤー
    pub player: GamePlayer,
    /// マップ上に落ちているアイテムのリスト
    pub item_list: Vec<RefCell<DroppedItem>>,
    /// マップ上に存在するモンスターのリスト
    pub mob_list: Vec<RefCell<GameMob>>,

    /// 倒したモンスターのIDのリスト
    pub defeated_mob_id: Vec<i32>,
    /// ゴールの位置
    pub goal_position: (i32, i32),
}

impl DynamicMapManager {
    /// 新しいインスタンスを生成する
    pub fn new() -> Self {
        Self {
            player: GamePlayer::new(),
            item_list: vec![],
            mob_list: vec![],
            defeated_mob_id: vec![],
            goal_position: (0, 0),
        }
    }

    /// シーン遷移した場合はリストだけ初期化する
    pub fn clear(&mut self) {
        self.item_list.clear();
        self.mob_list.clear();
        self.defeated_mob_id.clear();
    }
}
