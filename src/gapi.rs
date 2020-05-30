use crate::commands::*;

pub fn push_color_shader(commands_state: &mut CommandsState) {
    push_render_command(commands_state, RenderCommandType::PushColorShader);
}

pub fn push_color(commands_state: &mut CommandsState, color: Color) {
    push_render_command_data(
        commands_state,
        RenderCommandType::PushColor,
        CommandData::color(color),
    );
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

pub fn draw_lines(commands_state: &mut CommandsState) {
    push_render_command(commands_state, RenderCommandType::DrawLines);
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

pub fn update_camera_position(commands_state: &mut CommandsState, pos: Vec2f) {
    push_execution_command_data(
        commands_state,
        ExecutionCommandType::PushVec2f,
        CommandData::vec2f(pos),
    );
    push_execution_command(commands_state, ExecutionCommandType::UpdateCameraPosition);
}
