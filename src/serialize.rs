extern crate flatbuffers;

// use std::slice;

use crate::commands::{ExecutionCommand, RenderCommand, RequestCommand};
use crate::memory::MemoryState;
use crate::RawBuffer;

// use crate::flatbuffers_commands::tech_paws::schemes;

pub fn serialize_json_render_commands(
    _memory: &mut MemoryState,
    _commands: &[RenderCommand],
) -> RawBuffer {
    todo!();
    // let json = serde_json::to_vec(commands).expect("failed to serialize render commands");
    // let data = memory.serialize_buffer.alloc_slice_copy(json.as_slice());

    // RawBuffer {
    //     data: data.as_ptr(),
    //     length: json.len(),
    // }
}

pub fn serialize_json_exec_commands(
    _memory: &mut MemoryState,
    _commands: &[ExecutionCommand],
) -> RawBuffer {
    todo!();
    // let json = serde_json::to_vec(commands).expect("failed to serialize execution commands");
    // let data = memory.serialize_buffer.alloc_slice_copy(json.as_slice());

    // RawBuffer {
    //     data: data.as_ptr(),
    //     length: data.len(),
    // }
}

pub fn deserialize_json_request_commands(
    _data: RawBuffer,
) -> serde_json::Result<Vec<RequestCommand>> {
    todo!();
    // let bytes = unsafe { slice::from_raw_parts(data.data, data.length) };
    // serde_json::from_slice::<Vec<RequestCommand>>(bytes)
}

// TODO:
// pub fn serialize_flatbuffers_render_commands(
//     memory: &mut Memory,
//     commands: &[RenderCommand],
// ) -> RawBuffer {
//     let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);
//     let commands_schemes: Vec<flatbuffers::WIPOffset<schemes::RenderCommand>> = commands
//         .iter()
//         .map(|command| {
//             let data = create_scheme_render_command_data(&mut builder, command);
//             schemes::RenderCommand::create(
//                 &mut builder,
//                 &schemes::RenderCommandArgs {
//                     type_: create_scheme_render_command_type(command),
//                     data: Some(data),
//                 },
//             )
//         })
//         .collect();

//     let commands_flatbuffers_vec = builder.create_vector(&commands_schemes);

//     schemes::RenderCommands::create(
//         &mut builder,
//         &schemes::RenderCommandsArgs {
//             commands: Some(commands_flatbuffers_vec),
//         },
//     );

//     let data = builder.finished_data();

//     let start = memory.serialize_buffer.len();
//     let end = start + data.len();

//     memory.serialize_buffer.extend(data.iter());

//     let data = memory.serialize_buffer[start..end].as_ptr();

//     RawBuffer {
//         data,
//         length: end - start,
//     }
// }

// fn create_scheme_render_command_type(command: &RenderCommand) -> schemes::RenderCommandType {
//     match command {
//         RenderCommand::PushColor { .. } => schemes::RenderCommandType::PushColor,
//         RenderCommand::PushPos2f { .. } => schemes::RenderCommandType::PushPos2f,
//         RenderCommand::PushSize2f { .. } => schemes::RenderCommandType::PushSize2f,
//         RenderCommand::PushTexture { .. } => schemes::RenderCommandType::PushTexture,
//         RenderCommand::SetColorUniform => schemes::RenderCommandType::SetColorUniform,
//         RenderCommand::PushColorShader => schemes::RenderCommandType::PushColorShader,
//         RenderCommand::PushTextureShader => schemes::RenderCommandType::PushTextureShader,
//         RenderCommand::DrawLines => schemes::RenderCommandType::DrawLines,
//         RenderCommand::DrawPoints => schemes::RenderCommandType::DrawPoints,
//         RenderCommand::DrawQuads => schemes::RenderCommandType::DrawQuads,
//     }
// }

// fn create_scheme_render_command_data<'a, 'b>(
//     mut builder: &'a mut flatbuffers::FlatBufferBuilder<'b>,
//     command: &'a RenderCommand,
// ) -> flatbuffers::WIPOffset<schemes::Data<'b>> {
//     let mut vec2f: Option<&schemes::Vec2f> = None;
//     let mut color: Option<&schemes::Color> = None;

//     let vec2f_data;
//     let color_data;

//     match *command {
//         RenderCommand::PushColor { r, g, b, a } => {
//             color_data = schemes::Color::new(r, g, b, a);
//             color = Some(&color_data);
//         }
//         RenderCommand::PushPos2f { x, y } => {
//             vec2f_data = schemes::Vec2f::new(x, y);
//             vec2f = Some(&vec2f_data);
//         }
//         RenderCommand::PushSize2f { x, y } => {
//             vec2f_data = schemes::Vec2f::new(x, y);
//             vec2f = Some(&vec2f_data);
//         }
//         RenderCommand::PushTexture { .. } => {}
//         RenderCommand::SetColorUniform => {}
//         RenderCommand::PushColorShader => {}
//         RenderCommand::PushTextureShader => {}
//         RenderCommand::DrawLines => {}
//         RenderCommand::DrawPoints => {}
//         RenderCommand::DrawQuads => {}
//     }

//     schemes::Data::create(
//         &mut builder,
//         &schemes::DataArgs {
//             color,
//             vec2f,
//             vec2i: None,
//         },
//     )
// }

// pub fn serialize_flatbuffers_exec_commands(
//     memory: &mut Memory,
//     commands: &[ExecutionCommand],
// ) -> RawBuffer {
//     let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);
//     let commands_schemes: Vec<flatbuffers::WIPOffset<schemes::ExecutionCommand>> = commands
//         .iter()
//         .map(|command| {
//             let data = create_scheme_execution_command_data(&mut builder, command);
//             schemes::ExecutionCommand::create(
//                 &mut builder,
//                 &schemes::ExecutionCommandArgs {
//                     type_: create_scheme_execution_command_type(command),
//                     data: Some(data),
//                 },
//             )
//         })
//         .collect();

//     let commands_flatbuffers_vec = builder.create_vector(&commands_schemes);

//     schemes::ExecutionCommands::create(
//         &mut builder,
//         &schemes::ExecutionCommandsArgs {
//             commands: Some(commands_flatbuffers_vec),
//         },
//     );

//     let data = builder.finished_data();

//     let start = memory.serialize_buffer.len();
//     let end = start + data.len();

//     memory.serialize_buffer.extend(data.iter());

//     let data = memory.serialize_buffer[start..end].as_ptr();

//     RawBuffer {
//         data,
//         length: end - start,
//     }
// }

// fn create_scheme_execution_command_type(
//     command: &ExecutionCommand,
// ) -> schemes::ExecutionCommandType {
//     match command {
//         ExecutionCommand::PushPos2f { .. } => schemes::ExecutionCommandType::PushPos2f,
//         ExecutionCommand::UpdateCameraPosition => {
//             schemes::ExecutionCommandType::UpdateCameraPosition
//         }
//     }
// }

// fn create_scheme_execution_command_data<'a, 'b>(
//     mut builder: &'a mut flatbuffers::FlatBufferBuilder<'b>,
//     command: &'a ExecutionCommand,
// ) -> flatbuffers::WIPOffset<schemes::Data<'b>> {
//     let mut vec2f: Option<&schemes::Vec2f> = None;
//     let vec2f_data;

//     match *command {
//         ExecutionCommand::PushPos2f { x, y } => {
//             vec2f_data = schemes::Vec2f::new(x, y);
//             vec2f = Some(&vec2f_data);
//         }
//         ExecutionCommand::UpdateCameraPosition => {}
//     }

//     schemes::Data::create(
//         &mut builder,
//         &schemes::DataArgs {
//             vec2f,
//             vec2i: None,
//             color: None,
//         },
//     )
// }

// pub fn deserialize_flatbuffers_request_commands(data: RawBuffer) -> Vec<RequestCommand> {
//     let bytes = unsafe { slice::from_raw_parts(data.data, data.length) };
//     let flatbuffer_commands = flatbuffers::get_root::<schemes::RequestCommands>(bytes).commands();

//     match flatbuffer_commands {
//         Some(commands) => {
//             let mut vec = Vec::with_capacity(commands.len());

//             for command in commands {
//                 let command = create_request_command_from_flatbuffers(command);

//                 if let Some(command) = command {
//                     vec.push(command);
//                 }
//             }

//             vec
//         }
//         None => vec![],
//     }
// }

// fn create_request_command_from_flatbuffers(
//     command: schemes::RequestCommand,
// ) -> Option<RequestCommand> {
//     let data = command.data()?;

//     let command = match command.type_() {
//         schemes::RequestCommandType::SetViewportSize => RequestCommand::SetViewportSize {
//             width: data.vec2i()?.x(),
//             height: data.vec2i()?.y(),
//         },
//         schemes::RequestCommandType::OnTouchStart => RequestCommand::OnTouchStart {
//             x: data.vec2f()?.x(),
//             y: data.vec2f()?.y(),
//         },
//         schemes::RequestCommandType::OnTouchEnd => RequestCommand::OnTouchEnd {
//             x: data.vec2f()?.x(),
//             y: data.vec2f()?.y(),
//         },
//         schemes::RequestCommandType::OnTouchMove => RequestCommand::OnTouchMove {
//             x: data.vec2f()?.x(),
//             y: data.vec2f()?.y(),
//         },
//     };

//     Some(command)
// }
