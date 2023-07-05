use std::collections::VecDeque;

use petgraph::prelude::*;

use crate::components::{Component, Wire};

type ComponentIndex = NodeIndex;
type WireIndex = EdgeIndex;

pub struct Event {
    component: ComponentIndex,
    output: usize,
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

    pub fn step(&mut self) {

        if let Some(event) = self.events.pop_front() {
            let component = self.graph.node_weight_mut(event.component)
                .expect("Node must have Component");
            let output_signal = component.get_output(event.output);

            let mut neighbors = self.graph.neighbors_directed(event.component, Outgoing).detach();

            while let Some((edge, node)) = neighbors.next(&self.graph) {
                let wire = *self.graph.edge_weight(edge).expect("Edge must have Wire");
                
                if wire.input() == event.output {
                    let target = self.graph.node_weight_mut(node).expect("Node must have Component");
                    target.set_input(wire.output(), output_signal);
                    let change_events: Vec<_> = target.evaluate_outputs(wire.output())
                        .iter()
                        .map(|&changed_output| Event {
                            component: node,
                            output: changed_output
                        }).collect();
                    
                    self.events.append(&mut change_events.into());
                }
            }
        }
    }
}