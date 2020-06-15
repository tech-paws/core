use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::debug_services::commands::CommandsState;
use crate::debug_services::profile::ProfileState;

lazy_static! {
    pub static ref DEBUG_STATE: Mutex<DebugState> = Mutex::new(DebugState::default());
}

pub struct DebugState {
    pub _global_pause: bool,
    pub profile: ProfileState,
    pub commands: CommandsState,
}

impl Default for DebugState {
    fn default() -> Self {
        DebugState {
            _global_pause: false,
            profile: ProfileState::default(),
            commands: CommandsState::default(),
        }
    }
}
