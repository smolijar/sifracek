pub mod morse;
pub use morse::Morse;
pub mod reverse;
pub use reverse::Reverse;
pub mod snail;
pub use snail::Snail;

pub trait Encoder {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn encode(&self, input: &str) -> String;
    fn decode(&self, input: &str) -> String;
}