use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::debug_services::commands::CommandsState;
use crate::debug_services::profile::ProfileState;
use crate::commands::Rect;

lazy_static! {
    pub static ref DEBUG_STATE: Mutex<DebugState> = Mutex::new(DebugState::default());
}

pub enum DebugVariable {
    Bool(BoolVariable),
    Group(GroupVariable),

}

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

pub struct DebugState {
    pub _global_pause: bool,
    pub profile: ProfileState,
    pub commands: CommandsState,
    pub variables: GroupVariable,
}

impl Default for DebugState {
    fn default() -> Self {
        DebugState {
            _global_pause: false,
            profile: ProfileState::default(),
            commands: CommandsState::default(),
            variables: GroupVariable {
                is_expanded: false,
                is_hot: false,
                bounds: Rect::ZERO,
                name: "Debug menu",
                variables: vec![
                    DebugVariable::Bool(BoolVariable {
                        name: "Test Variable 1",
                        value: false,
                        is_hot: false,
                        bounds: Rect::ZERO,
                    }),
                    DebugVariable::Group(GroupVariable {
                        is_expanded: false,
                        is_hot: false,
                        bounds: Rect::ZERO,
                        name: "Group",
                        variables: vec![
                            DebugVariable::Bool(BoolVariable {
                                name: "Test Variable 1",
                                is_hot: false,
                                value: false,
                                bounds: Rect::ZERO,
                            }),
                            DebugVariable::Group(GroupVariable {
                                is_expanded: false,
                                is_hot: false,
                                bounds: Rect::ZERO,
                                name: "Group",
                                variables: vec![
                                    DebugVariable::Bool(BoolVariable {
                                        name: "Test Variable 1",
                                        is_hot: false,
                                        value: false,
                                        bounds: Rect::ZERO,
                                    }),
                                    DebugVariable::Bool(BoolVariable {
                                        name: "Test Variable 2",
                                        is_hot: false,
                                        value: true,
                                        bounds: Rect::ZERO,
                                    }),
                                ],
                            }),
                            DebugVariable::Bool(BoolVariable {
                                name: "Test Variable 2",
                                is_hot: false,
                                value: true,
                                bounds: Rect::ZERO,
                            }),
                        ],
                    }),
                    DebugVariable::Bool(BoolVariable {
                        name: "Test Variable 2",
                        is_hot: false,
                        value: true,
                        bounds: Rect::ZERO,
                    }),
                ],
            },
        }
    }
}
