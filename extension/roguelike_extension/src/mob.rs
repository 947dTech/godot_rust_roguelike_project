use crate::player::Direction;

pub struct GameMob {
    pub id: i32,
    pub position: (i32, i32),
    pub direction: Direction,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
}

impl GameMob {
    pub fn new(id: i32, x: i32, y: i32) -> Self {
        Self {
            id: id,
            position: (x, y),
            direction: Direction::Up,
            hp: 10,
            attack: 5,
            defense: 2,
        }
    }

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