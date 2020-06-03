use crate::commands;
use bumpalo::Bump;
use lazy_static::lazy_static;
use std::sync::{Mutex, MutexGuard};

#[derive(Default)]
pub struct CommandsDataMemory {
    pub vec2f_data: Vec<commands::Vec2f>,
    pub vec2i_data: Vec<commands::Vec2i>,
}

impl CommandsDataMemory {
    pub fn clear(&mut self) {
        self.vec2f_data.clear();
        self.vec2i_data.clear();
    }
}

pub struct MemoryState {
    pub serialize_buffer: Bump,
    pub frame_memory: Bump,
    pub commands_data: CommandsDataMemory,
}

impl Default for MemoryState {
    fn default() -> MemoryState {
        MemoryState {
            serialize_buffer: Bump::new(),
            frame_memory: Bump::new(),
            commands_data: CommandsDataMemory::default(),
        }
    }
}

impl MemoryState {
    pub fn flush(&mut self) {
        self.serialize_buffer.reset();
        self.commands_data.clear();
    }
}

lazy_static! {
    static ref MEMORY_STATE: Mutex<MemoryState> = Mutex::new(MemoryState::default());
}

// TODO: consider thread local
pub fn get_memory_state<'a>() -> MutexGuard<'a, MemoryState> {
    MEMORY_STATE.lock().expect("failed to get memory state")
}

pub fn flush() {
    let memory = &mut MEMORY_STATE.lock().expect("failed to get memory state");
    memory.serialize_buffer.reset();
    memory.commands_data.clear();
}

pub fn frame_alloc_vec<T>(memory: &mut MemoryState) -> bumpalo::collections::Vec<'_, T> {
    bumpalo::collections::Vec::<T>::new_in(&memory.frame_memory)
}
