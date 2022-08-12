#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gauge {
    /// unique ID of a Gauge
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// flag to show if it's perpetual or multi-epoch
    /// distribution incentives by third party
    #[prost(bool, tag = "2")]
    pub is_perpetual: bool,
    /// Rewards are distributed to lockups that are are returned by at least one of
    /// these queries
    #[prost(message, optional, tag = "3")]
    pub distribute_to: ::core::option::Option<super::lockup::QueryCondition>,
    /// total amount of Coins that has been in the gauge.
    /// can distribute multiple coins
    #[prost(message, repeated, tag = "4")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    /// distribution start time
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// number of epochs distribution will be done
    #[prost(uint64, tag = "6")]
    pub num_epochs_paid_over: u64,
    /// number of epochs distributed already
    #[prost(uint64, tag = "7")]
    pub filled_epochs: u64,
    /// already distributed coins
    #[prost(message, repeated, tag = "8")]
    pub distributed_coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockableDurationsInfo {
    #[prost(message, repeated, tag = "1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGauge {
    /// flag to show if it's perpetual or multi-epoch
    /// distribution incentives by third party
    #[prost(bool, tag = "1")]
    pub is_perpetual: bool,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// distribute condition of a lock which meet one of these conditions
    #[prost(message, optional, tag = "3")]
    pub distribute_to: ::core::option::Option<super::lockup::QueryCondition>,
    /// can distribute multiple coins
    #[prost(message, repeated, tag = "4")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    /// distribution start time
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// number of epochs distribution will be done
    #[prost(uint64, tag = "6")]
    pub num_epochs_paid_over: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGaugeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddToGauge {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub gauge_id: u64,
    #[prost(message, repeated, tag = "3")]
    pub rewards: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddToGaugeResponse {}
#[doc = r" Generated client implementations."]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MsgClient<tonic::transport::Channel> {
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
    impl<T> MsgClient<T>
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> MsgClient<InterceptedService<T, F>>
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
            MsgClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_gauge(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateGauge>,
        ) -> Result<tonic::Response<super::MsgCreateGaugeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.incentives.Msg/CreateGauge");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_to_gauge(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAddToGauge>,
        ) -> Result<tonic::Response<super::MsgAddToGaugeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.incentives.Msg/AddToGauge");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleToDistributeCoinsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleToDistributeCoinsResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleDistributedCoinsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleDistributedCoinsResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugeByIdRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugeByIdResponse {
    #[prost(message, optional, tag = "1")]
    pub gauge: ::core::option::Option<Gauge>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugesRequest {
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugesResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveGaugesRequest {
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveGaugesResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveGaugesPerDenomRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveGaugesPerDenomResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpcomingGaugesRequest {
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpcomingGaugesResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpcomingGaugesPerDenomRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpcomingGaugesPerDenomResponse {
    #[prost(message, repeated, tag = "1")]
    pub upcoming_gauges: ::prost::alloc::vec::Vec<Gauge>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsEstRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag = "2")]
    pub lock_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(int64, tag = "3")]
    pub end_epoch: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsEstResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockableDurationsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockableDurationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
}
#[doc = r" Generated client implementations."]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Query defines the gRPC querier service."]
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
        #[doc = " returns coins that is going to be distributed"]
        pub async fn module_to_distribute_coins(
            &mut self,
            request: impl tonic::IntoRequest<super::ModuleToDistributeCoinsRequest>,
        ) -> Result<tonic::Response<super::ModuleToDistributeCoinsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.incentives.Query/ModuleToDistributeCoins",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " returns coins that are distributed by module so far"]
        pub async fn module_distributed_coins(
            &mut self,
            request: impl tonic::IntoRequest<super::ModuleDistributedCoinsRequest>,
        ) -> Result<tonic::Response<super::ModuleDistributedCoinsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.incentives.Query/ModuleDistributedCoins",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " returns Gauge by id"]
        pub async fn gauge_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GaugeByIdRequest>,
        ) -> Result<tonic::Response<super::GaugeByIdResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.incentives.Query/GaugeByID");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " returns gauges both upcoming and active"]
        pub async fn gauges(
            &mut self,
            request: impl tonic::IntoRequest<super::GaugesRequest>,
        ) -> Result<tonic::Response<super::GaugesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.incentives.Query/Gauges");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " returns active gauges"]
        pub async fn active_gauges(
            &mut self,
            request: impl tonic::IntoRequest<super::ActiveGaugesRequest>,
        ) -> Result<tonic::Response<super::ActiveGaugesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.incentives.Query/ActiveGauges");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " returns active gauges per denom"]
        pub async fn active_gauges_per_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::ActiveGaugesPerDenomRequest>,
        ) -> Result<tonic::Response<super::ActiveGaugesPerDenomResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.incentives.Query/ActiveGaugesPerDenom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " returns scheduled gauges"]
        pub async fn upcoming_gauges(
            &mut self,
            request: impl tonic::IntoRequest<super::UpcomingGaugesRequest>,
        ) -> Result<tonic::Response<super::UpcomingGaugesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.incentives.Query/UpcomingGauges");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " returns scheduled gauges per denom"]
        pub async fn upcoming_gauges_per_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::UpcomingGaugesPerDenomRequest>,
        ) -> Result<tonic::Response<super::UpcomingGaugesPerDenomResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.incentives.Query/UpcomingGaugesPerDenom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " RewardsEst returns an estimate of the rewards at a future specific time."]
        #[doc = " The querier either provides an address or a set of locks"]
        #[doc = " for which they want to find the associated rewards."]
        pub async fn rewards_est(
            &mut self,
            request: impl tonic::IntoRequest<super::RewardsEstRequest>,
        ) -> Result<tonic::Response<super::RewardsEstResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.incentives.Query/RewardsEst");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " returns lockable durations that are valid to give incentives"]
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
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.incentives.Query/LockableDurations");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Params holds parameters for the incentives module
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// distribution epoch identifier
    #[prost(string, tag = "1")]
    pub distr_epoch_identifier: ::prost::alloc::string::String,
}
/// GenesisState defines the incentives module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of the module
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub gauges: ::prost::alloc::vec::Vec<Gauge>,
    #[prost(message, repeated, tag = "3")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
    #[prost(uint64, tag = "4")]
    pub last_gauge_id: u64,
}
