use crate::wcellular_automaton::WCellularAutomaton;
use crate::state::State;

pub struct Rules;
impl Rules {
    pub fn evaluate(neighbors: [WCellularAutomaton; 3]) -> State {
        let prev: State = neighbors[0].get_state();
        let curr: State = neighbors[1].get_state();
        let next: State = neighbors[2].get_state();

        match (prev, curr, next) {
            (State::ALIVE, State::ALIVE, State::ALIVE) => State::DEAD,
            (State::ALIVE, State::ALIVE, State::DEAD) => State::DEAD,
            (State::ALIVE, State::DEAD, State::ALIVE) => State::DEAD,
            (State::ALIVE, State::DEAD, State::DEAD) => State::ALIVE,
            (State::DEAD, State::ALIVE, State::ALIVE) => State::ALIVE,
            (State::DEAD, State::ALIVE, State::DEAD) => State::ALIVE,
            (State::DEAD, State::DEAD, State::ALIVE) => State::ALIVE,
            (State::DEAD, State::DEAD, State::DEAD) => State::DEAD,
        }
    }
}