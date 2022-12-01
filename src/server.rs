use crate::pb::chat_server::ChatServer;
use crate::pb::*;
use crate::pb::{chat_server::Chat, LoginRequest};
use futures::prelude::*;
use std::pin::Pin;
use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::codegen::http::StatusCode;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use tracing::{info, warn};
const MAX_MESSAGES: usize = 100;
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
            //filter out message
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
impl Default for ChatService {
    fn default() -> Self {
        let (tx, _rx) = broadcast::channel(MAX_MESSAGES);
        Self { tx }
    }
}
pub async fn start() {
    let svc = ChatServer::with_interceptor(ChatService::default(), check_auth);

    let addr = "0.0.0.0:8000".parse().unwrap();

    info!("Listening on http://{}", addr);
    Server::builder()
        .add_service(svc)
        .serve(addr)
        .await
        .unwrap();
}

fn check_auth(mut req: Request<()>) -> Result<Request<()>, Status> {
    let token = match req.metadata().get("authorization") {
        Some(v) => {
            let data = v
                .to_str()
                .map_err(|_| Status::new(tonic::Code::Unauthenticated, "Invalid token format"))?;
            Token::new(data.strip_prefix("Bearer").unwrap())
        }
        None => Token::default(),
    };

    req.extensions_mut().insert(token);
    Ok(req)
}
