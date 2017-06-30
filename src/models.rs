use std::collections::{ HashMap, VecDeque };
use std::iter::{ FromIterator, Iterator };

use rand::{ thread_rng, Rng };


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


#[derive(Debug)]
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

    fn as_blocks(state: &Vec<Vec<bool>>, x_offset: usize, y_offset: usize)
                 -> Vec<Block> {
        let mut blocks: Vec<Block> = vec![];
        for (y, row) in state.iter().enumerate() {
            for (x, &cell_is_valued) in row.iter().enumerate() {
                if cell_is_valued {
                    blocks.push(Block {
                        x: x + x_offset,
                        y: y + y_offset,
                    });
                }
            }
        }
        blocks
    }

    fn curr_as_blocks(&self, x_offset: usize, y_offset: usize) -> Vec<Block> {
        let idx = self.curr_idx;
        Rotation::as_blocks(self.internal.get(idx).unwrap(),
                         x_offset, y_offset)
    }


    fn peek_as_blocks(&self, x_offset: usize, y_offset: usize) -> Vec<Block> {
        let next_idx = self.next_idx();
        Rotation::as_blocks(self.internal.get(next_idx).unwrap(),
                         x_offset, y_offset)
    }

    fn change(&mut self) {
        self.curr_idx = self.next_idx();
    }
}


struct States {
    states: HashMap<TetriminoType, Vec<Vec<Vec<bool>>>>,
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
        States {
            states: tet_states,
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
}


impl Iterator for Tetriminos {
    type Item = Tetrimino;

    fn next(&mut self) -> Option<Tetrimino> {
        let next = self.queued.pop_front();
        match next {
            Some(tet) => Some(tet),
            None => {
                let mut rng = thread_rng();
                let mut shapes = self.types();
                rng.shuffle(&mut shapes);
                self.queued = VecDeque::from_iter(
                    shapes.into_iter()
                        .map(|shape| Tetrimino::new(shape, &self)));
                self.queued.pop_front()
            }
        }
    }
}


#[derive(Debug)]
pub struct Tetrimino {
    shape: TetriminoType,
    rotation: Rotation,
    x: usize,
    y: usize,
}


impl Tetrimino {
    pub fn new(shape: TetriminoType, tetriminos: &Tetriminos)
               -> Tetrimino {
        let mut rotation = Rotation::new(tetriminos.states().get(&shape).unwrap().clone());
        Tetrimino {
            shape,
            rotation,
            x: 0,
            y: 0,
        }
    }

    pub fn rotate(&mut self) {
        self.rotation.change();
    }

    pub fn peek(&mut self) {
        println!("{:?}", self.rotation.peek_as_blocks(self.x, self.y));
    }

    pub fn blocks(&mut self) -> Vec<Block> {
        let x_offset = self.x;
        let y_offset = self.y;
        self.rotation.curr_as_blocks(x_offset, y_offset)
    }
}


#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    x: usize,
    y: usize,
}


pub struct Grid {
    height: usize,
    width: usize,
    blocks: Vec<Block>,
}
