mod simple_map;
use simple_map::SimpleMapBuilder;
mod bsp_dungeon;
mod common;
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
    let builder = rng.roll_dice(1, 2);
    match builder {
        1 => Box::new(BspDungeonBuilder::new(new_depth)),
        _ => Box::new(SimpleMapBuilder::new(new_depth))
    }
}
