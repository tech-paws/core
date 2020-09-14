use std::sync::MutexGuard;

use crate::commands::{Rect, Vec2f};
use crate::components::{TouchState, Touch};
use crate::debug_services::state::*;

// TODO(sysint64): Move to another file
pub fn point_in_rect(point: Vec2f, rect: Rect) -> bool {
    return point.x >= rect.pos.x
        && point.x <= rect.pos.x + rect.size.x
        && point.y >= rect.pos.y
        && point.y <= rect.pos.y + rect.size.y;
}

pub fn step(debug_state: &mut MutexGuard<DebugState>, touch_state: &TouchState) {
    step_group_variables(touch_state, &mut debug_state.variables);
}

fn step_group_variables(touch_state: &TouchState, variable: &mut GroupVariable) {
    variable.is_hot = point_in_rect(touch_state.pos, variable.bounds);

    if variable.is_hot && touch_state.touch == Touch::Start {
        variable.is_expanded = !variable.is_expanded;
    }

    for mut v in variable.variables.iter_mut() {
        match &mut v {
            DebugVariable::Bool(variable) => {
                step_bool_variables(touch_state, variable);
            }
            DebugVariable::Group(group) => {
                step_group_variables(touch_state, group);
            }
        };
    }
}

fn step_bool_variables(touch_state: &TouchState, variable: &mut BoolVariable) {
    variable.is_hot = point_in_rect(touch_state.pos, variable.bounds);

    if variable.is_hot && touch_state.touch == Touch::Start {
        variable.value = !variable.value;
    }
}
