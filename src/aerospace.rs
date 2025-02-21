use crate::direction::Direction;
use crate::server::Server;

use std::error::Error;
use std::process::Command;

#[derive(Debug)]
pub struct Aerospace {}

impl Aerospace {
    pub fn new() -> Aerospace {
        Aerospace {}
    }
}

impl<'a> Server<'a> for Aerospace {
    fn navigate(&self, direction: Direction) -> Result<(), Box<dyn std::error::Error>> {
        let status = Command::new("aerospace")
            .args(["focus", direction.to_string().to_lowercase().as_str()])
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err("navigate failed".into())
        }
    }

    fn get_window_title(&self) -> Result<String, Box<dyn Error>> {
        let output = Command::new("aerospace")
            .args(["list-windows", "--focused"])
            .output()?;

        let output_str = String::from_utf8(output.stdout)?;

        let parts: Vec<&str> = output_str.split('|').map(|s| s.trim()).collect();

        match parts.last() {
            Some(title) => Ok(title.to_string()),
            None => Err("Failed to parse window title".into()),
        }
    }
}
