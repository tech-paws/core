use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::commands::Rect;
use crate::debug_services::commands::CommandsState;
use crate::debug_services::profile::ProfileState;

lazy_static! {
    pub static ref DEBUG_STATE: Mutex<DebugState> = Mutex::new(DebugState::default());
}

pub enum DebugVariable {
    Bool(BoolVariable),
    Group(GroupVariable),
    Profiler(ProfilerVariable),
    ProfilerLogSlider(ProfilerLogSliderVariable),
}

#[derive(Default)]
pub struct ProfilerLogSliderVariable {
    pub is_hot: bool,
    pub bounds: Rect,
}

#[derive(Default)]
pub struct ProfilerVariable {
    pub is_hot: bool,
    pub bounds: Rect,
}

#[derive(Default)]
pub struct BoolVariable {
    pub name: &'static str,
    pub value: bool,
    pub is_hot: bool,
    pub bounds: Rect,
}

pub struct GroupVariable {
    pub name: &'static str,
    pub is_expanded: bool,
    pub variables: Vec<DebugVariable>,
    pub is_hot: bool,
    pub bounds: Rect,
}

impl GroupVariable {
    fn new(name: &'static str, variables: Vec<DebugVariable>) -> Self {
        GroupVariable {
            name,
            is_expanded: false,
            variables,
            is_hot: false,
            bounds: Rect::ZERO,
        }
    }
}

pub struct DebugState {
    pub _global_pause: bool,
    pub commands: CommandsState,
    pub variables: GroupVariable,
}

impl Default for DebugState {
    fn default() -> Self {
        DebugState {
            _global_pause: false,
            commands: CommandsState::default(),
            variables: GroupVariable::new(
                "Debug Menu",
                vec![DebugVariable::Group(GroupVariable::new(
                    "Profiler",
                    vec![
                        DebugVariable::ProfilerLogSlider(ProfilerLogSliderVariable::default()),
                        DebugVariable::Profiler(ProfilerVariable::default()),
                    ],
                ))],
            ),
        }
    }
}
