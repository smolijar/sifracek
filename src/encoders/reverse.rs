use super::Encoder;


pub struct Reverse {}

impl Reverse {
    pub fn encode(input: &str) -> String {
        input.chars().rev().collect()
    }
    pub fn decode(input: &str) -> String {
        Reverse::encode(input)
    }
}

impl Encoder for Reverse {
    fn name(&self) -> &'static str {
        "Reverse"
    }
    fn description(&self) -> &'static str {
        "Reverses a string"
    }
    fn decode(&self, input: &str) -> String {
        Reverse::decode(input)
    }
    fn encode(&self, input: &str) -> String {
        Reverse::encode(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(
            Reverse::encode("hello world"),
            "dlrow olleh"
        );
    }

    #[test]
    fn test_decode() {
        assert_eq!(
            Reverse::decode("dlrow olleh"),
            "hello world"
        );
    }
    #[test]
    fn test_encode_decode() {
        assert_eq!(
            Reverse::decode(&Reverse::encode("hello world")),
            "hello world"
        );
    }
}
