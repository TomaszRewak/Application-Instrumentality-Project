use clap::App;
use common::MessageWriteBuffer;
use named_pipe::PipeClient;
use std::{path::Path, thread, time};

mod proto;

fn send(request: &proto::local_management::Request) {
    let pipe_path = Path::new(r"\\.\pipe\magi-workspace-manager");

    if !pipe_path.exists() {
        thread::sleep(time::Duration::from_secs(1));
    }

    let mut named_pipe = PipeClient::connect(pipe_path).unwrap();
    let mut message_write_buffer = MessageWriteBuffer::new();

    message_write_buffer.encode(request);
    message_write_buffer.write(&mut named_pipe);
}

fn main() {
    let matches = App::new("ritsuko")
        .version("1.0.0")
        .author("Tomasz Rewak")
        .about("Local management of the Magi")
        .subcommand(
            App::new("start")
                .about("starts a new instance of an applicatin")
                .arg_from_usage("-a, --app=[APPLICATION] 'application name'")
                .arg_from_usage("-x, --args=[APPLICATION] 'special arguments to pass to the application'"),
        )
        .subcommand(App::new("list"))
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("start") {
        let app = matches.value_of("app").unwrap();
        let args = match matches.value_of("args")
        {
            Some(args) => Some(args.to_string()),
            None => None
        };

        send(&proto::local_management::Request {
            one_of: Some(
                proto::local_management::request::OneOf::StartApplicationRequest(
                    proto::StartApplicationRequest {
                        application_name: Some(app.to_string()),
                        args: args,
                        path: None,
                        instance_name: None,
                    },
                ),
            ),
        });
    }
}
