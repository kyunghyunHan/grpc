use crate::pb::*;
use crate::pb::{chat_server::Chat, LoginRequest};
use futures::prelude::*;
use std::pin::Pin;
use tonic::{Request, Response, Status};
use tracing::info;
pub struct ChatService;
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

        Ok(Response::new(SendMessageResponse {}))
    }
    type GetMessagesStream = Pin<Box<dyn Stream<Item = Result<ChatMessage, tonic::Status>> + Send>>;

    async fn get_messages(
        &self,
        request: Request<GetMessagesRequest>,
    ) -> ChatResult<Self::GetMessagesStream> {
        println!("get_messages:{:?}", request.into_inner());
        todo!()
    }
}
