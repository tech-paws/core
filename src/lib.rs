#[allow(dead_code, unused_imports)]
#[allow(clippy::all)]
#[path = "../schemes/target/rust/commands_generated.rs"]
mod flatbuffers_commands;

pub mod commands;
pub mod components;
pub mod debug_services;
pub mod gapi;
pub mod layout;
pub mod memory;
pub mod render_state;
pub mod systems;

mod serialize;

use std::ffi::CStr;
use std::os::raw::c_int;
use std::slice;
use std::str;
use std::sync::{Mutex, MutexGuard};

use lazy_static::lazy_static;

use commands::*;
use components::*;
use legion::prelude::*;
use render_state::RENDER_STATE;
use serialize::*;
use systems::camera::camera_system;
use systems::grid::render_grid_system;
use systems::move_camera::{move_camera_system, render_touch_system};
use systems::work_area::render_work_area_system;

struct ApplicationState {
    _universe: Universe,
    world: World,
}

#[repr(C)]
pub enum SerializeFormat {
    Json = 0,
}

static mut SCHEDULER_PROGRESS: Option<Schedule> = None;
static mut SCHEDULER_RENDER_PASS1: Option<Schedule> = None;
static mut SCHEDULER_RENDER_PASS2: Option<Schedule> = None;

lazy_static! {
    static ref APPLICATION_STATE: Mutex<Option<ApplicationState>> = Mutex::new(None);
}

#[no_mangle]
pub extern "C" fn init_world() {
    env_logger::init();
    debug_services::init();

    let universe = Universe::new();
    let mut world = universe.create_world();

    world.resources.insert(CommandsState::default());
    world.resources.insert(ViewPortSize::default());
    world.resources.insert(TouchState::default());
    world.resources.insert(LayersState::default());

    world.insert(
        (),
        vec![(
            GridComponent {
                color: Color::rgba(0.0, 0.0, 0.0, 0.1),
                step: 16,
            },
            Camera2DPositionListener::new(gapi::CAMERA_ORTHO),
        )],
    );

    world.insert(
        (),
        vec![(WorkAreaComponent {
            title: String::from("Hello world!"),
            color: Color::rgba(0.0, 0.0, 1.0, 0.3),
            size: Vec2f::new(640.0, 480.0),
        },)],
    );

    world.insert(
        (),
        vec![(
            Camera2D {
                id: gapi::CAMERA_ORTHO,
                pos: Vec2f::new(-320.0, -240.0),
            },
            CameraMovable2D::default(),
            TouchState::default(),
            Camera2DPositionListener::new(gapi::CAMERA_ORTHO),
        )],
    );

    unsafe {
        SCHEDULER_PROGRESS = Some(
            Schedule::builder()
                .add_system(camera_system())
                .add_system(move_camera_system())
                .flush()
                .build(),
        );

        SCHEDULER_RENDER_PASS1 = Some(Schedule::builder().flush().build());

        SCHEDULER_RENDER_PASS2 = Some(
            Schedule::builder()
                .add_system(render_touch_system())
                .add_system(render_grid_system())
                .add_system(render_work_area_system())
                .flush()
                .build(),
        );
    }

    let application_state = ApplicationState {
        _universe: universe,
        world,
    };

    APPLICATION_STATE
        .lock()
        .expect("failed to get application state")
        .replace(application_state);
}

fn get_application_state<'a>() -> MutexGuard<'a, Option<ApplicationState>> {
    APPLICATION_STATE
        .lock()
        .expect("failed to get application state")
}

#[no_mangle]
pub extern "C" fn frame_start() {
    debug_services::debug_frame_start();
}

#[no_mangle]
pub extern "C" fn frame_end() {
    match get_application_state().as_mut() {
        Some(state) => {
            let mut touch_state = state
                .world
                .resources
                .get_mut::<TouchState>()
                .expect("failed to get touch state");

            touch_state.touch = Touch::None;

            let mut layers_state = state
                .world
                .resources
                .get_mut::<LayersState>()
                .expect("failed to get layers state");

            layers_state.reset();

            debug_services::debug_frame_end();
        }
        None => {
            panic!("failed to get application state");
        }
    }
}

#[no_mangle]
pub extern "C" fn flush() {
    match get_application_state().as_mut() {
        Some(state) => {
            state_flush(state);
            delete_action_entities(&mut state.world);
        }
        None => {
            panic!("failed to get application state");
        }
    }
}

#[no_mangle]
pub extern "C" fn step() {
    match get_application_state().as_mut() {
        Some(state) => {
            {
                handle_request_commands(state);

                let touch_state = state
                    .world
                    .resources
                    .get::<TouchState>()
                    .expect("failed to get touch state");

                let mut layers_state = state
                    .world
                    .resources
                    .get_mut::<LayersState>()
                    .expect("failed to get layers state");

                debug_services::ui_step_pass(&touch_state, &mut layers_state);
            }

            unsafe {
                SCHEDULER_PROGRESS
                    .as_mut()
                    .expect("failed to get scheduler")
                    .execute(&mut state.world);
            }
        }
        None => {
            panic!("failed to get application state");
        }
    }
}

#[no_mangle]
pub extern "C" fn render_pass1() {
    match get_application_state().as_mut() {
        Some(state) => {
            unsafe {
                SCHEDULER_RENDER_PASS1
                    .as_mut()
                    .expect("failed to get scheduler")
                    .execute(&mut state.world);
            }

            let commands_state = &mut state
                .world
                .resources
                .get_mut::<CommandsState>()
                .expect("failed to get commands state");

            let view_port = state
                .world
                .resources
                .get::<ViewPortSize>()
                .expect("failed to get commands state");

            debug_services::render_pass(commands_state, &view_port);
        }
        None => {
            panic!("failed to get application state");
        }
    }
}

#[no_mangle]
pub extern "C" fn render_pass2() {
    match get_application_state().as_mut() {
        Some(state) => {
            unsafe {
                SCHEDULER_RENDER_PASS2
                    .as_mut()
                    .expect("failed to get scheduler")
                    .execute(&mut state.world);
            }

            let commands_state = &mut state
                .world
                .resources
                .get_mut::<CommandsState>()
                .expect("failed to get commands state");

            let view_port = state
                .world
                .resources
                .get::<ViewPortSize>()
                .expect("failed to get commands state");

            debug_services::render_pass(commands_state, &view_port);
        }
        None => {
            panic!("failed to get application state");
        }
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
        .expect("failed to get commands state")
        .request_commands
        .to_vec();

    for command in state {
        handle_request_command(application_state, &command);
    }
}

fn handle_request_command(application_state: &mut ApplicationState, command: &RequestCommand) {
    let world = &mut application_state.world;

    // TODO:
    let memory = &mut memory::get_memory_state().commands_data;

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
            }
            else {
                log::warn!("data have not been provided to SetViewportSize request command");
            }

            memory.clear();
        }
        RequestCommand {
            command_type: RequestCommandType::OnTouchStart,
            ..
        } => {
            if let Some(vec2f) = memory.vec2f_data.pop() {
                on_touch_start(world, vec2f);
            }
            else {
                log::warn!("data have not been provided to OnTouchStart request command");
            }

            memory.clear();
        }
        RequestCommand {
            command_type: RequestCommandType::OnTouchEnd,
            ..
        } => {
            if let Some(vec2f) = memory.vec2f_data.pop() {
                on_touch_end(world, vec2f);
            }
            else {
                log::warn!("data have not been provided to OnTouchEnd request command");
            }

            memory.clear();
        }
        RequestCommand {
            command_type: RequestCommandType::OnTouchMove,
            ..
        } => {
            if let Some(vec2f) = memory.vec2f_data.pop() {
                on_touch_move(world, vec2f);
            }
            else {
                log::warn!("data have not been provided to OnTouchMove request command");
            }

            memory.clear();
        }
    }
}

fn state_flush(application_state: &mut ApplicationState) {
    let mut state = application_state
        .world
        .resources
        .get_mut::<CommandsState>()
        .expect("failed to get commands state");

    memory::flush();

    state.render_commands.clear();
    state.exec_commands.clear();
    state.request_commands.clear();

    let render_state = &mut RENDER_STATE.lock().expect("failed to get render state");
    render_state.bump_cursor();
}

pub fn render_state_flush() {
    let render_state = &mut RENDER_STATE.lock().expect("failed to get render state");
    render_state.clear();
}

#[repr(C)]
#[derive(Debug)]
pub struct RawBuffer {
    data: *const u8,
    length: usize,
}

impl RawBuffer {
    pub fn from_string(str: &str) -> RawBuffer {
        RawBuffer {
            data: str.as_ptr(),
            length: str.len(),
        }
    }

    pub fn from_bytes(raw: &[u8]) -> RawBuffer {
        RawBuffer {
            data: raw.as_ptr(),
            length: raw.len(),
        }
    }

    pub fn data_to_string(&self) -> String {
        let data = unsafe { slice::from_raw_parts(self.data, self.length) };
        // TODO: Handle error
        let utf8_str = str::from_utf8(data).unwrap();
        String::from(utf8_str)
    }
}

impl Default for RawBuffer {
    fn default() -> Self {
        RawBuffer {
            data: "".as_ptr(),
            length: 0,
        }
    }
}

impl Clone for RawBuffer {
    fn clone(&self) -> Self {
        RawBuffer {
            data: self.data,
            length: self.length,
        }
    }
}

// TODO: doc
unsafe impl Send for RawBuffer {}

// TODO: doc
unsafe impl Sync for RawBuffer {}

#[repr(C)]
pub struct RenderCommands {
    pub items: *const RenderCommand,
    pub length: c_int,
}

#[repr(C)]
pub struct ExecutionCommands {
    pub items: *const ExecutionCommand,
    pub length: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn c_push_timed_block(
    name: *const i8,
    file_name: *const i8,
    line: u32,
) -> u64 {
    debug_services::profile::push_timed_block(
        CStr::from_ptr(name).to_str().unwrap(),
        CStr::from_ptr(file_name).to_str().unwrap(),
        line,
    )
}

#[no_mangle]
pub extern "C" fn c_drop_timed_block(id: u64) {
    debug_services::profile::drop_timed_block_by_id(id);
}

#[no_mangle]
pub extern "C" fn c_get_render_commands() -> RenderCommands {
    debug_services::timed_block!("c_get_render_commands");

    match get_application_state().as_mut() {
        Some(application_state) => {
            let state = application_state
                .world
                .resources
                .get::<CommandsState>()
                .expect("failed to get commands state");

            RenderCommands {
                items: state.render_commands.as_ptr(),
                length: state.render_commands.len() as c_int,
            }
        }
        None => {
            panic!("failed to get application state");
        }
    }
}

#[no_mangle]
pub extern "C" fn c_get_exec_commands() -> ExecutionCommands {
    debug_services::timed_block!("c_get_exec_commands");

    match get_application_state().as_mut() {
        Some(application_state) => {
            let state = application_state
                .world
                .resources
                .get::<CommandsState>()
                .expect("failed to get commands state");

            ExecutionCommands {
                items: state.exec_commands.as_ptr(),
                length: state.exec_commands.len() as c_int,
            }
        }
        None => {
            panic!("failed to get application state");
        }
    }
}

#[no_mangle]
pub extern "C" fn c_execute_command(data: RawBuffer) {
    debug_services::timed_block!("c_execute_command");

    println!("{}", data.data_to_string());
    // TODO: Send error to frontend
    debug_services::execute_command(data.data_to_string().as_str()).unwrap();
}

pub fn push_set_view_port_size_request_command(size: Vec2i) {
    debug_services::timed_block!("push_set_view_port_size");

    match get_application_state().as_mut() {
        Some(application_state) => {
            let mut state = application_state
                .world
                .resources
                .get_mut::<CommandsState>()
                .expect("failed to get commands state");

            push_request_command_data(
                &mut state,
                RequestCommandType::PushVec2i,
                CommandData::vec2i(size),
            );
            push_request_command(&mut state, RequestCommandType::SetViewportSize);
        }
        None => {
            panic!("failed to get application state");
        }
    }
}

// TODO: Use commands
pub fn push_text_size(size: Vec2f) {
    debug_services::timed_block!("push_text_size");

    let state = &mut RENDER_STATE.lock().expect("failed to get render state");
    state.push(size);
}

pub fn push_on_touch_start_request_command(point: Vec2f) {
    debug_services::timed_block!("push_set_view_port_size");

    match get_application_state().as_mut() {
        Some(application_state) => {
            let mut state = application_state
                .world
                .resources
                .get_mut::<CommandsState>()
                .expect("failed to get commands state");

            push_request_command_data(
                &mut state,
                RequestCommandType::PushVec2f,
                CommandData::vec2f(point),
            );
            push_request_command(&mut state, RequestCommandType::OnTouchStart);
        }
        None => {
            panic!("failed to get application state");
        }
    }
}

pub fn push_on_touch_end_request_command(point: Vec2f) {
    debug_services::timed_block!("push_set_view_port_size");

    match get_application_state().as_mut() {
        Some(application_state) => {
            let mut state = application_state
                .world
                .resources
                .get_mut::<CommandsState>()
                .expect("failed to get commands state");

            push_request_command_data(
                &mut state,
                RequestCommandType::PushVec2f,
                CommandData::vec2f(point),
            );
            push_request_command(&mut state, RequestCommandType::OnTouchEnd);
        }
        None => {
            panic!("failed to get application state");
        }
    }
}

pub fn push_on_touch_move_request_command(point: Vec2f) {
    debug_services::timed_block!("push_set_view_port_size");

    match get_application_state().as_mut() {
        Some(application_state) => {
            let mut state = application_state
                .world
                .resources
                .get_mut::<CommandsState>()
                .expect("failed to get commands state");

            push_request_command_data(
                &mut state,
                RequestCommandType::PushVec2f,
                CommandData::vec2f(point),
            );
            push_request_command(&mut state, RequestCommandType::OnTouchMove);
        }
        None => {
            panic!("failed to get application state");
        }
    }
}

/// # Safety
///
/// TODO: Doc
#[no_mangle]
pub unsafe extern "C" fn c_send_request_commands(data: *const RequestCommand, length: c_int) {
    debug_services::timed_block!("c_send_request_commands");

    match get_application_state().as_mut() {
        Some(application_state) => {
            let mut state = application_state
                .world
                .resources
                .get_mut::<CommandsState>()
                .expect("failed to get commands state");

            let requests = slice::from_raw_parts(data, length as usize);
            state.request_commands.extend_from_slice(requests);
        }
        None => {
            panic!("failed to get application state");
        }
    }
}

#[no_mangle]
pub extern "C" fn get_render_commands(format: SerializeFormat) -> RawBuffer {
    debug_services::timed_block!("get_render_commands");

    match get_application_state().as_mut() {
        Some(application_state) => {
            let state = application_state
                .world
                .resources
                .get::<CommandsState>()
                .expect("failed to get commands state");

            let memory = &mut memory::get_memory_state();

            match format {
                SerializeFormat::Json => {
                    serialize_json_render_commands(memory, &state.render_commands)
                }
            }
        }
        None => {
            panic!("failed to get application state");
        }
    }
}

#[no_mangle]
pub extern "C" fn get_exec_commands_ser(format: SerializeFormat) -> RawBuffer {
    debug_services::timed_block!("get_exec_commands");

    match get_application_state().as_mut() {
        Some(application_state) => {
            let state = application_state
                .world
                .resources
                .get::<CommandsState>()
                .expect("failed to get commands state");

            let memory = &mut memory::get_memory_state();

            match format {
                SerializeFormat::Json => serialize_json_exec_commands(memory, &state.exec_commands),
            }
        }
        None => {
            panic!("failed to get application state");
        }
    }
}

#[no_mangle]
pub extern "C" fn send_request_commands(format: SerializeFormat, data: RawBuffer) {
    debug_services::timed_block!("send_request_commands");

    match get_application_state().as_mut() {
        Some(application_state) => {
            let mut state = application_state
                .world
                .resources
                .get_mut::<CommandsState>()
                .expect("failed to get application state");

            let requests = match format {
                SerializeFormat::Json => deserialize_json_request_commands(data),
            };

            match requests {
                Ok(data) => {
                    state.request_commands.extend(data.into_iter());
                }
                Err(err) => {
                    log::warn!("failed to deserialize request commands: {}", err);
                }
            }
        }
        None => {
            panic!("failed to get application state");
        }
    }
}

fn set_view_port_size(world: &mut World, width: i32, height: i32) {
    let mut view_port_size = world
        .resources
        .get_mut::<ViewPortSize>()
        .expect("falied to get viewport");

    view_port_size.width = width;
    view_port_size.height = height;
}

fn on_touch_start(world: &mut World, touch: Vec2f) {
    {
        let mut touch_state = world
            .resources
            .get_mut::<TouchState>()
            .expect("falied to get viewport");

        touch_state.touch = Touch::Start;
        touch_state.touch_start = touch;
        touch_state.touch_current = touch;
    }

    let query = <(Write<TouchState>,)>::query();

    for (mut touch_state,) in query.iter(world) {
        touch_state.touch = Touch::Start;
        touch_state.touch_start = touch;
        touch_state.touch_current = touch;
    }
}

fn on_touch_end(world: &mut World, touch: Vec2f) {
    {
        let mut touch_state = world
            .resources
            .get_mut::<TouchState>()
            .expect("falied to get viewport");

        touch_state.touch = Touch::End;
        touch_state.touch_current = touch;
    }

    let query = <(Write<TouchState>,)>::query();

    for (mut touch_state,) in query.iter(world) {
        if touch_state.touch == Touch::Start || touch_state.touch == Touch::Move {
            touch_state.touch = Touch::End;
            touch_state.touch_current = touch;
        }
    }
}

fn on_touch_move(world: &mut World, touch: Vec2f) {
    {
        let mut touch_state = world
            .resources
            .get_mut::<TouchState>()
            .expect("falied to get viewport");

        touch_state.pos = touch;
        touch_state.touch = Touch::Move;
        touch_state.touch_current = touch;
    }

    let query = <(Write<TouchState>,)>::query();

    for (mut touch_state,) in query.iter(world) {
        touch_state.pos = touch;

        if touch_state.touch == Touch::Start || touch_state.touch == Touch::Move {
            touch_state.touch = Touch::Move;
            touch_state.touch_current = touch;
        }
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         crate::init_world();
//         let json = "[{\"SetViewportSize\": {\"width: 100, \"height\": 200}}]".as_bytes();
//         let data = crate::RawBuffer {
//             data: json.as_ptr(),
//             length: json.len(),
//         };
//         crate::send_request_commands(crate::SerializeFormat::Json, data);
//         // crate::step();
//         // let mut commands_state = crate::commands::CommandsState::default();
//         // crate::render::gapi::push_color_shader(&mut commands_state);
//         let data = crate::get_render_commands(crate::SerializeFormat::Json);
//         println!("{:?}", data);
//         log::error!("Commencing yak shaving");
//         panic!(":(");
//         // crate::get_render_commands();
//         // crate::step();
//         // let data = crate::get_render_commands();

//         // dbg!(data);

//         assert_eq!(2 + 2, 4);
//     }
// }
