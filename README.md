# ETERNAL CAVERNS (Rust Edition) - By Pikhyle

This is a roguelike adventure based off the tutorial from http://bfnightly.bracketproductions.com/rustbook
written by [Herbert Wolverson](https://github.com/thebracket)

### Main Controls:
| Result        | Key                   |
|---------------|-----------------------|
| UP            | UpArrow, k, Numpad8   |
| DOWN          | DownArrow, j, Numpad2 |
| LEFT          | LeftArrow, h, Numpad4 |
| RIGHT         | RightArrow, l, Numpad6|
| UPLEFT        | y, Numpad7            |
| UPRIGHT       | u, Numpad9            |
| DOWNLEFT      | b, Numpad1            |
| DOWNRIGHT     | n, Numpad3            |
| GRAB          | g                     |
| SHOW INVENTORY| i                     |
| DROP ITEM     | d                     |
| REMOVE ITEM   | r                     |
| TAKE STAIRS   | >                     |
| WAIT          | Space, Numpad5        |

### Equipping Items
When in the inventory screen, press the corresponding key to the item to equip it. 

### Dequipping (Removing) Items
When in the Remove Item Screen, press the corresponding key to the item you want to dequip.

### Current Monsters
- Orc
- Goblin
- Troll

### Current Magic
- Fireball Scroll
    - Area of Effect attack. Targetable.
- Confusion Scroll
    - Target a single enemy to confuse them for a certain amount of turns.
- Magic Missile Scroll
    - Target a single enemy to fire a magical missile at them.
- Scroll of Magic Mapping
    - Rare scroll that will show you the entire map of the level.
    
### Current Potions
- Health Potion
    - Restores a certain amount of HP
  
### Map Generation

#### The following are the type of map generation utilized in this game:
- Simple Map
  - Generates rooms around the map, then adds horizontal and vertical passage ways.
- Binary Space Partition (BSP) 
  - BSP rooms as well as interior design (like inside of a structure)
- Cellular Automata
  - Organic looking maps like you're in a cave
- Drunkards walk
  - Three different implementations that utilize the Drunkards Walk generation
- Maze
  - Generates maze like structure
- Diffusion-limited aggregation (DLA)
  - Carves out floor from a central point, factors include: 
    - DLA Algorithm
    - DLA Symmetry
    - Brush size
    - Floor percentage