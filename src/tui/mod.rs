use crate::{
    common::{Margin, Padding},
    rect::Rect,
};

mod container;
use container::Container;
pub struct Terminal {
    free_space: Rect,
    pub stuff: Vec<Container>,
}

impl Terminal {
    pub fn new() -> Self {
        let (w, h) = term_size::dimensions().unwrap_or((80, 5));
        dbg!(&w, &h);
        let mut rect = Rect::new(w as f64, h as f64);
        let container = Container::new(rect.cut_right(w as f64 - 4.0))
            .with_padding(Padding::same(2.0))
            .with_margin(Margin::same(1.0));
        Self {
            free_space: rect,
            stuff: vec![container],
        }
    }
}
