mod render;

use legion::prelude::*;
use render::camera_system::camera_system;
use render::components::*;
use render::grid_system::grid_system;
use render::move_camera_system::move_camera_system;
use render::work_area::work_area_system;

struct Memory {
    serialize_buffer: Vec<u8>,
}

struct ApplicationState {
    universe: Universe,
    world: World,
    memory: Memory,
    schedule: Schedule,
}

#[repr(C)]
pub enum SerializeFormat {
    Json = 0,
}

static mut APPLICATION_STATE: Option<ApplicationState> = None;

#[no_mangle]
pub extern "C" fn init_world() {
    let universe = Universe::new();
    let mut world = universe.create_world();

    world.resources.insert(RenderState::default());
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
        vec![(Camera2D {
            tag: 0,
            pos: Pos2f {
                x: -320.0,
                y: -240.0,
            },
        },)],
    );

    let memory = Memory {
        serialize_buffer: Vec::with_capacity(1_000_000_000),
    };

    let schedule = Schedule::builder()
        .add_system(grid_system())
        .add_system(camera_system())
        .add_system(work_area_system())
        .add_system(move_camera_system())
        .flush()
        .build();

    unsafe {
        APPLICATION_STATE = Some(ApplicationState {
            universe,
            world,
            memory,
            schedule,
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
        handle_action_commands(&mut state.world);
        flush();

        state.schedule.execute(&mut state.world);
        delete_action_entities(&mut state.world);
    }
}

fn delete_action_entities(world: &mut World) {
    <(Read<Actions>,)>::query()
        // .filter(tag::<Actions>())
        .iter_entities(world)
        .map(|entity| entity.0)
        .collect::<Vec<Entity>>()
        .iter()
        .for_each(|entity| {
            world.delete(*entity);
        });
}

unsafe fn handle_action_commands(world: &mut World) {
    let state = world.resources.get::<RenderState>().unwrap();

    for command in &state.action_commands {
        handle_action_command(command);
    }
}

unsafe fn handle_action_command(action_command: &ActionCommand) {
    let state = get_application_state();
    let world = &mut state.world;

    match action_command {
        ActionCommand::OnTouchStart { x, y } => {
            world.insert(
                (OnCameraTouchStart,),
                vec![(Actions, Pos2f { x: *x, y: *y })],
            );
        }
        ActionCommand::OnTouchEnd { x, y } => {
            world.insert(
                (OnCameraTouchStart,),
                vec![(Actions, Pos2f { x: *x, y: *y })],
            );
        }
        ActionCommand::OnTouchMove { x, y } => {
            world.insert(
                (OnCameraTouchStart,),
                vec![(Actions, Pos2f { x: *x, y: *y })],
            );
        }
    }
}

unsafe fn flush() {
    let application_state = get_application_state();
    let mut state = application_state
        .world
        .resources
        .get_mut::<RenderState>()
        .unwrap();

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
    let state = application_state
        .world
        .resources
        .get::<RenderState>()
        .unwrap();

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
    let state = application_state
        .world
        .resources
        .get::<RenderState>()
        .unwrap();

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
        crate::step();
        crate::get_render_commands();
        crate::get_render_commands();
        crate::step();
        let data = crate::get_render_commands();

        dbg!(data);

        assert_eq!(2 + 2, 4);
    }
}
