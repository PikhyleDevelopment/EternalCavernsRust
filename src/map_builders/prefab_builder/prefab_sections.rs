#[allow(dead_code)]
#[derive(PartialEq, Copy, Clone)]
pub enum HorizontalPlacement { 
    Left, 
    Center, 
    Right 
}

#[allow(dead_code)]
#[derive(PartialEq, Copy, Clone)]
pub enum VerticalPlacement {
    Top, 
    Center, 
    Bottom 
}

#[allow(dead_code)]
#[derive(PartialEq, Copy, Clone)]
pub struct PrefabSection {
    pub template : &'static str,
    pub width : usize,
    pub height: usize,
    pub placement : (HorizontalPlacement, VerticalPlacement)
}

#[allow(dead_code)]
pub const UNDERGROUND_FORT : PrefabSection = PrefabSection{
    template : RIGHT_FORT,
    width: 15,
    height: 43,
    placement: ( HorizontalPlacement::Right, VerticalPlacement::Top )
};

#[allow(dead_code)]
/*
TODO: This constant was actually copied from www.github.com/thebracket source code.
    While trying to implement my own, I kept getting an index out of bound error at
    prefab_builder/mod.rs > apply_sectional() > self.char_to_map(string_vec[i], idx)
    I believe that the error originated from this constant declaration. When i copy/pasted from
    thebracket's source code, my IDE showed spaces as "NBSP". I replaced all with a ' ' and it
    made the code run as intended.
 */

const RIGHT_FORT : &str = "
     #         
  #######      
  #     #      
  #     #######
  #  g        #
  #     #######
  #     #      
  ### ###      
    # #        
    # #        
    # ##       
    ^          
    ^          
    # ##       
    # #        
    # #        
    # #        
    # #        
  ### ###      
  #     #      
  #     #      
  #  g  #      
  #     #      
  #     #      
  ### ###      
    # #        
    # #        
    # #        
    # ##       
    ^          
    ^          
    # ##       
    # #        
    # #        
    # #        
  ### ###      
  #     #      
  #     #######
  #  g        #
  #     #######
  #     #      
  #######      
     #         
";
