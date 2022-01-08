use clap::App;
use common::MessageWriteBuffer;
use named_pipe::PipeClient;
use std::path::Path;

mod proto;

fn send(request: proto::local_management::request::OneOf) {
    let pipe_path = Path::new(r"\\.\pipe\magi-workspace-manager");

    let mut named_pipe = PipeClient::connect(pipe_path).unwrap();
    let mut message_write_buffer = MessageWriteBuffer::new();

    let message = proto::local_management::Request {
        request_id: None,
        one_of: Some(request),
    };

    message_write_buffer.encode(&message);
    message_write_buffer.write(&mut named_pipe);
}

fn main() {
    let matches = App::new("ritsuko")
        .version("1.0.0")
        .author("Tomasz Rewak")
        .about("Local management of the Magi")
        .subcommand(
            App::new("create")
                .about("creates a new instance of an application")
                .arg_from_usage("-a, --app=[APPLICATION] 'application name'")
                .arg_from_usage("-i, --instance=[INSTANCE] 'instance name'"),
        )
        .subcommand(
            App::new("start")
                .about("starts an instance of an applicatin")
                .arg_from_usage("-a, --app=[APPLICATION] 'application name'")
                .arg_from_usage("-i, --instance=[INSTANCE] 'instance name'"),
        )
        .subcommand(
            App::new("stop")
                .about("stops an instance of an applicatin")
                .arg_from_usage("-a, --app=[APPLICATION] 'application name'")
                .arg_from_usage("-i, --instance=[INSTANCE] 'instance name'"),
        )
        .subcommand(App::new("list"))
        .get_matches();

    if let Some(ref create_command) = matches.subcommand_matches("create") {
        let app = create_command.value_of("app").unwrap();
        let instance = create_command.value_of("instance").unwrap();

        send(
            proto::local_management::request::OneOf::CreateInstanceRequest(
                proto::CreateInstanceRequest {
                    application_name: Some(String::from(app)),
                    instance_name: Some(String::from(instance)),
                },
            ),
        );
    }

    if let Some(ref start_command) = matches.subcommand_matches("start") {
        let app = start_command.value_of("app").unwrap();
        let instance = start_command.value_of("instance").unwrap();

        send(
            proto::local_management::request::OneOf::StartInstanceRequest(
                proto::StartInstanceRequest {
                    application_name: Some(String::from(app)),
                    instance_name: Some(String::from(instance)),
                },
            ),
        );
    }

    if let Some(ref stop_command) = matches.subcommand_matches("stop") {
        let app = stop_command.value_of("app").unwrap();
        let instance = stop_command.value_of("instance").unwrap();

        send(
            proto::local_management::request::OneOf::StopInstanceRequest(
                proto::StopInstanceRequest {
                    application_name: Some(String::from(app)),
                    instance_name: Some(String::from(instance)),
                },
            ),
        );
    }
}
