use glam::Vec2;

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

    pub fn with_position(self, position: Vec2) -> Self {
        Self { position, ..self }
    }

    fn aspect_ratio(width: f64, height: f64) -> f64 {
        width as f64 / height as f64
    }

    pub fn cut_top(&mut self, amount: f64) -> Self {
        assert!(amount >= 0.0);
        assert!(amount <= self.height);
        self.height -= amount;
        let rect = Self::new(self.width, amount).with_position(self.position);
        self.position.y += amount as f32;
        rect
    }

    pub fn cut_bottom(&mut self, amount: f64) -> Self {
        assert!(amount >= 0.0);
        assert!(amount <= self.height);
        self.height -= amount;

        let mut position = self.position;
        position.y += self.height as f32;
        Self::new(self.width, amount).with_position(position)
    }

    pub fn cut_left(&mut self, amount: f64) -> Self {
        assert!(amount >= 0.0);
        assert!(amount <= self.width);
        self.width -= amount;
        let rect = Self::new(amount, self.height).with_position(self.position);
        self.position.x += amount as f32;
        rect
    }

    pub fn cut_right(&mut self, amount: f64) -> Self {
        assert!(amount >= 0.0);
        assert!(amount <= self.height);
        self.width -= amount;

        let mut position = self.position;
        position.x += self.width as f32;
        Self::new(amount, self.height).with_position(position)
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
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
}
