use std::collections::{ HashMap, VecDeque };
use std::iter::{ Iterator };

use graphics::color::hex;
use rand::{ thread_rng, Rng };


pub enum Movement {
    Rotate,
    Shift(Direction),
}


#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Down,
}


#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TetriminoType {
    O,
    I,
    T,
    S,
    Z,
    J,
    L,
}


#[derive(Clone, Debug)]
struct Rotation {
    internal: Vec<Vec<Vec<bool>>>,
    curr_idx: usize,
}


impl Rotation {
    fn new(states: Vec<Vec<Vec<bool>>>) -> Rotation {
        Rotation {
            internal: states,
            curr_idx: 0,
        }
    }

    fn next_idx(&self) -> usize {
        let mut idx = self.curr_idx + 1;
        if idx >= self.internal.len() {
            idx = 0;
        }
        idx
    }

    fn as_blocks(state: &Vec<Vec<bool>>, x_offset: i32, y_offset: i32, color: [f32; 4])
                 -> Vec<Block> {
        let mut blocks: Vec<Block> = vec![];
        for (y, row) in state.iter().enumerate() {
            for (x, &cell_is_valued) in row.iter().enumerate() {
                if cell_is_valued {
                    blocks.push(Block {
                        x: x_offset + x as i32,
                        y: y_offset - y as i32,
                        color,
                    });
                }
            }
        }
        blocks
    }

    fn curr_as_blocks(&self, x_offset: i32, y_offset: i32, color: [f32; 4]) -> Vec<Block> {
        let idx = self.curr_idx;
        Rotation::as_blocks(self.internal.get(idx).unwrap(),
                         x_offset, y_offset, color)
    }


    fn peek_as_blocks(&self, x_offset: i32, y_offset: i32, color: [f32; 4]) -> Vec<Block> {
        let next_idx = self.next_idx();
        Rotation::as_blocks(self.internal.get(next_idx).unwrap(),
                         x_offset, y_offset, color)
    }

    fn change(&mut self) {
        self.curr_idx = self.next_idx();
    }
}


struct States {
    states: HashMap<TetriminoType, Vec<Vec<Vec<bool>>>>,
    colors: HashMap<TetriminoType, [f32; 4]>,
}


impl States {
    fn init() -> States {
        let tet_states: HashMap<TetriminoType, Vec<Vec<Vec<bool>>>> = [
            (TetriminoType::O, states!("O")),
            (TetriminoType::I, states!("I")),
            (TetriminoType::T, states!("T")),
            (TetriminoType::S, states!("S")),
            (TetriminoType::Z, states!("Z")),
            (TetriminoType::J, states!("J")),
            (TetriminoType::L, states!("L")),
        ].iter().cloned().collect();
        let tet_colors: HashMap<TetriminoType, [f32; 4]> = [
            (TetriminoType::O, hex("f0f000")),
            (TetriminoType::I, hex("00f0f0")),
            (TetriminoType::T, hex("a000f0")),
            (TetriminoType::S, hex("00f000")),
            (TetriminoType::Z, hex("f00000")),
            (TetriminoType::J, hex("0000f0")),
            (TetriminoType::L, hex("f0a000")),
        ].iter().cloned().collect();
        States {
            states: tet_states,
            colors: tet_colors,
        }
    }
}

pub struct Tetriminos {
    states: States,
    queued: VecDeque<Tetrimino>,
}


impl Tetriminos {
    pub fn init() -> Tetriminos {
        Tetriminos {
            states: States::init(),
            queued: VecDeque::new(),
        }
    }
    pub fn states(&self) -> &HashMap<TetriminoType, Vec<Vec<Vec<bool>>>> {
        &self.states.states
    }
    pub fn types(&self) -> Vec<TetriminoType> {
        self.states.states.keys().map(|k| k.clone()).collect()
    }

    fn maybe_refill_queue(&mut self) -> bool {
        let mut was_empty = false;
        if self.queued.is_empty() {
            let mut rng = thread_rng();
            let mut types = self.types();
            rng.shuffle(&mut types);
            let next_gen: VecDeque<Tetrimino> = types.into_iter()
                .map(|tet_type| Tetrimino::new(tet_type.clone(), &self))
                .collect();
            self.queued.extend(next_gen);
            was_empty = true;
        }
        was_empty
    }

    pub fn peek(&mut self) -> Tetrimino {
        self.maybe_refill_queue();
        self.queued[0].clone()
    }
}


impl Iterator for Tetriminos {
    type Item = Tetrimino;

    fn next(&mut self) -> Option<Tetrimino> {
        self.maybe_refill_queue();
        self.queued.pop_front()
    }
}


#[derive(Clone, Debug)]
pub struct Tetrimino {
    shape: TetriminoType,
    rotation: Rotation,
    x: i32,
    y: i32,
    color: [f32; 4],
}


impl Tetrimino {
    pub fn new(shape: TetriminoType, tetriminos: &Tetriminos)
               -> Tetrimino {
        let rotation = Rotation::new(tetriminos.states().get(&shape).unwrap().clone());
        let color = tetriminos.states.colors.get(&shape).unwrap().clone();
        Tetrimino {
            shape,
            rotation,
            color,
            x: 3,
            y: 21,
        }
    }

    pub fn shift(&mut self, direction: Direction, on_grid: &Grid) -> bool {
        let mut min_y = on_grid.height;
        let mut min_x = on_grid.width;
        let mut max_x = 0;
        for block in &self.blocks() {
            if block.y < min_y {
                min_y = block.y;
            }
            if block.x < min_x {
                min_x = block.x;
            }
            if block.x > max_x {
                max_x = block.x;
            }
        }
        match direction {
            Direction::Down if min_y > 1 => {
                self.y -= 1;
                true
            },
            Direction::Left if min_x > 0 => {
                self.x -= 1;
                true
            },
            Direction::Right if max_x < on_grid.width - 1 => {
                self.x += 1;
                true
            },
            _ => {
                false
            },
        }
    }

    pub fn rotate(&mut self, on_grid: &Grid) -> bool {
        let next = self.rotation.peek_as_blocks(self.x, self.y, self.color.clone());
        match on_grid.is_legal(&next) {
            true => { self.rotation.change(); true },
            false => { false },
        }
    }

    pub fn peek(&self, movement: &Movement) -> Vec<Block> {
        match movement {
            &Movement::Rotate => self.rotation.peek_as_blocks(self.x, self.y, self.color.clone()),
            &Movement::Shift(ref dir) => {
                let blocks = self.blocks();
                match dir {
                    &Direction::Down => blocks.iter().map(|block| Block {
                        x: block.x,
                        y: block.y - 1,
                        color: block.color.clone(),
                    }).collect(),
                    &Direction::Left => blocks.iter().map(|block| Block {
                        x: block.x - 1,
                        y: block.y,
                        color: block.color.clone(),
                    }).collect(),
                    &Direction::Right => blocks.iter().map(|block| Block {
                        x: block.x + 1,
                        y: block.y,
                        color: block.color.clone(),
                    }).collect(),
                }
            }
        }
    }

    pub fn blocks(&self) -> Vec<Block> {
        let x_offset = self.x;
        let y_offset = self.y;
        let color = self.color.clone();
        self.rotation.curr_as_blocks(x_offset, y_offset, color)
    }
}


#[derive(Clone, Debug)]
pub struct Block {
    pub x: i32,
    pub y: i32,
    pub color: [f32; 4],
}


impl PartialEq for Block {
    fn eq(&self, other: &Block) -> bool {
        self.x == other.x && self.y == other.y
    }
}


pub struct Grid {
    pub height: i32,
    pub width: i32,
    blocks: Vec<Block>,
}

impl Grid {
    pub fn new(height: i32, width: i32) -> Grid {
        let blocks: Vec<Block> = vec![];
        Grid {
            height,
            width,
            blocks,
        }
    }

    pub fn blocks(&self) -> Vec<Block> {
        let result: Vec<Block> = self.blocks.iter()
            .map(|block| block.clone())
            .collect();
        result
    }

    pub fn lock(&mut self, tetrimino: Tetrimino) {
        let blocks = tetrimino.blocks();
        self.blocks.extend(blocks);
    }

    fn decrement_rows_above(&mut self, row: i32) {
        let mut blocks = self.blocks();
        for block in &mut blocks {
            if block.y > row {
                block.y = block.y - 1;
            }
        }
        self.blocks = blocks;
    }

    fn delete_row(&mut self, row: i32) {
        let blocks: Vec<Block> = self.blocks().into_iter()
            .filter(|block| block.y != row)
            .collect();
        self.blocks = blocks;
    }

    pub fn clear_full_rows(&mut self) -> u32 {
        let mut num_cleared = 0;
        let rows = (0..self.height+1).rev();
        for row in rows {
            let num_blocks = self.blocks.iter()
                .filter(|&block| block.y == row)
                .count();
            if num_blocks as i32 == self.width {
                self.delete_row(row);
                self.decrement_rows_above(row);
                num_cleared += 1;
            }
        }
        num_clears
    }

    pub fn has_landed(&self, tetrimino: &Tetrimino) -> bool {
        tetrimino.blocks().iter()
            .any(|ref block| {
                block.y == 1 ||
                    self.blocks.contains(&Block {
                        x: block.x,
                        y: block.y - 1,
                        color: block.color.clone(),
                    })
            })
    }

    pub fn is_legal(&self, blocks: &Vec<Block>) -> bool {
        !blocks.iter().any(|ref block| {
            self.blocks.contains(&block) ||
            block.x >= self.width ||
            block.y > self.height ||
            block.y < 1 ||
            block.x < 0
        })
    }
}
