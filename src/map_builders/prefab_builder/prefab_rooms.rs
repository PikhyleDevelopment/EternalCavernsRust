#[allow(dead_code)]
#[derive(PartialEq, Copy, Clone)]
pub struct PrefabRoom {
    pub template: &'static str,
    pub width: usize,
    pub height: usize,
    pub first_depth: i32,
    pub last_depth: i32,
}

#[allow(dead_code)]
pub const TOTALLY_NOT_A_TRAP: PrefabRoom = PrefabRoom {
    template: TOTALLY_NOT_A_TRAP_MAP,
    width: 5,
    height: 5,
    first_depth: 0,
    last_depth: 100,
};

#[allow(dead_code)]
/*
 TODO: Just a note regarding these constants. As stated in this same style of
 todo statement in prefab_sections.rs, I had to copy and paste from thebracket's source code.
    Here, I tried to do the same thing as last time and it still wouldn't work. *SO* i added the
    line break to enforce the correct layout.
 */
const TOTALLY_NOT_A_TRAP_MAP: &str = "
     \n
 ^^^ \n
 ^!^ \n
 ^^^ \n
     \n
";

#[allow(dead_code)]
pub const SILLY_SMILE: PrefabRoom = PrefabRoom {
    template: SILLY_SMILE_MAP,
    width: 6,
    height: 6,
    first_depth: 0,
    last_depth: 100
};

#[allow(dead_code)]
const SILLY_SMILE_MAP: &str = "
      \n
 ^  ^ \n
  #   \n
      \n
 ###  \n
      \n
";

#[allow(dead_code)]
pub const CHECKERBOARD: PrefabRoom = PrefabRoom {
    template: CHECKERBOARD_MAP,
    width: 6,
    height: 6,
    first_depth: 0,
    last_depth: 100,
};

#[allow(dead_code)]
const CHECKERBOARD_MAP: &str = "
      &nbsp
 g#%# &nbsp
 #!#  &nbsp
 ^# # &nbsp
      &nbsp
";