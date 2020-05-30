use crate::commands::{CommandsState, Vec2f};
use crate::components::WorkAreaComponent;
use crate::gapi;

use legion::prelude::*;

pub fn work_area_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("work_area_system")
        .write_resource::<CommandsState>()
        .with_query(<(Read<WorkAreaComponent>,)>::query())
        .build(|_, mut world, commands_state, query| {
            for (work_area,) in query.iter(&mut world) {
                gapi::push_color_shader(commands_state);
                gapi::push_color(commands_state, work_area.color);
                gapi::set_color_uniform(commands_state);
                gapi::push_quad_lines(commands_state, Vec2f::zero(), work_area.size);
            }

            gapi::draw_lines(commands_state);
        })
}
