use crate::server::{Server, ServerError};
use hyprland::data::Client;
use hyprland::dispatch::{Direction, Dispatch, DispatchType};
use hyprland::prelude::*;

#[derive(Debug)]
pub struct Hypr {}

impl Hypr {
    pub fn new() -> Hypr {
        Hypr {}
    }
}

impl<'a> Server<'a> for Hypr {
    fn navigate(
        &self,
        direction: crate::direction::Direction,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(Dispatch::call(DispatchType::MoveFocus(
            to_hyprland_direction(direction),
        ))?)
    }

    fn get_window_title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let client = Client::get_active()?;
        match client {
            Some(client) => Ok(client.title),
            None => Err(Box::new(ServerError)),
        }
    }
}

fn to_hyprland_direction(direction: crate::direction::Direction) -> hyprland::dispatch::Direction {
    match direction {
        crate::direction::Direction::Left => Direction::Left,
        crate::direction::Direction::Right => Direction::Right,
        crate::direction::Direction::Down => Direction::Down,
        crate::direction::Direction::Up => Direction::Up,
    }
}
