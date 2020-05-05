use crate::render::components::{
    Camera2D, CameraPos2fListener, ExectutionCommand, Pos2f, RenderState, ViewPortSize,
};
use legion::prelude::*;

pub fn move_camera_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("move_camera_system")
        .with_query(<(Write<Camera2D>,)>::query())
        .build(|_, mut world, _, query| {

        })
}
