use super::proto;
use super::workstation::Workstation;
use futures_core::Stream;
use futures_util::StreamExt;
use std::collections::HashMap;
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};

#[derive(Debug, Default)]
pub struct WorkspaceManager {}

impl WorkspaceManager {}

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
        let mut stream = request.into_inner();
        let (sender, receiver) = async_channel::unbounded();
        let workstation = Workstation::create(stream, sender);

        let output = async_stream::try_stream! {
            while let Ok(message) = receiver.recv().await {
                yield message;
            }
        };

        Ok(Response::new(Box::pin(output) as Self::ListenStream))
    }
}
