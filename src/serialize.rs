extern crate flatbuffers;

use std::slice;

use crate::flatbuffers_execution_commands::tech_paws::backend::schemes as execution_schemes;
use crate::flatbuffers_render_commands::tech_paws::backend::schemes as render_schemes;
use crate::flatbuffers_request_commands::tech_paws::backend::schemes as request_schemes;
use crate::render::components::{ExecutionCommand, RenderCommand, RequestCommand};
use crate::{Memory, RawBuffer};

pub fn serialize_json_render_commands(
    memory: &mut Memory,
    commands: &[RenderCommand],
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
    commands: &[ExecutionCommand],
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
    commands: &[RenderCommand],
) -> RawBuffer {
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);
    let commands_schemes: Vec<flatbuffers::WIPOffset<render_schemes::RenderCommand>> = commands
        .iter()
        .map(|command| {
            let data = create_scheme_render_command_data(&mut builder, command);
            render_schemes::RenderCommand::create(
                &mut builder,
                &render_schemes::RenderCommandArgs {
                    type_: create_scheme_render_command_type(command),
                    data: Some(data),
                },
            )
        })
        .collect();

    let commands_flatbuffers_vec = builder.create_vector(&commands_schemes);

    render_schemes::RenderCommands::create(
        &mut builder,
        &render_schemes::RenderCommandsArgs {
            commands: Some(commands_flatbuffers_vec),
        },
    );

    let data = builder.finished_data();

    let start = memory.serialize_buffer.len();
    let end = start + data.len();

    memory.serialize_buffer.extend(data.iter());

    let data = memory.serialize_buffer[start..end].as_ptr();

    RawBuffer {
        data,
        length: end - start,
    }
}

fn create_scheme_render_command_type(command: &RenderCommand) -> render_schemes::RenderCommandType {
    match command {
        RenderCommand::PushColor { .. } => render_schemes::RenderCommandType::PushColor,
        RenderCommand::PushPos2f { .. } => render_schemes::RenderCommandType::PushPos2f,
        RenderCommand::PushSize2f { .. } => render_schemes::RenderCommandType::PushSize2f,
        RenderCommand::PushTexture { .. } => render_schemes::RenderCommandType::PushTexture,
        RenderCommand::SetColorUniform => render_schemes::RenderCommandType::SetColorUniform,
        RenderCommand::PushColorShader => render_schemes::RenderCommandType::PushColorShader,
        RenderCommand::PushTextureShader => render_schemes::RenderCommandType::PushTextureShader,
        RenderCommand::DrawLines => render_schemes::RenderCommandType::DrawLines,
        RenderCommand::DrawPoints => render_schemes::RenderCommandType::DrawPoints,
        RenderCommand::DrawQuads => render_schemes::RenderCommandType::DrawQuads,
    }
}

fn create_scheme_render_command_data<'a, 'b>(
    mut builder: &'a mut flatbuffers::FlatBufferBuilder<'b>,
    command: &'a RenderCommand,
) -> flatbuffers::WIPOffset<render_schemes::RenderCommandData<'b>> {
    let mut pos2f: Option<&render_schemes::Pos2f> = None;
    let mut size2f: Option<&render_schemes::Size2f> = None;
    let mut color: Option<&render_schemes::Color> = None;

    let pos2f_data;
    let size2f_data;
    let color_data;

    match *command {
        RenderCommand::PushColor { r, g, b, a } => {
            color_data = render_schemes::Color::new(r, g, b, a);
            color = Some(&color_data);
        }
        RenderCommand::PushPos2f { x, y } => {
            pos2f_data = render_schemes::Pos2f::new(x, y);
            pos2f = Some(&pos2f_data);
        }
        RenderCommand::PushSize2f { x, y } => {
            size2f_data = render_schemes::Size2f::new(x, y);
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

    render_schemes::RenderCommandData::create(
        &mut builder,
        &render_schemes::RenderCommandDataArgs {
            color,
            pos2f,
            size2f,
        },
    )
}

pub fn serialize_flatbuffers_exec_commands(
    memory: &mut Memory,
    commands: &[ExecutionCommand],
) -> RawBuffer {
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);
    let commands_schemes: Vec<flatbuffers::WIPOffset<execution_schemes::ExecutionCommand>> =
        commands
            .iter()
            .map(|command| {
                let data = create_scheme_execution_command_data(&mut builder, command);
                execution_schemes::ExecutionCommand::create(
                    &mut builder,
                    &execution_schemes::ExecutionCommandArgs {
                        type_: create_scheme_execution_command_type(command),
                        data: Some(data),
                    },
                )
            })
            .collect();

    let commands_flatbuffers_vec = builder.create_vector(&commands_schemes);

    execution_schemes::ExecutionCommands::create(
        &mut builder,
        &execution_schemes::ExecutionCommandsArgs {
            commands: Some(commands_flatbuffers_vec),
        },
    );

    let data = builder.finished_data();

    let start = memory.serialize_buffer.len();
    let end = start + data.len();

    memory.serialize_buffer.extend(data.iter());

    let data = memory.serialize_buffer[start..end].as_ptr();

    RawBuffer {
        data,
        length: end - start,
    }
}

fn create_scheme_execution_command_type(
    command: &ExecutionCommand,
) -> execution_schemes::ExecutionCommandType {
    match command {
        ExecutionCommand::PushPos2f { .. } => execution_schemes::ExecutionCommandType::PushPos2f,
        ExecutionCommand::UpdateCameraPosition => {
            execution_schemes::ExecutionCommandType::UpdateCameraPosition
        }
    }
}

fn create_scheme_execution_command_data<'a, 'b>(
    mut builder: &'a mut flatbuffers::FlatBufferBuilder<'b>,
    command: &'a ExecutionCommand,
) -> flatbuffers::WIPOffset<execution_schemes::ExecutionCommandData<'b>> {
    let mut pos2f: Option<&execution_schemes::Pos2f> = None;
    let pos2f_data;

    match *command {
        ExecutionCommand::PushPos2f { x, y } => {
            pos2f_data = execution_schemes::Pos2f::new(x, y);
            pos2f = Some(&pos2f_data);
        }
        ExecutionCommand::UpdateCameraPosition => {}
    }

    execution_schemes::ExecutionCommandData::create(
        &mut builder,
        &execution_schemes::ExecutionCommandDataArgs { pos2f },
    )
}

pub fn deserialize_flatbuffers_request_commands(data: RawBuffer) -> Vec<RequestCommand> {
    let bytes = unsafe { slice::from_raw_parts(data.data, data.length) };
    let flatbuffer_commands = request_schemes::get_root_as_request_commands(bytes).commands();

    match flatbuffer_commands {
        Some(commands) => {
            let mut vec = Vec::with_capacity(commands.len());

            for command in commands {
                let command = create_request_command_from_flatbuffers(command);

                if let Some(command) = command {
                    vec.push(command);
                }
            }

            vec
        }
        None => vec![],
    }
}

fn create_request_command_from_flatbuffers(
    command: request_schemes::RequestCommand,
) -> Option<RequestCommand> {
    let data = command.data()?;

    let command = match command.type_() {
        request_schemes::RequestCommandType::SetViewportSize => RequestCommand::SetViewportSize {
            width: data.size2i()?.width(),
            height: data.size2i()?.height(),
        },
        request_schemes::RequestCommandType::OnTouchStart => RequestCommand::OnTouchStart {
            x: data.pos2f()?.x(),
            y: data.pos2f()?.y(),
        },
        request_schemes::RequestCommandType::OnTouchEnd => RequestCommand::OnTouchEnd {
            x: data.pos2f()?.x(),
            y: data.pos2f()?.y(),
        },
        request_schemes::RequestCommandType::OnTouchMove => RequestCommand::OnTouchMove {
            x: data.pos2f()?.x(),
            y: data.pos2f()?.y(),
        },
    };

    Some(command)
}
