use crate::render::components::{RenderCommand, RenderState, WorkAreaComponent};
use specs::Join;
use specs::{ReadStorage, System, Write};

pub struct WorkAreaSystem;

impl<'a> System<'a> for WorkAreaSystem {
    type SystemData = (Write<'a, RenderState>, ReadStorage<'a, WorkAreaComponent>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut render_state, work_area) = data;
        let render_commands = &mut render_state.render_commands;

        for work_area in work_area.join() {
            render_commands.push(RenderCommand::PushColorShader);
            render_commands.push(RenderCommand::PushColor {
                r: work_area.color.r,
                g: work_area.color.g,
                b: work_area.color.b,
                a: work_area.color.a,
            });

            render_commands.push(RenderCommand::SetColorUniform);
            WorkAreaSystem::draw_lines(&work_area, render_commands);
        }

        render_commands.push(RenderCommand::DrawLines);
    }
}

impl WorkAreaSystem {
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
}
