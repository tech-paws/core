use crate::render::components::{
    Camera2D, CameraPos2fListener, ExectutionCommand, Pos2f, CommandsState, ViewPortSize,
};
use legion::prelude::*;

pub fn camera_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("camera_system")
        .write_resource::<CommandsState>()
        .read_resource::<ViewPortSize>()
        .with_query(<(Read<Camera2D>,)>::query())
        .with_query(<(Write<CameraPos2fListener>,)>::query())
        .build(|_, mut world, (render_state, view_port_size), (q1, q2)| {
            let exec_commands = &mut render_state.exec_commands;
            // TODO: Remove hardcode - 2
            let mut pos = vec![Pos2f::default(); 2];

            for (camera,) in q1.iter(&mut world) {
                pos[camera.tag].x = view_port_size.width as f32 / 2.0 + camera.pos.x;
                pos[camera.tag].y = view_port_size.height as f32 / 2.0 + camera.pos.y;

                exec_commands.push(ExectutionCommand::PushPos2f {
                    x: pos[camera.tag].x,
                    y: pos[camera.tag].y,
                });
                exec_commands.push(ExectutionCommand::UpdateCameraPosition);
            }

            for (mut camera_listener,) in q2.iter(&mut world) {
                camera_listener.pos = pos[camera_listener.tag].clone();
            }
        })
}
