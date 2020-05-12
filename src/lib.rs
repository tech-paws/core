#[allow(dead_code, unused_imports)]
#[allow(clippy::all)]
#[path = "../schemes/target/rust/commands_generated.rs"]
mod flatbuffers_commands;

mod render;
mod serialize;

use std::sync::Mutex;

use lazy_static::lazy_static;

use legion::prelude::*;
use render::camera_system::camera_system;
use render::components::*;
use render::grid_system::grid_system;
use render::move_camera_system::{move_camera_system, render_touch_system};
use render::work_area::work_area_system;
use serialize::*;

pub struct Memory {
    serialize_buffer: Vec<u8>,
}

struct ApplicationState {
    _universe: Universe,
    world: World,
    memory: Memory,
}

#[repr(C)]
pub enum SerializeFormat {
    Json = 0,
    Flatbuffers = 1,
}

static mut SCHEDULER: Option<Schedule> = None;

lazy_static! {
    static ref APPLICATION_STATE: Mutex<Option<ApplicationState>> = Mutex::new(None);
}

#[no_mangle]
pub extern "C" fn init_world() {
    let universe = Universe::new();
    let mut world = universe.create_world();

    world.resources.insert(CommandsState::default());
    world.resources.insert(ViewPortSize::default());

    world.insert(
        (),
        vec![(
            GridComponent {
                color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.2,
                },
                step: 32,
            },
            CameraPos2fListener::new(0),
        )],
    );

    world.insert(
        (),
        vec![(WorkAreaComponent {
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
        },)],
    );

    world.insert(
        (),
        vec![(
            Camera2D {
                tag: 0,
                pos: Pos2f {
                    x: -320.0,
                    y: -240.0,
                },
            },
            CameraMovable2D::default(),
            TouchState::default(),
            CameraPos2fListener::new(0),
        )],
    );

    let memory = Memory {
        serialize_buffer: Vec::with_capacity(1_000_000_000),
    };

    unsafe {
        SCHEDULER = Some(
            Schedule::builder()
                .add_system(move_camera_system())
                .add_system(camera_system())
                .add_system(grid_system())
                .add_system(work_area_system())
                .add_system(render_touch_system())
                .flush()
                .build(),
        );
    }

    let application_state = ApplicationState {
        _universe: universe,
        world,
        memory,
    };

    APPLICATION_STATE.lock().unwrap().replace(application_state);
}

#[no_mangle]
pub extern "C" fn step() {
    match APPLICATION_STATE.lock().unwrap().as_mut() {
        Some(state) => {
            handle_request_commands(state);
            flush(state);

            unsafe {
                SCHEDULER.as_mut().unwrap().execute(&mut state.world);
            }

            delete_action_entities(&mut state.world);
        }
        None => panic!(":("),
    }
}

fn delete_action_entities(world: &mut World) {
    <(Read<Actions>,)>::query()
        .iter_entities(world)
        .map(|entity| entity.0)
        .collect::<Vec<Entity>>()
        .iter()
        .for_each(|entity| {
            world.delete(*entity);
        });
}

fn handle_request_commands(application_state: &mut ApplicationState) {
    let state = application_state
        .world
        .resources
        .get::<CommandsState>()
        .unwrap()
        .request_commands
        .to_vec();

    for command in state {
        handle_request_command(application_state, &command);
    }
}

fn handle_request_command(
    application_state: &mut ApplicationState,
    action_command: &RequestCommand,
) {
    let world = &mut application_state.world;

    match action_command {
        RequestCommand::SetViewportSize { width, height } => {
            set_view_port_size(application_state, *width, *height);
        }
        RequestCommand::OnTouchStart { x, y } => {
            let query = <(Write<TouchState>,)>::query();

            for (mut touch_state,) in query.iter(world) {
                touch_state.touch = Touch::Start;
                touch_state.touch_start = Pos2f { x: *x, y: *y };
                touch_state.touch_current = Pos2f { x: *x, y: *y };
            }
        }
        RequestCommand::OnTouchEnd { x, y } => {
            let query = <(Write<TouchState>,)>::query();

            for (mut touch_state,) in query.iter(world) {
                if touch_state.touch == Touch::Start || touch_state.touch == Touch::Move {
                    touch_state.touch = Touch::End;
                    touch_state.touch_current = Pos2f { x: *x, y: *y };
                }
            }
        }
        RequestCommand::OnTouchMove { x, y } => {
            let query = <(Write<TouchState>,)>::query();

            for (mut touch_state,) in query.iter(world) {
                if touch_state.touch == Touch::Start || touch_state.touch == Touch::Move {
                    touch_state.touch = Touch::Move;
                    touch_state.touch_current = Pos2f { x: *x, y: *y };
                }
            }
        }
    }
}

fn flush(application_state: &mut ApplicationState) {
    let mut state = application_state
        .world
        .resources
        .get_mut::<CommandsState>()
        .unwrap();

    application_state.memory.serialize_buffer.clear();
    state.render_commands.clear();
    state.exec_commands.clear();
    state.request_commands.clear();
}

#[repr(C)]
#[derive(Debug)]
pub struct RawBuffer {
    data: *const u8,
    length: usize,
}

#[no_mangle]
pub extern "C" fn get_render_commands(format: SerializeFormat) -> RawBuffer {
    match APPLICATION_STATE.lock().unwrap().as_mut() {
        Some(application_state) => {
            let state = application_state
                .world
                .resources
                .get::<CommandsState>()
                .unwrap();

            match format {
                SerializeFormat::Json => serialize_json_render_commands(
                    &mut application_state.memory,
                    &state.render_commands,
                ),
                SerializeFormat::Flatbuffers => serialize_flatbuffers_render_commands(
                    &mut application_state.memory,
                    &state.render_commands,
                ),
            }
        }
        None => panic!(":("),
    }
}

#[no_mangle]
pub extern "C" fn get_exec_commands(format: SerializeFormat) -> RawBuffer {
    match APPLICATION_STATE.lock().unwrap().as_mut() {
        Some(application_state) => {
            let state = application_state
                .world
                .resources
                .get::<CommandsState>()
                .unwrap();

            match format {
                SerializeFormat::Json => serialize_json_exec_commands(
                    &mut application_state.memory,
                    &state.exec_commands,
                ),
                SerializeFormat::Flatbuffers => serialize_flatbuffers_exec_commands(
                    &mut application_state.memory,
                    &state.exec_commands,
                ),
            }
        }
        None => panic!(":("),
    }
}

#[no_mangle]
pub extern "C" fn send_request_commands(format: SerializeFormat, data: RawBuffer) {
    match APPLICATION_STATE.lock().unwrap().as_mut() {
        Some(application_state) => {
            let mut state = application_state
                .world
                .resources
                .get_mut::<CommandsState>()
                .unwrap();

            let requests = match format {
                SerializeFormat::Json => deserialize_json_request_commands(data),
                SerializeFormat::Flatbuffers => deserialize_flatbuffers_request_commands(data),
            };

            state.request_commands.extend(requests.into_iter());
        }
        None => panic!(":("),
    }
}

fn set_view_port_size(application_state: &mut ApplicationState, width: i32, height: i32) {
    let mut view_port_size = application_state
        .world
        .resources
        .get_mut::<ViewPortSize>()
        .unwrap();

    view_port_size.width = width;
    view_port_size.height = height;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::init_world();
        let json = "[{\"SetViewportSize\": {\"width\": 100, \"height\": 200}}]".as_bytes();
        let data = crate::RawBuffer {
            data: json.as_ptr(),
            length: json.len(),
        };
        crate::send_request_commands(crate::SerializeFormat::Json, data);
        crate::step();
        let data = crate::get_render_commands(crate::SerializeFormat::Json);
        println!("{:?}", data);
        // crate::get_render_commands();
        // crate::step();
        // let data = crate::get_render_commands();

        // dbg!(data);

        assert_eq!(2 + 2, 4);
    }
}
