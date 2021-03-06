use crate::commands::{CommandsState, Vec2f};
use crate::components::{
    Camera2D, Camera2DPositionListener, CameraMovable2D, LayersState, Touch, TouchState,
    ViewPortSize,
};
use crate::debug_services;
use crate::gapi;

use legion::prelude::*;

pub fn move_camera_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("move_camera_system")
        .write_resource::<LayersState>()
        .with_query(<(Write<Camera2D>, Write<CameraMovable2D>, Read<TouchState>)>::query())
        .build(|_, mut world, layers_state, query| {
            debug_services::timed_block!("move_camera_system");

            for (mut camera, mut camera_movable, touch_state) in query.iter(&mut world) {
                if layers_state.ui_layer_is_hot && !camera_movable.is_hot {
                    continue;
                }

                if camera_movable.is_hot {
                    layers_state.ui_layer_is_hot = true;
                }

                if touch_state.touch == Touch::Start {
                    layers_state.ui_layer_is_hot = true;
                    camera_movable.last_pos = camera.pos;
                }

                if touch_state.touch == Touch::None || touch_state.touch == Touch::End {
                    camera_movable.is_hot = false;
                    layers_state.ui_layer_is_hot = false;
                    break;
                }

                camera_movable.is_hot = true;
                camera.pos.x = camera_movable.last_pos.x - touch_state.touch_start.x
                    + touch_state.touch_current.x;

                camera.pos.y = camera_movable.last_pos.y - touch_state.touch_start.y
                    + touch_state.touch_current.y;
            }
        })
}

pub fn render_touch_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("render_touch_system")
        .write_resource::<CommandsState>()
        .read_resource::<ViewPortSize>()
        .with_query(<(
            Read<TouchState>,
            Read<CameraMovable2D>,
            Read<Camera2DPositionListener>,
        )>::query())
        .build(|_, mut world, (commands_state, _), query| {
            gapi::set_camera(commands_state, gapi::CAMERA_ORTHO);

            gapi::push_color_shader(commands_state);
            gapi::push_color_rgb(commands_state, 1.0, 0.0, 0.0);
            gapi::set_color_uniform(commands_state);

            for (touch, camera_movable, camera_listener) in query.iter(&mut world) {
                if !camera_movable.is_hot {
                    break;
                }

                let pos = Vec2f {
                    x: -camera_listener.pos.x - 16.0 + touch.touch_current.x,
                    // y: view_port_size.height as f32
                    //     - camera_listener.pos.y
                    //     - 16.0
                    //     - touch.touch_current.y,
                    y: -camera_listener.pos.y - 16.0 + touch.touch_current.y,
                };
                let size = Vec2f { x: 32.0, y: 32.0 };

                gapi::push_quad_lines(commands_state, pos, size);
            }

            gapi::draw_lines(commands_state);
        })
}
