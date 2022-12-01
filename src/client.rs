use crate::pb::{chat_client::ChatClient, *};
use dashmap::DashMap;
use std::ops::Deref;
use std::sync::Arc;
use tonic::codegen::InterceptedService;
use tonic::service::Interceptor;
use tonic::transport::Channel;
use tonic::Request;

#[derive(Default)]
struct Rooms(Arc<DashMap<String, Vec<ChatMessage>>>);

pub struct Client {
    username: String,
    conn: ChatClient<InterceptedService<Channel, AuthInterceptor>>,
    rooms: Rooms,
}

impl Deref for Rooms {
    type Target = DashMap<String, Vec<ChatMessage>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Client {
    pub async fn new(username: impl Into<String>) -> Self {
        let channel = Channel::from_static("http://[::1]:50051")
            .connect()
            .await
            .unwrap();
        let conn = ChatClient::with_interceptor(channel, AuthInterceptor);
        Self {
            username: username.into(),
            conn,
            rooms: Default::default(),
        }
    }
}
struct AuthInterceptor;
impl Interceptor for AuthInterceptor {
    fn call(&mut self, request: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
        todo!()
    }
}
