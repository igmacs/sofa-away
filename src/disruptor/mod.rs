pub trait Disruptor {
    fn disrupt(&mut self);
}

// pub mod dummy;
// pub use dummy::DummyDisruptor;

pub mod quasimodo;
pub use quasimodo::QuasimodoDisruptor;
