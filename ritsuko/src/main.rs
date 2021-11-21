use common::MessageWriteBuffer;
use named_pipe::PipeClient;
use std::{thread, time};

mod proto;

fn main() {
    thread::sleep(time::Duration::from_secs(2));

    let mut named_pipe = PipeClient::connect_ms(r"\\.\pipe\magi-workspace-manager", 2000).unwrap();
    let mut message_write_buffer = MessageWriteBuffer::new();

    message_write_buffer.encode(&proto::local_management::Request {
        one_of: Some(
            proto::local_management::request::OneOf::StartApplicationRequest(
                proto::StartApplicationRequest {
                    application_name: Some("notepad.exe".to_string()),
                    args: None,
                    path: None,
                    instance_name: None,
                },
            ),
        ),
    });

    message_write_buffer.write(&mut named_pipe);

    println!("End");
}
