use crate::direction::Direction;
use crate::server::{Server, ServerError};
use std::cell::RefCell;
use swayipc::Connection;
use swayipc::Node;

#[derive(Debug)]
pub struct Sway {
    conn: RefCell<Connection>,
}

impl Sway {
    pub fn new() -> Option<Sway> {
        let conn = Connection::new().ok()?;
        Some(Sway {
            conn: RefCell::new(conn),
        })
    }
}

// Recursively find the focused node in the tree
fn find_focused(node: &Node) -> Option<&Node> {
    if node.focused {
        return Some(node);
    }

    for child in &node.nodes {
        if let Some(focused) = find_focused(child) {
            return Some(focused);
        }
    }

    None
}

impl<'a> Server<'a> for Sway {
    fn navigate(&self, direction: Direction) -> Result<(), Box<dyn std::error::Error>> {
        let msg = match direction {
            Direction::Up => "focus up",
            Direction::Down => "focus down",
            Direction::Left => "focus left",
            _ => "focus right",
        };

        self.conn.borrow_mut().run_command(msg)?;

        Ok(())
    }

    fn get_window_title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let tree = self.conn.borrow_mut().get_tree()?;

        let focused = find_focused(&tree);
        match focused {
            None => Err(Box::new(ServerError)),
            Some(node) => node_name(&node.name),
        }
    }
}

fn node_name(name: &Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    match name {
        Some(name) => Ok(name.clone()),
        None => Err(Box::new(ServerError)),
    }
}
