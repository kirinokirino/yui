#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]

mod common;
mod rect;

#[cfg(feature = "tui")]
pub mod tui;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(2 == 2);
    }
}
