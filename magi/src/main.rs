mod application;
mod instance;
mod proto;
mod workstation_manager;
mod message_buffer;

use application::Application;
use bytes::Buf;
use bytes::BytesMut;
use mio::windows::NamedPipe;
use prost::Message;
use proto::client_server::workstation_manager_client::WorkstationManagerClient;
use std::io::Read;
use tokio::time::sleep;
use tokio::time::Duration;

use crate::message_buffer::MessageBuffer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async move {
        println!("Trying to create a named pipe");
        let mut named_pipe = NamedPipe::new(r"\\.\pipe\magi-workspace-manager").unwrap();
        println!("Created named pipe");

        let mut message_buffer = MessageBuffer::new();

        loop {
            message_buffer.read(&mut named_pipe);

            let message: Option<proto::local_management::Request> = message_buffer.decode();

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
