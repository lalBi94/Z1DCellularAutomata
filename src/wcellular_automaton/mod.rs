use crate::state::State;

#[derive(Debug, Clone, Copy)]
pub struct WCellularAutomaton {
    state: State,
}

impl WCellularAutomaton {
    pub fn new(state: State) -> WCellularAutomaton {
        WCellularAutomaton { state: state }
    }

    pub fn get_state(self: &Self) -> State {
        self.state
    }
}