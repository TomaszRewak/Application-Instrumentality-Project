use super::proto;
use async_channel::Sender;
use futures_util::StreamExt;
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::Status;
use tonic::Streaming;

pub struct Workstation {
    sender: Sender<proto::ServerToClientMessage>,
}

impl Workstation {
    pub fn create(
        mut receiver: Streaming<proto::ClientToServerMessage>,
        sender: Sender<proto::ServerToClientMessage>,
    ) -> Arc<Mutex<Workstation>> {
        let workstation = Arc::new(Mutex::new(Workstation {
            sender: sender,
        }));

        let workstation_copy = workstation.clone();

        tokio::spawn(async move {
            while let Some(Ok(message)) = receiver.next().await {
                let workstation = workstation_copy.lock().await;
                workstation.process(message);
            }
        });

        workstation
    }

    fn process(&self, message: proto::ClientToServerMessage) {
        match message.one_of {
            Some(proto::client_to_server_message::OneOf::LoginRequest(login_request)) => {
                self.process_login_request(login_request);
            }
            Some(proto::client_to_server_message::OneOf::StartApplicationReply(
                start_application_reply,
            )) => {
                self.process_start_application_reply(start_application_reply);
            }
            Some(proto::client_to_server_message::OneOf::RunTaskReply(run_task_reply)) => {
                self.process_run_task_reply(run_task_reply);
            }
            None => panic!("no message"),
        }
    }

    async fn process_login_request(&self, login_request: proto::LoginRequest) {
        self.sender.send(proto::ServerToClientMessage {
            one_of: Some(proto::server_to_client_message::OneOf::LoginReply(
                proto::LoginReply { error: None },
            )),
        });
    }

    fn process_start_application_reply(
        &self,
        start_application_reply: proto::StartApplicationReply,
    ) {
    }

    fn process_run_task_reply(&self, run_task_reply: proto::RunTaskReply) {}
}
