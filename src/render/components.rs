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
