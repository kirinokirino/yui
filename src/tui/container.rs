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

    content: String,
    lines_scrolled: usize,
}

impl Container {
    pub fn new(rect: Rect) -> Self {
        Self {
            domain: rect,
            margin: Margin::default(),
            padding: Padding::default(),
            border: TuiBorder::default(),

            content: String::new(),
            lines_scrolled: 0,
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

    fn what_side_is(&self, x: usize, y: usize, offset_x: usize, offset_y: usize) -> Side {
        let (pos, width, height) = self.domain.pos_width_height();
        let (width, height) = (
            width as i32 - offset_x as i32,
            height as i32 - offset_y as i32,
        );
        let (x, y) = (x as i32 - pos.x as i32, y as i32 - pos.y as i32);
        let (middle_x, middle_y) = (width as i32 / 2, height as i32 / 2);
        if x < middle_x {
            if y < middle_y {
                if x == y {
                    return Side::TopLeftCorner;
                } else if x > y {
                    return Side::Top;
                } else {
                    return Side::Left;
                }
            } else {
                if height - x == y {
                    return Side::BottomLeftCorner;
                } else if height - x > y {
                    return Side::Left;
                } else {
                    return Side::Bottom;
                }
            }
        } else {
            if y < middle_y {
                if width - y == x {
                    return Side::TopRightCorner;
                } else if width - y > x {
                    return Side::Top;
                } else {
                    return Side::Right;
                }
            } else {
                if height + x - width == y {
                    return Side::BottomRightCorner;
                } else if height + x - width > y {
                    return Side::Right;
                } else {
                    return Side::Bottom;
                }
            }
        }
    }

    fn draw_margin(&self, x: usize, y: usize) -> char {
        ' '
    }
    fn draw_padding(&self, x: usize, y: usize) -> char {
        ' '
    }
    fn draw_border(&self, x: usize, y: usize) -> char {
        let side = self.what_side_is(
            x - self.margin.left as usize,
            y - self.margin.top as usize,
            self.margin.right as usize + self.margin.left as usize,
            self.margin.bottom as usize + self.margin.top as usize,
        );
        match self.border {
            TuiBorder::None => ' ',
            TuiBorder::SmoothCorner => match side {
                Side::TopLeftCorner => '╭',
                Side::Top | Side::Bottom => '─',
                Side::TopRightCorner => '╮',
                Side::Right | Side::Left => '│',
                Side::BottomRightCorner => '╯',
                Side::BottomLeftCorner => '╰',
            },
        }
    }

    pub fn set_content(&mut self, mut content: String) {
        let (_pos, width, height) = self.domain.pos_width_height();
        let width = (width
            - (self.margin.left
                + self.margin.right
                + (self.border.size() as f64) * 2.0
                + self.padding.left
                + self.padding.right)) as usize;
        let height = (height
            - (self.margin.top
                + self.margin.bottom
                + (self.border.size() as f64) * 2.0
                + self.padding.top
                + self.padding.bottom)) as usize;

        textwrap::fill_inplace(&mut content, width);
        self.content = content;
    }

    fn draw_contents(&self, x: usize, y: usize) -> char {
        let (pos, ..) = self.domain.pos_width_height();
        let x = x
            - (self.margin.left + self.border.size() as f64 + self.padding.left + pos.x as f64)
                as usize;
        let y = y
            - (self.margin.top + self.border.size() as f64 + self.padding.top + pos.y as f64)
                as usize
            + self.lines_scrolled;
        if let Some(line) = self.content.lines().nth(y) {
            return line.chars().nth(x).unwrap_or(' ');
        }
        ' '
    }

    pub fn contents_of(&self, x: usize, y: usize) -> Option<char> {
        let (pos, width, height) = self.domain.pos_width_height();
        let (offset_x, offset_y, width, height) = (
            pos.x as usize,
            pos.y as usize,
            width as usize,
            height as usize,
        );
        let (max_x, max_y) = (offset_x + width, offset_y + height);

        if x <= offset_x || x >= max_x || y <= offset_y || y >= max_y {
            return None;
        }
        let (offset_margin_left, offset_margin_top, offset_margin_right, offset_margin_bottom) = (
            offset_x + self.margin.left as usize,
            offset_y + self.margin.top as usize,
            max_x - self.margin.right as usize,
            max_y - self.margin.bottom as usize,
        );
        if x <= offset_margin_left
            || x >= offset_margin_right
            || y <= offset_margin_top
            || y >= offset_margin_bottom
        {
            return Some(self.draw_margin(x, y));
        }
        let border_size = self.border.size();
        let (offset_border_left, offset_border_top, offset_border_right, offset_border_bottom) = (
            offset_margin_left + border_size,
            offset_margin_top + border_size,
            offset_margin_right - border_size,
            offset_margin_bottom - border_size,
        );
        if x <= offset_border_left
            || x >= offset_border_right
            || y <= offset_border_top
            || y >= offset_border_bottom
        {
            return Some(self.draw_border(x, y));
        };
        let (offset_padding_left, offset_padding_top, offset_padding_right, offset_padding_bottom) = (
            offset_border_left + self.padding.left as usize,
            offset_border_top + self.padding.top as usize,
            offset_border_right - self.padding.right as usize,
            offset_border_bottom - self.padding.bottom as usize,
        );
        if x <= offset_padding_left
            || x >= offset_padding_right
            || y <= offset_padding_top
            || y >= offset_padding_bottom
        {
            return Some(self.draw_padding(x, y));
        }
        return Some(self.draw_contents(x, y));
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
                        buffer.push(self.contents_of(x, y).unwrap_or(' '));
                    }
                }
            }
            buffer.push('\n');
        }
        f.write_str(&buffer)
    }
}

#[derive(Debug, Clone, Copy)]
enum Side {
    TopLeftCorner,
    Top,
    TopRightCorner,
    Right,
    BottomRightCorner,
    Bottom,
    BottomLeftCorner,
    Left,
}

#[derive(Debug, Clone, Copy)]
pub enum TuiBorder {
    None,
    SmoothCorner,
}

impl TuiBorder {
    pub const fn size(&self) -> usize {
        match self {
            TuiBorder::None => 0,
            TuiBorder::SmoothCorner => 1,
        }
    }
}

impl Default for TuiBorder {
    fn default() -> Self {
        Self::None
    }
}
