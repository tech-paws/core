use crate::render::components::{RenderCommand, RenderState};
use specs::{System, Write};

pub struct DemoRenderSystem;

impl<'a> System<'a> for DemoRenderSystem {
    type SystemData = (Write<'a, RenderState>,);

    fn run(&mut self, data: Self::SystemData) {
        let mut render_state = data.0;

        let demo_commands = vec![
            RenderCommand::PushColor {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            RenderCommand::PushPos2f { x: 100.0, y: 100.0 },
            RenderCommand::PushPos2f { x: 100.0, y: 200.0 },
            RenderCommand::DrawLine,
        ];

        render_state.render_commands.extend(demo_commands.into_iter());
    }
}
