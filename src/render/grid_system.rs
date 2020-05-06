use crate::render::components::{
    CameraPos2fListener, GridComponent, RenderCommand, CommandsState, ViewPortSize,
};
use legion::prelude::*;

pub fn grid_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("grid_system")
        .write_resource::<CommandsState>()
        .read_resource::<ViewPortSize>()
        .with_query(<(Read<GridComponent>, Read<CameraPos2fListener>)>::query())
        .build(|_, world, (render_state, view_port_size), query| {
            let render_commands = &mut render_state.render_commands;

            for (grid, camera) in query.iter(world) {
                render_commands.push(RenderCommand::PushColorShader);
                render_commands.push(RenderCommand::PushColor {
                    r: grid.color.r,
                    g: grid.color.g,
                    b: grid.color.b,
                    a: grid.color.a,
                });

                render_commands.push(RenderCommand::SetColorUniform);
                draw_lines(&grid, &view_port_size, &camera, render_commands);
            }

            render_commands.push(RenderCommand::DrawLines);
        })
}

fn draw_lines(
    grid: &GridComponent,
    size: &ViewPortSize,
    camera: &CameraPos2fListener,
    render_commands: &mut Vec<RenderCommand>,
) {
    let camera_x = camera.pos.x.round() as i32;
    let camera_y = camera.pos.y.round() as i32;

    // Vertical lines
    let from = -camera_x + camera_x % grid.step;
    let to = -camera_x + size.width;

    for i in (from..to).step_by(grid.step as usize) {
        render_commands.push(RenderCommand::PushPos2f {
            x: i as f32,
            y: -camera.pos.y,
        });
        render_commands.push(RenderCommand::PushPos2f {
            x: i as f32,
            y: size.height as f32 - camera.pos.y,
        });
    }

    // Horizontal lines
    let from = -camera_y + camera_y % grid.step;
    let to = -camera_y + size.height;

    for i in (from..to).step_by(grid.step as usize) {
        render_commands.push(RenderCommand::PushPos2f {
            x: -camera.pos.x,
            y: i as f32,
        });
        render_commands.push(RenderCommand::PushPos2f {
            x: size.width as f32 - camera.pos.x,
            y: i as f32,
        });
    }
}
