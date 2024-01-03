# scare

A command-line to-do list.

## About
**Scare** is a command-line tool built in ``rust`` designed to simplify task management. With ``scare``, you can efficiently create, organize, and manage your ``to-do lists`` directly from the ``command line``, making it easy to stay ``productive`` and ``organized``.

## Features
* Create and manage tasks from the command line.
* Streamlined interface for quick and efficient task management.
* Intuitive commands for adding, completing, and removing tasks.
* Cross-platform

## Usage
To get started with ``scare`` simply run
``` bash
$ scare
```

### Keybinds
- `C`: Create a new task
- `D`: Enter edit mode
- `Space`: Toggle task completion.
* `W`: Navigate to the previous task
* `Shift + W`: Navigate to the first task
- `S`: Navigate to the previous task
- `Shift + S`: Navigate to the last task
* `R`: Remove the current task ``(if done)``
* `Shift + R`: **Force** remove the current task
- `A` or `Ctrl + C`: Quit the application

## Installation

### Via Cargo (src)

If you have Rust installed on your machine, you can install ``scare`` by running
``` bash
$ cargo install scare
```
Alternatively, you can install ``scare`` directly from this repository by running
```bash
$ cargo install --git https://github.com/joaopugsley/scare.git
```

⚠️ If you do not have **cargo**, install using [Rust's Installation Documentation](https://doc.rust-lang.org/book/ch01-01-installation.html)

## License

The ``scare`` source files are distributed under the MIT License found in the LICENSE file.