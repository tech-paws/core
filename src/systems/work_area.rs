use crate::commands::{CommandsState, Vec2f};
use crate::components::WorkAreaComponent;
use crate::debug_services;
use crate::gapi;

use legion::prelude::*;

pub fn render_work_area_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("work_area_system")
        .write_resource::<CommandsState>()
        .with_query(<(Read<WorkAreaComponent>,)>::query())
        .build(|_, mut world, commands_state, query| {
            debug_services::timed_block!("work_area_system");

            for (work_area,) in query.iter(&mut world) {
                gapi::push_color_shader(commands_state);
                gapi::push_color(commands_state, work_area.color);
                gapi::set_color_uniform(commands_state);
                gapi::push_quad_lines(commands_state, Vec2f::ZERO, work_area.size);
            }

            gapi::set_camera(commands_state, gapi::CAMERA_ORTHO);
            gapi::draw_lines(commands_state);
        })
}
