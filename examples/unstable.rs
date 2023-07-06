use logical::{simulator::Simulator, components::{Component, Input, Wire, Signal}};

fn main() {
    let mut simulator = Simulator::new();

    let input = simulator.add_component(Component::Input(Input::new()));
    let and_gate = simulator.add_component(Component::new_nand(2));

    simulator.add_wire(input, and_gate, Wire::new(0, 0));
    simulator.add_wire(and_gate, and_gate, Wire::new(0, 1));

    simulator.set_input(input, Signal::High);

    let result = simulator.step_until_stable(20);

    assert!(result.is_err())
}