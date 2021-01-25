use super::{
    MetaMapBuilder, BuilderMap
};
use rltk::RandomNumberGenerator;

pub struct DoorPlacement {}

impl MetaMapBuilder for DoorPlacement {
    #[allow(dead_code)]
    fn build_map(&mut self, rng: &mut rltk::RandomNumberGenerator, build_data: &mut BuilderMap) {
	self.doors(rng, build_data);
    }
}

// TODO: implement DoorPlacement
