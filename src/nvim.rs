use crate::direction::Direction;
use crate::server::Server;
use neovim_lib::{Neovim, NeovimApi, Session};
use std::error::Error;
use std::fmt;

use human_regex::{
    any, beginning, end, named_capture, non_whitespace, one_or_more, text, whitespace, zero_or_more,
};

#[derive(Debug)]
pub struct NvimError {
    details: String,
}

impl NvimError {
    pub fn new(msg: &str) -> NvimError {
        NvimError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for NvimError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for NvimError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Default)]
pub struct Nvim<'a> {
    server_name: &'a str,
}

impl<'a> Nvim<'a> {
    pub fn new(window_name: &str) -> Option<Nvim> {
        println!("{window_name:?}");
        let caps = nvim_regex().captures(window_name)?;
        let server_name = caps.name("server_name")?.as_str();
        println!("{server_name:?}");
        Some(Nvim { server_name })
    }
}

impl<'a> Server<'a> for Nvim<'a> {
    fn navigate(&self, direction: Direction) -> Result<(), Box<dyn std::error::Error>> {
        println!("{direction:?}");
        let mut session = Session::new_unix_socket(self.server_name)?;
        session.start_event_loop();

        let mut nvim = Neovim::new(session);

        let old_window = nvim.get_current_win()?;
        let cmd = direction_to_wincmd(direction);
        println!("{cmd:?}");
        nvim.command(&cmd)?;

        let window = nvim.get_current_win()?;

        if old_window == window {
            return Err(Box::new(NvimError::new("no window movement")));
        } else {
            return Ok(());
        }
    }

    fn get_window_title(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok("".to_string())
    }
}

fn direction_to_wincmd(direction: Direction) -> String {
    let vim_direction = match direction {
        Direction::Left => "h",
        Direction::Right => "l",
        Direction::Up => "k",
        Direction::Down => "j",
    };

    format!("wincmd {}", vim_direction)
}
pub fn nvim_regex() -> regex::Regex {
    let regex_string = beginning()
        + zero_or_more(any())
        + text("nvim")
        + zero_or_more(whitespace())
        + named_capture(one_or_more(non_whitespace()), "directory")
        + zero_or_more(any())
        + zero_or_more(whitespace())
        + zero_or_more(any())
        + text("[")
        + named_capture(zero_or_more(any()), "server_name")
        + text("]")
        + zero_or_more(any())
        + end();

    regex_string.to_regex()
}
