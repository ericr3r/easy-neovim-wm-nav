mod aerospace;
mod direction;
mod hypr;
mod nvim;
mod server;

use crate::aerospace::Aerospace;
use crate::server::Server;
use clap::Parser;
use direction::{Backend, Cli, Direction};
use hypr::Hypr;
use nvim::Nvim;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let direction = args.direction;
    let server: Box<dyn Server> = match args.backend {
        Backend::Aerospace => Box::new(Aerospace::new()),
        Backend::Hyprland => Box::new(Hypr::new()),
    };

    let window_title = server.get_window_title()?;

    navigate(server, window_title, direction)
}

fn navigate(
    server: Box<dyn Server>,
    window_title: String,
    direction: Direction,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(nvim_server) = Nvim::new(&window_title) {
        match nvim_server.navigate(direction) {
            Err(_) => return server.navigate(direction),
            _ => return Ok(()),
        }
    } else {
        return server.navigate(direction);
    }
}
