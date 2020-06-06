use lazy_static::lazy_static;

use std::format;
use std::sync::{Mutex, MutexGuard};
use std::thread;
use std::time::{Duration, Instant};

use crate::commands::{Color, CommandsState, Vec2f};
use crate::gapi;

lazy_static! {
    static ref DEBUG_STATE: Mutex<DebugState> = Mutex::new(DebugState::default());
}

const DEBUG_LOG_MAX: usize = 100;

#[derive(Debug)]
pub struct DebugState {
    pub cycles: Vec<CycleDebugRecord>,
}

impl Default for DebugState {
    fn default() -> Self {
        DebugState {
            cycles: Vec::with_capacity(DEBUG_LOG_MAX),
        }
    }
}

#[macro_export]
macro_rules! timed_block {
    ($name:expr) => {
        crate::debug_services::api::TimedBlock::new($name, file!(), line!())
    };
}

#[derive(Debug)]
pub struct CycleDebugRecord {
    pub name: &'static str,
    pub file_name: &'static str,
    pub line: u32,
    pub elapsed: Duration,
    pub thread_id: thread::ThreadId,
    pub hits: u32,
}

pub struct TimedBlock {
    pub thread_id: thread::ThreadId,
    pub name: &'static str,
    pub file_name: &'static str,
    pub line: u32,
    pub timer: Instant,
}

impl TimedBlock {
    pub fn new(name: &'static str, file_name: &'static str, line: u32) -> TimedBlock {
        TimedBlock {
            name,
            file_name,
            line,
            thread_id: thread::current().id(),
            timer: Instant::now(),
        }
    }
}

impl Drop for TimedBlock {
    fn drop(&mut self) {
        let debug_state: &mut MutexGuard<DebugState> =
            &mut DEBUG_STATE.lock().expect("failed to get debug state");

        let mut hits = 1;
        let mut elapsed = self.timer.elapsed();
        let mut to_modify = false;
        let mut modify_idx: usize = 0;

        for (i, c) in debug_state.cycles.iter().enumerate() {
            if c.name == self.name
                && c.file_name == self.file_name
                && c.line == self.line
                && c.thread_id == self.thread_id
            {
                hits += c.hits;
                elapsed += c.elapsed;
                to_modify = true;
                modify_idx = i;
            }
        }

        if to_modify {
            debug_state.cycles[modify_idx] = CycleDebugRecord {
                name: self.name,
                file_name: self.file_name,
                line: self.line,
                thread_id: thread::current().id(),
                elapsed,
                hits,
            };
        } else {
            debug_state.cycles.push(CycleDebugRecord {
                name: self.name,
                file_name: self.file_name,
                line: self.line,
                thread_id: thread::current().id(),
                elapsed,
                hits,
            });
        }
    }
}

pub fn step(commands_state: &mut CommandsState) {
    let debug_state: &mut MutexGuard<DebugState> =
        &mut DEBUG_STATE.lock().expect("failed to get debug state");

    let mut offset_y: f32 = 10.0;
    let mut offset_x: f32 = 10.0;
    let line_size = 18.0;

    gapi::push_color(commands_state, Color::rgba(0.0, 0.0, 0.0, 0.5));
    gapi::push_quad(
        commands_state,
        Vec2f::new(0.0, 0.0),
        Vec2f::new(600.0, line_size * debug_state.cycles.len() as f32),
    );

    for cycle in debug_state.cycles.iter() {
        let text = format!("{:?}", cycle.thread_id);
        gapi::push_string_xy(commands_state, &text, offset_x, offset_y);
        offset_x += 100.0;

        gapi::push_string_xy(commands_state, &cycle.name, offset_x, offset_y);
        offset_x += 200.0;

        let text = format!("{}:{}", cycle.file_name, cycle.line);
        gapi::push_string_xy(commands_state, &text, offset_x, offset_y);
        offset_x += 300.0;

        let text = format!("{}h", cycle.hits);
        gapi::push_string_xy(commands_state, &text, offset_x, offset_y);
        offset_x += 50.0;

        let text = format!("{:?}", cycle.elapsed);
        gapi::push_string_xy(commands_state, &text, offset_x, offset_y);
        offset_x += 100.0;

        let text = format!("{:?} ns/h", cycle.elapsed.as_nanos() / cycle.hits as u128);
        gapi::push_string_xy(commands_state, &text, offset_x, offset_y);

        offset_y += line_size;
        offset_x = 10.0;
    }

    gapi::push_color_shader(commands_state);
    gapi::draw_quads(commands_state);
    gapi::push_text_shader(commands_state);
    gapi::draw_text(commands_state);

    debug_state.cycles.clear();
}
