use z1d_cellular_automata::{flag::Flag, state::State, wca_storage::WCAStorage};

fn main() {
    const MAX: usize = 20;
    let sequence: Vec<State> = vec![State::ALIVE,State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::ALIVE, State::DEAD, State::DEAD, State::ALIVE, State::ALIVE, State::ALIVE, State::ALIVE, State::DEAD, State::ALIVE, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD];

    let mut container: WCAStorage = WCAStorage::new(Some(sequence));
    container.push(State::DEAD);
    container.push(State::ALIVE);
    container.push(State::DEAD);
    container.push(State::DEAD);
    container.push(State::DEAD);
    container.push(State::ALIVE);
    container.push(State::DEAD);
    container.push(State::DEAD);
    container.push(State::DEAD);
    container.push(State::DEAD);
    container.push(State::DEAD);
    container.push(State::DEAD);
    container.push(State::DEAD);
    container.push(State::ALIVE);

    let running: Result<Flag, &'static str> = container.run(MAX);
    match running {
        Ok(_) => {
            println!(
                "ⓘ -> All fine ! {}x{} automatons generated !",
                container.len(),
                MAX
            )
        }
        Err(err) => {
            println!("{}", err)
        }
    }
}
