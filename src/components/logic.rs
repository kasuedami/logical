use crate::signal::{ObservableSignal, Signal};

use super::Node;

struct Base {
    inputs: Vec<ObservableSignal>,
    value: Signal,
    output: ObservableSignal,
}

struct Logic {
    base: Base,
    fucntion: Function,
}

impl Node for Logic {
    fn read_inputs(&mut self) {
        self.base.value = self.fucntion.calculate(self.base.inputs.iter().map(|v| v.get_signal()).collect())
    }

    fn write_inputs(&mut self) {
        self.base.output.set_signal(self.base.value);
    }
}

enum Function {
    And,
    Or,
    XOr,
    NAnd,
    NOr,
    XNOr,
}

impl Function {
    pub fn calculate(&self, inputs: Vec<Signal>) -> Signal {
        match self {
            Function::And => and(inputs),
            Function::Or => or(inputs),
            Function::XOr => xor(inputs),
            Function::NAnd => !and(inputs),
            Function::NOr => !or(inputs),
            Function::XNOr => !xor(inputs),
        }
    }
}

fn and(inputs: Vec<Signal>) -> Signal {
    let mut r = Signal::High;

    for input in inputs {
        r &= input;
    }

    r
}

fn or(inputs: Vec<Signal>) -> Signal {
    let mut r = Signal::Low;

    for input in inputs {
        r |= input;
    }

    r
}

fn xor(inputs: Vec<Signal>) -> Signal {
    let mut r = Signal::Low;

    for input in inputs {
        r ^= input;
    }

    r
}