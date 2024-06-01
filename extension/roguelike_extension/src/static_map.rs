//! 静的マップ(地形)を管理するモジュール

use crate::map_generator::{
    generate_dungeon,
    BSPNodeParams,
    Direction,
};


/// 静的マップを管理するクラス
pub struct StaticMapManager {
    /// マップの幅
    pub dungeon_width: i32,
    /// マップの高さ
    pub dungeon_height: i32,
    /// マップの内容を格納する2次元配列
    pub dungeon_map_2d: Vec<Vec<i32>>,
    /// 部屋の情報を格納する配列
    pub room_params: Vec<BSPNodeParams>,
}

impl StaticMapManager {
    /// 新しいインスタンスを生成する
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            dungeon_width: width,
            dungeon_height: height,
            dungeon_map_2d: vec![vec![0; height as usize]; width as usize],
            room_params: vec![],
        }
    }

    /// 大部屋を生成する
    pub fn generate_simple_map(&mut self, width: i32, height: i32) {
        self.dungeon_width = width;
        self.dungeon_height = height;
        self.dungeon_map_2d = vec![vec![0; height as usize]; width as usize];
        for y in 0..self.dungeon_height {
            self.dungeon_map_2d[0][y as usize] = 1;
            self.dungeon_map_2d[(self.dungeon_width - 1) as usize][y as usize] = 1;
        }

        for x in 1..(self.dungeon_width - 1) {
            for y in 1..(self.dungeon_height - 1) {
                self.dungeon_map_2d[x as usize][y as usize] = 0;
            }
            self.dungeon_map_2d[x as usize][0] = 1;
            self.dungeon_map_2d[x as usize][(self.dungeon_height - 1) as usize] = 1;
        }
        self.room_params = vec![];
        self.room_params.push(
            BSPNodeParams {
                x: 0,
                y: 0,
                width: self.dungeon_width - 2,
                height: self.dungeon_height - 2,
                room_center_x: self.dungeon_width / 2,
                room_center_y: self.dungeon_height / 2,
                connect_to: Direction::None,
            }
        );
    }

    /// 自動生成を使ってダンジョンを生成する
    pub fn generate_dungeon(&mut self, width: i32, height: i32) {
        (self.dungeon_map_2d, self.room_params) =
            generate_dungeon(width, height);
        self.dungeon_width = width;
        self.dungeon_height = height;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_map_manager() {
        let mut manager = StaticMapManager::new(10, 10);
        manager.generate_simple_map(10, 10);
        assert_eq!(manager.dungeon_map_2d[0][0], 1);
        assert_eq!(manager.dungeon_map_2d[9][9], 1);
        assert_eq!(manager.dungeon_map_2d[5][5], 0);
    }

    #[test]
    fn test_generate_dungeon() {
        let mut manager = StaticMapManager::new(64, 64);
        manager.generate_dungeon(64, 64);
        assert_eq!(manager.dungeon_map_2d.len(), 64);
    }
}