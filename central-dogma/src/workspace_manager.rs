use super::proto;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct WorkspaceManager {}

#[tonic::async_trait]
impl proto::workspace_manager_server::WorkspaceManager for WorkspaceManager {
    async fn login(
        &self,
        reqest: Request<proto::LoginRequest>,
    ) -> Result<Response<proto::LoginReply>, Status> {
        let reply = proto::LoginReply {
            workstation_id: Option::Some(1),
        };

        Ok(Response::new(reply))
    }

    async fn start_application(
        &self,
        request: Request<proto::StartApplicationRequest>,
    ) -> Result<Response<proto::StartApplicationReply>, Status> {
        let reply = proto::StartApplicationReply {
            error: Option::Some("gfgfgf".to_string()),
        };

        Ok(Response::new(reply))
    }

    async fn run_task(
        &self,
        request: Request<proto::RunTaskRequest>,
    ) -> Result<Response<proto::RunTaskReply>, Status> {
        let reply = proto::RunTaskReply {
            error: Option::Some("gfgfgf".to_string()),
        };

        Ok(Response::new(reply))
    }
}
