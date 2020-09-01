use std::sync::MutexGuard;

use crate::commands::{Color, CommandsState, Vec2f};
use crate::components::ViewPortSize;
use crate::debug_services::profile;
use crate::debug_services::state::DebugState;
use crate::gapi;
use crate::layout::StackLayout;

struct Context<'a, 'b> {
    pos: Vec2f,
    debug_state: &'a mut MutexGuard<'b, DebugState>,
    commands_state: &'a mut CommandsState,
    view_port: &'a ViewPortSize,
}

impl<'a, 'b> Context<'a, 'b> {
    fn apply_stack(&mut self, stack: &StackLayout) {
        self.pos.x += stack.pos().x;
        self.pos.y += stack.pos().y;
    }
}

pub fn render(
    debug_state: &mut MutexGuard<DebugState>,
    commands_state: &mut CommandsState,
    view_port: &ViewPortSize,
) {
    let mut stack = StackLayout::new(0., 0.);

    let mut context = Context {
        pos: stack.pos(),
        debug_state,
        commands_state,
        view_port,
    };

    render_profile(&mut context);
    render_frames_slider(&mut context);
    render_frame_time(&mut context);
}

fn render_profile(context: &mut Context) {
    let mut stack = StackLayout::new(35.0, 10.0);
    let line_size = 18.0;

    context.pos = stack.pos();
    // let profile_state = &debug_state.profile;

    // let snapshot = &profile_state.performance_counter_log[profile_state.snapshot_counter].records;

    let size = render_frames_slider(context);
    stack.push_vertical(size.y);
    context.pos = stack.pos();

    // // Background
    // gapi::push_color_shader(commands_state);
    // gapi::push_color(commands_state, Color::rgba(0.0, 0.0, 0.0, 0.5));
    // gapi::set_color_uniform(commands_state);

    // gapi::push_vec2f(commands_state, Vec2f::new(0.0, offset_y - 10.0));
    // gapi::push_vec2f(
    //     commands_state,
    //     Vec2f::new(
    //         view_port.width as f32,
    //         line_size * snapshot.len() as f32 + 20.0,
    //     ),
    // );
    // gapi::draw_quads(commands_state);

    // for cycle in snapshot.iter() {
    //     let text = format!("{:?}", cycle.thread_id);
    //     gapi::push_string_xy(commands_state, &text, offset_x, offset_y);
    //     offset_x += 100.0;

    //     let text = format!("{:.2}%", cycle.percent);
    //     gapi::push_string_xy(commands_state, &text, offset_x, offset_y);
    //     offset_x += 100.0;

    //     gapi::push_string_xy(commands_state, &cycle.name, offset_x, offset_y);
    //     offset_x += 250.0;

    //     let text = format!("{}:{}", cycle.file_name, cycle.line);
    //     gapi::push_string_xy(commands_state, &text, offset_x, offset_y);
    //     offset_x += 250.0;

    //     let text = format!("{}h", cycle.sum_hits / cycle.hits);
    //     gapi::push_string_xy(commands_state, &text, offset_x, offset_y);
    //     offset_x += 50.0;

    //     let text = format!("{:?}", cycle.sum_elapsed / cycle.hits);
    //     gapi::push_string_xy(commands_state, &text, offset_x, offset_y);
    //     offset_x += 100.0;

    //     let text = format!(
    //         "{:?} ns/h",
    //         cycle.sum_hits_over_elapsed / cycle.hits as u128
    //     );
    //     gapi::push_string_xy(commands_state, &text, offset_x, offset_y);

    //     offset_y += line_size;
    //     offset_x = 10.0;
    // }

    // // Text
    // gapi::push_text_shader(commands_state);
    // gapi::push_color(commands_state, Color::rgb(1.0, 1.0, 1.0));
    // gapi::set_color_uniform(commands_state);
    // gapi::draw_text(commands_state);
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
        } else {
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

fn render_frame_time(context: &mut Context) {
    let text = format!(
        "{:.2} ms",
        context.debug_state.profile.frame_elapsed.as_nanos() as f64 / 1_000_000.0
    );
    gapi::push_string_xy(context.commands_state, &text, 5.0, 5.0);

    gapi::push_text_shader(context.commands_state);
    gapi::push_color(context.commands_state, Color::rgb(0.0, 0.0, 0.0));
    gapi::set_color_uniform(context.commands_state);
    gapi::draw_text(context.commands_state);
}
