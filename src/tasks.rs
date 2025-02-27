use colored::*;
use serde::{Deserialize, Serialize};

use crate::task::Task;

#[derive(Serialize, Deserialize)]
pub struct Tasks {
    tasks: Vec<Task>,
}

impl Tasks {
    pub fn new() -> Tasks {
        Tasks {
            tasks: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn list(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            println!("{}. {}", (index + 1).to_string().yellow(), task);
        }
    }

    pub fn remove(&mut self, index: usize) -> Result<(), ColoredString> {
        if index == 0 || index > self.tasks.len() {
            Err(format!(
                "No task found at the given index. \nThe number of tasks are: {}",
                self.tasks.len()
            ).to_string().red())
        } else {
            self.tasks.remove(index - 1);
            Ok(())
        }
    }

    pub fn done(&mut self, index: usize) -> Result<(), ColoredString> {
        if index == 0 || index > self.tasks.len() {
            Err(format!(
                "No task found at the given index. \nThe number of tasks are: {}",
                self.tasks.len()
            ).to_string().red())
        } else {
            self.tasks[index - 1].done();
            Ok(())
        }
    }
}


