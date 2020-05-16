#[allow(dead_code, unused_imports)]
#[allow(clippy::all)]
#[path = "../schemes/target/rust/commands_generated.rs"]
mod flatbuffers_commands;

pub mod commands;
pub mod render;

mod serialize;

use std::os::raw::c_int;
use std::slice;
use std::sync::Mutex;

use lazy_static::lazy_static;

use bumpalo::Bump;
use commands::*;
use legion::prelude::*;
use render::camera_system::camera_system;
use render::components::*;
use render::grid_system::grid_system;
use render::move_camera_system::{move_camera_system, render_touch_system};
use render::work_area::work_area_system;
use serialize::*;

#[derive(Default)]
pub struct CommandsDataMemory {
    vec2f_data: Vec<Vec2f>,
    vec2i_data: Vec<Vec2i>,
}

impl CommandsDataMemory {
    pub fn clear(&mut self) {
        self.vec2f_data.clear();
        self.vec2i_data.clear();
    }
}

pub struct Memory {
    serialize_buffer: Bump,
    commands_data: CommandsDataMemory,
}

struct ApplicationState {
    _universe: Universe,
    world: World,
    memory: Memory,
}

#[repr(C)]
pub enum SerializeFormat {
    Json = 0,
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
                color: Color::rgba(0.0, 0.0, 0.0, 0.2),
                step: 32,
            },
            Camera2DPositionListener::new(0),
        )],
    );

    world.insert(
        (),
        vec![(WorkAreaComponent {
            title: String::from("Hello world!"),
            color: Color::rgb(0.0, 0.0, 1.0),
            size: Vec2f::new(640.0, 480.0),
        },)],
    );

    world.insert(
        (),
        vec![(
            Camera2D {
                tag: 0,
                pos: Vec2f::new(-320.0, -240.0),
            },
            CameraMovable2D::default(),
            TouchState::default(),
            Camera2DPositionListener::new(0),
        )],
    );

    let memory = Memory {
        serialize_buffer: Bump::new(),
        commands_data: CommandsDataMemory::default(),
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

fn handle_request_command(application_state: &mut ApplicationState, command: &RequestCommand) {
    let world = &mut application_state.world;
    let memory = &mut application_state.memory.commands_data;

    match command {
        RequestCommand {
            command_type: RequestCommandType::PushVec2f,
            data: CommandData { vec2f, .. },
        } => {
            memory.vec2f_data.push(*vec2f);
        }
        RequestCommand {
            command_type: RequestCommandType::PushVec2i,
            data: CommandData { vec2i, .. },
        } => {
            memory.vec2i_data.push(*vec2i);
        }
        RequestCommand {
            command_type: RequestCommandType::SetViewportSize,
            ..
        } => {
            if let Some(vec2i) = memory.vec2i_data.pop() {
                set_view_port_size(world, vec2i.x, vec2i.y);
            } else {
                todo!("warning log");
            }

            memory.clear();
        }
        RequestCommand {
            command_type: RequestCommandType::OnTouchStart,
            ..
        } => {
            if let Some(vec2f) = memory.vec2f_data.pop() {
                let query = <(Write<TouchState>,)>::query();

                for (mut touch_state,) in query.iter(world) {
                    touch_state.touch = Touch::Start;
                    touch_state.touch_start = vec2f;
                    touch_state.touch_current = vec2f;
                }
            } else {
                todo!("warning log");
            }

            memory.clear();
        }
        RequestCommand {
            command_type: RequestCommandType::OnTouchEnd,
            ..
        } => {
            if let Some(vec2f) = memory.vec2f_data.pop() {
                let query = <(Write<TouchState>,)>::query();

                for (mut touch_state,) in query.iter(world) {
                    if touch_state.touch == Touch::Start || touch_state.touch == Touch::Move {
                        touch_state.touch = Touch::End;
                        touch_state.touch_current = vec2f;
                    }
                }
            } else {
                todo!("warning log");
            }

            memory.clear();
        }
        RequestCommand {
            command_type: RequestCommandType::OnTouchMove,
            ..
        } => {
            if let Some(vec2f) = memory.vec2f_data.pop() {
                let query = <(Write<TouchState>,)>::query();

                for (mut touch_state,) in query.iter(world) {
                    if touch_state.touch == Touch::Start || touch_state.touch == Touch::Move {
                        touch_state.touch = Touch::Move;
                        touch_state.touch_current = vec2f;
                    }
                }
            } else {
                todo!("warning log");
            }

            memory.clear();
        }
    }
}

fn flush(application_state: &mut ApplicationState) {
    let mut state = application_state
        .world
        .resources
        .get_mut::<CommandsState>()
        .unwrap();

    application_state.memory.serialize_buffer.reset();
    application_state.memory.commands_data.clear();

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

#[repr(C)]
pub struct RenderCommands {
    items: *const RenderCommand,
    length: c_int,
}

#[repr(C)]
pub struct ExecutionCommands {
    items: *const ExecutionCommand,
    length: c_int,
}

#[no_mangle]
pub extern "C" fn c_get_render_commands() -> RenderCommands {
    match APPLICATION_STATE.lock().unwrap().as_mut() {
        Some(application_state) => {
            let state = application_state
                .world
                .resources
                .get::<CommandsState>()
                .unwrap();

            RenderCommands {
                items: state.render_commands.as_ptr(),
                length: state.render_commands.len() as c_int,
            }
        }
        None => panic!(":("),
    }
}

#[no_mangle]
pub extern "C" fn c_get_exec_commands() -> ExecutionCommands {
    match APPLICATION_STATE.lock().unwrap().as_mut() {
        Some(application_state) => {
            let state = application_state
                .world
                .resources
                .get::<CommandsState>()
                .unwrap();

            ExecutionCommands {
                items: state.exec_commands.as_ptr(),
                length: state.exec_commands.len() as c_int,
            }
        }
        None => panic!(":("),
    }
}

#[no_mangle]
pub unsafe extern "C" fn c_send_request_commands(data: *const RequestCommand, length: c_int) {
    match APPLICATION_STATE.lock().unwrap().as_mut() {
        Some(application_state) => {
            let mut state = application_state
                .world
                .resources
                .get_mut::<CommandsState>()
                .unwrap();

            let requests = slice::from_raw_parts(data, length as usize);
            state.request_commands.extend_from_slice(requests);
        }
        None => panic!(":("),
    }
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
            };

            state.request_commands.extend(requests.into_iter());
        }
        None => panic!(":("),
    }
}

fn set_view_port_size(world: &mut World, width: i32, height: i32) {
    let mut view_port_size = world.resources.get_mut::<ViewPortSize>().unwrap();

    view_port_size.width = width;
    view_port_size.height = height;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::init_world();
        let json = "[{\"SetViewportSize\": {\"width\": 100, \"height\": 200}}]".as_bytes();
        // let data = crate::RawBuffer {
        // data: json.as_ptr(),
        // length: json.len(),
        // };
        // crate::send_request_commands(crate::SerializeFormat::Json, data);
        // crate::step();
        // let mut commands_state = crate::commands::CommandsState::default();
        // crate::render::gapi::push_color_shader(&mut commands_state);
        let data = crate::get_render_commands(crate::SerializeFormat::Json);
        println!("{:?}", data);
        // crate::get_render_commands();
        // crate::step();
        // let data = crate::get_render_commands();

        // dbg!(data);

        assert_eq!(2 + 2, 4);
    }
}
