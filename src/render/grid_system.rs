use crate::render::components::{GridComponent, RenderCommand, RenderState, ViewPortSize};
use specs::Join;
use specs::{Read, ReadStorage, System, Write};

pub struct GridSystem;

impl<'a> System<'a> for GridSystem {
    type SystemData = (
        Write<'a, RenderState>,
        Read<'a, ViewPortSize>,
        ReadStorage<'a, GridComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut render_state, viewport_size, grid) = data;
        let render_commands = &mut render_state.render_commands;

        for grid in grid.join() {
            render_commands.push(RenderCommand::PushColorShader);
            render_commands.push(RenderCommand::PushColor {
                r: grid.color.r,
                g: grid.color.g,
                b: grid.color.b,
                a: grid.color.a,
            });

            render_commands.push(RenderCommand::SetColorUniform);
            GridSystem::draw_lines(&grid, &viewport_size, render_commands);
        }

        render_commands.push(RenderCommand::DrawLines);
    }
}

impl GridSystem {
    fn draw_lines(
        grid: &GridComponent,
        size: &ViewPortSize,
        render_commands: &mut Vec<RenderCommand>,
    ) {
        // Vertical lines
        for i in (0..size.width as i32).step_by(grid.step as usize) {
            render_commands.push(RenderCommand::PushPos2f {
                x: i as f32,
                y: 0.0,
            });
            render_commands.push(RenderCommand::PushPos2f {
                x: i as f32,
                y: size.height as f32,
            });
        }

        // Horizontal lines
        for i in (0..size.height as i32).step_by(grid.step as usize) {
            render_commands.push(RenderCommand::PushPos2f {
                x: 0.0,
                y: i as f32,
            });

            render_commands.push(RenderCommand::PushPos2f {
                x: size.width as f32,
                y: i as f32,
            });
        }
    }
}
