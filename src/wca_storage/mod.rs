use crate::wcellular_automaton::WCellularAutomaton;
use crate::flag::Flag;
use crate::state::State;
use crate::rules::Rules;


#[derive(Debug, Clone)]
pub struct WCAStorage {
    list: Vec<WCellularAutomaton>,
}

impl WCAStorage {
    pub fn new(pre_list: Option<Vec<State>>) -> WCAStorage {
        WCAStorage {
            list: match pre_list {
                Some(l) => {
                    let mut container: Vec<WCellularAutomaton> = Vec::new();

                    for i in 0..l.len() {
                        container.push(WCellularAutomaton::new(l[i]));
                    }

                    container
                }
                None => vec![],
            },
        }
    }

    pub fn push(self: &mut Self, ca: State) -> () {
        self.list.push(WCellularAutomaton::new(ca));
    }

    pub fn run(self: &mut Self, limit: usize) -> Result<Flag, &'static str> {
        const SYMB_ALIVE: &str = "  ";
        const SYMB_DEAD: &str = "██";

        println!("ⓘ -> For {}", self.get_sequence());

        for _ in 0..limit {
            let mut stock: Vec<WCellularAutomaton> = vec![self.list[0]];
            for j in 1..self.list.len() - 1 {
                let prev: WCellularAutomaton;
                let cur: WCellularAutomaton;
                let next: WCellularAutomaton;

                match (self.list.get(j - 1), self.list.get(j), self.list.get(j + 1)) {
                    (Some(p), Some(c), Some(n)) => {
                        prev = *p;
                        cur = *c;
                        next = *n;
                    }
                    _ => return Err("Cannot retreive index of one of neighbors."),
                }

                let res: State = Rules::evaluate([prev, cur, next]);
                stock.push(WCellularAutomaton::new(res));

                print!(
                    "{}",
                    match res {
                        State::ALIVE => SYMB_ALIVE,
                        State::DEAD => SYMB_DEAD,
                    }
                );
            }

            stock.push(self.list[self.list.len() - 1]);
            self.list = stock;

            print!("\n");
        }

        Ok(Flag::AllFine)
    }

    pub fn get_sequence(self: &Self) -> String {
        let mut str: String = String::new();

        for i in 0..self.list.len() {
            str.push(match self.list[i].get_state() {
                State::ALIVE => '0',
                State::DEAD => '1',
            });
        }

        str
    }

    pub fn len(self: &Self) -> usize {
        self.list.len()
    }
}