pub mod logic;

pub trait Node {
    fn read_inputs(&mut self);
    fn write_inputs(&mut self);
}

pub trait Observer {
    fn has_changed(&mut self);
}