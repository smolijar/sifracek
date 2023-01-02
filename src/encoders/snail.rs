use std::collections::HashMap;

use super::{Encoder};

pub struct Snail {}

#[derive(Debug)]
enum Edge {
    Upper,
    Left,
    Lower,
    Right,
}

impl Snail {
    pub fn encode(input: &str) -> String {
        let square_size = (input.len() as f32).sqrt().ceil() as usize;
        let mut input_cursor = square_size * square_size;
        let mut current_edge_size = square_size;
        let mut edge_cursor = square_size;
        let mut current_row = 0;
        let mut current_col = current_edge_size;
        let mut result_map = HashMap::new();
        let mut current_edge = if square_size % 2 == 0 {
            current_row = current_edge_size;
            current_col = 0;
            Edge::Lower
        } else {
            Edge::Upper
        };
        loop {
            if edge_cursor == 0 {
                match current_edge {
                    Edge::Upper => {
                        current_edge = Edge::Left;
                        current_edge_size -= 1
                    }
                    Edge::Left => {
                        current_edge = Edge::Lower;
                    }
                    Edge::Lower => {
                        current_edge = Edge::Right;
                        current_edge_size -= 1
                    }
                    Edge::Right => {
                        current_edge = Edge::Upper;
                    }
                }
                edge_cursor = current_edge_size
            }
            match input.chars().nth(input_cursor) {
                Some(x) => {
                    result_map.insert((current_row, current_col), x);
                }
                None => {}
            };
            if input_cursor == 0 {
                break;
            }
            input_cursor -= 1;
            edge_cursor -= 1;
            match current_edge {
                Edge::Upper => current_col -= 1,
                Edge::Left => current_row += 1,
                Edge::Lower => current_col += 1,
                Edge::Right => current_row -= 1,
            }
        }
        let mut result_string = String::new();
        for r in 0..=square_size {
            let mut line = String::new();
            let mut line_not_empty = false;
            for c in 0..=square_size {
                match result_map.get(&(r, c)) {
                    Some(x) => {
                        line.push_str(&format!("{} ", x));
                        line_not_empty = true;
                    }
                    None => {
                        line.push_str(&format!("{} ", ' '));
                    }
                }
            }
            if line_not_empty {
                result_string.push_str(&line.trim_end());
                result_string.push('\n');
            }
        }
        return result_string.trim_end().to_string();
    }
    pub fn decode(_input: &str) -> String {
        todo!();
    }
}

impl Encoder for Snail {
    fn name(&self) -> &'static str {
        "Snail"
    }
    fn description(&self) -> &'static str {
        "Curl input in spiral from center outward to a square plane"
    }
    fn decode(&self, input: &str) -> String {
        Snail::decode(input)
    }
    fn encode(&self, input: &str) -> String {
        Snail::encode(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        // Snail::encode("123456789");
        assert_eq!(
            Snail::encode("JÁJSEMMALÝŠNEČEK"),
            "  M A L Ý
  M J Á Š
  E S J N
  K E Č E"
        );
        assert_eq!(
            Snail::encode("SNAIL"),
            "  S N
L I A"
        );
        assert_eq!(
            Snail::encode("car"),
            "  c a
    r"
        );
    }
}
