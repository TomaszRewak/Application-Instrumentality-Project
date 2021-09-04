use super::proto;
use futures_core::Stream;
use std::pin::Pin;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};

#[derive(Debug, Default)]
pub struct WorkspaceManager {}

#[tonic::async_trait]
impl proto::workspace_manager_server::WorkspaceManager for WorkspaceManager {
    type ListenStream = Pin<
        Box<
            dyn Stream<Item = Result<proto::ServerToClientMessage, Status>> + Send + Sync + 'static,
        >,
    >;

    async fn listen(
        &self,
        request: Request<Streaming<proto::ClientToServerMessage>>,
    ) -> Result<Response<Self::ListenStream>, Status> {
        unimplemented!()
    }
}
