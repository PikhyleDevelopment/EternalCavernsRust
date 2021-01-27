use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum TileType {
    Wall,
    Floor,
    DownStairs,
    Road,
    Grass,
    ShallowWater,
    DeepWater,
    WoodFloor,
    Bridge
}

pub fn tile_walkable(tt: TileType) -> bool {
    match tt {
	TileType::Floor | TileType::DownStairs | TileType::Road | TileType::Grass |
	TileType::ShallowWater | TileType::WoodFloor | TileType::Bridge
	    => true,
	_ => false
    }
}

pub fn tile_opaque(tt: TileType) -> bool {
    match tt {
	TileType::Wall => true,
	_ => false
    }
}
