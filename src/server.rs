use crate::pb::*;
use crate::pb::{chat_server::Chat, LoginRequest};
use futures::prelude::*;
use std::pin::Pin;
use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{Request, Response, Status};
use tracing::info;
use tracing::log::warn;
pub struct ChatService {
    tx: broadcast::Sender<ChatMessage>,
}
pub type ChatResult<T> = Result<Response<T>, Status>;
#[tonic::async_trait]
impl Chat for ChatService {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<Token>, Status> {
        let info = request.into_inner();
        info!("login:{info:?}");
        let token = info.into_token();
        Ok(Response::new(token))
    }

    async fn send_message(
        &self,
        request: Request<NewChatMessage>,
    ) -> ChatResult<SendMessageResponse> {
        let sender = "tyr";
        let info = request.into_inner();
        info!("send_message:{info:?}");
        let msg = info.into_chat_message(sender);
        self.tx.send(msg).unwrap();
        Ok(Response::new(SendMessageResponse {}))
    }
    type GetMessagesStream = Pin<Box<dyn Stream<Item = Result<ChatMessage, tonic::Status>> + Send>>;

    async fn get_messages(
        &self,
        request: Request<GetMessagesRequest>,
    ) -> ChatResult<Self::GetMessagesStream> {
        let info = request.into_inner();
        let mut rx = self.tx.subscribe();
        info!("subscribe to message{info:?}");
        let (sender, receiver) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if let Err(_) = sender.send(Ok(msg)) {
                    warn!("Failed to send.Sender might be closed");
                }
            }
        });

        let stream = UnboundedReceiverStream::new(receiver);

        Ok(Response::new(Box::pin(stream)))
    }
}
