mod simple_map;
use simple_map::SimpleMapBuilder;
mod bsp_dungeon;
mod bsp_interior;
mod cellular_automata;
mod common;
mod dla;
mod drunkard;
mod maze;
mod prefab_builder;
mod room_based_spawn;
mod voronoi;
mod waveform_collapse;
use room_based_spawn::RoomBasedSpawner;
mod room_based_starting_position;
use room_based_starting_position::RoomBasedStartingPosition;
mod area_starting_points;
use area_starting_points::*;
mod cull_unreachable;
use cull_unreachable::*;
mod room_based_stairs;
use room_based_stairs::*;
mod distant_exit;
mod door_placement;
mod room_corner_rounding;
mod room_corridors_spawner;
mod room_draw;
mod room_exploder;
mod room_sorter;
mod rooms_corridors_bsp;
mod rooms_corridors_dogleg;
mod rooms_corridors_lines;
mod rooms_corridors_nearest;
mod voronoi_spawning;

use door_placement::*;

use room_corridors_spawner::*;

use rooms_corridors_lines::*;

use rooms_corridors_nearest::*;

use room_draw::*;

use room_sorter::*;

use rooms_corridors_bsp::*;

use rooms_corridors_dogleg::*;

use room_corner_rounding::*;

use room_exploder::*;

use voronoi_spawning::*;

use distant_exit::*;

use prefab_builder::*;

use waveform_collapse::WaveformCollapseBuilder;

use voronoi::*;

use dla::DLABuilder;

use maze::MazeBuilder;

use drunkard::DrunkardsWalkBuilder;

use cellular_automata::CellularAutomataBuilder;

use bsp_interior::BspInteriorBuilder;

use bsp_dungeon::BspDungeonBuilder;

use common::*;

use super::{spawner, Map, Position, Rect, TileType, World, SHOW_MAPGEN_VISUALIZER};

pub struct BuilderMap {
    pub spawn_list: Vec<(usize, String)>,
    pub map: Map,
    pub starting_position: Option<Position>,
    pub rooms: Option<Vec<Rect>>,
    pub corridors: Option<Vec<Vec<usize>>>,
    pub history: Vec<Map>,
    pub width: i32,
    pub height: i32,
}

impl BuilderMap {
    fn take_snapshot(&mut self) {
        if SHOW_MAPGEN_VISUALIZER {
            let mut snapshot = self.map.clone();
            for v in snapshot.revealed_tiles.iter_mut() {
                *v = true;
            }
            self.history.push(snapshot);
        }
    }
}

pub struct BuilderChain {
    starter: Option<Box<dyn InitialMapBuilder>>,
    builders: Vec<Box<dyn MetaMapBuilder>>,
    pub build_data: BuilderMap,
}

impl BuilderChain {
    pub fn new(new_depth: i32, width: i32, height: i32) -> BuilderChain {
        BuilderChain {
            starter: None,
            builders: Vec::new(),
            build_data: BuilderMap {
                spawn_list: Vec::new(),
                map: Map::new(new_depth, width, height),
                starting_position: None,
                rooms: None,
                corridors: None,
                history: Vec::new(),
                width,
                height,
            },
        }
    }

    pub fn start_with(&mut self, starter: Box<dyn InitialMapBuilder>) {
        match self.starter {
            None => self.starter = Some(starter),
            Some(_) => panic!("You can only have one starting builder!"),
        };
    }

    pub fn with(&mut self, metabuilder: Box<dyn MetaMapBuilder>) {
        self.builders.push(metabuilder);
    }

    pub fn build_map(&mut self, rng: &mut rltk::RandomNumberGenerator) {
        match &mut self.starter {
            None => panic!("Cannot run a map builder chain without a starting build system!"),
            Some(starter) => {
                // Build the starting map
                starter.build_map(rng, &mut self.build_data);
            }
        }

        // Build additional layers in turn
        for metabuilder in self.builders.iter_mut() {
            metabuilder.build_map(rng, &mut self.build_data);
        }
    }

    pub fn spawn_entities(&mut self, ecs: &mut World) {
        for entity in self.build_data.spawn_list.iter() {
            spawner::spawn_entity(ecs, &(&entity.0, &entity.1));
        }
    }
}

pub trait InitialMapBuilder {
    fn build_map(&mut self, rng: &mut rltk::RandomNumberGenerator, build_data: &mut BuilderMap);
}

pub trait MetaMapBuilder {
    fn build_map(&mut self, rng: &mut rltk::RandomNumberGenerator, build_data: &mut BuilderMap);
}

fn random_start_position(rng: &mut rltk::RandomNumberGenerator) -> (XStart, YStart) {
    let x;
    let x_roll = rng.roll_dice(1, 3);
    match x_roll {
        1 => x = XStart::LEFT,
        2 => x = XStart::CENTER,
        _ => x = XStart::RIGHT,
    }

    let y;
    let y_roll = rng.roll_dice(1, 3);
    match y_roll {
        1 => y = YStart::TOP,
        2 => y = YStart::CENTER,
        _ => y = YStart::BOTTOM,
    }

    (x, y)
}

fn random_shape_builder(rng: &mut rltk::RandomNumberGenerator, builder: &mut BuilderChain) {
    let builder_roll = rng.roll_dice(1, 16);
    match builder_roll {
        1 => builder.start_with(CellularAutomataBuilder::new()),
        2 => builder.start_with(DrunkardsWalkBuilder::open_area()),
        3 => builder.start_with(DrunkardsWalkBuilder::open_halls()),
        4 => builder.start_with(DrunkardsWalkBuilder::winding_passages()),
        5 => builder.start_with(DrunkardsWalkBuilder::fat_passages()),
        6 => builder.start_with(DrunkardsWalkBuilder::fearful_symmetry()),
        7 => builder.start_with(MazeBuilder::new()),
        8 => builder.start_with(DLABuilder::walk_inwards()),
        9 => builder.start_with(DLABuilder::walk_outwards()),
        10 => builder.start_with(DLABuilder::central_attraction()),
        11 => builder.start_with(DLABuilder::insectoid()),
        12 => builder.start_with(VoronoiCellBuilder::pythagoras()),
        13 => builder.start_with(VoronoiCellBuilder::manhattan()),
        14 => builder.start_with(VoronoiCellBuilder::chebyshev()),
        _ => builder.start_with(PrefabBuilder::constant(
            prefab_builder::prefab_levels::POPULATED_MAP,
        )),
    }

    // Set the start to the center and cull
    builder.with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER));
    builder.with(CullUnreachable::new());

    // Now set the start to a random starting area
    let (start_x, start_y) = random_start_position(rng);
    builder.with(AreaStartingPosition::new(start_x, start_y));

    // Setup an exit and spawn mobs
    builder.with(VoronoiSpawning::new());
    builder.with(DistantExit::new());
}

fn random_room_builder(rng: &mut rltk::RandomNumberGenerator, builder: &mut BuilderChain) {
    let build_roll = rng.roll_dice(1, 3);
    match build_roll {
        1 => builder.start_with(SimpleMapBuilder::new()),
        2 => builder.start_with(BspDungeonBuilder::new()),
        _ => builder.start_with(BspInteriorBuilder::new()),
    }

    // BSP Interior still makes holes in the walls
    if build_roll != 3 {
        // Sort by one of the 5 available algorithms
        let sort_roll = rng.roll_dice(1, 5);
        match sort_roll {
            1 => builder.with(RoomSorter::new(RoomSort::LEFTMOST)),
            2 => builder.with(RoomSorter::new(RoomSort::RIGHTMOST)),
            3 => builder.with(RoomSorter::new(RoomSort::TOPMOST)),
            4 => builder.with(RoomSorter::new(RoomSort::BOTTOMMOST)),
            _ => builder.with(RoomSorter::new(RoomSort::CENTRAL)),
        }

        builder.with(RoomDrawer::new());

        let corridor_roll = rng.roll_dice(1, 4);
        match corridor_roll {
            1 => builder.with(DoglegCorridors::new()),
            2 => builder.with(NearestCorridors::new()),
            3 => builder.with(StraightLineCorridors::new()),
            _ => builder.with(BspCorridors::new()),
        }

        let cspawn_roll = rng.roll_dice(1, 2);
        if cspawn_roll == 1 {
            builder.with(CorridorSpawner::new());
        }

        let modifier_roll = rng.roll_dice(1, 6);
        match modifier_roll {
            1 => builder.with(RoomExploder::new()),
            2 => builder.with(RoomCornerRounder::new()),
            _ => {}
        }
    }

    let start_roll = rng.roll_dice(1, 2);
    match start_roll {
        1 => builder.with(RoomBasedStartingPosition::new()),
        _ => {
            let (start_x, start_y) = random_start_position(rng);
            builder.with(AreaStartingPosition::new(start_x, start_y));
        }
    }

    let exit_roll = rng.roll_dice(1, 2);
    match exit_roll {
        1 => builder.with(RoomBasedStairs::new()),
        _ => builder.with(DistantExit::new()),
    }

    let spawn_roll = rng.roll_dice(1, 2);
    match spawn_roll {
        1 => builder.with(RoomBasedSpawner::new()),
        _ => builder.with(VoronoiSpawning::new()),
    }
}

pub fn random_builder(
    new_depth: i32,
    rng: &mut rltk::RandomNumberGenerator,
    width: i32,
    height: i32,
) -> BuilderChain {
    let mut builder = BuilderChain::new(new_depth, width, height);
    let type_roll = rng.roll_dice(1, 2);
    match type_roll {
        1 => random_room_builder(rng, &mut builder),
        _ => random_shape_builder(rng, &mut builder),
    }

    if rng.roll_dice(1, 3) == 1 {
        builder.with(WaveformCollapseBuilder::new());

        // Now set the start to a random starting area
        let (start_x, start_y) = random_start_position(rng);
        builder.with(AreaStartingPosition::new(start_x, start_y));

        // Setup an exit and spawn mobs
        builder.with(VoronoiSpawning::new());
        builder.with(DistantExit::new());
    }

    if rng.roll_dice(1, 20) == 1 {
        builder.with(PrefabBuilder::sectional(
            prefab_builder::prefab_sections::UNDERGROUND_FORT,
        ));
    }

    builder.with(DoorPlacement::new());
    builder.with(PrefabBuilder::vaults());

    /*
    let mut builder = BuilderChain::new(new_depth);
    builder.start_with(BspInteriorBuilder::new());
    builder.with(DoorPlacement::new());
    builder.with(RoomBasedSpawner::new());
    builder.with(RoomBasedStairs::new());
    builder.with(RoomBasedStartingPosition::new());
    builder.with(RoomDrawer::new());
    builder.with(RoomSorter::new(RoomSort::LEFTMOST));
    builder.with(StraightLineCorridors::new());
    builder.with(RoomBasedSpawner::new());
    builder.with(CorridorSpawner::new());
    builder.with(RoomBasedStairs::new());
    builder.with(RoomBasedStartingPosition::new());
    builder.with(DoorPlacement::new());
    */
    builder
}
