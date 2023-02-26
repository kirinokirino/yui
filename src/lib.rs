#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
use glam::Vec2;
use log::warn;

#[derive(Clone, Debug, PartialEq)]
struct Rect {
    position: Vec2,
    width: f64,
    height: f64,

    aspect_ratio: f64,
}

impl Rect {
    pub fn new(width: f64, height: f64) -> Self {
        assert!(width >= 0.0);
        assert!(height >= 0.0);
        Self {
            position: Vec2::new(0.0, 0.0),
            width,
            height,
            aspect_ratio: Self::aspect_ratio(width, height),
        }
    }

    pub const fn with_position(self, position: Vec2) -> Self {
        Self { position, ..self }
    }

    fn aspect_ratio(width: f64, height: f64) -> f64 {
        width / height
    }

    fn update_aspect_ratio(&mut self) {
        self.aspect_ratio = self.width / self.height;
    }

    pub fn cut_top(&mut self, amount: f64) -> Self {
        assert!(amount >= 0.0);
        assert!(amount <= self.height);
        self.height -= amount;
        let rect = Self::new(self.width, amount).with_position(self.position);
        self.position.y += amount as f32;
        self.update_aspect_ratio();
        rect
    }

    pub fn cut_bottom(&mut self, amount: f64) -> Self {
        assert!(amount >= 0.0);
        assert!(amount <= self.height);
        self.height -= amount;

        let mut position = self.position;
        position.y += self.height as f32;
        self.update_aspect_ratio();
        Self::new(self.width, amount).with_position(position)
    }

    pub fn cut_left(&mut self, amount: f64) -> Self {
        assert!(amount >= 0.0);
        assert!(amount <= self.width);
        self.width -= amount;
        let rect = Self::new(amount, self.height).with_position(self.position);
        self.position.x += amount as f32;
        self.update_aspect_ratio();
        rect
    }

    pub fn cut_right(&mut self, amount: f64) -> Self {
        assert!(amount >= 0.0);
        assert!(amount <= self.height);
        self.width -= amount;

        let mut position = self.position;
        position.x += self.width as f32;
        self.update_aspect_ratio();
        Self::new(amount, self.height).with_position(position)
    }

    pub fn divide_horizontally(mut self, into_parts: usize) -> Vec<Self> {
        assert!(into_parts >= 1);
        if into_parts == 1 {
            warn!("Dividing Rect into 1 horizontal part.");
        }
        let mut result = Vec::with_capacity(into_parts);
        let division_width = self.width / into_parts as f64;
        for _ in 0..into_parts - 1 {
            result.push(self.cut_left(division_width));
        }
        result.push(self);
        result
    }

    pub fn divide_vertically(mut self, into_parts: usize) -> Vec<Self> {
        assert!(into_parts >= 1);
        if into_parts == 1 {
            warn!("Dividing Rect into 1 vertical part.");
        }
        let mut result = Vec::with_capacity(into_parts);
        let division_height = self.height / into_parts as f64;
        for _ in 0..into_parts - 1 {
            result.push(self.cut_top(division_height));
        }
        result.push(self);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aspect_ratio() {
        let rect = Rect::new(640.0, 480.0);
        let Rect { aspect_ratio, .. } = rect;
        assert_eq!(aspect_ratio, (4.0 / 3.0) as f64);
    }

    #[test]
    fn cut_top() {
        let mut rect = Rect::new(640.0, 480.0);
        let top = rect.cut_top(10.0);
        let expected = Rect::new(640.0, 10.0).with_position(Vec2::new(0.0, 0.0));
        assert_eq!(top, expected);
        assert_eq!(rect.height, 470.0);
        assert_eq!(rect.position.y, 10.0);
        assert_eq!(rect.width, 640.0);
        assert_eq!(rect.position.x, 0.0);
    }

    #[test]
    fn cut_bottom() {
        let mut rect = Rect::new(640.0, 480.0);
        let bottom = rect.cut_bottom(10.0);
        let expected = Rect::new(640.0, 10.0).with_position(Vec2::new(0.0, 470.0));
        assert_eq!(bottom, expected);
        assert_eq!(rect.height, 470.0);
        assert_eq!(rect.position.y, 0.0);
        assert_eq!(rect.width, 640.0);
        assert_eq!(rect.position.x, 0.0);
    }

    #[test]
    fn cut_left() {
        let mut rect = Rect::new(640.0, 480.0);
        let left = rect.cut_left(10.0);
        let expected = Rect::new(10.0, 480.0).with_position(Vec2::new(0.0, 0.0));
        assert_eq!(left, expected);
        assert_eq!(rect.width, 630.0);
        assert_eq!(rect.position.x, 10.0);
        assert_eq!(rect.height, 480.0);
        assert_eq!(rect.position.y, 0.0);
    }

    #[test]
    fn cut_right() {
        let mut rect = Rect::new(640.0, 480.0);
        let right = rect.cut_right(10.0);
        let expected = Rect::new(10.0, 480.0).with_position(Vec2::new(630.0, 0.0));
        assert_eq!(right, expected);
        assert_eq!(rect.width, 630.0);
        assert_eq!(rect.position.x, 0.0);
        assert_eq!(rect.height, 480.0);
        assert_eq!(rect.position.y, 0.0);
    }

    #[test]
    fn divide_horizontally() {
        let rect = Rect::new(640.0, 480.0);
        let mut division = rect.divide_horizontally(2);
        let right_half = division.pop().unwrap();
        let left_half = division.pop().unwrap();
        let expected_left = Rect::new(320.0, 480.0).with_position(Vec2::new(0.0, 0.0));
        let expected_right = Rect::new(320.0, 480.0).with_position(Vec2::new(320.0, 0.0));
        assert_eq!(right_half, expected_right);
        assert_eq!(left_half, expected_left);
        assert!(division.is_empty());
    }

    #[test]
    fn divide_vertically() {
        let rect = Rect::new(640.0, 480.0);
        let mut division = rect.divide_vertically(3);
        let bottom_third = division.pop().unwrap();
        let middle_third = division.pop().unwrap();
        let top_third = division.pop().unwrap();
        let expected_bottom = Rect::new(640.0, 160.0).with_position(Vec2::new(0.0, 320.0));
        let expected_middle = Rect::new(640.0, 160.0).with_position(Vec2::new(0.0, 160.0));
        let expected_top = Rect::new(640.0, 160.0).with_position(Vec2::new(0.0, 0.0));
        assert_eq!(bottom_third, expected_bottom);
        assert_eq!(middle_third, expected_middle);
        assert_eq!(top_third, expected_top);
        assert!(division.is_empty());
    }
}
