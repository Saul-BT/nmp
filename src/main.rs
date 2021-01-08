/*
  AUTHOR: Saul Blanco Tejero (@elGolpista)
  LICENSE: GPLv3
*/

use clap::{load_yaml, App};
use nmp::models::task::{Task, TaskPriority, TaskState};
use nmp::models::task_stack::TaskStack;

fn main() {
    // The YAML file has the cli arguments config
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    //if let Some(c) = matches.value_of("config") {
    //    println!("Value for config: {}", c);
    //}

    match matches.subcommand_name() {
        Some("add") => println!("TODO add"),
        Some("remove") => println!("TODO remove"),
        Some("list") => println!("TODO list"),
        _ => println!("Launching TUI..."),
    }
}
