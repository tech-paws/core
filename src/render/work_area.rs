use crate::render::components::{RenderCommand, RenderState, WorkAreaComponent};
use legion::prelude::*;

pub fn work_area_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("work_area_system")
        .write_resource::<RenderState>()
        .with_query(<(Read<WorkAreaComponent>,)>::query())
        .build(|_, mut world, render_state, query| {
            let render_commands = &mut render_state.render_commands;

            for (work_area,) in query.iter(&mut world) {
                render_commands.push(RenderCommand::PushColorShader);
                render_commands.push(RenderCommand::PushColor {
                    r: work_area.color.r,
                    g: work_area.color.g,
                    b: work_area.color.b,
                    a: work_area.color.a,
                });

                render_commands.push(RenderCommand::SetColorUniform);
                draw_lines(&work_area, render_commands);
            }

            render_commands.push(RenderCommand::DrawLines);
        })
}

fn draw_lines(work_area: &WorkAreaComponent, render_commands: &mut Vec<RenderCommand>) {
    render_commands.push(RenderCommand::PushPos2f { x: 0.0, y: 0.0 });
    render_commands.push(RenderCommand::PushPos2f {
        x: work_area.size.width,
        y: 0.0,
    });

    render_commands.push(RenderCommand::PushPos2f {
        x: work_area.size.width,
        y: 0.0,
    });
    render_commands.push(RenderCommand::PushPos2f {
        x: work_area.size.width,
        y: work_area.size.height,
    });

    render_commands.push(RenderCommand::PushPos2f {
        x: work_area.size.width,
        y: work_area.size.height,
    });
    render_commands.push(RenderCommand::PushPos2f {
        x: 0.0,
        y: work_area.size.height,
    });

    render_commands.push(RenderCommand::PushPos2f {
        x: 0.0,
        y: work_area.size.height,
    });
    render_commands.push(RenderCommand::PushPos2f { x: 0.0, y: 0.0 });
}
