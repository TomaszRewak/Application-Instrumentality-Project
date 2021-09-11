use super::proto;
use super::workstation::Workstation;
use futures_core::Stream;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status, Streaming};

pub struct WorkstationManager {
    workstations: Vec<Arc<Mutex<Workstation>>>,
}

impl WorkstationManager {
    pub fn new() -> WorkstationManager {
        let workstation_manager = WorkstationManager {
            workstations: vec![],
        };

        workstation_manager
    }
}

#[tonic::async_trait]
impl proto::client_server::workstation_manager_server::WorkstationManager for WorkstationManager {
    type ListenStream = Pin<
        Box<
            dyn Stream<Item = Result<proto::client_server::ServerToClientMessage, Status>> + Send + Sync + 'static,
        >,
    >;

    async fn listen(
        &self,
        request: Request<Streaming<proto::client_server::ClientToServerMessage>>,
    ) -> Result<Response<Self::ListenStream>, Status> {
        let stream = request.into_inner();
        let (sender, receiver) = async_channel::unbounded();

        let workstation = Workstation::new(stream, sender);

        let output = async_stream::try_stream! {
            while let Ok(message) = receiver.recv().await {
                yield message;
            }
        };

        Ok(Response::new(Box::pin(output) as Self::ListenStream))
    }
}
