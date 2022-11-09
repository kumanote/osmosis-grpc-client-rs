/// Gauge is an object that stores and distributes yields to recipients who
/// satisfy certain conditions. Currently gauges support conditions around the
/// duration for which a given denom is locked.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gauge {
    /// id is the unique ID of a Gauge
    #[prost(uint64, tag="1")]
    pub id: u64,
    /// is_perpetual is a flag to show if it's a perpetual or non-perpetual gauge
    /// Non-perpetual gauges distribute their tokens equally per epoch while the
    /// gauge is in the active period. Perpetual gauges distribute all their tokens
    /// at a single time and only distribute their tokens again once the gauge is
    /// refilled, Intended for use with incentives that get refilled daily.
    #[prost(bool, tag="2")]
    pub is_perpetual: bool,
    /// distribute_to is where the gauge rewards are distributed to.
    /// This is queried via lock duration or by timestamp
    #[prost(message, optional, tag="3")]
    pub distribute_to: ::core::option::Option<super::lockup::QueryCondition>,
    /// coins is the total amount of coins that have been in the gauge
    /// Can distribute multiple coin denoms
    #[prost(message, repeated, tag="4")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    /// start_time is the distribution start time
    #[prost(message, optional, tag="5")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// num_epochs_paid_over is the number of total epochs distribution will be
    /// completed over
    #[prost(uint64, tag="6")]
    pub num_epochs_paid_over: u64,
    /// filled_epochs is the number of epochs distribution has been completed on
    /// already
    #[prost(uint64, tag="7")]
    pub filled_epochs: u64,
    /// distributed_coins are coins that have been distributed already
    #[prost(message, repeated, tag="8")]
    pub distributed_coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockableDurationsInfo {
    /// List of incentivised durations that gauges will pay out to
    #[prost(message, repeated, tag="1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
}
/// MsgCreateGauge creates a gague to distribute rewards to users
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGauge {
    /// is_perpetual shows if it's a perpetual or non-perpetual gauge
    /// Non-perpetual gauges distribute their tokens equally per epoch while the
    /// gauge is in the active period. Perpetual gauges distribute all their tokens
    /// at a single time and only distribute their tokens again once the gauge is
    /// refilled
    #[prost(bool, tag="1")]
    pub is_perpetual: bool,
    /// owner is the address of gauge creator
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    /// distribute_to show which lock the gauge should distribute to by time
    /// duration or by timestamp
    #[prost(message, optional, tag="3")]
    pub distribute_to: ::core::option::Option<super::lockup::QueryCondition>,
    /// coins are coin(s) to be distributed by the gauge
    #[prost(message, repeated, tag="4")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    /// start_time is the distribution start time
    #[prost(message, optional, tag="5")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// num_epochs_paid_over is the number of epochs distribution will be completed
    /// over
    #[prost(uint64, tag="6")]
    pub num_epochs_paid_over: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGaugeResponse {
}
/// MsgAddToGauge adds coins to a previously created gauge
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddToGauge {
    /// owner is the gauge owner's address
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// gauge_id is the ID of gauge that rewards are getting added to
    #[prost(uint64, tag="2")]
    pub gauge_id: u64,
    /// rewards are the coin(s) to add to gauge
    #[prost(message, repeated, tag="3")]
    pub rewards: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddToGaugeResponse {
}
/// Generated client implementations.
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
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
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn create_gauge(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateGauge>,
        ) -> Result<tonic::Response<super::MsgCreateGaugeResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.incentives.Msg/CreateGauge",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_to_gauge(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAddToGauge>,
        ) -> Result<tonic::Response<super::MsgAddToGaugeResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.incentives.Msg/AddToGauge",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleToDistributeCoinsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleToDistributeCoinsResponse {
    /// Coins that have yet to be distributed
    #[prost(message, repeated, tag="1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleDistributedCoinsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleDistributedCoinsResponse {
    /// Coins that have been distributed already
    #[prost(message, repeated, tag="1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugeByIdRequest {
    /// Gague ID being queried
    #[prost(uint64, tag="1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugeByIdResponse {
    /// Gauge that corresponds to provided gague ID
    #[prost(message, optional, tag="1")]
    pub gauge: ::core::option::Option<Gauge>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugesRequest {
    /// Pagination defines pagination for the request
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugesResponse {
    /// Upcoming and active gauges
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// Pagination defines pagination for the response
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveGaugesRequest {
    /// Pagination defines pagination for the request
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveGaugesResponse {
    /// Active gagues only
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// Pagination defines pagination for the response
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveGaugesPerDenomRequest {
    /// Desired denom when querying active gagues
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    /// Pagination defines pagination for the request
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveGaugesPerDenomResponse {
    /// Active gagues that match denom in query
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// Pagination defines pagination for the response
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpcomingGaugesRequest {
    /// Pagination defines pagination for the request
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpcomingGaugesResponse {
    /// Gauges whose distribution is upcoming
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// Pagination defines pagination for the response
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpcomingGaugesPerDenomRequest {
    /// Filter for upcoming gagues that match specific denom
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    /// Pagination defines pagination for the request
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpcomingGaugesPerDenomResponse {
    /// Upcoming gagues that match denom in query
    #[prost(message, repeated, tag="1")]
    pub upcoming_gauges: ::prost::alloc::vec::Vec<Gauge>,
    /// Pagination defines pagination for the response
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsEstRequest {
    /// Address that is being queried for future estimated rewards
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// Lock IDs included in future reward estimation
    #[prost(uint64, repeated, tag="2")]
    pub lock_ids: ::prost::alloc::vec::Vec<u64>,
    /// Upper time limit of reward estimation
    /// Lower limit is current epoch
    #[prost(int64, tag="3")]
    pub end_epoch: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsEstResponse {
    /// Estimated coin rewards that will be recieved at provided address
    /// from specified locks between current time and end epoch
    #[prost(message, repeated, tag="1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockableDurationsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockableDurationsResponse {
    /// Time durations that users can lock coins for in order to recieve rewards
    #[prost(message, repeated, tag="1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
}
/// Generated client implementations.
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Query defines the gRPC querier service
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
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
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// ModuleToDistributeCoins returns coins that are going to be distributed
        pub async fn module_to_distribute_coins(
            &mut self,
            request: impl tonic::IntoRequest<super::ModuleToDistributeCoinsRequest>,
        ) -> Result<
            tonic::Response<super::ModuleToDistributeCoinsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        /// ModuleDistributedCoins returns coins that are distributed by the module so
        /// far
        pub async fn module_distributed_coins(
            &mut self,
            request: impl tonic::IntoRequest<super::ModuleDistributedCoinsRequest>,
        ) -> Result<
            tonic::Response<super::ModuleDistributedCoinsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        /// GaugeByID returns gauges by their respective ID
        pub async fn gauge_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GaugeByIdRequest>,
        ) -> Result<tonic::Response<super::GaugeByIdResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.incentives.Query/GaugeByID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gauges returns both upcoming and active gauges
        pub async fn gauges(
            &mut self,
            request: impl tonic::IntoRequest<super::GaugesRequest>,
        ) -> Result<tonic::Response<super::GaugesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.incentives.Query/Gauges",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ActiveGauges returns active gauges
        pub async fn active_gauges(
            &mut self,
            request: impl tonic::IntoRequest<super::ActiveGaugesRequest>,
        ) -> Result<tonic::Response<super::ActiveGaugesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.incentives.Query/ActiveGauges",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ActiveGaugesPerDenom returns active gauges by denom
        pub async fn active_gauges_per_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::ActiveGaugesPerDenomRequest>,
        ) -> Result<
            tonic::Response<super::ActiveGaugesPerDenomResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        /// Returns scheduled gauges that have not yet occured
        pub async fn upcoming_gauges(
            &mut self,
            request: impl tonic::IntoRequest<super::UpcomingGaugesRequest>,
        ) -> Result<tonic::Response<super::UpcomingGaugesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.incentives.Query/UpcomingGauges",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// UpcomingGaugesPerDenom returns scheduled gauges that have not yet occured
        /// by denom
        pub async fn upcoming_gauges_per_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::UpcomingGaugesPerDenomRequest>,
        ) -> Result<
            tonic::Response<super::UpcomingGaugesPerDenomResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        /// RewardsEst returns an estimate of the rewards from now until a specified
        /// time in the future The querier either provides an address or a set of locks
        /// for which they want to find the associated rewards
        pub async fn rewards_est(
            &mut self,
            request: impl tonic::IntoRequest<super::RewardsEstRequest>,
        ) -> Result<tonic::Response<super::RewardsEstResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.incentives.Query/RewardsEst",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// LockableDurations returns lockable durations that are valid to distribute
        /// incentives for
        pub async fn lockable_durations(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLockableDurationsRequest>,
        ) -> Result<
            tonic::Response<super::QueryLockableDurationsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.incentives.Query/LockableDurations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Params holds parameters for the incentives module
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// distr_epoch_identifier is what epoch type distribution will be triggered by
    /// (day, week, etc.)
    #[prost(string, tag="1")]
    pub distr_epoch_identifier: ::prost::alloc::string::String,
}
/// GenesisState defines the incentives module's various parameters when first
/// initialized
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params are all the parameters of the module
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    /// gauges are all gauges that should exist at genesis
    #[prost(message, repeated, tag="2")]
    pub gauges: ::prost::alloc::vec::Vec<Gauge>,
    /// lockable_durations are all lockup durations that gauges can be locked for
    /// in order to recieve incentives
    #[prost(message, repeated, tag="3")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
    /// last_gauge_id is what the gauge number will increment from when creating
    /// the next gauge after genesis
    #[prost(uint64, tag="4")]
    pub last_gauge_id: u64,
}
