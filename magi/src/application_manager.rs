use super::proto as proto;

use tonic::{Request, Response, Status};

pub struct ApplicationManager {}

impl proto::application_manager_server::ApplicationManager for ApplicationManager {
    async fn start_application(&self, request: Request<proto::StartApplicationRequest>) -> Reply<Response<proto::StartApplicationReply>, Status>
    {
        let relpy = proto::StartApplicationReply {
            error: Option::Some("gfgfgf".to_string())
        };

        Ok(Response::new(reply))
    }
}
