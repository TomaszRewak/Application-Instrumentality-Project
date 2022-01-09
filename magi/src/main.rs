mod application;
mod instance;
mod proto;
mod workstation_manager;

use application::Application;
use common::MessageReadBuffer;
use common::MessageWriteBuffer;
use named_pipe::PipeOptions;
use proto::client_server::workstation_manager_client::WorkstationManagerClient;
use tonic::codegen::http::request;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::SystemTime;
use tokio::time::sleep;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut applications = HashMap::from([
        (
            String::from("notepad"),
            Application::new("notepad.exe".to_string(), "".to_string()),
        ),
        (
            String::from("cmd"),
            Application::new("cmd.exe".to_string(), "".to_string()),
        ),
        (
            String::from("explorer"),
            Application::new("explorer".to_string(), "".to_string()),
        ),
    ]);

    let mut applications = Arc::new(Mutex::new(applications));

    tokio::spawn(async move {
        let mut pipe_options = PipeOptions::new(r"\\.\pipe\magi-workspace-manager");

        loop {
            println!("Trying to create a named pipe");
            let pipe_server = pipe_options.single().unwrap();
            let mut named_pipe = pipe_server.wait().unwrap();
            println!("Created named pipe");

            pipe_options.first(false);

            let mut applications = applications.clone();

            tokio::spawn(async move {
                let pipe_lifespan = Duration::from_secs(5);

                let mut message_read_buffer = MessageReadBuffer::new();
                let mut message_write_buffer = MessageWriteBuffer::new();
                let mut destroy_after = Some(SystemTime::now() + pipe_lifespan);

                loop {
                    message_read_buffer.read(&mut named_pipe);

                    if let Some(request) =
                        message_read_buffer.decode::<proto::local_management::Request>()
                    {
                        println!("LOCAL: {:?}", request);

                        if request.keep_alive() {
                            destroy_after = Some(SystemTime::now() + pipe_lifespan);
                        } else {
                            destroy_after = None;
                        }

                        match request.one_of {
                            Some(
                                proto::local_management::request::OneOf::CreateInstanceRequest(
                                    create_instance_request,
                                ),
                            ) => {
                                let mut applications = applications.lock().unwrap();

                                let application = applications
                                    .get_mut(&create_instance_request.application_name.unwrap());

                                let error = match application {
                                    Some(application) => {
                                        application.add_instance(
                                            create_instance_request.instance_name.unwrap(),
                                        );

                                        None
                                    }
                                    None => Some(String::from("Application not found")),
                                };

                                message_write_buffer.encode(&proto::local_management::Reply {
                                    request_id: request.request_id,
                                    one_of: Some(
                                        proto::local_management::reply::OneOf::CreateInstanceReply(
                                            proto::CreateInstanceReply { error: error },
                                        ),
                                    ),
                                });
                            }
                            Some(
                                proto::local_management::request::OneOf::StartInstanceRequest(
                                    start_insntace_request,
                                ),
                            ) => {
                                let mut applications = applications.lock().unwrap();

                                let application = applications
                                    .get_mut(&start_insntace_request.application_name.unwrap());

                                let error = match application {
                                    Some(application) => {
                                        application.start_instnace(
                                            start_insntace_request.instance_name.unwrap(),
                                        );

                                        None
                                    }
                                    None => Some(String::from("Application not found")),
                                };

                                message_write_buffer.encode(&proto::local_management::Reply {
                                    request_id: request.request_id,
                                    one_of: Some(
                                        proto::local_management::reply::OneOf::StartInstanceReply(
                                            proto::StartInstanceReply { error: error },
                                        ),
                                    ),
                                });
                            }
                            Some(proto::local_management::request::OneOf::RunTaskRequest(
                                run_task_request,
                            )) => {
                                println!("LOCAL: run task request: {:?}", run_task_request);

                                message_write_buffer.encode(&proto::local_management::Reply {
                                    request_id: request.request_id,
                                    one_of: Some(
                                        proto::local_management::reply::OneOf::RunTaskReply(
                                            proto::RunTaskReply {
                                                error: Some("Some error".to_string()),
                                            },
                                        ),
                                    ),
                                });
                            }
                            _ => todo!(),
                        };
                    }

                    message_write_buffer.write(&mut named_pipe);

                    if let Some(time) = destroy_after {
                        if SystemTime::now() < time {
                            sleep(Duration::from_millis(200)).await;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                println!("Closed named pipe");
            });

            sleep(Duration::from_micros(1)).await;
        }
    });

    let mut client = WorkstationManagerClient::connect("http://[::1]:5122").await?;
    workstation_manager::listen(&mut client).await?;

    Ok(())
}
