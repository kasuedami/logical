use logical::components::*;
use logical::simulator::*;

fn main() {
    let mut simulator = Simulator::new();
    let component = Component::new();

    let comp0 = simulator.add_component(component);
    let comp1 = simulator.add_component(Component::new());

    let wire = simulator.add_wire(comp0, comp1, Wire::new(0, 0));

    simulator.step();
}