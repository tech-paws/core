use crate::render::components::{
    Camera2D, CameraPos2fListener, ExectutionCommand, Pos2f, RenderState, ViewPortSize,
};
use specs::Join;
use specs::{Read, ReadStorage, System, Write, WriteStorage};

pub struct CameraSystem;

impl<'a> System<'a> for CameraSystem {
    type SystemData = (
        Write<'a, RenderState>,
        Read<'a, ViewPortSize>,
        ReadStorage<'a, Camera2D>,
        WriteStorage<'a, CameraPos2fListener>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut render_state, view_port_size, camera, mut listener) = data;
        let exec_commands = &mut render_state.exec_commands;
        let mut pos = Pos2f::default();

        for camera in camera.join() {
            pos.x = view_port_size.width as f32 / 2.0 + camera.pos.x;
            pos.y = view_port_size.height as f32 / 2.0 + camera.pos.y;

            exec_commands.push(ExectutionCommand::PushPos2f { x: pos.x, y: pos.y });
            exec_commands.push(ExectutionCommand::UpdateCameraPosition);
        }

        for camera_listener in (&mut listener).join() {
            camera_listener.pos = pos.clone();
        }
    }
}
