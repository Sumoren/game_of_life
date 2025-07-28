#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
    Dead,
    Alive,
}

#[allow(dead_code)]
pub struct Simulation {
    grid: Vec<State>,
    off_grid: Vec<State>,
    size: usize,
}

#[allow(dead_code)]
impl Simulation {
    pub fn new(size: usize) -> Simulation {
        Simulation {
            size,
            grid: vec![State::Dead; size * size],
            off_grid: vec![State::Dead; size * size],
        }
    }

    pub fn new_from_state(initial_state: &[State]) -> Simulation {
        let size = (initial_state.len() as f64).sqrt() as usize;
        if (size * size) != initial_state.len() {
            panic!("initial state is not square");
        }

        Simulation {
            size,
            grid: initial_state.to_vec(),
            off_grid: vec![State::Dead; size * size],
        }
    }

    pub fn get_state_at(&self, column: usize, line: usize) -> State {
        self.grid[line * self.size + column]
    }

    pub fn set_state_at(&mut self, column: usize, line: usize, state: State) {
        self.grid[line * self.size + column] = state;
    }

    pub fn tick(&mut self) {
        std::mem::swap(&mut self.grid, &mut self.off_grid);

        for i in 0..self.grid.len() {
            self.grid[i] = self.get_next_state(i);
        }
    }

    fn get_next_state(&self, i: usize) -> State {
        todo!();
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

    #[test]
    fn test_tick_1() {
        let initial_state = [
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Alive,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Alive,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Alive,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
        ];

        let mut sim = Simulation::new_from_state(&initial_state);
        sim.tick();

        let expected = [
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Alive,
            State::Alive,
            State::Alive,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
        ];

        assert!(sim.grid.iter().eq(expected.iter()));
    }
}
