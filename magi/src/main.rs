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
        let mut buffer = BytesMut::with_capacity(1024);
        let mut next_message_length: usize = 0;
        let mut is_next_message_length_complete = false;
        let mut number_of_next_message_length_chunks = 0;

        loop {
            named_pipe.read(&mut buffer).unwrap();

            if !is_next_message_length_complete && !buffer.is_empty() {
                let message_size_chunk = buffer.get_u8() as usize;

                next_message_length = next_message_length
                    | ((message_size_chunk & 0b0111_1111)
                        << (number_of_next_message_length_chunks * 7));

                if message_size_chunk & 0b1000_0000 != 0 {
                    is_next_message_length_complete = true;
                }

                number_of_next_message_length_chunks += 1;
            }

            if is_next_message_length_complete && buffer.len() == next_message_length {
                let mut sub_buffer = buffer.take(next_message_length);

                proto::local_management::Request::decode(&mut sub_buffer).unwrap();

                buffer = sub_buffer.into_inner();
            }

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
