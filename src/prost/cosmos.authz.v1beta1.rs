/// GenericAuthorization gives the grantee unrestricted permissions to execute
/// the provided method on behalf of the granter's account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericAuthorization {
    /// Msg, identified by it's type URL, to grant unrestricted permissions to execute
    #[prost(string, tag="1")]
    pub msg: ::prost::alloc::string::String,
}
/// Grant gives permissions to execute
/// the provide method with expiration time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Grant {
    #[prost(message, optional, tag="1")]
    pub authorization: ::core::option::Option<::prost_types::Any>,
    #[prost(message, optional, tag="2")]
    pub expiration: ::core::option::Option<::prost_types::Timestamp>,
}
/// MsgGrant is a request type for Grant method. It declares authorization to the grantee
/// on behalf of the granter with the provided expiration time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGrant {
    #[prost(string, tag="1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub grant: ::core::option::Option<Grant>,
}
/// MsgExecResponse defines the Msg/MsgExecResponse response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub results: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// MsgExec attempts to execute the provided messages using
/// authorizations granted to the grantee. Each message should have only
/// one signer corresponding to the granter of the authorization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExec {
    #[prost(string, tag="1")]
    pub grantee: ::prost::alloc::string::String,
    /// Authorization Msg requests to execute. Each msg must implement Authorization interface
    /// The x/authz will try to find a grant matching (msg.signers\[0\], grantee, MsgTypeURL(msg))
    /// triple and validate it.
    #[prost(message, repeated, tag="2")]
    pub msgs: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
/// MsgGrantResponse defines the Msg/MsgGrant response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGrantResponse {
}
/// MsgRevoke revokes any authorization with the provided sdk.Msg type on the
/// granter's account with that has been granted to the grantee.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevoke {
    #[prost(string, tag="1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub msg_type_url: ::prost::alloc::string::String,
}
/// MsgRevokeResponse defines the Msg/MsgRevokeResponse response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevokeResponse {
}
/// Generated client implementations.
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Msg defines the authz Msg service.
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
        /// Grant grants the provided authorization to the grantee on the granter's
        /// account with the provided expiration time. If there is already a grant
        /// for the given (granter, grantee, Authorization) triple, then the grant
        /// will be overwritten.
        pub async fn grant(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgGrant>,
        ) -> Result<tonic::Response<super::MsgGrantResponse>, tonic::Status> {
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
                "/cosmos.authz.v1beta1.Msg/Grant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Exec attempts to execute the provided messages using
        /// authorizations granted to the grantee. Each message should have only
        /// one signer corresponding to the granter of the authorization.
        pub async fn exec(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgExec>,
        ) -> Result<tonic::Response<super::MsgExecResponse>, tonic::Status> {
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
                "/cosmos.authz.v1beta1.Msg/Exec",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Revoke revokes any authorization corresponding to the provided method name on the
        /// granter's account that has been granted to the grantee.
        pub async fn revoke(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRevoke>,
        ) -> Result<tonic::Response<super::MsgRevokeResponse>, tonic::Status> {
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
                "/cosmos.authz.v1beta1.Msg/Revoke",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// QueryGrantsRequest is the request type for the Query/Grants RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGrantsRequest {
    #[prost(string, tag="1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grantee: ::prost::alloc::string::String,
    /// Optional, msg_type_url, when set, will query only grants matching given msg type.
    #[prost(string, tag="3")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag="4")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryGrantsResponse is the response type for the Query/Authorizations RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGrantsResponse {
    /// authorizations is a list of grants granted for grantee by granter.
    #[prost(message, repeated, tag="1")]
    pub grants: ::prost::alloc::vec::Vec<Grant>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
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
        /// Returns list of `Authorization`, granted to the grantee by the granter.
        pub async fn grants(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGrantsRequest>,
        ) -> Result<tonic::Response<super::QueryGrantsResponse>, tonic::Status> {
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
                "/cosmos.authz.v1beta1.Query/Grants",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// EventGrant is emitted on Msg/Grant
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventGrant {
    /// Msg type URL for which an autorization is granted
    #[prost(string, tag="2")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// Granter account address
    #[prost(string, tag="3")]
    pub granter: ::prost::alloc::string::String,
    /// Grantee account address
    #[prost(string, tag="4")]
    pub grantee: ::prost::alloc::string::String,
}
/// EventRevoke is emitted on Msg/Revoke
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRevoke {
    /// Msg type URL for which an autorization is revoked
    #[prost(string, tag="2")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// Granter account address
    #[prost(string, tag="3")]
    pub granter: ::prost::alloc::string::String,
    /// Grantee account address
    #[prost(string, tag="4")]
    pub grantee: ::prost::alloc::string::String,
}
/// GenesisState defines the authz module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag="1")]
    pub authorization: ::prost::alloc::vec::Vec<GrantAuthorization>,
}
/// GrantAuthorization defines the GenesisState/GrantAuthorization type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrantAuthorization {
    #[prost(string, tag="1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub authorization: ::core::option::Option<::prost_types::Any>,
    #[prost(message, optional, tag="4")]
    pub expiration: ::core::option::Option<::prost_types::Timestamp>,
}
