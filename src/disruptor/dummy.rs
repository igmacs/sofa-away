use super::Disruptor;

pub struct DummyDisruptor;

impl DummyDisruptor {
    pub fn new() -> Self {
        Self
    }
}

impl Disruptor for DummyDisruptor {
    fn disrupt(&mut self) {
        println!("Very annoying disruption")
    }
}
