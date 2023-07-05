use logical::components::*;
use logical::simulator::*;

fn main() {
    let mut simulator = Simulator::new();

    let comp0 = simulator.add_component(Component::Input(Input::new()));
    let comp1 = simulator.add_component(Component::Logic(Logic::new_and(2)));
    let comp2 = simulator.add_component(Component::Input(Input::new()));

    simulator.add_wire(comp0, comp1, Wire::new(0, 0));
    simulator.add_wire(comp2, comp1, Wire::new(0, 1));

    simulator.set_input(comp0, Signal::High);
    simulator.set_input(comp2, Signal::High);

    simulator.step();
    simulator.step();
}