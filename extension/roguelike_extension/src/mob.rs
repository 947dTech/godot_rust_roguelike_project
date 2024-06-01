//! 敵のステータスを管理するモジュール

use crate::player::Direction;

/// 敵のステータス
// TODO: 敵の種類を増やしたいときはどうするかを決める
pub struct GameMob {
    pub id: i32,
    pub position: (i32, i32),
    pub direction: Direction,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub exp_point: i32,
}

impl GameMob {
    /// 新しいインスタンスを生成する
    pub fn new(id: i32, x: i32, y: i32) -> Self {
        Self {
            id: id,
            position: (x, y),
            direction: Direction::Up,
            hp: 10,
            attack: 5,
            defense: 2,
            exp_point: 1,
        }
    }

    /// 攻撃を行う
    pub fn attack(&self, result: &mut Vec<(i32, i32, i32)>) {
        let (mut x, mut y) = self.position;
        let mut damage = self.attack;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mob = GameMob::new(1, 10, 20);
        assert_eq!(mob.id, 1);
        assert_eq!(mob.position, (10, 20));
        assert_eq!(mob.direction, Direction::Up);
        assert_eq!(mob.hp, 10);
        assert_eq!(mob.attack, 5);
        assert_eq!(mob.defense, 2);
        assert_eq!(mob.exp_point, 1);
    }

    #[test]
    fn test_attack() {
        let mut mob = GameMob::new(1, 10, 20);
        let mut result = vec![];
        mob.attack(&mut result);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], (10, 19, 5));
    }
}