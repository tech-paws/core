use crate::render::components::{
    Camera2D, CameraPos2fListener, CommandsState, Pos2f, RenderCommand, Size2f, TouchState,
    ViewPortSize, Touch
};
use legion::prelude::*;

pub fn move_camera_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("move_camera_system")
        .with_query(<(Write<Camera2D>,)>::query())
        .build(|_, mut _world, _, _query| {})
}

pub fn render_touch_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("render_touch_system")
        .write_resource::<CommandsState>()
        .read_resource::<ViewPortSize>()
        .with_query(<(Read<TouchState>, Read<CameraPos2fListener>)>::query())
        .build(|_, mut world, (render_state, view_port_size), query| {
            let render_commands = &mut render_state.render_commands;

            render_commands.push(RenderCommand::PushColorShader);
            render_commands.push(RenderCommand::PushColor {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            });

            render_commands.push(RenderCommand::SetColorUniform);

            for (touch, camera_listener) in query.iter(&mut world) {
                if touch.touch == Touch::None || touch.touch == Touch::End {
                    break;
                }

                let size = Size2f {
                    width: 32.0,
                    height: 32.0,
                };
                let pos = Pos2f {
                    x: -camera_listener.pos.x - 16.0 + touch.touch_current.x,
                    y: view_port_size.height as f32 - camera_listener.pos.y - 16.0
                        - touch.touch_current.y,
                };
                render_quad_lines(pos, size, render_commands);
            }

            render_commands.push(RenderCommand::DrawLines);
        })
}

pub fn render_quad_lines(pos: Pos2f, size: Size2f, render_commands: &mut Vec<RenderCommand>) {
    render_commands.push(RenderCommand::PushPos2f { x: pos.x, y: pos.y });
    render_commands.push(RenderCommand::PushPos2f {
        x: pos.x + size.width,
        y: pos.y,
    });

    render_commands.push(RenderCommand::PushPos2f {
        x: pos.x + size.width,
        y: pos.y,
    });
    render_commands.push(RenderCommand::PushPos2f {
        x: pos.x + size.width,
        y: pos.y + size.height,
    });

    render_commands.push(RenderCommand::PushPos2f {
        x: pos.x + size.width,
        y: pos.y + size.height,
    });
    render_commands.push(RenderCommand::PushPos2f {
        x: pos.x,
        y: pos.y + size.height,
    });

    render_commands.push(RenderCommand::PushPos2f {
        x: pos.x,
        y: pos.y + size.height,
    });
    render_commands.push(RenderCommand::PushPos2f { x: pos.x, y: pos.y });
}
