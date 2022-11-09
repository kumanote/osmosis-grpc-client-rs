/// SuperfluidAsset stores the pair of superfluid asset type and denom pair
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidAsset {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    /// AssetType indicates whether the superfluid asset is a native token or an lp
    /// share
    #[prost(enumeration="SuperfluidAssetType", tag="2")]
    pub asset_type: i32,
}
/// SuperfluidIntermediaryAccount takes the role of intermediary between LP token
/// and OSMO tokens for superfluid staking. The intermediary account is the
/// actual account responsible for delegation, not the validator account itself.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidIntermediaryAccount {
    /// Denom indicates the denom of the superfluid asset.
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub val_addr: ::prost::alloc::string::String,
    /// perpetual gauge for rewards distribution
    #[prost(uint64, tag="3")]
    pub gauge_id: u64,
}
/// The Osmo-Equivalent-Multiplier Record for epoch N refers to the osmo worth we
/// treat an LP share as having, for all of epoch N. Eventually this is intended
/// to be set as the Time-weighted-average-osmo-backing for the entire duration
/// of epoch N-1. (Thereby locking whats in use for epoch N as based on the prior
/// epochs rewards) However for now, this is not the TWAP but instead the spot
/// price at the boundary. For different types of assets in the future, it could
/// change.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsmoEquivalentMultiplierRecord {
    #[prost(int64, tag="1")]
    pub epoch_number: i64,
    /// superfluid asset denom, can be LP token or native token
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub multiplier: ::prost::alloc::string::String,
}
/// SuperfluidDelegationRecord is a struct used to indicate superfluid
/// delegations of an account in the state machine in a user friendly form.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidDelegationRecord {
    #[prost(string, tag="1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub delegation_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag="4")]
    pub equivalent_staked_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// LockIdIntermediaryAccountConnection is a struct used to indicate the
/// relationship between the underlying lock id and superfluid delegation done
/// via lp shares.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockIdIntermediaryAccountConnection {
    #[prost(uint64, tag="1")]
    pub lock_id: u64,
    #[prost(string, tag="2")]
    pub intermediary_account: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnpoolWhitelistedPools {
    #[prost(uint64, repeated, tag="1")]
    pub ids: ::prost::alloc::vec::Vec<u64>,
}
/// SuperfluidAssetType indicates whether the superfluid asset is
/// a native token itself or the lp share of a pool.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SuperfluidAssetType {
    Native = 0,
    /// SuperfluidAssetTypeLendingShare = 2; // for now not exist
    LpShare = 1,
}
impl SuperfluidAssetType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SuperfluidAssetType::Native => "SuperfluidAssetTypeNative",
            SuperfluidAssetType::LpShare => "SuperfluidAssetTypeLPShare",
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidDelegate {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub lock_id: u64,
    #[prost(string, tag="3")]
    pub val_addr: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidDelegateResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidUndelegate {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub lock_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidUndelegateResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidUnbondLock {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub lock_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidUnbondLockResponse {
}
// message MsgSuperfluidRedelegate {
//    string sender = 1 [ (gogoproto.moretags) = "yaml:\"sender\"" ];
//    uint64 lock_id = 2;
//    string new_val_addr = 3;
// }
// message MsgSuperfluidRedelegateResponse {}

/// MsgLockAndSuperfluidDelegate locks coins with the unbonding period duration,
/// and then does a superfluid lock from the newly created lockup, to the
/// specified validator addr.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLockAndSuperfluidDelegate {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="3")]
    pub val_addr: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLockAndSuperfluidDelegateResponse {
    #[prost(uint64, tag="1")]
    pub id: u64,
}
/// MsgUnPoolWhitelistedPool Unpools every lock the sender has, that is
/// associated with pool pool_id. If pool_id is not approved for unpooling by
/// governance, this is a no-op. Unpooling takes the locked gamm shares, and runs
/// "ExitPool" on it, to get the constituent tokens. e.g. z gamm/pool/1 tokens
/// ExitPools into constituent tokens x uatom, y uosmo. Then it creates a new
/// lock for every constituent token, with the duration associated with the lock.
/// If the lock was unbonding, the new lockup durations should be the time left
/// until unbond completion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnPoolWhitelistedPool {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnPoolWhitelistedPoolResponse {
    #[prost(uint64, repeated, tag="1")]
    pub exited_lock_ids: ::prost::alloc::vec::Vec<u64>,
}
/// Generated client implementations.
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Msg defines the Msg service.
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
        /// Execute superfluid delegation for a lockup
        pub async fn superfluid_delegate(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSuperfluidDelegate>,
        ) -> Result<
            tonic::Response<super::MsgSuperfluidDelegateResponse>,
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
                "/osmosis.superfluid.Msg/SuperfluidDelegate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Execute superfluid undelegation for a lockup
        pub async fn superfluid_undelegate(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSuperfluidUndelegate>,
        ) -> Result<
            tonic::Response<super::MsgSuperfluidUndelegateResponse>,
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
                "/osmosis.superfluid.Msg/SuperfluidUndelegate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// For a given lock that is being superfluidly undelegated,
        /// also unbond the underlying lock.
        pub async fn superfluid_unbond_lock(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSuperfluidUnbondLock>,
        ) -> Result<
            tonic::Response<super::MsgSuperfluidUnbondLockResponse>,
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
                "/osmosis.superfluid.Msg/SuperfluidUnbondLock",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Execute lockup lock and superfluid delegation in a single msg
        pub async fn lock_and_superfluid_delegate(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgLockAndSuperfluidDelegate>,
        ) -> Result<
            tonic::Response<super::MsgLockAndSuperfluidDelegateResponse>,
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
                "/osmosis.superfluid.Msg/LockAndSuperfluidDelegate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn un_pool_whitelisted_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUnPoolWhitelistedPool>,
        ) -> Result<
            tonic::Response<super::MsgUnPoolWhitelistedPoolResponse>,
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
                "/osmosis.superfluid.Msg/UnPoolWhitelistedPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Params holds parameters for the superfluid module
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// minimum_risk_factor is to be cut on OSMO equivalent value of lp tokens for
    /// superfluid staking, default: 5%. The minimum risk factor works
    /// to counter-balance the staked amount on chain's exposure to various asset
    /// volatilities, and have base staking be 'resistant' to volatility.
    #[prost(string, tag="1")]
    pub minimum_risk_factor: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetTypeRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetTypeResponse {
    #[prost(enumeration="SuperfluidAssetType", tag="1")]
    pub asset_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllAssetsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllAssetsResponse {
    #[prost(message, repeated, tag="1")]
    pub assets: ::prost::alloc::vec::Vec<SuperfluidAsset>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetMultiplierRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetMultiplierResponse {
    #[prost(message, optional, tag="1")]
    pub osmo_equivalent_multiplier: ::core::option::Option<OsmoEquivalentMultiplierRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidIntermediaryAccountInfo {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub val_addr: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub gauge_id: u64,
    #[prost(string, tag="4")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllIntermediaryAccountsRequest {
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllIntermediaryAccountsResponse {
    #[prost(message, repeated, tag="1")]
    pub accounts: ::prost::alloc::vec::Vec<SuperfluidIntermediaryAccountInfo>,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectedIntermediaryAccountRequest {
    #[prost(uint64, tag="1")]
    pub lock_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectedIntermediaryAccountResponse {
    #[prost(message, optional, tag="1")]
    pub account: ::core::option::Option<SuperfluidIntermediaryAccountInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalDelegationByValidatorForDenomRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalDelegationByValidatorForDenomResponse {
    #[prost(message, repeated, tag="1")]
    pub assets: ::prost::alloc::vec::Vec<Delegations>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Delegations {
    #[prost(string, tag="1")]
    pub val_addr: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub amount_sfsd: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub osmo_equivalent: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalSuperfluidDelegationsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalSuperfluidDelegationsResponse {
    #[prost(string, tag="1")]
    pub total_delegations: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidDelegationAmountRequest {
    #[prost(string, tag="1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidDelegationAmountResponse {
    #[prost(message, repeated, tag="1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidDelegationsByDelegatorRequest {
    #[prost(string, tag="1")]
    pub delegator_address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidDelegationsByDelegatorResponse {
    #[prost(message, repeated, tag="1")]
    pub superfluid_delegation_records: ::prost::alloc::vec::Vec<SuperfluidDelegationRecord>,
    #[prost(message, repeated, tag="2")]
    pub total_delegated_coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag="3")]
    pub total_equivalent_staked_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidUndelegationsByDelegatorRequest {
    #[prost(string, tag="1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidUndelegationsByDelegatorResponse {
    #[prost(message, repeated, tag="1")]
    pub superfluid_delegation_records: ::prost::alloc::vec::Vec<SuperfluidDelegationRecord>,
    #[prost(message, repeated, tag="2")]
    pub total_undelegated_coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag="3")]
    pub synthetic_locks: ::prost::alloc::vec::Vec<super::lockup::SyntheticLock>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidDelegationsByValidatorDenomRequest {
    #[prost(string, tag="1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidDelegationsByValidatorDenomResponse {
    #[prost(message, repeated, tag="1")]
    pub superfluid_delegation_records: ::prost::alloc::vec::Vec<SuperfluidDelegationRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateSuperfluidDelegatedAmountByValidatorDenomRequest {
    #[prost(string, tag="1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateSuperfluidDelegatedAmountByValidatorDenomResponse {
    #[prost(message, repeated, tag="1")]
    pub total_delegated_coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalDelegationByDelegatorRequest {
    #[prost(string, tag="1")]
    pub delegator_address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalDelegationByDelegatorResponse {
    #[prost(message, repeated, tag="1")]
    pub superfluid_delegation_records: ::prost::alloc::vec::Vec<SuperfluidDelegationRecord>,
    #[prost(message, repeated, tag="2")]
    pub delegation_response: ::prost::alloc::vec::Vec<super::super::cosmos::staking::v1beta1::DelegationResponse>,
    #[prost(message, repeated, tag="3")]
    pub total_delegated_coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag="4")]
    pub total_equivalent_staked_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// Generated client implementations.
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Query defines the gRPC querier service.
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
        /// Params returns the total set of superfluid parameters.
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status> {
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
                "/osmosis.superfluid.Query/Params",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns superfluid asset type, whether if it's a native asset or an lp
        /// share.
        pub async fn asset_type(
            &mut self,
            request: impl tonic::IntoRequest<super::AssetTypeRequest>,
        ) -> Result<tonic::Response<super::AssetTypeResponse>, tonic::Status> {
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
                "/osmosis.superfluid.Query/AssetType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns all registered superfluid assets.
        pub async fn all_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::AllAssetsRequest>,
        ) -> Result<tonic::Response<super::AllAssetsResponse>, tonic::Status> {
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
                "/osmosis.superfluid.Query/AllAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the osmo equivalent multiplier used in the most recent epoch.
        pub async fn asset_multiplier(
            &mut self,
            request: impl tonic::IntoRequest<super::AssetMultiplierRequest>,
        ) -> Result<tonic::Response<super::AssetMultiplierResponse>, tonic::Status> {
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
                "/osmosis.superfluid.Query/AssetMultiplier",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns all superfluid intermediary accounts.
        pub async fn all_intermediary_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::AllIntermediaryAccountsRequest>,
        ) -> Result<
            tonic::Response<super::AllIntermediaryAccountsResponse>,
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
                "/osmosis.superfluid.Query/AllIntermediaryAccounts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns intermediary account connected to a superfluid staked lock by id
        pub async fn connected_intermediary_account(
            &mut self,
            request: impl tonic::IntoRequest<super::ConnectedIntermediaryAccountRequest>,
        ) -> Result<
            tonic::Response<super::ConnectedIntermediaryAccountResponse>,
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
                "/osmosis.superfluid.Query/ConnectedIntermediaryAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the amount of delegations of specific denom for all validators
        pub async fn total_delegation_by_validator_for_denom(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryTotalDelegationByValidatorForDenomRequest,
            >,
        ) -> Result<
            tonic::Response<super::QueryTotalDelegationByValidatorForDenomResponse>,
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
                "/osmosis.superfluid.Query/TotalDelegationByValidatorForDenom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the total amount of osmo superfluidly staked.
        /// Response is denominated in uosmo.
        pub async fn total_superfluid_delegations(
            &mut self,
            request: impl tonic::IntoRequest<super::TotalSuperfluidDelegationsRequest>,
        ) -> Result<
            tonic::Response<super::TotalSuperfluidDelegationsResponse>,
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
                "/osmosis.superfluid.Query/TotalSuperfluidDelegations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the coins superfluid delegated for the delegator, validator, denom
        /// triplet
        pub async fn superfluid_delegation_amount(
            &mut self,
            request: impl tonic::IntoRequest<super::SuperfluidDelegationAmountRequest>,
        ) -> Result<
            tonic::Response<super::SuperfluidDelegationAmountResponse>,
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
                "/osmosis.superfluid.Query/SuperfluidDelegationAmount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns all the delegated superfluid poistions for a specific delegator.
        pub async fn superfluid_delegations_by_delegator(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SuperfluidDelegationsByDelegatorRequest,
            >,
        ) -> Result<
            tonic::Response<super::SuperfluidDelegationsByDelegatorResponse>,
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
                "/osmosis.superfluid.Query/SuperfluidDelegationsByDelegator",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns all the undelegating superfluid poistions for a specific delegator.
        pub async fn superfluid_undelegations_by_delegator(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SuperfluidUndelegationsByDelegatorRequest,
            >,
        ) -> Result<
            tonic::Response<super::SuperfluidUndelegationsByDelegatorResponse>,
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
                "/osmosis.superfluid.Query/SuperfluidUndelegationsByDelegator",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns all the superfluid positions of a specific denom delegated to one
        /// validator
        pub async fn superfluid_delegations_by_validator_denom(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SuperfluidDelegationsByValidatorDenomRequest,
            >,
        ) -> Result<
            tonic::Response<super::SuperfluidDelegationsByValidatorDenomResponse>,
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
                "/osmosis.superfluid.Query/SuperfluidDelegationsByValidatorDenom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the amount of a specific denom delegated to a specific validator
        /// This is labeled an estimate, because the way it calculates the amount can
        /// lead rounding errors from the true delegated amount
        pub async fn estimate_superfluid_delegated_amount_by_validator_denom(
            &mut self,
            request: impl tonic::IntoRequest<
                super::EstimateSuperfluidDelegatedAmountByValidatorDenomRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::EstimateSuperfluidDelegatedAmountByValidatorDenomResponse,
            >,
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
                "/osmosis.superfluid.Query/EstimateSuperfluidDelegatedAmountByValidatorDenom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the specified delegations for a specific delegator
        pub async fn total_delegation_by_delegator(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryTotalDelegationByDelegatorRequest,
            >,
        ) -> Result<
            tonic::Response<super::QueryTotalDelegationByDelegatorResponse>,
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
                "/osmosis.superfluid.Query/TotalDelegationByDelegator",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// GenesisState defines the module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    /// superfluid_assets defines the registered superfluid assets that have been
    /// registered via governance.
    #[prost(message, repeated, tag="2")]
    pub superfluid_assets: ::prost::alloc::vec::Vec<SuperfluidAsset>,
    /// osmo_equivalent_multipliers is the records of osmo equivalent amount of
    /// each superfluid registered pool, updated every epoch.
    #[prost(message, repeated, tag="3")]
    pub osmo_equivalent_multipliers: ::prost::alloc::vec::Vec<OsmoEquivalentMultiplierRecord>,
    /// intermediary_accounts is a secondary account for superfluid staking that
    /// plays an intermediary role between validators and the delegators.
    #[prost(message, repeated, tag="4")]
    pub intermediary_accounts: ::prost::alloc::vec::Vec<SuperfluidIntermediaryAccount>,
    #[prost(message, repeated, tag="5")]
    pub intemediary_account_connections: ::prost::alloc::vec::Vec<LockIdIntermediaryAccountConnection>,
}
