mod gol;

use gol::simulation::*;

fn main() {
    let mut simulation = Simulation::new(10);
    simulation.set_state_at(5, 5, State::Alive);
}
