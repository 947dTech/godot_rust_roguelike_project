pub struct GameMob {
    pub position: (i32, i32),
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
}

impl GameMob {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            position: (x, y),
            hp: 10,
            attack: 5,
            defense: 2,
        }
    }

    pub fn attack(&self) -> Vec<(i32, i32, i32)> {
        let mut attack_info = vec![];
        attack_info.push((self.position.0, self.position.1, self.attack));
        attack_info
    }
}