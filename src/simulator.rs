use std::{collections::VecDeque, fmt};

use petgraph::prelude::*;

use crate::components::{Component, Wire, Signal};

type ComponentIndex = NodeIndex;
type WireIndex = EdgeIndex;

pub struct Event {
    component: ComponentIndex,
    output: usize,
}

#[derive(Debug, Clone)]
pub struct UnstableError;

impl fmt::Display for UnstableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Simulation is unstable")
    }
}

pub struct Simulator {
    graph: Graph<Component, Wire, Directed>,
    events: VecDeque<Event>,
}

impl Simulator {
    pub fn new() -> Self {
        Self { graph: Graph::new(), events: VecDeque::new() }
    }

    pub fn add_component(&mut self, component: Component) -> ComponentIndex {
        self.graph.add_node(component)
    }

    pub fn add_wire(
        &mut self,
        component0: ComponentIndex,
        component1: ComponentIndex,
        wire: Wire
    ) -> WireIndex {
        self.graph.add_edge(component0, component1, wire)
    }

    pub fn set_input(&mut self, node: ComponentIndex, value: Signal) {
        let component = self.graph.node_weight_mut(node)
            .expect("Node must exist");

        match component {
            Component::Logic(_) => panic!("Stupid user lul"),
            Component::Input(input) => {
                if input.get_value() != value {
                    input.set_value(value);
                    self.events.push_back(Event {
                        component: node,
                        output: 0
                    });
                }
            },
        }
    }

    pub fn step(&mut self) {

        if let Some(event) = self.events.pop_front() {
            let component = self.graph.node_weight_mut(event.component)
                .expect("Node must have Component");

            match component {
                Component::Logic(logic) => {

                    let output_signal = logic.get_output(event.output);

                    let mut neighbors = self.graph.neighbors_directed(event.component, Outgoing).detach();
        
                    while let Some((edge, node)) = neighbors.next(&self.graph) {
                        let wire = *self.graph.edge_weight(edge).expect("Edge must have Wire");
                        
                        if wire.input() == event.output {
                            let target = self.graph.node_weight_mut(node)
                                .expect("Node must have Component");

                            match target {
                                Component::Logic(target_logic) => {
                                    target_logic.set_input(wire.output(), output_signal);
                                    let change_events: Vec<_> = target_logic.evaluate_outputs(wire.output())
                                        .iter()
                                        .map(|&changed_output| Event {
                                            component: node,
                                            output: changed_output
                                        }).collect();

                                        dbg!(target_logic);
                                    
                                    self.events.append(&mut change_events.into());
                                },
                                Component::Input(_) => todo!(),
                            }
                            
                        }
                    }
                },
                Component::Input(input) => {

                    let output_signal = input.get_value();

                    let mut neighbors = self.graph.neighbors_directed(event.component, Outgoing).detach();
        
                    while let Some((edge, node)) = neighbors.next(&self.graph) {
                        let wire = *self.graph.edge_weight(edge).expect("Edge must have Wire");
                        
                        if wire.input() == event.output {
                            let target = self.graph.node_weight_mut(node)
                                .expect("Node must have Component");

                            match target {
                                Component::Logic(target_logic) => {
                                    target_logic.set_input(wire.output(), output_signal);
                                    let change_events: Vec<_> = target_logic.evaluate_outputs(wire.output())
                                        .iter()
                                        .map(|&changed_output| Event {
                                            component: node,
                                            output: changed_output
                                        }).collect();
                                    
                                    dbg!(target_logic);

                                    self.events.append(&mut change_events.into());
                                },
                                Component::Input(_) => todo!(),
                            }
                            
                        }
                    }
                },
            }
        }
    }

    pub fn step_until_stable(&mut self, limit: usize) -> Result<usize, UnstableError> {
        let mut iteration = 0;
        
        while !self.events.is_empty() {

            if iteration < limit {
                iteration += 1;
            } else {
                return Err(UnstableError);
            }

            self.step();
        }

        Ok(iteration)
    }
}