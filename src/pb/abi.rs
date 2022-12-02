#[derive(Hash, Eq, serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LoginRequest {
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
}
/// 서버에서 생성된 토큰
#[derive(Hash, Eq, serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Token {
    ///토큰 데이터
    #[prost(string, tag = "1")]
    pub data: ::prost::alloc::string::String,
}
///
///새로운 메시지를 보내기 위한 데이터 구조
///- 고유한 방 이름
///- "tyr"와 같은 사용자 이름일 수 있고 "engineering"이라는 그룹 이름일 수 있습니다.
///또는 쉼표로 구분된 사용자 이름 목록 "tyr,joe"
///- 채팅 내용
#[derive(Hash, Eq, serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct NewChatMessage {
    #[prost(string, tag = "1")]
    pub room: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
}
/// 빈 전송 메시지 응답
#[derive(Hash, Eq, serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SendMessageResponse {}
/// 비어 있는 메시지 가져오기 요청
#[derive(Hash, Eq, serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetMessagesRequest {}
///
///채팅 메시지
///- 이 메시지를 보낸 사람
///- 이 메시지가 전송된 방
///- 메시지 내용
///- 타임스탬프
#[derive(Hash, Eq, serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ChatMessage {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub room: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub timestamp: i64,
}
#[doc = r" Generated client implementations."]
pub mod chat_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = ""]
    #[doc = "- 사용자 로그인 및 토큰 가져오기"]
    #[doc = "- 방에 메시지 보내기"]
    #[doc = "- 구독하고 모든 메시지 받기"]
    #[derive(Debug, Clone)]
    pub struct ChatClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ChatClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ChatClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> ChatClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ChatClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn login(
            &mut self,
            request: impl tonic::IntoRequest<super::LoginRequest>,
        ) -> Result<tonic::Response<super::Token>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.Chat/Login");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn send_message(
            &mut self,
            request: impl tonic::IntoRequest<super::NewChatMessage>,
        ) -> Result<tonic::Response<super::SendMessageResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.Chat/SendMessage");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_messages(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMessagesRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ChatMessage>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.Chat/GetMessages");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod chat_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ChatServer."]
    #[async_trait]
    pub trait Chat: Send + Sync + 'static {
        async fn login(
            &self,
            request: tonic::Request<super::LoginRequest>,
        ) -> Result<tonic::Response<super::Token>, tonic::Status>;
        async fn send_message(
            &self,
            request: tonic::Request<super::NewChatMessage>,
        ) -> Result<tonic::Response<super::SendMessageResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the GetMessages method."]
        type GetMessagesStream: futures_core::Stream<Item = Result<super::ChatMessage, tonic::Status>>
            + Send
            + 'static;
        async fn get_messages(
            &self,
            request: tonic::Request<super::GetMessagesRequest>,
        ) -> Result<tonic::Response<Self::GetMessagesStream>, tonic::Status>;
    }
    #[doc = ""]
    #[doc = "- 사용자 로그인 및 토큰 가져오기"]
    #[doc = "- 방에 메시지 보내기"]
    #[doc = "- 구독하고 모든 메시지 받기"]
    #[derive(Debug)]
    pub struct ChatServer<T: Chat> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Chat> ChatServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ChatServer<T>
    where
        T: Chat,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/abi.Chat/Login" => {
                    #[allow(non_camel_case_types)]
                    struct LoginSvc<T: Chat>(pub Arc<T>);
                    impl<T: Chat> tonic::server::UnaryService<super::LoginRequest> for LoginSvc<T> {
                        type Response = super::Token;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LoginRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).login(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LoginSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.Chat/SendMessage" => {
                    #[allow(non_camel_case_types)]
                    struct SendMessageSvc<T: Chat>(pub Arc<T>);
                    impl<T: Chat> tonic::server::UnaryService<super::NewChatMessage> for SendMessageSvc<T> {
                        type Response = super::SendMessageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewChatMessage>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_message(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.Chat/GetMessages" => {
                    #[allow(non_camel_case_types)]
                    struct GetMessagesSvc<T: Chat>(pub Arc<T>);
                    impl<T: Chat> tonic::server::ServerStreamingService<super::GetMessagesRequest>
                        for GetMessagesSvc<T>
                    {
                        type Response = super::ChatMessage;
                        type ResponseStream = T::GetMessagesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMessagesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_messages(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMessagesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Chat> Clone for ChatServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Chat> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Chat> tonic::transport::NamedService for ChatServer<T> {
        const NAME: &'static str = "abi.Chat";
    }
}
