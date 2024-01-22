use std::sync::{Arc, Mutex};

use xcb_util::ewmh;

use crate::{
    screen::Screen,
    tag::TagID,
};

#[derive(Clone)]
pub struct EventContext {
    pub conn: Arc<ewmh::Connection>,
    pub screen: Arc<Mutex<Screen>>,
    curr_tag_id: TagID,
}

impl EventContext {
    pub fn new(conn: Arc<ewmh::Connection>, screen: Arc<Mutex<Screen>>) -> Self {
        let curr_tag_id = {
            screen.lock().unwrap().get_focused_tag().unwrap().id
        };

        Self {
            conn,
            screen,
            curr_tag_id,
        }
    }
}

impl EventContext {
    pub fn curr_tag_id(&self) -> TagID {
        self.curr_tag_id
    }

    pub fn spawn(&self, process: &str) -> Result<(), String> {
        let process: Vec<&str> = process.split_whitespace().collect();
        let (command, args) = process.split_first().ok_or("Process called in `spawn` is an empty string.")?;

        std::process::Command::new(command)
            .args(args)
            .spawn()
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
