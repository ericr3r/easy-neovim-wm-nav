mod aerospace;
mod direction;
mod hypr;
mod nvim;
mod server;
mod sway;

use crate::aerospace::Aerospace;
use crate::server::Server;
use clap::Parser;
use direction::{Backend, Cli, Direction};
use hypr::Hypr;
use nvim::Nvim;
use std::env;
use std::error::Error;
use sway::Sway;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let direction = args.direction;
    let server: Box<dyn Server> = match args.backend {
        Backend::Aerospace => Box::new(Aerospace::new()),
        Backend::Auto => detect_backend()?,
        Backend::Hyprland => Box::new(Hypr::new()),
        Backend::Sway => Box::new(Sway::new()?),
    };

    let window_title = server.get_window_title()?;

    navigate(server, window_title, direction)
}

fn detect_backend<'a>() -> Result<Box<dyn Server<'a>>, Box<dyn Error>> {
    if let Ok(_signature) = env::var("HYPRLAND_INSTANCE_SIGNATURE") {
        Ok(Box::new(Hypr::new()))
    } else if let Ok(_socket) = env::var("SWAYSOCK") {
        Ok(Box::new(Sway::new()?))
    } else {
        Err("Unable to detect backend".into())
    }
}

fn navigate(
    server: Box<dyn Server>,
    window_title: String,
    direction: Direction,
) -> Result<(), Box<dyn Error>> {
    if let Some(nvim_server) = Nvim::new(&window_title) {
        match nvim_server.navigate(direction) {
            Err(_) => return server.navigate(direction),
            _ => return Ok(()),
        }
    } else {
        return server.navigate(direction);
    }
}
