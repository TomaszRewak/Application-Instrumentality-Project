use super::proto;
use std::error::Error;
use tokio::time;
use tokio::time::Duration;
use tonic::transport::Channel;
use tonic::Request;

pub async fn listen(
    client: &mut proto::client_server::workstation_manager_client::WorkstationManagerClient<Channel>,
) -> Result<(), Box<dyn Error>> {
    let start = time::Instant::now();
    let (sender, receiver) = async_channel::unbounded();

    let outbound = async_stream::stream! {
        while let Ok(message) = receiver.recv().await {
            yield message;
        }
    };

    let response = client.listen(Request::new(outbound)).await?;
    let mut inbound = response.into_inner();

    let hostname = match gethostname::gethostname().to_str() {
        Some(str) => Some(String::from(str)),
        None => None,
    };

    println!("Sending login request {:?}", hostname);

    sender
        .send(proto::client_server::ClientToServerMessage {
            one_of: Some(proto::client_server::client_to_server_message::OneOf::LoginRequest(
                proto::LoginRequest { hostname: hostname },
            )),
        })
        .await;

    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(3));
        while let time = interval.tick().await {
            println!("Sending a heartbeat = {:?}", time);

            sender
                .send(proto::client_server::ClientToServerMessage {
                    one_of: Some(proto::client_server::client_to_server_message::OneOf::Heartbeat(
                        proto::Heartbeat {},
                    )),
                })
                .await;
        }
    });

    while let Some(note) = inbound.message().await? {
        println!("NOTE = {:?}", note);
    }

    Ok(())
}
