# Todo CLI Application

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Development and Future Improvements](#development-and-future-improvements)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Overview

A simple command-line to-do list application written in Rust. This application allows you to manage tasks from the
terminal, including listing, adding, removing, and marking tasks as done. It uses clap for command-line argument parsing
and serde for JSON file handling.

## Features

- List all tasks
- Add a new task with a title and optional description
- Remove a task by its ID
- Mark a task as done by its ID
- Persistent storage using JSON file

## Installation

To build and install the application, follow these steps:

#### 1. Clone the repository

```shell
git clone https://github.com/yourusername/todo-cli.git
cd todo-cli
```

#### 2. Build the application:

Make sure you have Rust installed. If not, you can install
it from [rust-lang.org](https://www.rust-lang.org/tools/install).

```shell
cargo build --release
```

#### 3. Install the application

You can copy the compiled binary to a directory in your `PATH` for easy access.

```shell
cp target/release/todo /usr/local/bin/
```

On Windows, you may use a similar approach to add the binary to your PATH or run it directly from the build directory.

## Usage

### Listing tasks

```shell
todo list

# 1. Buy groceries - Bananas, eggs, tomatoes [Done]
# 2. Learn Rust - Learn about results and option in Rust [Pending]
# 3. English homework [Pending]
```

This command lists all tasks with their current status.

### Adding a task

```shell
todo add "Task Title" "Task Description"
# Or
todo add Learn piano
```

This command adds a new task with the specified title and optional description.

### Removing a task

```shell
todo remove 1
```

This command removes the task with the specified ID.

### Marking a Task as Done

```shell
todo done 1
```

This command marks the task with the specified ID as done.

### Help Command

To view the help information and available commands:

```shell
todo --help
# Or
todo -h

#A simple to-do list application

#Usage: todo.exe <COMMAND>

#Commands:
  #list    [List all tasks in the todo list]
  #add     [Add a new task to the todo list]
  #remove  [Remove a task from the todo list by its ID]
  #done    [Mark a task as done by its ID]
  #help    Print this message or the help of the given subcommand(s)

#Options:
  #-h, --help  Print help
```

## Development and Future Improvements

This project was created as a learning exercise to explore various aspects of Rust programming. It is a work in progress
and will be continuously improved over time.

## License

This project is licensed under the General Public License version 3 (GPL-3.0 license) - see the [LICENSE](LICENSE) file
for details.

## Acknowledgments

- [Clap](https://docs.rs/clap/latest/clap/) for command-line argument parsing.
- [Serde](https://docs.rs/serde/latest/serde/) for JSON serialization/deserialization.
- [Colored](https://docs.rs/colored/latest/colored/index.html) for terminal color output.