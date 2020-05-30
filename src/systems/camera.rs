use crate::components::{Camera2D, Camera2DPositionListener, ViewPortSize};
use crate::commands::{CommandsState, Vec2f};
use crate::gapi;

use legion::prelude::*;

pub fn camera_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("camera_system")
        .write_resource::<CommandsState>()
        .read_resource::<ViewPortSize>()
        .with_query(<(Read<Camera2D>,)>::query())
        .with_query(<(Write<Camera2DPositionListener>,)>::query())
        .build(|_, mut world, (commands_state, view_port_size), (q1, q2)| {
            // TODO: Remove hardcode - 2
            let mut pos = vec![Vec2f::zero(); 2];

            for (camera,) in q1.iter(&mut world) {
                pos[camera.tag].x = view_port_size.width as f32 / 2.0 + camera.pos.x;
                pos[camera.tag].y = view_port_size.height as f32 / 2.0 + camera.pos.y;

                gapi::update_camera_position(commands_state, pos[camera.tag]);
            }

            for (mut camera_listener,) in q2.iter(&mut world) {
                camera_listener.pos = pos[camera_listener.tag];
            }
        })
}
