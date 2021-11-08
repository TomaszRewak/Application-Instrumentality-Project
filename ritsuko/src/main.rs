use common::MessageWriteBuffer;
use mio::windows::NamedPipe;
use std::{
    fs::OpenOptions,
    io::Write,
    os::windows::prelude::{FromRawHandle, IntoRawHandle, OpenOptionsExt},
};

mod proto;

fn main() {
    let mut opts = OpenOptions::new();
    opts.read(true).write(true).custom_flags(1073741824);

    let file = opts.open(r"\\.\pipe\magi-workspace-manager").unwrap();
    let mut named_pipe = unsafe { NamedPipe::from_raw_handle(file.into_raw_handle()) };

    let mut message_write_buffer = MessageWriteBuffer::new();

    message_write_buffer.encode(&proto::local_management::Request {
        one_of: Some(
            proto::local_management::request::OneOf::StartApplicationRequest(
                proto::StartApplicationRequest { 
                    application_name: Some("notepad.exe".to_string()),
                    args: None,
                    path: None,
                    instance_name: None
                },
            ),
        ),
    });

    message_write_buffer.write(&mut named_pipe);
}
