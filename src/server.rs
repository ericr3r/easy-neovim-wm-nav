use crate::direction::Direction;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ServerError {
    details: String,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ServerError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub trait Server<'a> {
    fn navigate(&self, direction: Direction) -> Result<(), Box<dyn std::error::Error>>;

    fn get_window_title(&self) -> Result<String, Box<dyn Error>>;
}
