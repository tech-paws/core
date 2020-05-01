mod render;

use render::components::{RenderCommand, RenderState};
use render::system::DemoRenderSystem;
use specs::{Dispatcher, DispatcherBuilder, World, WorldExt};

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
    world.insert(RenderState::default());

    let dispatcher = DispatcherBuilder::new()
        .with(DemoRenderSystem, "demo_render", &[])
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
        flush();

        state.dispatcher.dispatch(&mut state.world);
        state.world.maintain();
    }
}

unsafe fn flush() {
    let application_state = get_application_state();
    let mut state = application_state.world.write_resource::<RenderState>();

    application_state.memory.serialize_buffer.clear();
    state.render_commands.clear();
}

#[repr(C)]
#[derive(Debug)]
pub struct RawBuffer {
    data: *const u8,
    length: usize,
}

#[no_mangle]
pub extern "C" fn get_render_commands() -> RawBuffer {
    unsafe {
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
