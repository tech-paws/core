use serde::{Serialize, Deserialize};
use specs::{Component, DenseVecStorage};

#[derive(Component, Debug,Serialize, Deserialize)]
pub enum RenderCommand {
    PushColor { r: f32, g: f32, b: f32, a: f32 },
    PushPos2f { x: f32, y: f32 },
    PushSize2f { x: f32, y: f32 },
    PushTexture { name: String },
    PushColorShader,
    PushTextureShader,
    DrawLine,
    DrawLines,
    DrawPoint,
    DrawPoints,
    DrawQuad,
    DrawQuads,
}

/*
// Clear screen
PushColor { r: 0, g: 0, b: 0, a: 1 }
Clear

// Draw Linew
PushColor { r: 1, g: 1, b: 1, a: 1 }
PushColorShader,
PushPos2f { x: 100, y: 100 }
PushPos2f { x: 100, y: 200 }
*/
