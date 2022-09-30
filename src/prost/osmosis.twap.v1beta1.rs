/// A TWAP record should be indexed in state by pool_id, (asset pair), timestamp
/// The asset pair assets should be lexicographically sorted.
/// Technically (pool_id, asset_0_denom, asset_1_denom, height) do not need to
/// appear in the struct however we view this as the wrong performance tradeoff
/// given SDK today. Would rather we optimize for readability and correctness,
/// than an optimal state storage format. The system bottleneck is elsewhere for
/// now.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TwapRecord {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    /// Lexicographically smaller denom of the pair
    #[prost(string, tag = "2")]
    pub asset0_denom: ::prost::alloc::string::String,
    /// Lexicographically larger denom of the pair
    #[prost(string, tag = "3")]
    pub asset1_denom: ::prost::alloc::string::String,
    /// height this record corresponds to, for debugging purposes
    #[prost(int64, tag = "4")]
    pub height: i64,
    /// This field should only exist until we have a global registry in the state
    /// machine, mapping prior block heights within {TIME RANGE} to times.
    #[prost(message, optional, tag = "5")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// We store the last spot prices in the struct, so that we can interpolate
    /// accumulator values for times between when accumulator records are stored.
    #[prost(string, tag = "6")]
    pub p0_last_spot_price: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub p1_last_spot_price: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub p0_arithmetic_twap_accumulator: ::prost::alloc::string::String,
    /// string geometric_twap_accumulator = 7 [(gogoproto.customtype) =
    /// "github.com/cosmos/cosmos-sdk/types.Dec",
    /// (gogoproto.nullable) = false];
    #[prost(string, tag = "9")]
    pub p1_arithmetic_twap_accumulator: ::prost::alloc::string::String,
    /// This field contains the time in which the last spot price error occured.
    /// It is used to alert the caller if they are getting a potentially erroneous
    /// TWAP, due to an unforeseen underlying error.
    #[prost(message, optional, tag = "11")]
    pub last_error_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Params holds parameters for the twap module
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(string, tag = "1")]
    pub prune_epoch_identifier: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub record_history_keep_period: ::core::option::Option<::prost_types::Duration>,
}
/// GenesisState defines the twap module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// twaps is the collection of all twap records.
    #[prost(message, repeated, tag = "1")]
    pub twaps: ::prost::alloc::vec::Vec<TwapRecord>,
    /// params is the container of twap parameters.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArithmeticTwapRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub base_asset: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote_asset: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "5")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArithmeticTwapResponse {
    #[prost(string, tag = "1")]
    pub arithmetic_twap: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArithmeticTwapToNowRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub base_asset: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote_asset: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArithmeticTwapToNowResponse {
    #[prost(string, tag = "1")]
    pub arithmetic_twap: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[doc = r" Generated client implementations."]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
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
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
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
            QueryClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::ParamsRequest>,
        ) -> Result<tonic::Response<super::ParamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.twap.v1beta1.Query/Params");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn arithmetic_twap(
            &mut self,
            request: impl tonic::IntoRequest<super::ArithmeticTwapRequest>,
        ) -> Result<tonic::Response<super::ArithmeticTwapResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.twap.v1beta1.Query/ArithmeticTwap");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn arithmetic_twap_to_now(
            &mut self,
            request: impl tonic::IntoRequest<super::ArithmeticTwapToNowRequest>,
        ) -> Result<tonic::Response<super::ArithmeticTwapToNowResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.twap.v1beta1.Query/ArithmeticTwapToNow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
