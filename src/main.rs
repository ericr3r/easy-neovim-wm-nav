mod aerospace;
mod direction;
mod server;

use crate::aerospace::Aerospace;
use crate::server::Server;
use clap::Parser;
use direction::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let direction = args.direction;

    let server: Box<dyn Server> = Box::new(Aerospace::new());
    let _title = server.get_window_title();

    return server.navigate(direction);
}
