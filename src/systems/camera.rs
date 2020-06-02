use crate::commands::{CommandsState, Vec2f};
use crate::components::{Camera2D, Camera2DPositionListener, ViewPortSize};
use crate::debug_services;
use crate::gapi;
use crate::memory;

use legion::prelude::*;

pub fn camera_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("camera_system")
        .write_resource::<CommandsState>()
        .read_resource::<ViewPortSize>()
        .with_query(<(Read<Camera2D>,)>::query())
        .with_query(<(Write<Camera2DPositionListener>,)>::query())
        .build(|_, mut world, (commands_state, view_port_size), (q1, q2)| {
            debug_services::timed_block!("camera_system");

            let memory_state = &mut memory::get_memory_state();
            let mut pos = memory::frame_alloc_vec::<Vec2f>(memory_state);

            for (camera,) in q1.iter(&mut world) {
                pos.push(Vec2f::new(
                    view_port_size.width as f32 / 2.0 + camera.pos.x,
                    view_port_size.height as f32 / 2.0 + camera.pos.y,
                ));
                gapi::update_camera_position(commands_state, pos[camera.tag]);
            }

            for (mut camera_listener,) in q2.iter(&mut world) {
                camera_listener.pos = pos[camera_listener.tag];
            }
        })
}
