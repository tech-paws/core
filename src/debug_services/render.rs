use std::sync::MutexGuard;

use crate::commands::{Color, CommandsState, Vec2f};
use crate::components::ViewPortSize;
use crate::debug_services::profile;
use crate::debug_services::state::*;
use crate::gapi;

struct Context<'a, 'b> {
    pos: Vec2f,
    debug_state: &'a mut MutexGuard<'b, DebugState>,
    commands_state: &'a mut CommandsState,
    view_port: &'a ViewPortSize,
}

struct Context2 {
    pos: Vec2f,
}

pub fn render(
    debug_state: &mut MutexGuard<DebugState>,
    commands_state: &mut CommandsState,
    view_port: &ViewPortSize,
) {
    // let mut context = Context {
    //     pos: Vec2f::new(0.0, 0.0),
    //     debug_state,
    //     commands_state,
    //     view_port,
    // };
    let mut context = Context2 {
        pos: Vec2f::new(10.0, 10.0),
    };

    render_group_variables(&mut context, commands_state, &debug_state.variables);
    // let size = render_frame_time(&mut context);

    // context.pos.y += size.y;
    // context.pos.x = 5.;

    // let size = render_frames_slider(&mut context);

    // context.pos.y += size.y;
    // context.pos.x = 0.0;

    // render_profile(&mut context);
}

fn render_group_variables(
    context: &mut Context2,
    commands_state: &mut CommandsState,
    variable: &GroupVariable,
) {
    let text = format!(
        "{} {}",
        if variable.is_expanded { "-" } else { "+" },
        variable.name
    );
    gapi::push_string_vec2f(commands_state, &text, context.pos);

    gapi::push_text_shader(commands_state);
    gapi::push_color(commands_state, Color::rgb(0.0, 0.0, 0.0));
    gapi::set_color_uniform(commands_state);
    gapi::draw_text(commands_state);

    context.pos.x += 20.;
    context.pos.y += 14.;

    for v in variable.variables.iter() {
        match &v {
            DebugVariable::Bool(variable) => {
                render_bool_variable(context, commands_state, variable);
            }
            DebugVariable::Group(group) => {
                render_group_variables(context, commands_state, group);
            }
        };
    }

    context.pos.x -= 20.;
}

fn render_bool_variable(
    context: &mut Context2,
    commands_state: &mut CommandsState,
    variable: &BoolVariable,
) {
    let text = format!("{}: {}", variable.name, variable.value);
    gapi::push_string_vec2f(commands_state, &text, context.pos);

    gapi::push_text_shader(commands_state);
    gapi::push_color(commands_state, Color::rgb(0.0, 0.0, 0.0));
    gapi::set_color_uniform(commands_state);
    gapi::draw_text(commands_state);

    context.pos.y += 14.;
}

fn render_profile(context: &mut Context) -> Vec2f {
    let mut pos = context.pos;

    let line_size = 18.0;
    let profile_state = &context.debug_state.profile;

    let snapshot = &profile_state.performance_counter_log[profile_state.snapshot_counter].records;

    let size = Vec2f::new(
        context.view_port.width as f32,
        line_size * snapshot.len() as f32 + 20.0,
    );

    // Background
    gapi::push_color_shader(context.commands_state);
    gapi::push_color(context.commands_state, Color::rgba(0.0, 0.0, 0.0, 0.5));
    gapi::set_color_uniform(context.commands_state);

    gapi::push_vec2f(context.commands_state, Vec2f::new(pos.x, pos.y));
    gapi::push_vec2f(context.commands_state, size);
    gapi::draw_quads(context.commands_state);

    pos.y += 10.0;

    for cycle in snapshot.iter() {
        pos.x = context.pos.x + 10.0;

        let text = format!("{:?}", cycle.thread_id);
        gapi::push_string_xy(context.commands_state, &text, pos.x, pos.y);
        pos.x += 100.0;

        let text = format!("{:.2}%", cycle.percent);
        gapi::push_string_xy(context.commands_state, &text, pos.x, pos.y);
        pos.x += 100.0;

        gapi::push_string_xy(context.commands_state, &cycle.name, pos.x, pos.y);
        pos.x += 250.0;

        // let text = format!("{}:{}", cycle.file_name, cycle.line);
        // gapi::push_string_xy(context.commands_state, &text, pos.x, pos.y);
        // pos.x += 250.0;

        let text = format!("{}h", cycle.sum_hits / cycle.hits);
        gapi::push_string_xy(context.commands_state, &text, pos.x, pos.y);
        pos.x += 50.0;

        let text = format!("{:?}", cycle.sum_elapsed / cycle.hits);
        gapi::push_string_xy(context.commands_state, &text, pos.x, pos.y);
        pos.x += 100.0;

        let text = format!(
            "{:?} ns/h",
            cycle.sum_hits_over_elapsed / cycle.hits as u128
        );
        gapi::push_string_xy(context.commands_state, &text, pos.x, pos.y);

        pos.y += line_size;
    }

    // Text
    gapi::push_text_shader(context.commands_state);
    gapi::push_color(context.commands_state, Color::rgb(1.0, 1.0, 1.0));
    gapi::set_color_uniform(context.commands_state);
    gapi::draw_text(context.commands_state);

    size
}

fn render_frames_slider(context: &mut Context) -> Vec2f {
    let mut offset_x = context.pos.x;
    let offset_y = context.pos.y;
    let bar_width = 3.0;
    let bar_height = 25.0;
    let bar_space = 2.0;
    let border_width = 2.0;

    // Background
    gapi::push_color_shader(context.commands_state);

    gapi::push_vec2f(
        context.commands_state,
        Vec2f::new(offset_x - border_width, offset_y - border_width),
    );

    gapi::push_color(context.commands_state, Color::rgb(0.5, 0.5, 0.5));
    gapi::set_color_uniform(context.commands_state);

    let width = (bar_width + bar_space) * profile::PERFORMANCE_COUNTER_LOG_SIZE as f32 - bar_space
        + border_width * 2.0;
    let height = bar_height + border_width * 2.0;

    gapi::push_vec2f(context.commands_state, Vec2f::new(width, height));
    gapi::draw_quads(context.commands_state);

    let current_snapshot = context.debug_state.profile.snapshot_counter;

    for i in 0..profile::PERFORMANCE_COUNTER_LOG_SIZE {
        if current_snapshot == i {
            gapi::push_color(context.commands_state, Color::rgb(0.0, 1.0, 0.0));
            gapi::set_color_uniform(context.commands_state);
        }
        else {
            gapi::push_color(context.commands_state, Color::rgb(0.2, 0.2, 0.2));
            gapi::set_color_uniform(context.commands_state);
        }

        gapi::push_vec2f(context.commands_state, Vec2f::new(offset_x, offset_y));
        gapi::push_vec2f(context.commands_state, Vec2f::new(bar_width, bar_height));
        gapi::draw_quads(context.commands_state);

        offset_x += bar_width + bar_space;
    }

    Vec2f::new(width, height)
}

fn render_frame_time(context: &mut Context) -> Vec2f {
    let text = format!(
        "{:.2} ms",
        context.debug_state.profile.frame_elapsed.as_nanos() as f64 / 1_000_000.0
    );
    gapi::push_string_xy(context.commands_state, &text, 5.0, 5.0);

    gapi::push_text_shader(context.commands_state);
    gapi::push_color(context.commands_state, Color::rgb(0.0, 0.0, 0.0));
    gapi::set_color_uniform(context.commands_state);
    gapi::draw_text(context.commands_state);

    Vec2f::new(0., 28.)
}
