use godot::prelude::*;

use crate::map_generator::{
    generate_dungeon,
    BSPNodeParams,
    Direction,
};

// Godotと関係ないクラスは頭にGdをつけない。
pub struct StaticMapManager {
    pub dungeon_width: i32,
    pub dungeon_height: i32,
    pub dungeon_map_2d: Vec<Vec<i32>>,
    pub room_params: Vec<BSPNodeParams>,
}

impl StaticMapManager {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            dungeon_width: width,
            dungeon_height: height,
            dungeon_map_2d: vec![vec![0; height as usize]; width as usize],
            room_params: vec![],
        }
    }

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

    pub fn generate_dungeon(&mut self, width: i32, height: i32) {
        (self.dungeon_map_2d, self.room_params) =
            generate_dungeon(width, height);
        self.dungeon_width = width;
        self.dungeon_height = height;
    }
}

// GodotのクラスはGdから始まるものとする。
#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct GdStaticMapManager {
    #[export]
    dungeon_width: i32,
    #[export]
    dungeon_height: i32,
    #[export]
    dungeon_map_1d: Array<i32>,
    static_map_manager: StaticMapManager,
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for GdStaticMapManager {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            dungeon_width: 100,
            dungeon_height: 100,
            dungeon_map_1d: Array::new(),
            static_map_manager: StaticMapManager::new(100, 100),
            base,
        }
    }

    fn to_string(&self) -> GString {
        let Self {dungeon_width, dungeon_height, ..} = &self;
        format!("MapManager(dungeon_width={dungeon_width}, dungeon_height={dungeon_height})").into()
    }
}

#[godot_api]
impl GdStaticMapManager {
    #[constant]
    const DEFAULT_DUNGEON_WIDTH: i32 = 100;
    #[constant]
    const DEFAULT_DUNGEON_HEIGHT: i32 = 100;

    #[func]
    fn from_width_height(width: i32, height: i32) -> Gd<Self> {
        Gd::from_init_fn(|base| {
            Self {
                dungeon_width: width,
                dungeon_height: height,
                dungeon_map_1d: Array::new(),
                static_map_manager: StaticMapManager::new(100, 100),
                base,
            }
        })
    }

    #[func]
    fn get_tile(&self, x: i32, y: i32) -> i32 {
        self.dungeon_map_1d.get(
            (y * self.dungeon_width + x).try_into().unwrap()
        ).try_into().unwrap()
    }

    #[func]
    fn set_tile(&mut self, x: i32, y: i32, tile: i32) {
        self.dungeon_map_1d.set(
            (y * self.dungeon_width + x).try_into().unwrap(),
            tile);
    }

    #[func]
    fn generate_map(&mut self, width: i32, height: i32) {
        self.static_map_manager.generate_simple_map(width, height);
        self.copy_from_map_manager();
    }

    #[func]
    fn generate_dungeon(&mut self, width: i32, height: i32) {
        self.static_map_manager.generate_dungeon(width, height);
        self.copy_from_map_manager();
    }

    fn copy_from_map_manager(&mut self) {
        self.dungeon_width = self.static_map_manager.dungeon_width;
        self.dungeon_height = self.static_map_manager.dungeon_height;
        self.dungeon_map_1d.resize((self.dungeon_width * self.dungeon_height).try_into().unwrap(), &0);
        for y in 0..self.dungeon_height {
            for x in 0..self.dungeon_width {
                self.set_tile(x, y, self.static_map_manager.dungeon_map_2d[x as usize][y as usize]);
            }
        }
    }


}
