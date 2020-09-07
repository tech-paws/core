use crate::RawBuffer;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::ops;

pub struct Rect {
    pub pos: Vec2f,
    pub size: Vec2f,
}

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

impl ops::Add<Vec2f> for Vec2f {
    type Output = Vec2f;

    fn add(self, rhs: Vec2f) -> Vec2f {
        Vec2f::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::AddAssign<Vec2f> for Vec2f {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
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
#[derive(Debug, Clone, Default)]
pub struct CommandData {
    pub int32: i32,
    pub vec2f: Vec2f,
    pub vec2i: Vec2i,
    pub color: Color,
    pub string: RawBuffer,
}

impl CommandData {
    pub fn int32(data: i32) -> CommandData {
        CommandData {
            int32: data,
            vec2f: Vec2f::default(),
            vec2i: Vec2i::default(),
            color: Color::default(),
            string: RawBuffer::default(),
        }
    }

    pub fn vec2f(data: Vec2f) -> CommandData {
        CommandData {
            int32: i32::default(),
            vec2f: data,
            vec2i: Vec2i::default(),
            color: Color::default(),
            string: RawBuffer::default(),
        }
    }

    pub fn vec2i(data: Vec2i) -> CommandData {
        CommandData {
            int32: i32::default(),
            vec2f: Vec2f::default(),
            vec2i: data,
            color: Color::default(),
            string: RawBuffer::default(),
        }
    }

    pub fn color(data: Color) -> CommandData {
        CommandData {
            int32: i32::default(),
            vec2f: Vec2f::default(),
            vec2i: Vec2i::default(),
            color: data,
            string: RawBuffer::default(),
        }
    }

    pub fn string(data: &str) -> CommandData {
        CommandData {
            int32: i32::default(),
            vec2f: Vec2f::default(),
            vec2i: Vec2i::default(),
            color: Color::default(),
            string: RawBuffer::from_string(data),
        }
    }

    pub fn string_bytes(data: &[u8]) -> CommandData {
        CommandData {
            int32: i32::default(),
            vec2f: Vec2f::default(),
            vec2i: Vec2i::default(),
            color: Color::default(),
            string: RawBuffer::from_bytes(data),
        }
    }
}

// last: 11
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum RenderCommandType {
    PushColor = 0,
    PushVec2f = 1,
    PushInt32 = 11,
    SetCamera = 10,
    SetColorUniform = 2,
    PushColorShader = 3,
    PushTextShader = 9,
    PushString = 8,
    DrawLines = 4,
    DrawPoints = 5,
    DrawQuads = 6,
    DrawText = 7,
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

// last: 2
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ExecutionCommandType {
    PushVec2f = 0,
    PushInt32 = 2,
    UpdateCameraPosition = 1,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct RenderCommand {
    pub command_type: RenderCommandType,
    pub data: CommandData,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ExecutionCommand {
    pub command_type: ExecutionCommandType,
    pub data: CommandData,
}

#[repr(C)]
#[derive(Debug, Clone)]
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

pub fn push_request_command(commands_state: &mut CommandsState, command_type: RequestCommandType) {
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
