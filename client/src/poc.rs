#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobRegisterRequest {
    #[prost(bool, tag = "1")]
    pub register: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobRegisterResponse {
    #[prost(bool, tag = "1")]
    pub status: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderUpdateRequest {
    #[prost(int32, tag = "1")]
    pub value: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderUpdateResponse {
    #[prost(int32, tag = "1")]
    pub sum: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimpleRequest {
    #[prost(string, tag = "1")]
    pub user_id: std::string::String,
    #[prost(oneof = "simple_request::Request", tags = "2, 3")]
    pub request: ::std::option::Option<simple_request::Request>,
}
pub mod simple_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag = "2")]
        JobRegisterRequest(super::JobRegisterRequest),
        #[prost(message, tag = "3")]
        OrderUpdateRequest(super::OrderUpdateRequest),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimpleResponse {
    #[prost(string, tag = "1")]
    pub job_id: std::string::String,
    #[prost(string, tag = "4")]
    pub manager_id: std::string::String,
    #[prost(oneof = "simple_response::Response", tags = "2, 3")]
    pub response: ::std::option::Option<simple_response::Response>,
}
pub mod simple_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "2")]
        RegisterResponse(super::JobRegisterResponse),
        #[prost(message, tag = "3")]
        UpdateResponse(super::OrderUpdateResponse),
    }
}
#[doc = r" Generated client implementations."]
pub mod simple_connect_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct SimpleConnectClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SimpleConnectClient<tonic::transport::Channel> {
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
    impl<T> SimpleConnectClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn simple_connect(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::SimpleRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::SimpleResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/poc.SimpleConnect/SimpleConnect");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for SimpleConnectClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SimpleConnectClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SimpleConnectClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod simple_connect_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with SimpleConnectServer."]
    #[async_trait]
    pub trait SimpleConnect: Send + Sync + 'static {
        #[doc = "Server streaming response type for the SimpleConnect method."]
        type SimpleConnectStream: Stream<Item = Result<super::SimpleResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn simple_connect(
            &self,
            request: tonic::Request<tonic::Streaming<super::SimpleRequest>>,
        ) -> Result<tonic::Response<Self::SimpleConnectStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SimpleConnectServer<T: SimpleConnect> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: SimpleConnect> SimpleConnectServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for SimpleConnectServer<T>
    where
        T: SimpleConnect,
        B: HttpBody + Send + Sync + 'static,
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
                "/poc.SimpleConnect/SimpleConnect" => {
                    #[allow(non_camel_case_types)]
                    struct SimpleConnectSvc<T: SimpleConnect>(pub Arc<T>);
                    impl<T: SimpleConnect> tonic::server::StreamingService<super::SimpleRequest>
                        for SimpleConnectSvc<T>
                    {
                        type Response = super::SimpleResponse;
                        type ResponseStream = T::SimpleConnectStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::SimpleRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).simple_connect(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = SimpleConnectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: SimpleConnect> Clone for SimpleConnectServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: SimpleConnect> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SimpleConnect> tonic::transport::NamedService for SimpleConnectServer<T> {
        const NAME: &'static str = "poc.SimpleConnect";
    }
}
