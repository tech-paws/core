use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::MutexGuard;

use crate::debug_services::state::{DebugState, DEBUG_STATE};

pub const COMMANDS_HISTORY_CAPACITY: usize = 100;

pub struct CommandsState {
    pub history: Vec<String>,
    pub registry: Vec<CommandRegistryEntry>,
    pub index: HashMap<String, Command>,
}

impl Default for CommandsState {
    fn default() -> Self {
        CommandsState {
            history: Vec::with_capacity(COMMANDS_HISTORY_CAPACITY),
            registry: Vec::new(),
            index: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum CommandArgument {
    Number(f64),
    String(String),
}

pub struct Command {
    pub namespace: String,
    pub name: String,
    pub executor: fn(&mut MutexGuard<DebugState>, &[CommandArgument]) -> Result<(), String>,
}

pub struct CommandRequest {
    pub command: String,
    pub arguments: Vec<CommandArgument>,
}

pub struct CommandRegistryEntry {
    pub namespace: String,
    pub name: String,
    pub args: String,
    pub desc: &'static str,
}

pub fn register_command(
    debug_state: &mut MutexGuard<DebugState>,
    desc: &'static str,
    command: Command,
) {
    debug_state.commands.registry.push(CommandRegistryEntry {
        namespace: command.namespace.clone(),
        name: command.name.clone(),
        args: String::from("<arguments: int>"),
        desc: desc,
    });

    debug_state.commands.index.insert(
        format!("{}::{}", &command.namespace, &command.name),
        command,
    );
}

pub fn execute_command(command: &str) -> Result<(), String> {
    let debug_state = &mut DEBUG_STATE.lock().expect("failed to get debug state");
    debug_state.commands.history.push(String::from(command));
    let request = parse_command(command)?;
    execute_command_request(debug_state, &request)
}

fn parse_command(command: &str) -> Result<CommandRequest, String> {
    let mut components = command.split_whitespace();

    if components.clone().count() < 1 {
        Err(String::from("Command can't be empty"))
    } else {
        let command = components.next().unwrap();
        let command = String::from(command);
        let arguments = Vec::new();

        for _ in components {
            // TODO
        }

        Ok(CommandRequest { command, arguments })
    }
}

fn execute_command_request(
    debug_state: &mut MutexGuard<DebugState>,
    request: &CommandRequest,
) -> Result<(), String> {
    match debug_state.commands.index.get(&request.command) {
        Some(command) => {
            let executor = command.executor;
            executor(debug_state, &request.arguments)
        }
        None => Err(String::from("Command hasn't found")),
    }
}

pub fn require(cond: bool, msg: &str) -> Result<(), String> {
    if cond {
        Ok(())
    } else {
        Err(String::from(msg))
    }
}
