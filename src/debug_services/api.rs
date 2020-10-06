use crate::commands::CommandsState;
use crate::components::{LayersState, TouchState, ViewPortSize};
use crate::debug_services::commands_registry;
use crate::debug_services::profile;
use crate::debug_services::profile::PROFILE_STATE;
use crate::debug_services::render;
use crate::debug_services::state::DEBUG_STATE;
use crate::debug_services::step;
use crate::render_state::RENDER_STATE;

pub use crate::debug_services::commands::*;

pub fn debug_frame_end() {
    let profile_state = &mut PROFILE_STATE.lock().expect("failed to get profile state");
    profile::frame_end(profile_state);
}

pub fn debug_frame_start() {
    let profile_state = &mut PROFILE_STATE.lock().expect("failed to get profile state");
    profile::frame_start(profile_state);
}

pub fn init() {
    let debug_state = &mut DEBUG_STATE.lock().expect("failed to get debug state");
    commands_registry::init(debug_state);
}

pub fn render_pass(commands_state: &mut CommandsState, view_port: &ViewPortSize) {
    let debug_state = &mut DEBUG_STATE.lock().expect("failed to get debug state");
    let render_state = &mut RENDER_STATE.lock().expect("failed to get render state");
    let profile_state = &mut PROFILE_STATE.lock().expect("failed to get profile state");

    render::render(
        debug_state,
        render_state,
        profile_state,
        commands_state,
        view_port,
    );
}

pub fn ui_step_pass(touch_state: &TouchState, layers_state: &mut LayersState) {
    let debug_state = &mut DEBUG_STATE.lock().expect("failed to get debug state");
    step::step(debug_state, touch_state, layers_state);
}
