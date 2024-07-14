use colored::*;

use crate::task::Task;

pub struct Tasks {
    tasks: Vec<Task>,
}

impl Tasks {
    pub fn new() -> Tasks {
        Tasks {
            tasks: Vec::new(),
        }
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
            Err("No task found at the given index".to_string().red())
        } else {
            self.tasks.remove(index - 1);
            Ok(())
        }
    }

    pub fn done(&mut self, index: usize) -> Result<(), ColoredString> {
        if index == 0 || index > self.tasks.len() {
            Err("No task found at the given index".to_string().red())
        } else {
            self.tasks[index - 1].done();
            Ok(())
        }
    }
}
