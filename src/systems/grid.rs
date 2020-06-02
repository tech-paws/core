use crate::commands::CommandsState;
use crate::components::{Camera2DPositionListener, GridComponent, ViewPortSize};
use crate::debug_services;
use crate::gapi;

use legion::prelude::*;

pub fn grid_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("grid_system")
        .write_resource::<CommandsState>()
        .read_resource::<ViewPortSize>()
        .with_query(<(Read<GridComponent>, Read<Camera2DPositionListener>)>::query())
        .build(|_, world, (commands_state, view_port_size), query| {
            debug_services::timed_block!("grid_system");

            for (grid, camera) in query.iter(world) {
                gapi::push_color_shader(commands_state);
                gapi::push_color(commands_state, grid.color);
                gapi::set_color_uniform(commands_state);

                push_lines(commands_state, &grid, &view_port_size, &camera);
            }

            gapi::draw_lines(commands_state);
        })
}

fn push_lines(
    render_state: &mut CommandsState,
    grid: &GridComponent,
    size: &ViewPortSize,
    camera: &Camera2DPositionListener,
) {
    let camera_x = camera.pos.x.round() as i32;
    let camera_y = camera.pos.y.round() as i32;

    // Vertical lines
    let from = -camera_x + camera_x % grid.step;
    let to = -camera_x + size.width;

    for i in (from..to).step_by(grid.step as usize) {
        gapi::push_vec2f_xy(render_state, i as f32, -camera.pos.y);
        gapi::push_vec2f_xy(render_state, i as f32, size.height as f32 - camera.pos.y);
    }

    // Horizontal lines
    let from = -camera_y + camera_y % grid.step;
    let to = -camera_y + size.height;

    for i in (from..to).step_by(grid.step as usize) {
        gapi::push_vec2f_xy(render_state, -camera.pos.x, i as f32);
        gapi::push_vec2f_xy(render_state, size.width as f32 - camera.pos.x, i as f32);
    }
}
