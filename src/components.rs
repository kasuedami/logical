use std::ops::{BitAnd, BitOr};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    High,
    Low,
    #[default]
    Undefined,
}

impl BitAnd for Signal {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Signal::High, Signal::High) => Signal::High,
            (_, Signal::Low) => Signal::Low,
            (Signal::Low, _) => Signal::Low,
            (_, _) => Signal::Undefined,
        }
    }
}

impl BitOr for Signal {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Signal::Low, Signal::Low) => Signal::Low,
            (_, Signal::High) => Signal::High,
            (Signal::High, _) => Signal::High,
            (_, _) => Signal::Undefined,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Wire {
    input: usize,
    output: usize,
}

impl Wire {
    pub fn new(input: usize, output: usize) -> Self {
        Self { input, output }
    }

    pub fn input(&self) -> usize {
        self.input
    }

    pub fn output(&self) -> usize {
        self.output
    }
}

//TODO: replace Vec<usize> with bitmask
#[derive(Debug, Clone)]
pub enum Operation {
    And(Vec<usize>),
    Or(Vec<usize>),
    Nand(Vec<usize>),
}

impl Operation {
    fn get_inputs(&self) -> &Vec<usize> {
        match self {
            Operation::And(inputs) => inputs,
            Operation::Or(inputs) => inputs,
            Operation::Nand(inputs) => inputs,
        }
    }

    fn execute(&self, inputs: Vec<Signal>) -> Signal {
        match self {
            Operation::And(input_indices) => {
                if input_indices.len() != inputs.len() {
                    panic!("Wrong number of inputs passed in")
                }

                let mut out = Signal::High;

                for input in inputs {
                    out = out & input;
                }

                out
            },
            Operation::Or(input_indices) => {
                if input_indices.len() != inputs.len() {
                    panic!("Wrong number of inputs passed in")
                }

                let mut out = Signal::Low;

                for input in inputs {
                    out = out | input;
                }

                out
            },
            Operation::Nand(input_indices) => {
                if input_indices.len() != inputs.len() {
                    panic!("Wrong number of inputs passed in")
                }

                for input in inputs {
                    if input == Signal::Low {
                        return Signal::High;
                    }
                }

                Signal::Low
            },
        }
    }
}

pub enum Component {
    Logic(Logic),
    Input(Input),
}

impl Component {
    pub fn new_and(inputs: usize) -> Self {
        Self::Logic(Logic::new_and(inputs))
    }

    pub fn new_nand(inputs: usize) -> Self {
        Self::Logic(Logic::new_nand(inputs))
    }
}

#[derive(Debug)]
pub struct Logic {
    inputs: Vec<Signal>,
    outputs: Vec<Signal>,
    operations: Vec<Operation>,
}

impl Logic {
    pub fn new() -> Self {
        Self { inputs: vec![], outputs: vec![], operations: vec![] }
    }

    pub fn new_and(inputs: usize) -> Self {
        Self { inputs: vec![Signal::Low; inputs], outputs: vec![Signal::Low; 1], operations: vec![Operation::And((0..inputs).collect()); 1] }
    }

    pub fn new_nand(inputs: usize) -> Self {
        Self { inputs: vec![Signal::Low; inputs], outputs: vec![Signal::Low; 1], operations: vec![Operation::Nand((0..inputs).collect()); 1] }
    }

    pub fn evaluate_outputs(&mut self, changed_input: usize) -> Vec<usize> {
        let mut events = vec![];

        for (index, operation) in self.operations.iter().enumerate() {
            if operation.get_inputs().contains(&changed_input) {
                let operation_inputs = self.inputs.iter()
                    .enumerate()
                    .filter(|(i, _)| operation.get_inputs().contains(i))
                    .map(|(_, value)| *value).collect();
                let new_value = operation.execute(operation_inputs);

                if self.outputs[index] != new_value {
                    self.outputs[index] = new_value;
                    events.push(index);
                }
            }
        }
        
        events
    }

    pub fn set_input(&mut self, input: usize, signal: Signal) {
        self.inputs[input] = signal;
    }

    pub fn get_output(&mut self, output: usize) -> Signal {
        self.outputs[output]
    }
}

pub struct Input {
    value: Signal,
}

impl Input {
    pub fn new() -> Self {
        Self { value: Signal::Low }
    }

    pub(crate) fn set_value(&mut self, value: Signal) {
        self.value = value;
    }

    pub fn get_value(&self) -> Signal {
        self.value
    }
}