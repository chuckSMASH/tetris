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
struct State {
    strings: Vec<String>,
    curr_idx: usize,
}




impl State {
    fn new(states: Vec<String>) -> State {
        State {
            strings: states,
            curr_idx: 0,
        }
    }
}


impl Iterator for State {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut curr_idx = self.curr_idx;
        if curr_idx >= self.strings.len() {
            curr_idx = 0;
        }
        self.curr_idx = curr_idx + 1;
        Some(self.strings.get(curr_idx).unwrap().clone())
    }
}


struct States {
    states: HashMap<TetriminoType, Vec<String>>,
}


impl States {
    fn init() -> States {
        let tet_states: HashMap<TetriminoType, Vec<String>> = [
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
    pub fn states(&self) -> &HashMap<TetriminoType, Vec<String>> {
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
    states: State,
    rotation: String,
    x: usize,
    y: usize,
}


impl Tetrimino {
    pub fn new(shape: TetriminoType, tetriminos: &Tetriminos)
               -> Tetrimino {
        let mut rotations = State::new(tetriminos.states().get(&shape).unwrap().clone());
        let rotation = rotations.next().unwrap();
        Tetrimino {
            shape,
            rotation,
            states: rotations,
            x: 0,
            y: 0,
        }
    }

    pub fn rotate(&mut self) {
        self.rotation = self.states.next().unwrap();
    }

    pub fn as_blocks(&mut self) -> Vec<Block> {
        let x_offset = self.x;
        let y_offset = self.y;
        let mut blocks: Vec<Block> = vec![];
        for (y, line) in self.rotation.lines().enumerate() {
            for (x, digit) in line.chars().enumerate() {
                if digit == '1' {
                    blocks.push(Block {
                        x: x + x_offset,
                        y: y + y_offset,
                    });
                }
            }
        }
        blocks
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
