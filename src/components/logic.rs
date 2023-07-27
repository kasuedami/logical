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
}

impl Function {
    pub fn calculate(&self, inputs: Vec<Signal>) -> Signal {
        todo!()
    }
}