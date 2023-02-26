#[derive(Clone, Debug)]
pub struct Margin {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl Margin {
    pub fn same(value: f64) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }
    pub fn sides(left: f64, right: f64) -> Self {
        Self {
            top: 0.0,
            right,
            bottom: 0.0,
            left,
        }
    }
    pub fn vertical(top: f64, bottom: f64) -> Self {
        Self {
            top,
            right: 0.0,
            bottom,
            left: 0.0,
        }
    }
    pub fn top(top: f64) -> Self {
        Self {
            top,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        }
    }
    pub fn right(right: f64) -> Self {
        Self {
            top: 0.0,
            right,
            bottom: 0.0,
            left: 0.0,
        }
    }
    pub fn bottom(bottom: f64) -> Self {
        Self {
            top: 0.0,
            right: 0.0,
            bottom,
            left: 0.0,
        }
    }
    pub fn left(left: f64) -> Self {
        Self {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left,
        }
    }
}

impl Default for Margin {
    fn default() -> Self {
        Self::same(0.0)
    }
}

#[derive(Debug, Clone)]
pub struct Padding {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}
impl Padding {
    pub fn same(value: f64) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }
    pub fn sides(left: f64, right: f64) -> Self {
        Self {
            top: 0.0,
            right,
            bottom: 0.0,
            left,
        }
    }
    pub fn vertical(top: f64, bottom: f64) -> Self {
        Self {
            top,
            right: 0.0,
            bottom,
            left: 0.0,
        }
    }
    pub fn top(top: f64) -> Self {
        Self {
            top,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        }
    }
    pub fn right(right: f64) -> Self {
        Self {
            top: 0.0,
            right,
            bottom: 0.0,
            left: 0.0,
        }
    }
    pub fn bottom(bottom: f64) -> Self {
        Self {
            top: 0.0,
            right: 0.0,
            bottom,
            left: 0.0,
        }
    }
    pub fn left(left: f64) -> Self {
        Self {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left,
        }
    }
}

impl Default for Padding {
    fn default() -> Self {
        Self::same(0.0)
    }
}
