extern crate flatbuffers;

use std::slice;

use crate::render::components::{ExectutionCommand, RenderCommand, RequestCommand};
use crate::schemas::render_commands_generated::tech_paws::backend::schemes;
use crate::{Memory, RawBuffer};

pub fn serialize_json_render_commands(
    memory: &mut Memory,
    commands: &Vec<RenderCommand>,
) -> RawBuffer {
    let json = serde_json::to_vec(commands).unwrap();

    let start = memory.serialize_buffer.len();
    let end = start + json.len();

    memory.serialize_buffer.extend(json.into_iter());

    let data = memory.serialize_buffer[start..end].as_ptr();

    RawBuffer {
        data,
        length: end - start,
    }
}

pub fn serialize_json_exec_commands(
    memory: &mut Memory,
    commands: &Vec<ExectutionCommand>,
) -> RawBuffer {
    let json = serde_json::to_vec(commands).unwrap();

    let start = memory.serialize_buffer.len();
    let end = start + json.len();

    memory.serialize_buffer.extend(json.into_iter());

    let data = memory.serialize_buffer[start..end].as_ptr();

    RawBuffer {
        data,
        length: end - start,
    }
}

pub fn deserialize_json_request_commands(data: RawBuffer) -> Vec<RequestCommand> {
    let bytes = unsafe { slice::from_raw_parts(data.data, data.length) };
    serde_json::from_slice::<Vec<RequestCommand>>(bytes).unwrap()
}

pub fn serialize_flatbuffers_render_commands(
    memory: &mut Memory,
    commands: &Vec<RenderCommand>,
) -> RawBuffer {
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);
    let commands_schemes: Vec<flatbuffers::WIPOffset<schemes::RenderCommand>> = commands
        .iter()
        .map(|command| {
            let data = create_scheme_render_command_data(&mut builder, command);
            schemes::RenderCommand::create(
                &mut builder,
                &schemes::RenderCommandArgs {
                    type_: create_scheme_render_command_type(command),
                    data: Some(data),
                },
            )
        })
        .collect();

    let commands_flatbuffers_vec = builder.create_vector(&commands_schemes);

    schemes::RenderCommands::create(
        &mut builder,
        &schemes::RenderCommandsArgs {
            commands: Some(commands_flatbuffers_vec),
        },
    );

    let data = builder.finished_data();

    let start = memory.serialize_buffer.len();
    let end = start + data.len();

    memory.serialize_buffer.extend(data.into_iter());

    let data = memory.serialize_buffer[start..end].as_ptr();

    RawBuffer {
        data,
        length: end - start,
    }
}

fn create_scheme_render_command_type(command: &RenderCommand) -> schemes::RenderCommandType {
    match command {
        RenderCommand::PushColor { .. } => schemes::RenderCommandType::PushColor,
        RenderCommand::PushPos2f { .. } => schemes::RenderCommandType::PushPos2f,
        RenderCommand::PushSize2f { .. } => schemes::RenderCommandType::PushSize2f,
        RenderCommand::PushTexture { .. } => schemes::RenderCommandType::PushTexture,
        RenderCommand::SetColorUniform => schemes::RenderCommandType::SetColorUniform,
        RenderCommand::PushColorShader => schemes::RenderCommandType::PushColorShader,
        RenderCommand::PushTextureShader => schemes::RenderCommandType::PushTextureShader,
        RenderCommand::DrawLines => schemes::RenderCommandType::DrawLines,
        RenderCommand::DrawPoints => schemes::RenderCommandType::DrawPoints,
        RenderCommand::DrawQuads => schemes::RenderCommandType::DrawQuads,
    }
}

fn create_scheme_render_command_data<'a, 'b>(
    mut builder: &'a mut flatbuffers::FlatBufferBuilder<'b>,
    command: &'a RenderCommand,
) -> flatbuffers::WIPOffset<schemes::RenderCommandData<'b>> {
    let mut pos2f: Option<&schemes::Pos2f> = None;
    let mut size2f: Option<&schemes::Size2f> = None;
    let mut color: Option<&schemes::Color> = None;

    let pos2f_data;
    let size2f_data;
    let color_data;

    match *command {
        RenderCommand::PushColor { r, g, b, a } => {
            color_data = schemes::Color::new(r, g, b, a);
            color = Some(&color_data);
        }
        RenderCommand::PushPos2f { x, y } => {
            pos2f_data = schemes::Pos2f::new(x, y);
            pos2f = Some(&pos2f_data);
        }
        RenderCommand::PushSize2f { x, y } => {
            size2f_data = schemes::Size2f::new(x, y);
            size2f = Some(&size2f_data);
        }
        RenderCommand::PushTexture { .. } => {}
        RenderCommand::SetColorUniform => {}
        RenderCommand::PushColorShader => {}
        RenderCommand::PushTextureShader => {}
        RenderCommand::DrawLines => {}
        RenderCommand::DrawPoints => {}
        RenderCommand::DrawQuads => {}
    }

    return schemes::RenderCommandData::create(
        &mut builder,
        &schemes::RenderCommandDataArgs {
            color,
            pos2f,
            size2f,
        },
    );
}

pub fn serialize_flatbuffers_exec_commands(
    _memory: &mut Memory,
    _commands: &Vec<ExectutionCommand>,
) -> RawBuffer {
    todo!()
}

pub fn deserialize_flatbuffers_request_commands(_commands: RawBuffer) -> Vec<RequestCommand> {
    todo!()
}
