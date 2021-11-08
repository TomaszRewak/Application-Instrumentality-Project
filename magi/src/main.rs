mod application;
mod instance;
mod proto;
mod workstation_manager;

use common::MessageReadBuffer;
use common::MessageWriteBuffer;
use mio::windows::NamedPipe;
use proto::client_server::workstation_manager_client::WorkstationManagerClient;
use tokio::time::sleep;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async move {
        println!("Trying to create a named pipe");
        let mut named_pipe = NamedPipe::new(r"\\.\pipe\magi-workspace-manager").unwrap();
        named_pipe.connect();
        println!("Created named pipe");

        let mut message_read_buffer = MessageReadBuffer::new();
        let mut message_write_buffer = MessageWriteBuffer::new();

        loop {
            message_read_buffer.read(&mut named_pipe);

            if let Some(request) = message_read_buffer.decode::<proto::local_management::Request>()
            {
                match request.one_of {
                    Some(proto::local_management::request::OneOf::StartApplicationRequest(
                        start_application_request,
                    )) => {
                        message_write_buffer.encode(&proto::local_management::Reply {
                            one_of: Some(
                                proto::local_management::reply::OneOf::StartApplicationReply(
                                    proto::StartApplicationReply { error: None },
                                ),
                            ),
                        });
                    }
                    Some(proto::local_management::request::OneOf::RunTaskRequest(
                        run_task_request,
                    )) => message_write_buffer.encode(&proto::local_management::Reply {
                        one_of: Some(proto::local_management::reply::OneOf::RunTaskReply(
                            proto::RunTaskReply {
                                error: Some("Some error".to_string()),
                            },
                        )),
                    }),
                    None => todo!(),
                };
            }

            message_write_buffer.write(&mut named_pipe);

            sleep(Duration::from_millis(1000)).await;
        }
    });

    let mut client = WorkstationManagerClient::connect("http://[::1]:5122").await?;
    workstation_manager::listen(&mut client).await?;

    Ok(())

    // let a = StartApplicationReply {
    //     error: Option::Some("gfgfgf".to_string()),
    // };
    // let application_manager = ApplicationManager::default;
    // println!("{:?}", a.error);

    // let mut application = Application::new(
    //     "test".to_string(),
    //     r"C:\Windows\System32\notepad.exe".to_string(),
    //     "".to_string(),
    // );
    // application.add_instance("test".to_string());
    // application.start("test");
    // println!("Hello, world 1!");
}
