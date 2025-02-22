use clap::{Parser, ValueEnum};
use std::fmt;

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(value_enum)]
    pub direction: Direction,
    #[clap(short, long, value_enum, default_value_t = Backend::Hyprland)]
    pub backend: Backend,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum Backend {
    Aerospace,
    Hyprland,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
