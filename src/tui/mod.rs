use crate::{
    common::{Margin, Padding},
    rect::Rect,
};

mod container;
use container::Container;

use self::container::TuiBorder;
#[derive(Debug)]
pub struct Terminal {
    width: usize,
    height: usize,
    free_space: Option<Rect>,
    pub containers: Vec<Container>,
}

impl Terminal {
    pub fn new() -> Self {
        let (width, height) = term_size::dimensions().unwrap_or((80, 5));
        let mut rect = Rect::new(width as f64, height as f64);
        let container1 = Container::new(rect.cut_right((width / 3) as f64))
            .with_padding(Padding::same(2.0))
            .with_margin(Margin::top(3.0))
            .with_border(TuiBorder::SmoothCorner);
        let container2 = Container::new(rect.cut_left((width / 3) as f64))
            .with_padding(Padding::same(2.0))
            .with_margin(Margin::same(1.0));
        let rects = rect.divide_vertically(3);
        let mut containers: Vec<Container> = rects
            .into_iter()
            .map(|rect| {
                Container::new(rect)
                    .with_margin(Margin::top(1.0))
                    .with_border(TuiBorder::SmoothCorner)
            })
            .collect();

        let mut containers: Vec<Container> = vec![container1, container2]
            .into_iter()
            .chain(containers.into_iter())
            .collect();

        for container in &mut containers {
            let words = [
                "A sentence number 1.".to_string(),
                "Another example sentence.".to_string(),
                "Cat.".to_string(),
                "A sentence number 2.".to_string(),
                "Molto a qui pensare.".to_string(),
                "Parrot.".to_string(),
                "Some more content.".to_string(),
            ]
            .into_iter()
            .cycle();
            container.set_content(words.take(50).collect());
        }

        Self {
            width,
            height,
            free_space: None,
            containers,
        }
    }

    pub fn update(&mut self) {
        let (width, height) = term_size::dimensions().unwrap_or((80, 5));
        self.width = width;
        self.height = height;
    }

    pub fn display(&self) {
        let (width, height) = (self.width, self.height);
        let capacity = (width + 1) * height;
        let mut buffer = String::with_capacity(capacity);

        for y in 0..height {
            for x in 0..width {
                let contents = self
                    .containers
                    .iter()
                    .filter_map(|c| c.contents_of(x, y))
                    .next()
                    .unwrap_or(' ');

                buffer.push(contents);
            }
            buffer.push('\n');
        }
        println!("{CLEAR}");
        println!("{buffer}");
    }
}

const CLEAR: &str = "\x1B[2J\x1B[1;1H";