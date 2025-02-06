use bevy_console::{
    clap::{command, Parser},
    ConsoleCommand,
};

/// Example command
#[derive(Parser, ConsoleCommand)]
#[command(name = "example")]
pub struct ExampleCommand {
    /// Some message
    msg: String,
}

pub fn example_command(mut log: ConsoleCommand<ExampleCommand>) {
    if let Some(Ok(ExampleCommand { msg })) = log.take() {
        println!("{}", msg);
    }
}
