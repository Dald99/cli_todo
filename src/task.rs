use std::fmt;
use std::fmt::Formatter;

use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum State {
    Done,
    NotDone,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    title: String,
    description: Option<String>,
    state: State,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let state = match self.state {
            State::Done => "[Done]".to_string().green(),
            State::NotDone => "[Pending]".to_string().red(),
        };

        let description = self.description.as_deref().unwrap_or("");

        if description.trim() != "" {
            write!(f, "{} - {} {}", self.title, description, state)
        } else {
            write!(f, "{} {}", self.title, state)
        }
    }
}

impl Task {
    pub fn new(title: String, description: String) -> Result<Task, ColoredString> {
        let desc = if description.is_empty() { None } else { Some(description.to_string()) };

        if title.trim() == "" {
            Err("Task title cannot be empty. \nExample: `todo add \"Buy groceries\" \"Milk, Eggs, \
            Bread\"`".to_string().red())
        } else {
            Ok(Task {
                title,
                description: desc,
                state: State::NotDone,
            })
        }
    }

    pub fn done(&mut self) {
        self.state = State::Done;
    }
}