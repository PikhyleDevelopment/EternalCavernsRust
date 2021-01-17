use rltk::rex::XpFile;

rltk::embedded_resource!(SMALL_DUNGEON, "../resources/menu_background.xp");
rltk::embedded_resource!(CUSTOM_MAP, "../resources/custom_map.xp");

pub struct RexAssets {
    pub menu: XpFile,
}

impl RexAssets {
    #[allow(clippy::new_without_default)]
    pub fn new() -> RexAssets {
        rltk::link_resource!(SMALL_DUNGEON, "../resources/menu_background.xp");
        rltk::link_resource!(CUSTOM_MAP, "../resources/custom_map.xp");

        RexAssets {
            menu: XpFile::from_resource("../resources/menu_background.xp").unwrap(),
        }
    }
}
