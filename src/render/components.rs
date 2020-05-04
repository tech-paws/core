use std::default::Default;

use serde::{Deserialize, Serialize};
use specs::{Component, DenseVecStorage};

#[derive(Component, Debug, Serialize, Deserialize)]
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

pub struct RenderState {
    pub render_commands: Vec<RenderCommand>,
}

impl Default for RenderState {
    fn default() -> Self {
        RenderState {
            render_commands: Vec::new(),
        }
    }
}

#[derive(Component, Default, Debug)]
pub struct Pos2f {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Default, Debug)]
pub struct Size2f {
    pub width: f32,
    pub height: f32,
}

#[derive(Component, Default, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Component, Default, Debug)]
pub struct GridComponent {
    pub step: i32,
    pub color: Color,
}

#[derive(Default)]
pub struct ViewPortSize {
    pub width: i32,
    pub height: i32,
}

#[derive(Component, Default, Debug)]
pub struct WorkAreaComponent {
    pub title: String,
    pub color: Color,
    pub size: Size2f,
}
