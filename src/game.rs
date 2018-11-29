use piston_window::*;

const ROW_COUNT: u16 = 24;
const COL_COUNT: u16 = 14;

// Possible cell types
enum CellType {
    Empty,
    Filled,
    Resolved
}

// Struct for a cell
pub struct Cell {
    type: CellType,
}

impl Cell {
    fn new(type: CellType) -> Cell {
        Cell {
            type
        }
    }
}


// Tetris is the possible shapes
enum TetrisType {
    // I shaped
    Idom,
    // T shaped
    Tarom,
    // Z shaped
    Zorro,
    // Rectangle
    Rex,
}

// Struct for the tetris (the shape)
struct Tetris {
    // the type
    type: TetrisType,
    // middle block position coordinates
    pos_x: u32,
    pos_y: u32,
}

impl Tetris {
    pub fn new(type: TetrisType) -> Tetris {
        Tetris {
            type,
            pos_x: 7,
            pos_y: 0
        }
    }
}



pub struct Game {
    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,

    grid: [[[CellType::new(CellType::Empty); COL_COUNT as usize]; ROW_COUNT as usize]],
    tetris: Tetris
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            width,
            height,
            game_over: false,
            waiting_time: 0.0,

            grid: [[0; ROW_COUNT as usize]; COL_COUNT as usize],
            tetris: Tetris::new(TetrisType::Rex)
        }
    }

    pub fn key_pressed(&self, key: Key) {
        if self.game_over {
            self.restart_game()
        };

        let dir = match key {
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None
        };

        self.tetris.update_dir(dir);
    }

    pub fn restart_game(&mut self) {
        // Should restart the game
        return;
    }
}