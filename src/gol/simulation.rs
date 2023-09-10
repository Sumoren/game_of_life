#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
    Dead,
    Alive
}

pub struct Simulation {
    grid: Vec<State>,
    size: usize
}

impl Simulation {
    pub fn new(size: usize) -> Simulation {
        Simulation {
            size,
            grid: vec![State::Dead; size * size]
        }
    }

    pub fn get_state_at(&self, column: usize, line: usize) -> State {
        self.grid[line * self.size + column]
    }

    pub fn set_state_at(&mut self, column: usize, line: usize, state : State) {
        self.grid[line * self.size + column] = state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_dead_at_init() {
        let sim = Simulation::new(2);
        assert!(sim.grid.iter().all(|&x| x == State::Dead));
    }

    #[test]
    fn test_set_state_at() {
        let mut sim = Simulation::new(2);
        sim.set_state_at(0, 0, State::Alive);
        assert_eq!(sim.get_state_at(0, 0), State::Alive);
    }
}
