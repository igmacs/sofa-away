use super::Disruptor;

pub struct QuasimodoDisruptor;

impl QuasimodoDisruptor {
    pub fn new() -> Self {
        Self
    }
}

impl Disruptor for QuasimodoDisruptor {
    fn disrupt(&mut self) {
        let _ = open::that("https://static.wikia.nocookie.net/disney/images/7/7f/Quasimodo.jpeg/revision/latest?cb=20190301095734");
    }
}
