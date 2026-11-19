use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FadeType { #[default] In, Out }

#[derive(Debug, Clone, PartialEq)]
pub struct Fade {
    pub kind: FadeType,
    pub duration: Duration,
}
