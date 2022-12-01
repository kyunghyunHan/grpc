use crate::pb::*;
use crate::pb::{chat_server::Chat, LoginRequest};
use futures::prelude::*;
use std::pin::Pin;
use tonic::{Request, Response, Status};
pub struct ChatService;
pub type ChatResult<T> = Result<Response<T>, Status>;
#[tonic::async_trait]
impl Chat for ChatService {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<Token>, Status> {
        println!("login:{:?}", request.into_inner());
        todo!()
    }
    async fn send_message(
        &self,
        request: Request<NewChatMessage>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        println!("send_message:{:?}", request.into_inner());
        todo!()
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
