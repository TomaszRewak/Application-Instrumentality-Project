mod application;
mod instance;
mod proto;
mod workstation_manager;

use std::io::Read;
use prost::Message;
use tokio::time::sleep;
use tokio::time::Duration;
use mio::windows::NamedPipe;
use application::Application;
use proto::client_server::workstation_manager_client::WorkstationManagerClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async move {
        println!("Trying to create a named pipe");
        let mut named_pipe = NamedPipe::new(r"\\.\pipe\magi-workspace-manager").unwrap();
        println!("Created named pipe");

        let mut buffer = [0u8; 1000];

        named_pipe.read(&mut buffer);

        //named_pipe.

        proto::local_management::Request::decode_length_delimited(buffer);

        sleep(Duration::from_millis(5000)).await;
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
