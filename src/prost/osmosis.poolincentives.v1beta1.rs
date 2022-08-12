#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// minted_denom is the denomination of the coin expected to be minted by the
    /// minting module. Pool-incentives module doesn’t actually mint the coin
    /// itself, but rather manages the distribution of coins that matches the
    /// defined minted_denom.
    #[prost(string, tag = "1")]
    pub minted_denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockableDurationsInfo {
    #[prost(message, repeated, tag = "1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistrInfo {
    #[prost(string, tag = "1")]
    pub total_weight: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub records: ::prost::alloc::vec::Vec<DistrRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistrRecord {
    #[prost(uint64, tag = "1")]
    pub gauge_id: u64,
    #[prost(string, tag = "2")]
    pub weight: ::prost::alloc::string::String,
}
/// ReplacePoolIncentivesProposal is a gov Content type for updating the pool
/// incentives. If a ReplacePoolIncentivesProposal passes, the proposal’s records
/// override the existing DistrRecords set in the module. Each record has a
/// specified gauge id and weight, and the incentives are distributed to each
/// gauge according to weight/total_weight. The incentives are put in the fee
/// pool and it is allocated to gauges and community pool by the DistrRecords
/// configuration. Note that gaugeId=0 represents the community pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplacePoolIncentivesProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub records: ::prost::alloc::vec::Vec<DistrRecord>,
}
// UpdatePoolIncentivesProposal is a gov Content type for updating the pool
// incentives. If a UpdatePoolIncentivesProposal passes, all the DistrRecords
// in the proposals are edited. An existing DistrRecord is not overriden unless
// explicitly included in the proposal.
// This differs from an ReplacePoolIncentivesProposal because it only does an
// in place update of the DistrRecords for gauges explicitly mentioned in the
// proposal.

/// For example: if the existing DistrRecords were:
/// [(Gauge 0, 5), (Gauge 1, 6), (Gauge 2, 6)]
/// An UpdatePoolIncentivesProposal includes
/// [(Gauge 1, 0), (Gauge 2, 4), (Gauge 3, 10)]
/// This would delete Gauge 1, Edit Gauge 2, and Add Gauge 3
/// The result DistrRecords in state would be:
/// [(Gauge 0, 5), (Gauge 2, 4), (Gauge 3, 10)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePoolIncentivesProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub records: ::prost::alloc::vec::Vec<DistrRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGaugeIdsRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGaugeIdsResponse {
    #[prost(message, repeated, tag = "1")]
    pub gauge_ids_with_duration:
        ::prost::alloc::vec::Vec<query_gauge_ids_response::GaugeIdWithDuration>,
}
/// Nested message and enum types in `QueryGaugeIdsResponse`.
pub mod query_gauge_ids_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GaugeIdWithDuration {
        #[prost(uint64, tag = "1")]
        pub gauge_id: u64,
        #[prost(message, optional, tag = "2")]
        pub duration: ::core::option::Option<::prost_types::Duration>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDistrInfoRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDistrInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub distr_info: ::core::option::Option<DistrInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockableDurationsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockableDurationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIncentivizedPoolsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncentivizedPool {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(message, optional, tag = "2")]
    pub lockable_duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(uint64, tag = "3")]
    pub gauge_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIncentivizedPoolsResponse {
    #[prost(message, repeated, tag = "1")]
    pub incentivized_pools: ::prost::alloc::vec::Vec<IncentivizedPool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExternalIncentiveGaugesRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExternalIncentiveGaugesResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<super::super::incentives::Gauge>,
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
        #[doc = " GaugeIds takes the pool id and returns the matching gauge ids and durations"]
        pub async fn gauge_ids(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGaugeIdsRequest>,
        ) -> Result<tonic::Response<super::QueryGaugeIdsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolincentives.v1beta1.Query/GaugeIds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn distr_info(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDistrInfoRequest>,
        ) -> Result<tonic::Response<super::QueryDistrInfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolincentives.v1beta1.Query/DistrInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolincentives.v1beta1.Query/Params",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn lockable_durations(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLockableDurationsRequest>,
        ) -> Result<tonic::Response<super::QueryLockableDurationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolincentives.v1beta1.Query/LockableDurations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn incentivized_pools(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryIncentivizedPoolsRequest>,
        ) -> Result<tonic::Response<super::QueryIncentivizedPoolsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolincentives.v1beta1.Query/IncentivizedPools",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn external_incentive_gauges(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryExternalIncentiveGaugesRequest>,
        ) -> Result<tonic::Response<super::QueryExternalIncentiveGaugesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolincentives.v1beta1.Query/ExternalIncentiveGauges",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// GenesisState defines the pool incentives module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the paramaters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
    #[prost(message, optional, tag = "3")]
    pub distr_info: ::core::option::Option<DistrInfo>,
}
