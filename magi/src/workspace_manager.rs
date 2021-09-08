use super::proto;
use std::error::Error;
use std::time;
use tonic::transport::Channel;
use tonic::Request;

pub async fn listen(
    client: &mut proto::workspace_manager_client::WorkspaceManagerClient<Channel>,
) -> Result<(), Box<dyn Error>> {
    let start = time::Instant::now();

    let outbound = async_stream::stream! {
        let hostname = match gethostname::gethostname().to_str() {
            Some(str) => Some(String::from(str)),
            None => None
        };

        println!("Sending login request {:?}", hostname);

        yield proto::ClientToServerMessage{ one_of: Some(proto::client_to_server_message::OneOf::LoginRequest(proto::LoginRequest{hostname: hostname})) };

        // let mut interval = time::interval(Duration::from_secs(1));
        // while let time = interval.tick().await {
        //     let elapsed = time.duration_since(start);
        //     let note = RouteNote {
        //         location: Some(Point {
        //             latitude: 409146138 + elapsed.as_secs() as i32,
        //             longitude: -746188906,
        //         }),
        //         message: format!("at {:?}", elapsed),
        //     };

        //     yield note;
        // }
    };

    let response = client.listen(Request::new(outbound)).await?;
    let mut inbound = response.into_inner();

    while let Some(note) = inbound.message().await? {
        println!("NOTE = {:?}", note);
    }

    Ok(())
}
