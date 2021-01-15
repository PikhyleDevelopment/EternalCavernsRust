mod simple_map;
use simple_map::SimpleMapBuilder;
mod bsp_dungeon;
mod bsp_interior;
mod cellular_automata;
mod common;
mod dla;
mod drunkard;
mod maze;
mod voronoi;

use voronoi::VoronoiCellBuilder;

use dla::DLABuilder;

use maze::MazeBuilder;

use drunkard::{DrunkSpawnMode, DrunkardSettings, DrunkardsWalkBuilder};

use cellular_automata::CellularAutomataBuilder;

use bsp_interior::BspInteriorBuilder;

use bsp_dungeon::BspDungeonBuilder;

use common::*;

use super::{spawner, Map, Position, Rect, TileType, World, SHOW_MAPGEN_VISUALIZER};

pub trait MapBuilder {
    fn build_map(&mut self);
    fn spawn_entities(&mut self, ecs: &mut World);
    fn get_map(&self) -> Map;
    fn get_starting_position(&self) -> Position;
    fn get_snapshot_history(&self) -> Vec<Map>;
    fn take_snapshot(&mut self);
}

pub fn random_builder(new_depth: i32) -> Box<dyn MapBuilder> {
    // Randomize MapBuilder per dungeon level
    let mut rng = rltk::RandomNumberGenerator::new();
    let builder = rng.roll_dice(1, 17);
    match builder {
        1 => Box::new(BspDungeonBuilder::new(new_depth)),
        2 => Box::new(BspInteriorBuilder::new(new_depth)),
        3 => Box::new(CellularAutomataBuilder::new(new_depth)),
        4 => Box::new(DrunkardsWalkBuilder::open_area(new_depth)),
        5 => Box::new(DrunkardsWalkBuilder::open_halls(new_depth)),
        6 => Box::new(DrunkardsWalkBuilder::winding_passages(new_depth)),
        7 => Box::new(DrunkardsWalkBuilder::fat_passages(new_depth)),
        8 => Box::new(DrunkardsWalkBuilder::fearful_symmetry(new_depth)),
        9 => Box::new(MazeBuilder::new(new_depth)),
        10 => Box::new(DLABuilder::walk_inwards(new_depth)),
        11 => Box::new(DLABuilder::vertical_walk(new_depth)),
        12 => Box::new(DLABuilder::central_attraction(new_depth)),
        13 => Box::new(DLABuilder::insectoid(new_depth)),
        14 => Box::new(VoronoiCellBuilder::chebyshev(new_depth)),
        15 => Box::new(VoronoiCellBuilder::manhattan(new_depth)),
        16 => Box::new(VoronoiCellBuilder::pythagoras(new_depth)),
        _ => Box::new(SimpleMapBuilder::new(new_depth)),
    }
}
