use crate::direction::Direction;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ServerError;

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A server error occured")
    }
}

impl Error for ServerError {}

pub trait Server<'a> {
    fn navigate(&self, direction: Direction) -> Result<(), Box<dyn std::error::Error>>;

    fn get_window_title(&self) -> Result<String, Box<dyn Error>>;
}
