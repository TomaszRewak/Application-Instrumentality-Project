use clap::{App, Arg, Parser, SubCommand};
use common::MessageWriteBuffer;
use named_pipe::PipeClient;
use std::{path::Path, thread, time};

mod proto;

fn main() {
    let matches = App::new("ritsuko")
        .version("1.0.0")
        .author("Tomasz Rewak")
        .about("Local management of the Magi")
        .subcommand(
            App::new("start")
                .about("starts a new instance of an applicatin")
                .arg("-a, --a=[APPLICATION] 'application name'")
                .arg("-x, --args=[APPLICATION] 'special arguments to pass to the application'"),
        )
        .subcommand(App::new("list"))
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("start") {
        matches.value_of("id")
    }

    let pipe_path = Path::new(r"\\.\pipe\magi-workspace-manager");

    if !pipe_path.exists() {
        thread::sleep(time::Duration::from_secs(1));
    }

    let mut named_pipe = PipeClient::connect(pipe_path).unwrap();
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
}
