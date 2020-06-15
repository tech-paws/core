use crate::commands::CommandsState;
use crate::components::ViewPortSize;
use crate::debug_services::commands_registry;
use crate::debug_services::profile;
use crate::debug_services::render;
use crate::debug_services::state::DEBUG_STATE;

pub fn debug_frame_end() {
    let debug_state = &mut DEBUG_STATE.lock().expect("failed to get debug state");
    profile::frame_end(debug_state);
}

pub fn debug_frame_start() {
    let debug_state = &mut DEBUG_STATE.lock().expect("failed to get debug state");
    profile::frame_start(debug_state);
}

pub fn init() {
    let debug_state = &mut DEBUG_STATE.lock().expect("failed to get debug state");
    commands_registry::init(debug_state);
}

pub fn step(commands_state: &mut CommandsState, view_port: &ViewPortSize) {
    let debug_state = &mut DEBUG_STATE.lock().expect("failed to get debug state");

    render::profile(debug_state, commands_state, view_port);
    render::frame_time(debug_state, commands_state);
    render::frames_log(debug_state, commands_state);
}
