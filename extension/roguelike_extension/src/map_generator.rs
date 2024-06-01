//! マップ生成アルゴリズムを提供するモジュール

use rand;
use std::io::Cursor;
use std::cmp::{
max, min
};
  
  
/// 部屋をつなぐための方向を定義したenum
#[derive(Debug, PartialEq)]
pub enum Direction {
    /// 上
    North,
    /// 下
    South,
    /// 右
    East,
    /// 左
    West,
    /// なし
    None,
}
  
/// 二分木が持つべき構造体を定義
pub struct BSPNodeParams {
    /// 部屋の左上のx座標
    pub x: i32,
    /// 部屋の左上のy座標
    pub y: i32,
    /// 部屋の幅
    pub width: i32,
    /// 部屋の高さ
    pub height: i32,
    /// 部屋の中心のx座標
    pub room_center_x: i32,
    /// 部屋の中心のy座標
    pub room_center_y: i32,
    /// どの方向に部屋をつなぐか
    pub connect_to: Direction,
}
  
/// 二分木を構成するenumを定義
pub enum BSPTree {
    /// ノード
    Node {
        /// ノードのもつパラメータ
        value: BSPNodeParams,
        /// 左の子ノード
        left: Box<BSPTree>,
        /// 右の子ノード
        right: Box<BSPTree>,
    },
    /// 終端ノード
    Nil,
}
  
/// 二分木の内容を出力する関数
pub fn print_tree(tree: &BSPTree) {
    match tree {
        BSPTree::Node { value, left, right } => {
            println!("(x, y, width, height) = ({}, {}, {}, {})",
                value.x, value.y, value.width, value.height);
            println!("(room_center_x, room_center_y, connect_to) = ({}, {}, {:?})",
                value.room_center_x, value.room_center_y, value.connect_to);
            print_tree(left);
            print_tree(right);
        },
        BSPTree::Nil => {
            println!("^ this is terminal node.");
        }
    }
}
  
/// 二分木を生成する再帰関数
///
/// ノードが必ず左右に存在することを保証しなければならない。
///
pub fn generate_bsp_tree(x: i32, y: i32, width: i32, height: i32, connect_to: Direction, level: i32) -> BSPTree {
    let min_room_size = 16;
    if width < min_room_size || height < min_room_size {
        return BSPTree::Nil;
    }
  
    let max_level = 3;
    if level > max_level {
        return BSPTree::Nil;
    }
  
    // let split = level % 2 == 0;
    let split = rand::random::<i32>() % 2 == 0;
    if split {
        let split_x = max(min(x + 1 + rand::random::<i32>() % (width - 2), x + width - min_room_size), x + min_room_size);
        // let split_x = width / 2 + x;
        // for i in y..y + height {
        //   dungeon[split_x as usize][i as usize] = 0;
        // }
        let mut left = generate_bsp_tree(x, y, split_x - x, height, Direction::East, level + 1);
        let mut right = generate_bsp_tree(split_x, y, x + width - split_x, height, Direction::West, level + 1);
        if let BSPTree::Nil = left {
            right = BSPTree::Nil;
        }
        if let BSPTree::Nil = right {
            left = BSPTree::Nil;
        }
        return BSPTree::Node {
            value: BSPNodeParams {
                x: x,
                y: y,
                width: width,
                height: height,
                room_center_x: x + width / 2,
                room_center_y: y + height / 2,
                connect_to: connect_to,
            },
            left: Box::new(left),
            right: Box::new(right),
        };
    } else {
        let split_y = max(min(y + 1 + rand::random::<i32>() % (height - 2), y + height - min_room_size), y + min_room_size);
        // let split_y = height / 2 + y;
        // for i in x..x + width {
        //   dungeon[i as usize][split_y as usize] = 0;
        // }
        let mut left = generate_bsp_tree(x, y, width, split_y - y, Direction::South, level + 1);
        let mut right = generate_bsp_tree(x, split_y, width, y + height - split_y, Direction::North, level + 1);
        if let BSPTree::Nil = left {
            right = BSPTree::Nil;
        }
        if let BSPTree::Nil = right {
            left = BSPTree::Nil;
        }
        return BSPTree::Node {
            value: BSPNodeParams {
                x: x,
                y: y,
                width: width,
                height: height,
                room_center_x: x + width / 2,
                room_center_y: y + height / 2,
                connect_to: connect_to,
            },
            left: Box::new(left),
            right: Box::new(right),
        };
    }
}
  
/// ダンジョンを生成する関数
pub fn generate_dungeon(
    width: i32,
    height: i32
) -> (Vec<Vec<i32>>, Vec<BSPNodeParams>) {
    // dense matrixとしてdungeonを定義
    let mut dungeon = vec![vec![0; height as usize]; width as usize];
    for i in 0..width {
        for j in 0..height {
            dungeon[i as usize][j as usize] = 1;
        }
    }
    // 確実に壁に埋まらない場所として、BSPTreeで確保した部屋の中心をすべて戻す
    // 各部屋の大きさがわかれば、部屋の中心から壁ではない場所を容易に探せる
    let mut room_params = vec![];

    println!("generate dungeon");
  
    // 二分木を生成する関数を使う
    let mut tree = generate_bsp_tree(0, 0, width, height, Direction::None, 0);
    // BSPTreeを使ってdungeonに反映
    fn fill_minimum_nodes(dungeon: &mut Vec<Vec<i32>>, tree: &mut BSPTree) {
        // 各部屋の塗りつぶしのアルゴリズム
        // 自分が終端ノードだった場合、そこで初めて塗りつぶしを行う。
        // それ以外の場合、左右の子ノードに対して再帰的に塗りつぶしを行う。
        match tree {
            BSPTree::Node { value, left, right } => {
                // if value.width < 25 || value.height < 25 {
                //   return;
                // }
    
                // 先に再帰呼び出しを行わないと、子ノードのroom_center_x, room_center_yが更新されない。
                if let BSPTree::Node { value: left_value, left: left_left, right: left_right } = left.as_ref() {
                    fill_minimum_nodes(dungeon, left);
                }
                if let BSPTree::Node { value: right_value, left: right_left, right: right_right } = right.as_ref() {
                    fill_minimum_nodes(dungeon, right);
                }
  
                // 左右の子ノードのroom_center_x, room_center_yを確認して、自分のconnect_toに近いほうを選び、
                // 自分のroom_center_x, room_center_yを更新する。
                let mut left_room_center_x = None;
                let mut left_room_center_y = None;
                let mut right_room_center_x = None;
                let mut right_room_center_y = None;
                if let BSPTree::Node { value: left_value, left: left_left, right: left_right } = left.as_ref() {
                    left_room_center_x = Some(left_value.room_center_x);
                    left_room_center_y = Some(left_value.room_center_y);
                }
        
                if let BSPTree::Node { value: right_value, left: right_left, right: right_right } = right.as_ref() {
                    right_room_center_x = Some(right_value.room_center_x);
                    right_room_center_y = Some(right_value.room_center_y);
                }
    
                if let Some(..) = left_room_center_x {
                    if let Some(..) = left_room_center_y {
                        if let Some(..) = right_room_center_x {
                            if let Some(..) = right_room_center_y {
                                if value.connect_to == Direction::North {
                                    if left_room_center_y.unwrap() < right_room_center_y.unwrap() {
                                        value.room_center_x = left_room_center_x.unwrap();
                                        value.room_center_y = left_room_center_y.unwrap();
                                    } else {
                                        value.room_center_x = right_room_center_x.unwrap();
                                        value.room_center_y = right_room_center_y.unwrap();
                                    }
                                } else if value.connect_to == Direction::South {
                                    if left_room_center_y.unwrap() > right_room_center_y.unwrap() {
                                        value.room_center_x = left_room_center_x.unwrap();
                                        value.room_center_y = left_room_center_y.unwrap();
                                    } else {
                                        value.room_center_x = right_room_center_x.unwrap();
                                        value.room_center_y = right_room_center_y.unwrap();
                                    }
                                } else if value.connect_to == Direction::East {
                                    if left_room_center_x.unwrap() > right_room_center_x.unwrap() {
                                        value.room_center_x = left_room_center_x.unwrap();
                                        value.room_center_y = left_room_center_y.unwrap();
                                    } else {
                                        value.room_center_x = right_room_center_x.unwrap();
                                        value.room_center_y = right_room_center_y.unwrap();
                                    }
                                } else if value.connect_to == Direction::West {
                                    if left_room_center_x.unwrap() < right_room_center_x.unwrap() {
                                        value.room_center_x = left_room_center_x.unwrap();
                                        value.room_center_y = left_room_center_y.unwrap();
                                    } else {
                                        value.room_center_x = right_room_center_x.unwrap();
                                        value.room_center_y = right_room_center_y.unwrap();
                                    }
                                }
                            }
                        }
                    }
                }
  
                // 左右どちらかがNilであった場合に初めて塗りつぶしを行う
                if let BSPTree::Nil = left.as_ref() {
                    // 内側をborderサイズ分だけ残して0で塗りつぶす
                    // borderは2--(2+3)のうちランダムで決める
                    let border = 2 + rand::random::<u8>() % 3;
                    for i in value.x + border as i32..value.x + value.width - border as i32 {
                        for j in value.y + border as i32..value.y + value.height - border as i32 {
                            dungeon[i as usize][j as usize] = 0;
                        }
                    }
                }
            },
            BSPTree::Nil => {
                return;
            }
        }
    }
    fill_minimum_nodes(&mut dungeon, &mut tree);
    print_tree(&tree);
  
    // room_center_x, room_center_yを使って部屋同士をつなぐ
    fn connect_rooms(dungeon: &mut Vec<Vec<i32>>, tree: &mut BSPTree) {
        match tree {
            BSPTree::Node { value, left, right } => {
                if let BSPTree::Node { value: left_value, left: left_left, right: left_right } = left.as_ref() {
                    connect_rooms(dungeon, left);
                }
                if let BSPTree::Node { value: right_value, left: right_left, right: right_right } = right.as_ref() {
                    connect_rooms(dungeon, right);
                }
    
                if let BSPTree::Node { value: left_value, left: left_left, right: left_right } = left.as_ref() {
                    if let BSPTree::Node { value: right_value, left: right_left, right: right_right } = right.as_ref() {
                        // 最終的に端点同士をつなぐために、どこまで線を引いたかを記録しておく。
                        let mut left_connect_x = None;
                        let mut left_connect_y = None;
                        let mut right_connect_x = None;
                        let mut right_connect_y = None;
    
                        // 左辺の部屋から通路を伸ばす
                        match left_value.connect_to {
                            Direction::North => {
                                println!("north");
                                left_connect_x = Some(left_value.room_center_x);
                                for i in (right_value.y + right_value.height)..left_value.room_center_y {
                                    dungeon[left_value.room_center_x as usize][i as usize] = 0;
                                }
                            },
                            Direction::South => {
                                println!("south");
                                left_connect_x = Some(left_value.room_center_x);
                                for i in left_value.room_center_y..right_value.y {
                                    dungeon[left_value.room_center_x as usize][i as usize] = 0;
                                }
                            },
                            Direction::East => {
                                println!("east");
                                left_connect_y = Some(left_value.room_center_y);
                                for i in left_value.room_center_x..right_value.x {
                                    dungeon[i as usize][left_value.room_center_y as usize] = 0;
                                }
                            },
                            Direction::West => {
                                println!("west");
                                left_connect_y = Some(left_value.room_center_y);
                                for i in (right_value.x + right_value.width)..left_value.room_center_x {
                                    dungeon[i as usize][left_value.room_center_y as usize] = 0;
                                }
                            },
                            _ => {
                                println!("none == root node");
                            }
                        }
  
                        // 右辺の部屋から通路を伸ばす
                        match right_value.connect_to {
                            Direction::North => {
                                println!("north");
                                println!("from = {}", left_value.y + left_value.height);
                                println!("to = {}", right_value.room_center_y);
                                right_connect_x = Some(right_value.room_center_x);
                                for i in (left_value.y + left_value.height)..right_value.room_center_y {
                                    dungeon[right_value.room_center_x as usize][i as usize] = 0;
                                }
                            },
                            Direction::South => {
                                println!("south");
                                right_connect_x = Some(right_value.room_center_x);
                                for i in right_value.room_center_y..left_value.y {
                                    dungeon[right_value.room_center_x as usize][i as usize] = 0;
                                }
                            },
                            Direction::East => {
                                println!("east");
                                right_connect_y = Some(right_value.room_center_y);
                                for i in right_value.room_center_x..left_value.x {
                                    dungeon[i as usize][right_value.room_center_y as usize] = 0;
                                }
                            },
                            Direction::West => {
                                println!("west");
                                right_connect_y = Some(right_value.room_center_y);
                                for i in (left_value.x + left_value.width)..right_value.room_center_x {
                                    dungeon[i as usize][right_value.room_center_y as usize] = 0;
                                }
                            },
                            _ => {
                                println!("none == root node");
                            }
                        }
  
                        // 端点同士をつなぐ、境界線上に線を引く
                        // 横につなぐ場合
                        if let Some(..) = left_connect_x {
                            if let Some(..) = right_connect_x {
                                let y = if (left_value.y > right_value.y) { left_value.y } else { right_value.y };
                                if left_connect_x.unwrap() < right_connect_x.unwrap() {
                                    for i in left_connect_x.unwrap()..(right_connect_x.unwrap() + 1) {
                                        dungeon[i as usize][y as usize] = 0;
                                    }
                                } else {
                                    for i in right_connect_x.unwrap()..(left_connect_x.unwrap() + 1) {
                                        dungeon[i as usize][y as usize] = 0;
                                    }
                                }
                            }
                        }
                        // 縦につなぐ場合
                        if let Some(..) = left_connect_y {
                            if let Some(..) = right_connect_y {
                                let x = if (left_value.x > right_value.x) { left_value.x } else { right_value.x };
                                if left_connect_y.unwrap() < right_connect_y.unwrap() {
                                    for i in left_connect_y.unwrap()..(right_connect_y.unwrap() + 1) {
                                        dungeon[x as usize][i as usize] = 0;
                                    }
                                } else {
                                    for i in right_connect_y.unwrap()..(left_connect_y.unwrap() + 1) {
                                        dungeon[x as usize][i as usize] = 0;
                                    }
                                }
                            }
                        }
  
                    }
                }
            }
            _ => {},
        }
    }
    connect_rooms(&mut dungeon, &mut tree);

    // room_paramsに部屋の情報をすべて格納する
    fn get_room_dimensions(
        room_params: &mut Vec<BSPNodeParams>, tree: &mut BSPTree
    ) {
        match tree {
            BSPTree::Node { value, left, right } => {
                if let BSPTree::Node { value: left_value, left: left_left, right: left_right } = left.as_ref() {
                    get_room_dimensions(room_params, left);
                }
                if let BSPTree::Node { value: right_value, left: right_left, right: right_right } = right.as_ref() {
                    get_room_dimensions(room_params, right);
                }

                // 左右どちらかがNilであった場合に初めて部屋の中心とサイズを保存する
                if let BSPTree::Nil = left.as_ref() {
                    room_params.push(
                        BSPNodeParams {
                            x: value.x,
                            y: value.y,
                            width: value.width,
                            height: value.height,
                            room_center_x: value.room_center_x,
                            room_center_y: value.room_center_y,
                            connect_to: Direction::None,
                        }
                    );
                }

            },
            _ => {}
        }
    }
    get_room_dimensions(&mut room_params, &mut tree);
  
    (dungeon, room_params)
}


#[cfg(test)]
mod tests {
    use super::*;
  
    #[test]
    fn test_generate_bsp_tree() {
        let tree = generate_bsp_tree(0, 0, 64, 64, Direction::None, 0);
        fn check_tree_params(tree: &BSPTree) {
            match tree {
                BSPTree::Node { value, left, right } => {
                    // 中間のノードでは、各パラメータに齟齬がないかチェックする。
                    // 生成時に以下のようになっているはず
                    // room_center_x: x + width / 2,
                    // room_center_y: y + height / 2,
                    assert_eq!(value.room_center_x, (value.x + value.width / 2));
                    assert_eq!(value.room_center_y, (value.y + value.height / 2));
                    check_tree_params(left);
                    check_tree_params(right);
                },
                BSPTree::Nil => {
                    // 終端ノードなのでチェックをする必要はない
                    println!("^ this is terminal node.");
                }
            }
        }
    }
  
    #[test]
    fn test_generate_dungeon() {
        let (dungeon, room_params) = generate_dungeon(64, 64);
        // dungeonの中身を確認
        // dungeonのサイズは64x64で、壁は1、通路は0で表現されている
        assert_eq!(dungeon.len(), 64);
        for i in 0..64 {
            assert_eq!(dungeon[i].len(), 64);
        }
        // dungeonの全要素を確認
        for i in 0..64 {
            for j in 0..64 {
                assert!(dungeon[i][j] == 0 || dungeon[i][j] == 1);
            }
        }

        // room_paramsの中身を確認
        for room in room_params {
            // roomのサイズは16x16以上であること
            assert!(room.width >= 16);
            assert!(room.height >= 16);
            // roomの中心座標が正しいこと
            assert_eq!(room.room_center_x, room.x + room.width / 2);
            assert_eq!(room.room_center_y, room.y + room.height / 2);
        }
    }
}
