mod render;

use render::camera_system::CameraSystem;
use render::components::{
    ActionCommand, Camera2D, CameraPos2fListener, Color, GridComponent, Pos2f, RenderCommand,
    RenderState, Size2f, ViewPortSize, WorkAreaComponent,
};
use render::grid_system::GridSystem;
use render::work_area::WorkAreaSystem;
use render::move_camera_system::MoveCameraSystem;
use specs::{Builder, Dispatcher, DispatcherBuilder, World, WorldExt};

struct Memory {
    serialize_buffer: Vec<u8>,
}

struct ApplicationState {
    dispatcher: Dispatcher<'static, 'static>,
    world: World,
    memory: Memory,
}

#[repr(C)]
pub enum SerializeFormat {
    Json = 0,
}

static mut APPLICATION_STATE: Option<ApplicationState> = None;

#[no_mangle]
pub extern "C" fn init_world() {
    let mut world = World::new();

    world.register::<RenderCommand>();
    world.register::<GridComponent>();
    world.register::<WorkAreaComponent>();
    world.register::<Camera2D>();
    world.register::<CameraPos2fListener>();

    // Resources
    world.insert(RenderState::default());
    world.insert(ViewPortSize::default());

    world
        .create_entity()
        .with(GridComponent {
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.2,
            },
            step: 32,
        })
        .with(CameraPos2fListener::new(0))
        .build();

    world
        .create_entity()
        .with(WorkAreaComponent {
            title: String::from("Hello world!"),
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 1.0,
                a: 1.0,
            },
            size: Size2f {
                width: 640.0,
                height: 480.0,
            },
        })
        .build();

    world
        .create_entity()
        .with(Camera2D {
            tag: 0,
            pos: Pos2f {
                x: -320.0,
                y: -240.0,
            },
        })
        .build();

    let dispatcher = DispatcherBuilder::new()
        .with(WorkAreaSystem, "work_area", &[])
        .with(MoveCameraSystem::default(), "move_camera", &[])
        .with(CameraSystem, "camera", &["move_camera"])
        .with(GridSystem, "grid", &["camera"])
        .build();

    let memory = Memory {
        serialize_buffer: Vec::with_capacity(1_000_000_000),
    };

    unsafe {
        APPLICATION_STATE = Some(ApplicationState {
            world,
            dispatcher,
            memory,
        });
    }
}

unsafe fn get_application_state() -> &'static mut ApplicationState {
    APPLICATION_STATE
        .as_mut()
        .expect("ApplicationState should be presented")
}

#[no_mangle]
pub extern "C" fn step() {
    unsafe {
        let state = get_application_state();
        handle_action_commands(&state.world);
        flush();

        state.dispatcher.dispatch(&mut state.world);
        state.world.maintain();
    }
}

fn handle_action_commands(world: &World) {
    let state = world.read_resource::<RenderState>();

    for command in &state.action_commands {
        handle_action_command(command);
    }
}

fn handle_action_command(action_command: &ActionCommand) {
    match action_command {
        ActionCommand::OnTouchStart { x, y } => {

        }
        ActionCommand::OnTouchEnd { x, y } => {}
        ActionCommand::OnTouchMove { x, y } => {}
    }
}

unsafe fn flush() {
    let application_state = get_application_state();
    let mut state = application_state.world.write_resource::<RenderState>();

    application_state.memory.serialize_buffer.clear();
    state.render_commands.clear();
    state.exec_commands.clear();
    state.action_commands.clear();
}

#[repr(C)]
#[derive(Debug)]
pub struct RawBuffer {
    data: *const u8,
    length: usize,
}

#[no_mangle]
pub unsafe extern "C" fn get_render_commands() -> RawBuffer {
    let application_state = get_application_state();
    let state = application_state.world.read_resource::<RenderState>();

    let json = serde_json::to_vec(&state.render_commands).unwrap();

    let start = application_state.memory.serialize_buffer.len();
    let end = start + json.len();

    application_state
        .memory
        .serialize_buffer
        .extend(json.into_iter());

    let data = application_state.memory.serialize_buffer[start..end].as_ptr();

    RawBuffer {
        data,
        length: end - start,
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_exec_commands() -> RawBuffer {
    let application_state = get_application_state();
    let state = application_state.world.read_resource::<RenderState>();

    let json = serde_json::to_vec(&state.exec_commands).unwrap();

    let start = application_state.memory.serialize_buffer.len();
    let end = start + json.len();

    application_state
        .memory
        .serialize_buffer
        .extend(json.into_iter());

    let data = application_state.memory.serialize_buffer[start..end].as_ptr();

    RawBuffer {
        data,
        length: end - start,
    }
}

#[no_mangle]
pub unsafe extern "C" fn set_view_port_size(width: i32, height: i32) {
    let application_state = get_application_state();
    let mut view_port_size = application_state.world.write_resource::<ViewPortSize>();

    view_port_size.width = width;
    view_port_size.height = height;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::init_world();
        crate::step();
        crate::get_render_commands();
        crate::get_render_commands();
        crate::step();
        let data = crate::get_render_commands();

        dbg!(data);

        assert_eq!(2 + 2, 4);
    }
}
