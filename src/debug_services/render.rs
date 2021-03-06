use std::sync::MutexGuard;

use crate::commands::{Color, CommandsState, Rect, Vec2f};
use crate::components::ViewPortSize;
use crate::debug_services::profile;
use crate::debug_services::profile::ProfileState;
use crate::debug_services::state::*;
use crate::gapi;
use crate::render_state::RenderState;

struct Context<'a> {
    pos: Vec2f,
    view_port: &'a ViewPortSize,
    commands_state: &'a mut CommandsState,
    render_state: &'a mut RenderState,
    profile_state: &'a ProfileState,
}

pub fn render(
    debug_state: &mut MutexGuard<DebugState>,
    render_state: &mut MutexGuard<RenderState>,
    profile_state: &mut MutexGuard<ProfileState>,
    commands_state: &mut CommandsState,
    view_port: &ViewPortSize,
) {
    gapi::set_camera(commands_state, gapi::CAMERA_UI);

    let mut context = Context {
        pos: Vec2f::new(10.0, 10.0),
        view_port,
        commands_state,
        render_state,
        profile_state,
    };

    let size = render_frame_time(&mut context, &profile_state);

    context.pos.y += size.y;
    context.pos.x = 5.;

    render_group_variables(&mut context, &mut debug_state.variables);

    // context.pos.y += 10.;

    // let size = render_frames_slider(&mut context, &debug_state.profile);

    // context.pos.y += size.y;
    // context.pos.x = 0.0;

    // render_profile(&mut context, &debug_state.profile);
}

fn render_group_variables(context: &mut Context, variable: &mut GroupVariable) {
    let text_size = gapi::push_string_vec2f(
        context.commands_state,
        context.render_state,
        variable.name,
        context.pos,
    );

    variable.bounds = Rect::new(context.pos, text_size);

    gapi::push_text_shader(context.commands_state);

    if variable.is_hot {
        gapi::push_color(context.commands_state, Color::rgb(0.0, 0.5, 0.0));
    }
    else {
        gapi::push_color(context.commands_state, Color::rgb(0.0, 0.0, 0.0));
    }

    gapi::set_color_uniform(context.commands_state);
    gapi::draw_text(context.commands_state);

    gapi::push_quad_lines(context.commands_state, context.pos, text_size);
    gapi::draw_lines(context.commands_state);

    context.pos.y += text_size.y;

    if !variable.is_expanded {
        return;
    }

    context.pos.x += 20.;

    for mut v in variable.variables.iter_mut() {
        match &mut v {
            DebugVariable::Bool(variable) => {
                render_bool_variable(context, variable);
            }
            DebugVariable::Group(variable) => {
                render_group_variables(context, variable);
            }
            DebugVariable::Profiler(variable) => {
                render_profiler_variable(context, variable);
            }
            DebugVariable::ProfilerLogSlider(variable) => {
                render_profiler_log_slider_variable(context, variable);
            }
        };
    }

    context.pos.x -= 20.;
}

fn render_bool_variable(context: &mut Context, variable: &mut BoolVariable) {
    let text = format!("{}: {}", variable.name, variable.value);

    // Draw text shadow
    gapi::push_string_vec2f(
        context.commands_state,
        context.render_state,
        &text,
        context.pos + Vec2f::new(-1., 1.),
    );

    gapi::push_color(context.commands_state, Color::rgb(1.0, 1.0, 1.0));
    gapi::push_text_shader(context.commands_state);
    gapi::set_color_uniform(context.commands_state);
    gapi::draw_text(context.commands_state);

    // Draw text
    if variable.is_hot {
        gapi::push_color(context.commands_state, Color::rgb(0.0, 0.5, 0.0));
    }
    else {
        gapi::push_color(context.commands_state, Color::rgb(0.0, 0.0, 0.0));
    }

    let text_size = gapi::push_string_vec2f(
        context.commands_state,
        context.render_state,
        &text,
        context.pos,
    );

    variable.bounds = Rect::new(context.pos, text_size);

    gapi::push_text_shader(context.commands_state);
    gapi::set_color_uniform(context.commands_state);
    gapi::draw_text(context.commands_state);

    gapi::push_quad_lines(context.commands_state, context.pos, text_size);
    gapi::draw_lines(context.commands_state);

    context.pos.y += text_size.y;
}

fn render_frame_time(context: &mut Context, profile_state: &profile::ProfileState) -> Vec2f {
    let text = format!(
        "{:.2} ms",
        profile_state.frame_elapsed.as_nanos() as f64 / 1_000_000.0
    );
    gapi::push_string_xy(
        context.commands_state,
        context.render_state,
        &text,
        5.0,
        5.0,
    );

    gapi::push_text_shader(context.commands_state);
    gapi::push_color(context.commands_state, Color::rgb(0.0, 0.0, 0.0));
    gapi::set_color_uniform(context.commands_state);
    gapi::draw_text(context.commands_state);

    Vec2f::new(0., 28.)
}

fn render_profiler_variable(context: &mut Context, variable: &mut ProfilerVariable) {
    let pos = context.pos;
    let size = render_profile(context, context.profile_state);
    variable.bounds = Rect::new(pos, size);
    context.pos.y += size.y;
}

fn render_profiler_log_slider_variable(
    context: &mut Context,
    variable: &mut ProfilerLogSliderVariable,
) {
    let pos = context.pos;
    let size = render_frames_slider(context, context.profile_state);
    variable.bounds = Rect::new(pos, size);
    context.pos.y += size.y;
}

fn render_profile(context: &mut Context, profile_state: &profile::ProfileState) -> Vec2f {
    let mut pos = context.pos;

    let line_size = 18.0;
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
        gapi::push_string_xy(
            context.commands_state,
            context.render_state,
            &text,
            pos.x,
            pos.y,
        );
        pos.x += 100.0;

        let text = format!("{:.2}%", cycle.percent);
        gapi::push_string_xy(
            context.commands_state,
            context.render_state,
            &text,
            pos.x,
            pos.y,
        );
        pos.x += 100.0;

        gapi::push_string_xy(
            context.commands_state,
            context.render_state,
            &cycle.name,
            pos.x,
            pos.y,
        );
        pos.x += 250.0;

        // let text = format!("{}:{}", cycle.file_name, cycle.line);
        // gapi::push_string_xy(context.commands_state, &text, pos.x, pos.y);
        // pos.x += 250.0;

        let text = format!("{}h", cycle.sum_hits / cycle.hits);
        gapi::push_string_xy(
            context.commands_state,
            context.render_state,
            &text,
            pos.x,
            pos.y,
        );
        pos.x += 50.0;

        let text = format!("{:?}", cycle.sum_elapsed / cycle.hits);
        gapi::push_string_xy(
            context.commands_state,
            context.render_state,
            &text,
            pos.x,
            pos.y,
        );
        pos.x += 100.0;

        let text = format!(
            "{:?} ns/h",
            cycle.sum_hits_over_elapsed / cycle.hits as u128
        );
        gapi::push_string_xy(
            context.commands_state,
            context.render_state,
            &text,
            pos.x,
            pos.y,
        );

        pos.y += line_size;
    }

    // Text
    gapi::push_text_shader(context.commands_state);
    gapi::push_color(context.commands_state, Color::rgb(1.0, 1.0, 1.0));
    gapi::set_color_uniform(context.commands_state);
    gapi::draw_text(context.commands_state);

    size
}

fn render_frames_slider(context: &mut Context, profile_state: &profile::ProfileState) -> Vec2f {
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

    let current_snapshot = profile_state.snapshot_counter;

    for i in 0..profile::PERFORMANCE_COUNTER_LOG_SIZE {
        if current_snapshot == i {
            gapi::push_color(context.commands_state, Color::rgb(1.0, 1.0, 1.0));
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
