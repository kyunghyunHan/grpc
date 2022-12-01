use tonic::codegen::http::request;
use tonic::{Request, Response, Status};

use crate::pb::*;
use crate::pb::{chat_server::Chat, LoginRequest};
pub struct ChatService;

// #[tonic::async_trait]
// impl Chat for ChatService {
//     async fn login(&self, request: Request<LoginRequest>) -> Result<Response<Token>, Status> {
//         println!("login:{:?}", request.into_inner());
//         todo!()
//     }
//     async fn send_message(
//         &self,
//         request: Request<NewChatMessage>,
//     ) -> Result<Response<SendMessageResponse>, Status> {
//         println!("send_message:{:?}", request.into_inner());
//         todo!()
//     }
//     async fn get_messages(
//         &self,
//         request: Request<GetChatMessagesRequest>,
//     ) -> ChatResult<GetMessagesRequest> {

//     }
// }
