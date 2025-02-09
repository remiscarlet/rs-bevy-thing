use bevy::prelude::*;
use bevy_console::ConsoleCommand;

pub fn process_console_command<T>(mut cmd: ConsoleCommand<T>) -> Option<T> {
    match cmd.take() {
        Some(Ok(cmd)) => Some(cmd),
        Some(Err(err)) => {
            eprintln!("Could not execute command: {:?}", err);
            None
        }
        None => None,
    }
}
