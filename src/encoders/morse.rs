use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

use super::Encoder;

static CODEC: [(char, &str); 37] = [
    ('A', ".-"),
    ('B', "-..."),
    ('C', "-.-."),
    ('D', "-.."),
    ('E', "."),
    ('F', "..-."),
    ('G', "--."),
    ('H', "...."),
    ('I', ".."),
    ('J', ".---"),
    ('K', "-.-"),
    ('L', ".-.."),
    ('M', "--"),
    ('N', "-."),
    ('O', "---"),
    ('P', ".--."),
    ('Q', "--.-"),
    ('R', ".-."),
    ('S', "..."),
    ('T', "-"),
    ('U', "..-"),
    ('V', "...-"),
    ('W', ".--"),
    ('X', "-..-"),
    ('Y', "-.--"),
    ('Z', "--.."),
    ('0', "-----"),
    ('1', ".----"),
    ('2', "..---"),
    ('3', "...--"),
    ('4', "....-"),
    ('5', "....."),
    ('6', "-...."),
    ('7', "--..."),
    ('8', "---.."),
    ('9', "----."),
    ('?', "........"),
];
pub struct Morse {}

impl Morse {
    pub fn encode(input: &str) -> String {
        let mut map = HashMap::new();
        for (plain, morse) in CODEC {
            map.insert(plain, morse);
        }
        lazy_static! {
            static ref PUNC: Regex = Regex::new(r#"[ ,.!,;:-]+"#).unwrap();
        }
        PUNC.split(&input.to_uppercase())
            .filter(|w| w.len() > 0)
            .map(|word| {
                word.chars()
                    .map(|c| map.get(&c).map(|s| *s).unwrap_or("........"))
                    .collect::<Vec<&str>>()
                    .join("/")
            })
            .collect::<Vec<String>>()
            .join("//")
            .trim()
            .to_string()
    }
    pub fn decode(input: &str) -> String {
        lazy_static! {
            static ref DELIMITERS: Regex = Regex::new(r#"[^\.-]"#).unwrap();
        }
        let mut map = HashMap::new();
        for (plain, morse) in CODEC {
            map.insert(morse, plain);
        }
        DELIMITERS
            .split(input)
            .map(|letter| match map.get(letter) {
                Some(resolved) => resolved.to_string(),
                _ => " ".to_string(),
            })
            .collect::<Vec<String>>()
            .join("")
    }
}

impl Encoder for Morse {
    fn name(&self) -> &'static str {
        "Morse"
    }
    fn description(&self) -> &'static str {
        "Standard morse-code encoding with dots and dashes"
    }
    fn decode(&self, input: &str) -> String {
        Morse::decode(input)
    }
    fn encode(&self, input: &str) -> String {
        Morse::encode(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(
            Morse::encode("hello world"),
            "...././.-../.-../---//.--/---/.-./.-../-.."
        );
        assert_eq!(Morse::encode("  "), "");
        assert_eq!(Morse::encode("SOS"), ".../---/...");
        assert_eq!(Morse::encode("SOS"), Morse::encode("sos"));
        assert_eq!(Morse::encode("#metoo"), Morse::encode("*metoo"));
        assert_eq!(Morse::encode("#"), "........");
    }

    #[test]
    fn test_decode() {
        assert_eq!(
            Morse::decode("...././.-../.-../---//.--/---/.-./.-../-.."),
            "HELLO WORLD"
        );
    }
    #[test]
    fn test_encode_decode() {
        assert_eq!(
            Morse::decode(&Morse::encode("hello WORLD 42!")),
            "HELLO WORLD 42"
        );
    }

    #[test]
    fn test_encode_decode_punc() {
        assert_eq!(
            Morse::decode(&Morse::encode("Hello Mr. Cat, please have a seat!")),
            "HELLO MR CAT PLEASE HAVE A SEAT"
        );
    }
    // ┌───┐┌┐   ┌┐┌┐┌───┐
}
