use std::collections::{ HashMap, VecDeque };
use std::iter::FromIterator;

use rand::{ thread_rng, Rng };

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum TetriminoType {
    O,
    I,
    T,
    S,
    Z,
    J,
    L,
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
                        .map(|shape| Tetrimino { shape }));
                self.queued.pop_front()
            }
        }
    }
}


#[derive(Debug)]
pub struct Tetrimino {
    pub shape: TetriminoType,
}
