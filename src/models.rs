use std::collections::HashMap;

#[macro_use]
use macros;


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
}

impl Tetriminos {
    pub fn init() -> Tetriminos {
        Tetriminos {
            states: States::init(),
        }
    }
    pub fn states(&self) -> &HashMap<TetriminoType, Vec<String>> {
        &self.states.states
    }
}
