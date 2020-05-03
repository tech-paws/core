use crate::render::components::{GridComponent, RenderCommand, RenderState};
use specs::Join;
use specs::{ReadStorage, System, Write};

pub struct GridSystem;

impl<'a> System<'a> for GridSystem {
    type SystemData = (Write<'a, RenderState>, ReadStorage<'a, GridComponent>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut render_state, grid) = data;
        let render_commands = &mut render_state.render_commands;

        for grid in grid.join() {
            // println!("{:?}", grid);
            render_commands.push(RenderCommand::PushColorShader);
            render_commands.push(RenderCommand::PushColor {
                r: grid.color.r,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            });

            render_commands.push(RenderCommand::SetColorUniform);
            GridSystem::draw_lines(&grid, render_commands);
        }

        render_commands.push(RenderCommand::DrawLines);
    }
}

impl GridSystem {
    fn draw_lines(grid: &GridComponent, render_commands: &mut Vec<RenderCommand>) {
        // Vertical lines
        for i in (0..grid.size.width as i32).step_by(grid.step as usize) {
            render_commands.push(RenderCommand::PushPos2f {
                x: grid.position.x + i as f32,
                y: grid.position.y,
            });

            render_commands.push(RenderCommand::PushPos2f {
                x: grid.position.x + i as f32,
                y: grid.position.y + grid.size.height,
            });
        }

        // Horizontal lines
        for i in (0..grid.size.height as i32).step_by(grid.step as usize) {
            render_commands.push(RenderCommand::PushPos2f {
                x: grid.position.x,
                y: grid.position.y + i as f32,
            });

            render_commands.push(RenderCommand::PushPos2f {
                x: grid.position.x + grid.size.width,
                y: grid.position.y + i as f32,
            });
        }
    }
}
