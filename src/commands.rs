use serde::{Deserialize, Serialize};
use std::default::Default;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    pub fn zero() -> Vec2f {
        Vec2f { x: 0.0, y: 0.0 }
    }

    pub fn new(x: f32, y: f32) -> Vec2f {
        Vec2f { x, y }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub fn zero() -> Vec2i {
        Vec2i { x: 0, y: 0 }
    }

    pub fn new(x: i32, y: i32) -> Vec2i {
        Vec2i { x, y }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    pub fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b, a: 1.0 }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommandData {
    pub vec2f: Vec2f,
    pub vec2i: Vec2i,
    pub color: Color,
}

impl CommandData {
    pub fn vec2f(data: Vec2f) -> CommandData {
        CommandData {
            vec2f: data,
            vec2i: Vec2i::default(),
            color: Color::default(),
        }
    }

    pub fn vec2i(data: Vec2i) -> CommandData {
        CommandData {
            vec2f: Vec2f::default(),
            vec2i: data,
            color: Color::default(),
        }
    }

    pub fn color(data: Color) -> CommandData {
        CommandData {
            vec2f: Vec2f::default(),
            vec2i: Vec2i::default(),
            color: data,
        }
    }
}

// last: 6
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum RenderCommandType {
    PushColor = 0,
    PushVec2f = 1,
    SetColorUniform = 2,
    PushColorShader = 3,
    DrawLines = 4,
    DrawPoints = 5,
    DrawQuads = 6,
}

// last: 5
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum RequestCommandType {
    PushVec2f = 0,
    PushVec2i = 5,
    SetViewportSize = 1,
    OnTouchStart = 2,
    OnTouchEnd = 3,
    OnTouchMove = 4,
}

// last: 1
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ExecutionCommandType {
    PushVec2f = 0,
    UpdateCameraPosition = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderCommand {
    pub command_type: RenderCommandType,
    pub data: CommandData,
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionCommand {
    pub command_type: ExecutionCommandType,
    pub data: CommandData,
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestCommand {
    pub command_type: RequestCommandType,
    pub data: CommandData,
}

pub struct CommandsState {
    pub render_commands: Vec<RenderCommand>,
    pub exec_commands: Vec<ExecutionCommand>,
    pub request_commands: Vec<RequestCommand>,
}

impl Default for CommandsState {
    fn default() -> Self {
        CommandsState {
            render_commands: Vec::new(),
            exec_commands: Vec::new(),
            request_commands: Vec::new(),
        }
    }
}

pub fn push_render_command(commands_state: &mut CommandsState, command_type: RenderCommandType) {
    let command = RenderCommand {
        command_type,
        data: CommandData::default(),
    };

    commands_state.render_commands.push(command);
}

pub fn push_render_command_data(
    commands_state: &mut CommandsState,
    command_type: RenderCommandType,
    data: CommandData,
) {
    let command = RenderCommand { command_type, data };
    commands_state.render_commands.push(command);
}

pub fn push_execution_command(
    commands_state: &mut CommandsState,
    command_type: ExecutionCommandType,
) {
    let command = ExecutionCommand {
        command_type,
        data: CommandData::default(),
    };

    commands_state.exec_commands.push(command);
}

pub fn push_execution_command_data(
    commands_state: &mut CommandsState,
    command_type: ExecutionCommandType,
    data: CommandData,
) {
    let command = ExecutionCommand { command_type, data };
    commands_state.exec_commands.push(command);
}

pub fn push_request_command(
    commands_state: &mut CommandsState,
    command_type: RequestCommandType,
) {
    let command = RequestCommand {
        command_type,
        data: CommandData::default(),
    };

    commands_state.request_commands.push(command);
}

pub fn push_request_command_data(
    commands_state: &mut CommandsState,
    command_type: RequestCommandType,
    data: CommandData,
) {
    let command = RequestCommand { command_type, data };
    commands_state.request_commands.push(command);
}
