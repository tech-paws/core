use crate::render::components::{
    Camera2D, CameraPos2fListener, ExectutionCommand, Pos2f, RenderState,
    ViewPortSize,
};
use specs::Join;
use specs::{Read, ReadStorage, System, Write, WriteStorage};

#[derive(Default)]
pub struct MoveCameraSystem {
    touch_start: Pos2f,
}

impl<'a> System<'a> for MoveCameraSystem {
    type SystemData = (
        ReadStorage<'a, RenderState>,
        WriteStorage<'a, Camera2D>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (render_state, mut camera) = data;
        // let action_commands = &render_state.action_commands;

        for camera in (&mut camera).join() {
            // pos.x = view_port_size.width as f32 / 2.0 + camera.pos.x;
            // pos.y = view_port_size.height as f32 / 2.0 + camera.pos.y;
            // exec_commands.push(ExectutionCommand::PushPos2f { x: pos.x, y: pos.y });
            // exec_commands.push(ExectutionCommand::UpdateCameraPosition);
        }
    }
}
