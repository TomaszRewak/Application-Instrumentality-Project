use super::proto;
use async_channel::Sender;
use futures_util::StreamExt;
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::Status;
use tonic::Streaming;

pub struct Workstation {
    sender: Sender<proto::client_server::ServerToClientMessage>,
}

impl Workstation {
    pub fn new(
        receiver: Streaming<proto::client_server::ClientToServerMessage>,
        sender: Sender<proto::client_server::ServerToClientMessage>,
    ) -> Arc<Mutex<Workstation>> {
        let workstation = Arc::new(Mutex::new(Workstation { sender: sender }));

        Workstation::run(workstation.clone(), receiver);

        workstation
    }

    fn run(
        workstation: Arc<Mutex<Workstation>>,
        mut receiver: Streaming<proto::client_server::ClientToServerMessage>,
    ) {
        tokio::spawn(async move {
            while let Some(Ok(message)) = receiver.next().await {
                let workstation = workstation.lock().await;
                workstation.process(message).await;
            }
        });
    }

    async fn process(&self, message: proto::client_server::ClientToServerMessage) {
        match message.one_of {
            Some(proto::client_server::client_to_server_message::OneOf::LoginRequest(login_request)) => {
                self.process_login_request(login_request).await;
            }
            Some(proto::client_server::client_to_server_message::OneOf::StartApplicationReply(
                start_application_reply,
            )) => {
                self.process_start_application_reply(start_application_reply);
            }
            Some(proto::client_server::client_to_server_message::OneOf::RunTaskReply(run_task_reply)) => {
                self.process_run_task_reply(run_task_reply);
            }
            Some(proto::client_server::client_to_server_message::OneOf::Heartbeat(heartbeat)) => {
                self.process_heartbeat(heartbeat);
            }
            None => panic!("no message"),
        }
    }

    async fn process_login_request(&self, login_request: proto::LoginRequest) {
        println!("Received login request {:?}", login_request.hostname);

        self.sender.send(proto::client_server::ServerToClientMessage {
            one_of: Some(proto::client_server::server_to_client_message::OneOf::LoginReply(
                proto::LoginReply { error: None },
            )),
        }).await;
    }

    fn process_start_application_reply(
        &self,
        start_application_reply: proto::StartApplicationReply,
    ) {
    }

    fn process_run_task_reply(&self, run_task_reply: proto::RunTaskReply) {}

    fn process_heartbeat(&self, heartbeat: proto::Heartbeat)
    {
        println!("Received heartbeat {:?}", heartbeat);
    }
}
