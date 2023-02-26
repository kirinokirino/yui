use std::fmt::Debug;

use crate::{
    common::{Margin, Padding},
    rect::Rect,
};

#[derive(Clone)]
pub struct Container {
    domain: Rect,
    margin: Margin,
    padding: Padding,
    border: TuiBorder,
}

impl Container {
    pub fn new(rect: Rect) -> Self {
        Self {
            domain: rect,
            margin: Margin::default(),
            padding: Padding::default(),
            border: TuiBorder::default(),
        }
    }

    pub fn with_margin(mut self, margin: Margin) -> Self {
        Self { margin, ..self }
    }

    pub fn with_padding(mut self, padding: Padding) -> Self {
        Self { padding, ..self }
    }

    pub fn with_border(mut self, border: TuiBorder) -> Self {
        Self { border, ..self }
    }

    fn what_is_on_this_point(&self, x: usize, y: usize) -> char {
        let (pos, width, height) = self.domain.pos_width_height();
        let (offset_x, offset_y, width, height) = (
            pos.x as usize,
            pos.y as usize,
            width as usize,
            height as usize,
        );
        let (max_x, max_y) = (offset_x + width, offset_y + height);

        if x < offset_x || x >= max_x || y < offset_y || y >= max_y {
            return ' ';
        }
        let (offset_margin_left, offset_margin_top, offset_margin_right, offset_margin_bottom) = (
            offset_x + self.margin.left as usize,
            offset_y + self.margin.top as usize,
            max_x - self.margin.right as usize,
            max_y - self.margin.bottom as usize,
        );
        if x < offset_margin_left
            || x >= offset_margin_right
            || y < offset_margin_top
            || y >= offset_margin_bottom
        {
            return 'M';
        }
        let border_size = match self.border {
            TuiBorder::None => 0,
        };
        let (offset_border_left, offset_border_top, offset_border_right, offset_border_bottom) = (
            offset_margin_left + border_size,
            offset_margin_top + border_size,
            offset_margin_right - border_size,
            offset_margin_bottom - border_size,
        );
        if x < offset_border_left
            || x >= offset_border_right
            || y < offset_border_top
            || y >= offset_border_bottom
        {
            return 'B';
        };
        let (offset_padding_left, offset_padding_top, offset_padding_right, offset_padding_bottom) = (
            offset_border_left + self.padding.left as usize,
            offset_border_top + self.padding.top as usize,
            offset_border_right - self.padding.right as usize,
            offset_border_bottom - self.padding.bottom as usize,
        );
        if x < offset_padding_left
            || x >= offset_padding_right
            || y < offset_padding_top
            || y >= offset_padding_bottom
        {
            return 'P';
        }
        return 'C';
    }
}

impl Debug for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (pos, width, height) = self.domain.pos_width_height();

        let capacity =
            (f64::from(pos.x) + width + 1.0) as usize * (f64::from(pos.y) + height) as usize;
        let mut buffer = String::with_capacity(capacity);
        for y in 0..(f64::from(pos.y) + height) as usize {
            if y < pos.y as usize {
                buffer.push(' ');
            } else {
                for x in 0..(f64::from(pos.x) + width) as usize {
                    if x < pos.x as usize {
                        buffer.push(' ');
                    } else {
                        buffer.push(self.what_is_on_this_point(x, y));
                    }
                }
            }
            buffer.push('\n');
        }
        f.write_str(&buffer)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TuiBorder {
    None,
}

impl Default for TuiBorder {
    fn default() -> Self {
        Self::None
    }
}
