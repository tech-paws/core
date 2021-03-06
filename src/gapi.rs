use crate::commands::*;
use crate::memory;
use crate::render_state::RenderState;

pub const CAMERA_UI: usize = 0;
pub const CAMERA_ORTHO: usize = 1;
pub const CAMERA_COUNT: usize = 2;

pub fn push_color_shader(commands_state: &mut CommandsState) {
    push_render_command(commands_state, RenderCommandType::PushColorShader);
}

pub fn push_text_shader(commands_state: &mut CommandsState) {
    push_render_command(commands_state, RenderCommandType::PushTextShader);
}

pub fn push_color(commands_state: &mut CommandsState, color: Color) {
    push_render_command_data(
        commands_state,
        RenderCommandType::PushColor,
        CommandData::color(color),
    );
}

/// Returns string size
pub fn push_string(
    commands_state: &mut CommandsState,
    render_state: &mut RenderState,
    str: &str,
) -> Vec2f {
    let memory_state = memory::get_memory_state();
    let data = memory_state.frame_memory.alloc_slice_copy(str.as_bytes());

    push_render_command_data(
        commands_state,
        RenderCommandType::PushString,
        CommandData::string_bytes(data),
    );

    render_state.next_text_size()
}

pub fn push_string_xy(
    commands_state: &mut CommandsState,
    render_state: &mut RenderState,
    str: &str,
    x: f32,
    y: f32,
) -> Vec2f {
    push_vec2f_xy(commands_state, x, y);
    push_string(commands_state, render_state, str)
}

/// Returns size of string
pub fn push_string_vec2f(
    commands_state: &mut CommandsState,
    render_state: &mut RenderState,
    str: &str,
    pos: Vec2f,
) -> Vec2f {
    push_vec2f(commands_state, pos);
    push_string(commands_state, render_state, str)
}

pub fn push_color_rgb(commands_state: &mut CommandsState, r: f32, g: f32, b: f32) {
    push_color(commands_state, Color::rgb(r, b, g));
}

pub fn push_color_rgba(commands_state: &mut CommandsState, r: f32, g: f32, b: f32, a: f32) {
    push_color(commands_state, Color::rgba(r, b, g, a));
}

pub fn set_color_uniform(commands_state: &mut CommandsState) {
    push_render_command(commands_state, RenderCommandType::SetColorUniform);
}

pub fn set_camera(commands_state: &mut CommandsState, camera_id: usize) {
    push_render_command_data(
        commands_state,
        RenderCommandType::PushInt32,
        CommandData::int32(camera_id as i32),
    );
    push_render_command(commands_state, RenderCommandType::SetCamera);
}

pub fn draw_lines(commands_state: &mut CommandsState) {
    push_render_command(commands_state, RenderCommandType::DrawLines);
}

pub fn draw_text(commands_state: &mut CommandsState) {
    push_render_command(commands_state, RenderCommandType::DrawText);
}

pub fn draw_quads(commands_state: &mut CommandsState) {
    push_render_command(commands_state, RenderCommandType::DrawQuads);
}

pub fn push_vec2f(commands_state: &mut CommandsState, vec2f: Vec2f) {
    push_render_command_data(
        commands_state,
        RenderCommandType::PushVec2f,
        CommandData::vec2f(vec2f),
    );
}

pub fn push_vec2f_xy(commands_state: &mut CommandsState, x: f32, y: f32) {
    push_render_command_data(
        commands_state,
        RenderCommandType::PushVec2f,
        CommandData::vec2f(Vec2f::new(x, y)),
    );
}

pub fn push_quad_lines(commands_state: &mut CommandsState, pos: Vec2f, size: Vec2f) {
    push_vec2f_xy(commands_state, pos.x, pos.y);
    push_vec2f_xy(commands_state, pos.x + size.x, pos.y);

    push_vec2f_xy(commands_state, pos.x + size.x, pos.y);
    push_vec2f_xy(commands_state, pos.x + size.x, pos.y + size.y);

    push_vec2f_xy(commands_state, pos.x + size.x, pos.y + size.y);
    push_vec2f_xy(commands_state, pos.x, pos.y + size.y);

    push_vec2f_xy(commands_state, pos.x, pos.y + size.y);
    push_vec2f_xy(commands_state, pos.x, pos.y);
}

pub fn update_camera_position(commands_state: &mut CommandsState, id: usize, pos: Vec2f) {
    push_execution_command_data(
        commands_state,
        ExecutionCommandType::PushInt32,
        CommandData::int32(id as i32),
    );
    push_execution_command_data(
        commands_state,
        ExecutionCommandType::PushVec2f,
        CommandData::vec2f(pos),
    );
    push_execution_command(commands_state, ExecutionCommandType::UpdateCameraPosition);
}
