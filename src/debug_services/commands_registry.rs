use std::sync::MutexGuard;

use crate::debug_services::commands::*;
use crate::debug_services::profile;
use crate::debug_services::state::{DebugState, DEBUG_STATE};

pub fn init(debug_state: &mut MutexGuard<DebugState>) {
    register_command(
        debug_state,
        "Update snapshot interval",
        Command {
            namespace: String::from("profile"),
            name: String::from("set_snapshot_interval"),
            executor: set_snapshot_interval_command,
        },
    );
}

fn set_snapshot_interval_command(
    _: &mut MutexGuard<DebugState>,
    arguments: &[CommandArgument],
) -> Result<(), String> {
    let debug_state = &mut DEBUG_STATE.lock().expect("failed to get debug state");

    require(arguments.len() == 1, "bad arguments length")?;

    let interval = match arguments[0] {
        CommandArgument::Number(val) => Ok(val),
        _ => Err(String::from("Argument should be int")),
    }?;

    profile::update_snapshot_interval(debug_state, interval as usize);

    Ok(())
}
