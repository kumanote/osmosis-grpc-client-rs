/// FeeToken is a struct that specifies a coin denom, and pool ID pair.
/// This marks the token as eligible for use as a tx fee asset in Osmosis.
/// Its price in osmo is derived through looking at the provided pool ID.
/// The pool ID must have osmo as one of its assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeToken {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
}
/// UpdateFeeTokenProposal is a gov Content type for adding a new whitelisted fee
/// token. It must specify a denom along with gamm pool ID to use as a spot price
/// calculator. It can be used to add a new denom to the whitelist It can also be
/// used to update the Pool to associate with the denom. If Pool ID is set to 0,
/// it will remove the denom from the whitelisted set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFeeTokenProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub feetoken: ::core::option::Option<FeeToken>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeTokensRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeTokensResponse {
    #[prost(message, repeated, tag = "1")]
    pub fee_tokens: ::prost::alloc::vec::Vec<FeeToken>,
}
/// QueryDenomSpotPriceRequest defines grpc request structure for querying spot
/// price for the specified tx fee denom
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomSpotPriceRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryDenomSpotPriceRequest defines grpc response structure for querying spot
/// price for the specified tx fee denom
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomSpotPriceResponse {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub spot_price: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomPoolIdRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomPoolIdResponse {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBaseDenomRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBaseDenomResponse {
    #[prost(string, tag = "1")]
    pub base_denom: ::prost::alloc::string::String,
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
        #[doc = " FeeTokens returns a list of all the whitelisted fee tokens and their"]
        #[doc = " corresponding pools It does not include the BaseDenom, which has its own"]
        #[doc = " query endpoint"]
        pub async fn fee_tokens(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryFeeTokensRequest>,
        ) -> Result<tonic::Response<super::QueryFeeTokensResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.txfees.v1beta1.Query/FeeTokens");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn denom_spot_price(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDenomSpotPriceRequest>,
        ) -> Result<tonic::Response<super::QueryDenomSpotPriceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.txfees.v1beta1.Query/DenomSpotPrice",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn denom_pool_id(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDenomPoolIdRequest>,
        ) -> Result<tonic::Response<super::QueryDenomPoolIdResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.txfees.v1beta1.Query/DenomPoolId");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn base_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBaseDenomRequest>,
        ) -> Result<tonic::Response<super::QueryBaseDenomResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.txfees.v1beta1.Query/BaseDenom");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// GenesisState defines the txfees module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(string, tag = "1")]
    pub basedenom: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub feetokens: ::prost::alloc::vec::Vec<FeeToken>,
}
