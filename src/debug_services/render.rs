use std::sync::MutexGuard;

use crate::commands::{Color, CommandsState, Vec2f};
use crate::components::ViewPortSize;
use crate::debug_services::profile;
use crate::debug_services::state::DebugState;
use crate::gapi;

pub fn profile(
    debug_state: &mut MutexGuard<DebugState>,
    commands_state: &mut CommandsState,
    view_port: &ViewPortSize,
) {
    let mut offset_y: f32 = 35.0;
    let mut offset_x: f32 = 10.0;
    let line_size = 18.0;

    let profile_state = &debug_state.profile;
    let snapshot = &profile_state.performance_counter_log[profile_state.snapshot_counter].records;

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
        // let text = format!("{:?}", cycle.thread_id);
        // gapi::push_string_xy(commands_state, &text, offset_x, offset_y);
        // offset_x += 100.0;

        let text = format!("{:.2}%", cycle.percent);
        gapi::push_string_xy(commands_state, &text, offset_x, offset_y);
        offset_x += 100.0;

        gapi::push_string_xy(commands_state, &cycle.name, offset_x, offset_y);
        offset_x += 250.0;

        let text = format!("{}:{}", cycle.file_name, cycle.line);
        gapi::push_string_xy(commands_state, &text, offset_x, offset_y);
        offset_x += 250.0;

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

pub fn frame_time(debug_state: &mut MutexGuard<DebugState>, commands_state: &mut CommandsState) {
    let text = format!(
        "{:.2} ms",
        debug_state.profile.frame_elapsed.as_nanos() as f64 / 1_000_000.0
    );
    gapi::push_string_xy(commands_state, &text, 5.0, 5.0);

    gapi::push_text_shader(commands_state);
    gapi::push_color(commands_state, Color::rgb(0.0, 0.0, 0.0));
    gapi::set_color_uniform(commands_state);
    gapi::draw_text(commands_state);
}

pub fn frames_log(debug_state: &mut MutexGuard<DebugState>, commands_state: &mut CommandsState) {
    let mut offset_x = 10.0;
    let offset_y = 500.0;
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
            (bar_width + bar_space) * profile::PERFORMANCE_COUNTER_LOG_SIZE as f32 - bar_space
                + border_width * 2.0,
            bar_height + border_width * 2.0,
        ),
    );
    gapi::draw_quads(commands_state);

    let current_snapshot = debug_state.profile.snapshot_counter;

    for i in 0..profile::PERFORMANCE_COUNTER_LOG_SIZE {
        if current_snapshot == i {
            gapi::push_color(commands_state, Color::rgb(0.0, 1.0, 0.0));
            gapi::set_color_uniform(commands_state);
        }
        else {
            gapi::push_color(commands_state, Color::rgb(0.2, 0.2, 0.2));
            gapi::set_color_uniform(commands_state);
        }

        gapi::push_vec2f(commands_state, Vec2f::new(offset_x, offset_y));
        gapi::push_vec2f(commands_state, Vec2f::new(bar_width, bar_height));
        gapi::draw_quads(commands_state);

        offset_x += bar_width + bar_space;
    }
}
