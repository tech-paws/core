use crate::render::components::Camera2D;
use legion::prelude::*;

pub fn move_camera_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("move_camera_system")
        .with_query(<(Write<Camera2D>,)>::query())
        .build(|_, mut _world, _, _query| {})
}
