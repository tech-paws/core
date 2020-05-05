use std::default::Default;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum RenderCommand {
    PushColor { r: f32, g: f32, b: f32, a: f32 },
    PushPos2f { x: f32, y: f32 },
    PushSize2f { x: f32, y: f32 },
    PushTexture { name: String },
    SetColorUniform,
    PushColorShader,
    PushTextureShader,
    DrawLines,
    DrawPoints,
    DrawQuads,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExectutionCommand {
    PushPos2f { x: f32, y: f32 },
    UpdateCameraPosition,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ActionCommand {
    OnTouchStart { x: f32, y: f32 },
    OnTouchEnd { x: f32, y: f32 },
    OnTouchMove { x: f32, y: f32 },
}

pub struct RenderState {
    pub render_commands: Vec<RenderCommand>,
    pub exec_commands: Vec<ExectutionCommand>,
    pub action_commands: Vec<ActionCommand>,
}

impl Default for RenderState {
    fn default() -> Self {
        RenderState {
            render_commands: Vec::new(),
            exec_commands: Vec::new(),
            action_commands: Vec::new(),
        }
    }
}

#[derive(Copy, Default, Debug, Clone)]
pub struct Pos2f {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Size2f {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, PartialEq, Copy, Default, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct GridComponent {
    pub step: i32,
    pub color: Color,
}

#[derive(Default)]
pub struct ViewPortSize {
    pub width: i32,
    pub height: i32,
}

#[derive(Default, Debug)]
pub struct WorkAreaComponent {
    pub title: String,
    pub color: Color,
    pub size: Size2f,
}

#[derive(Default, Debug)]
pub struct Camera2D {
    pub tag: usize,
    pub pos: Pos2f,
}

#[derive(Clone, Copy, Debug)]
pub struct CameraPos2fListener {
    pub tag: usize,
    pub pos: Pos2f,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Actions;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OnCameraTouchStart;

#[derive(Clone, Copy, Debug)]
pub struct OnCameraTouchMove;

#[derive(Clone, Copy, Debug)]
pub struct OnCameraTouchEnd;

impl CameraPos2fListener {
    pub fn new(tag: usize) -> CameraPos2fListener {
        return CameraPos2fListener {
            tag,
            pos: Pos2f::default(),
        };
    }
}
