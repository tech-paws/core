use lazy_static::lazy_static;

use std::collections::HashMap;
use std::format;
use std::sync::{Mutex, MutexGuard};
use std::thread;
use std::time::{Duration, Instant};

use crate::commands::{Color, CommandsState, Vec2f};
use crate::components::ViewPortSize;
use crate::gapi;

lazy_static! {
    static ref DEBUG_STATE: Mutex<DebugState> = Mutex::new(DebugState::default());
}

const PERFORMANCE_RECORDS_CAPACITY: usize = 512;
const PERFORMANCE_COUNTER_LOG_SIZE: usize = 120; // max 120 entires
                                                 // const SNAPSHOT_INTERVAL: usize = 10; // every 5 frames

pub struct FrameStaticstic {
    start_frame: Duration,
    s1_time: Duration,
    s2_time: Duration,
    end_frame: Duration,
}

#[derive(Clone)]
pub struct PerformanceCounterState {
    pub records: Vec<ClocsDebugRecord>,
}

#[derive(Clone, Default, Debug)]
pub struct PerformanceCounterStatisticsRecord {
    pub name: &'static str,
    pub file_name: &'static str,
    pub line: u32,
    pub sum_elapsed: Duration,
    pub sum_hits: u32,
    pub sum_hits_over_elapsed: u128,
    pub hits: u32,
    pub percent: f32,
    pub thread_id: String,
}

#[derive(Clone, Debug)]
pub struct PerformanceCounterStatistics {
    pub records: Vec<PerformanceCounterStatisticsRecord>,
}

pub struct DebugState {
    _global_pause: bool,
    snapshot_interval: usize,
    frame_timer: Instant,
    frame_elapsed: Duration,
    frame_counter: usize,
    snapshot_counter: usize,
    performance_counter_states: Vec<PerformanceCounterState>,
    performance_counter_log: Vec<PerformanceCounterStatistics>,
}

impl Default for PerformanceCounterStatistics {
    fn default() -> Self {
        PerformanceCounterStatistics {
            records: Vec::with_capacity(PERFORMANCE_RECORDS_CAPACITY),
        }
    }
}

impl Default for PerformanceCounterState {
    fn default() -> Self {
        PerformanceCounterState {
            records: Vec::with_capacity(PERFORMANCE_RECORDS_CAPACITY),
        }
    }
}

impl Default for DebugState {
    fn default() -> Self {
        let snapshot_interval = 1;

        DebugState {
            _global_pause: false,
            frame_counter: 0,
            snapshot_counter: 0,
            snapshot_interval: snapshot_interval,
            performance_counter_states: vec![PerformanceCounterState::default(); snapshot_interval],
            performance_counter_log: vec![
                PerformanceCounterStatistics::default();
                PERFORMANCE_COUNTER_LOG_SIZE
            ],
            frame_timer: Instant::now(),
            frame_elapsed: Duration::from_nanos(0),
        }
    }
}

#[macro_export]
macro_rules! timed_block {
    ($name:expr) => {
        crate::debug_services::api::TimedBlock::new($name, file!(), line!())
    };
}

#[derive(Debug, Clone)]
pub struct ClocsDebugRecord {
    pub name: &'static str,
    pub file_name: &'static str,
    pub line: u32,
    pub elapsed: Duration,
    pub hits: u32,
    pub thread_id: thread::ThreadId,
}

impl Default for ClocsDebugRecord {
    fn default() -> Self {
        ClocsDebugRecord {
            name: "",
            file_name: "",
            line: 0,
            elapsed: Duration::from_nanos(0),
            hits: 0,
            thread_id: thread::current().id(),
        }
    }
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

        let frame_counter = debug_state.frame_counter;
        let records = &mut debug_state.performance_counter_states[frame_counter].records;

        // NOTE(govorun): Right now this method is faster than Map
        for (i, c) in records.iter().enumerate() {
            if c.name == self.name && c.file_name == self.file_name && c.line == self.line {
                hits += c.hits;
                elapsed += c.elapsed;
                to_modify = true;
                modify_idx = i;
            }
        }

        if to_modify {
            records[modify_idx] = ClocsDebugRecord {
                name: self.name,
                file_name: self.file_name,
                line: self.line,
                thread_id: thread::current().id(),
                elapsed,
                hits,
            };
        } else {
            records.push(ClocsDebugRecord {
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

pub fn debug_frame_end() {
    let debug_state: &mut MutexGuard<DebugState> =
        &mut DEBUG_STATE.lock().expect("failed to get debug state");

    debug_state.frame_counter += 1;
    debug_state.frame_elapsed = debug_state.frame_timer.elapsed();

    let snapshot_interval = debug_state.snapshot_interval;

    if debug_state.frame_counter >= snapshot_interval {
        take_snapshot(debug_state);
        debug_state.frame_counter = 0;

        for i in 0..snapshot_interval {
            debug_state.performance_counter_states[i] = PerformanceCounterState::default();
        }
    }
}

pub fn debug_frame_start() {
    let debug_state: &mut MutexGuard<DebugState> =
        &mut DEBUG_STATE.lock().expect("failed to get debug state");

    debug_state.frame_timer = Instant::now();
}

fn take_snapshot(debug_state: &mut MutexGuard<DebugState>) {
    debug_state.snapshot_counter += 1;

    if debug_state.snapshot_counter >= PERFORMANCE_COUNTER_LOG_SIZE {
        debug_state.snapshot_counter = 0;
    }

    let mut statistics: HashMap<String, PerformanceCounterStatisticsRecord> = HashMap::new();

    for state in debug_state.performance_counter_states.iter() {
        for record in state.records.iter() {
            let key = String::from(record.name) + record.file_name + &record.line.to_string();
            let element = statistics.entry(key).or_default();

            element.name = record.name;
            element.file_name = record.file_name;
            element.line = record.line;
            element.sum_elapsed += record.elapsed;
            element.sum_hits += record.hits;
            element.sum_hits_over_elapsed += record.elapsed.as_nanos() / record.hits as u128;
            element.hits += 1;
            element.thread_id = format!("{:?}", record.thread_id);
        }
    }

    let total_elapsed: u128 = statistics
        .values()
        .map(|record| record.sum_elapsed.as_nanos())
        .sum();

    for record in statistics.values_mut() {
        record.percent =
            (record.sum_elapsed.as_nanos() as f64 / total_elapsed as f64) as f32 * 100.0;
    }

    let mut records: Vec<PerformanceCounterStatisticsRecord> =
        statistics.values().cloned().collect();
    records.sort_by(|a, b| b.percent.partial_cmp(&a.percent).unwrap());

    let counter = debug_state.snapshot_counter;
    let snapshot = &mut debug_state.performance_counter_log[counter].records;

    snapshot.clear();
    snapshot.append(&mut records);
}

pub fn update_snapshot_interval(debug_state: &mut MutexGuard<DebugState>, new_interval: usize) {
    debug_state.snapshot_interval = new_interval;
    debug_state
        .performance_counter_states
        .resize(new_interval, PerformanceCounterState::default());
}

pub fn step(commands_state: &mut CommandsState, view_port: &ViewPortSize) {
    let debug_state = &mut DEBUG_STATE.lock().expect("failed to get debug state");

    render_profile(debug_state, commands_state, view_port);
    render_frame_time(debug_state, commands_state);
    render_frames_log(debug_state, commands_state);
}

fn render_profile(
    debug_state: &mut MutexGuard<DebugState>,
    commands_state: &mut CommandsState,
    view_port: &ViewPortSize,
) {
    let mut offset_y: f32 = 35.0;
    let mut offset_x: f32 = 10.0;
    let line_size = 18.0;

    let snapshot = &debug_state.performance_counter_log[debug_state.snapshot_counter].records;

    // Background
    gapi::push_color_shader(commands_state);
    gapi::push_color(commands_state, Color::rgba(0.0, 0.0, 0.0, 0.5));
    gapi::set_color_uniform(commands_state);

    gapi::push_vec2f(commands_state, Vec2f::new(0.0, offset_y - 10.0));
    gapi::push_vec2f(
        commands_state,
        Vec2f::new(
            view_port.width as f32,
            line_size * snapshot.len() as f32 + 20.0,
        ),
    );
    gapi::draw_quads(commands_state);

    for cycle in snapshot.iter() {
        let text = format!("{}", cycle.thread_id);
        gapi::push_string_xy(commands_state, &text, offset_x, offset_y);
        offset_x += 100.0;

        let text = format!("{:.2}%", cycle.percent);
        gapi::push_string_xy(commands_state, &text, offset_x, offset_y);
        offset_x += 100.0;

        gapi::push_string_xy(commands_state, &cycle.name, offset_x, offset_y);
        offset_x += 200.0;

        let text = format!("{}:{}", cycle.file_name, cycle.line);
        gapi::push_string_xy(commands_state, &text, offset_x, offset_y);
        offset_x += 300.0;

        let text = format!("{}h", cycle.sum_hits / cycle.hits);
        gapi::push_string_xy(commands_state, &text, offset_x, offset_y);
        offset_x += 50.0;

        let text = format!("{:?}", cycle.sum_elapsed / cycle.hits);
        gapi::push_string_xy(commands_state, &text, offset_x, offset_y);
        offset_x += 100.0;

        let text = format!(
            "{:?} ns/h",
            cycle.sum_hits_over_elapsed / cycle.hits as u128
        );
        gapi::push_string_xy(commands_state, &text, offset_x, offset_y);

        offset_y += line_size;
        offset_x = 10.0;
    }

    // Text
    gapi::push_text_shader(commands_state);
    gapi::push_color(commands_state, Color::rgb(1.0, 1.0, 1.0));
    gapi::set_color_uniform(commands_state);
    gapi::draw_text(commands_state);
}

fn render_frame_time(debug_state: &mut MutexGuard<DebugState>, commands_state: &mut CommandsState) {
    let text = format!(
        "{:.2} ms",
        debug_state.frame_elapsed.as_nanos() as f64 / 1_000_000.0
    );
    gapi::push_string_xy(commands_state, &text, 5.0, 5.0);

    gapi::push_text_shader(commands_state);
    gapi::push_color(commands_state, Color::rgb(0.0, 0.0, 0.0));
    gapi::set_color_uniform(commands_state);
    gapi::draw_text(commands_state);
}

fn render_frames_log(debug_state: &mut MutexGuard<DebugState>, commands_state: &mut CommandsState) {
    let mut offset_x = 10.0;
    let offset_y = 250.0;
    let bar_width = 3.0;
    let bar_height = 25.0;
    let bar_space = 2.0;
    let border_width = 2.0;

    // Background
    gapi::push_color_shader(commands_state);

    gapi::push_vec2f(
        commands_state,
        Vec2f::new(offset_x - border_width, offset_y - border_width),
    );

    gapi::push_color(commands_state, Color::rgb(0.5, 0.5, 0.5));
    gapi::set_color_uniform(commands_state);

    gapi::push_vec2f(
        commands_state,
        Vec2f::new(
            (bar_width + bar_space) * PERFORMANCE_COUNTER_LOG_SIZE as f32 - bar_space
                + border_width * 2.0,
            bar_height + border_width * 2.0,
        ),
    );
    gapi::draw_quads(commands_state);

    let current_snapshot = debug_state.snapshot_counter;

    for i in 0..PERFORMANCE_COUNTER_LOG_SIZE {
        if current_snapshot == i {
            gapi::push_color(commands_state, Color::rgb(0.0, 1.0, 0.0));
            gapi::set_color_uniform(commands_state);
        } else {
            gapi::push_color(commands_state, Color::rgb(0.2, 0.2, 0.2));
            gapi::set_color_uniform(commands_state);
        }

        gapi::push_vec2f(commands_state, Vec2f::new(offset_x, offset_y));
        gapi::push_vec2f(commands_state, Vec2f::new(bar_width, bar_height));
        gapi::draw_quads(commands_state);

        offset_x += bar_width + bar_space;
    }
}
