use std::ops::{BitAnd, BitOr, BitAndAssign, BitOrAssign, Not, BitXor, BitXorAssign};

use crate::components::Observer;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    High,
    Low,
    #[default]
    Undefined,
}

impl Not for Signal {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Signal::High => Signal::Low,
            Signal::Low => Signal::High,
            Signal::Undefined => Signal::Undefined,
        }
    }
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

impl BitAndAssign for Signal {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
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

impl BitOrAssign for Signal {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for Signal {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Signal::High, Signal::Low) => Signal::High,
            (Signal::Low, Signal::High) => Signal::High,
            (Signal::Low, Signal::Low) => Signal::Low,
            (Signal::High, Signal::High) => Signal::Low,
            (_, _) => Signal::Undefined,
        }
    }
}

impl BitXorAssign for Signal {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

pub struct ObservableSignal {
    signal: Signal,
    observers: Vec<Box<dyn Observer>>,
}

impl ObservableSignal {
    pub fn set_signal(&mut self, value: Signal) {
        self.signal = value;
        self.notify_has_changed();
    }

    pub fn get_signal(&self) -> Signal {
        self.signal
    }

    fn notify_has_changed(&mut self) {
        for observer in &mut self.observers {
            observer.as_mut().has_changed();
        }
    }
}