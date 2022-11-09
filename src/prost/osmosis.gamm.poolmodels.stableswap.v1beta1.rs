/// PoolParams defined the parameters that will be managed by the pool
/// governance in the future. This params are not managed by the chain
/// governance. Instead they will be managed by the token holders of the pool.
/// The pool's token holders are specified in future_pool_governor.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolParams {
    #[prost(string, tag="1")]
    pub swap_fee: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub exit_fee: ::prost::alloc::string::String,
}
/// Pool is the stableswap Pool struct
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub id: u64,
    #[prost(message, optional, tag="3")]
    pub pool_params: ::core::option::Option<PoolParams>,
    /// This string specifies who will govern the pool in the future.
    /// Valid forms of this are:
    /// {token name},{duration}
    /// {duration}
    /// where {token name} if specified is the token which determines the
    /// governor, and if not specified is the LP token for this pool.duration is
    /// a time specified as 0w,1w,2w, etc. which specifies how long the token
    /// would need to be locked up to count in governance. 0w means no lockup.
    #[prost(string, tag="4")]
    pub future_pool_governor: ::prost::alloc::string::String,
    /// sum of all LP shares
    #[prost(message, optional, tag="5")]
    pub total_shares: ::core::option::Option<super::super::super::super::super::cosmos::base::v1beta1::Coin>,
    /// assets in the pool
    #[prost(message, repeated, tag="6")]
    pub pool_liquidity: ::prost::alloc::vec::Vec<super::super::super::super::super::cosmos::base::v1beta1::Coin>,
    /// for calculation amognst assets with different precisions
    #[prost(uint64, repeated, packed="false", tag="7")]
    pub scaling_factor: ::prost::alloc::vec::Vec<u64>,
    /// scaling_factor_governor is the address can adjust pool scaling factors
    #[prost(string, tag="8")]
    pub scaling_factor_governor: ::prost::alloc::string::String,
}
/// ===================== MsgCreatePool
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateStableswapPool {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub pool_params: ::core::option::Option<PoolParams>,
    #[prost(message, repeated, tag="3")]
    pub initial_pool_liquidity: ::prost::alloc::vec::Vec<super::super::super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, repeated, packed="false", tag="4")]
    pub scaling_factors: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag="5")]
    pub future_pool_governor: ::prost::alloc::string::String,
}
/// Returns a poolID with custom poolName.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateStableswapPoolResponse {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
}
/// Sender must be the pool's scaling_factor_governor in order for the tx to
/// succeed. Adjusts stableswap scaling factors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgStableSwapAdjustScalingFactors {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
    #[prost(uint64, repeated, packed="false", tag="3")]
    pub scaling_factors: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgStableSwapAdjustScalingFactorsResponse {
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
        pub async fn create_stableswap_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateStableswapPool>,
        ) -> Result<
            tonic::Response<super::MsgCreateStableswapPoolResponse>,
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
                "/osmosis.gamm.poolmodels.stableswap.v1beta1.Msg/CreateStableswapPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn stable_swap_adjust_scaling_factors(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgStableSwapAdjustScalingFactors>,
        ) -> Result<
            tonic::Response<super::MsgStableSwapAdjustScalingFactorsResponse>,
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
                "/osmosis.gamm.poolmodels.stableswap.v1beta1.Msg/StableSwapAdjustScalingFactors",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
