#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateSale {
    /// Sale creator and the account which provides token (token_out) to the sale.
    /// When processing this message, token_out
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// token_in is a denom used to buy `token_out`. May be referred as a
    /// "quote currency".
    #[prost(string, tag = "2")]
    pub token_in: ::prost::alloc::string::String,
    /// token_out is a coin supply (denom + amount) to sell. May be referred as
    /// "base currency". The whole supply will be transferred from the creator
    /// to the module and will be sold during the sale.
    #[prost(message, optional, tag = "3")]
    pub token_out: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// Maximum fee the creator is going to pay for creating a sale. The creator
    /// will be charged params.SaleCreationFee. Transaction will fail if
    /// max_fee is smaller than params.SaleCreationFee. If empty, the creator
    /// doesn't accept any fee.
    #[prost(message, repeated, tag = "4")]
    pub max_fee: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// start time when the token sale starts.
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// duration time that the sale takes place over
    #[prost(message, optional, tag = "6")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// Recipient is the account which receives earned `token_in` from when the
    /// sale is finalized. If not defined (empty) the creator
    /// account will be used.
    #[prost(string, tag = "7")]
    pub recipient: ::prost::alloc::string::String,
    /// Name for the sale, max 40 characters, min 4. Required.
    #[prost(string, tag = "8")]
    pub name: ::prost::alloc::string::String,
    /// URL with sale and project details. Can be a link a link to IPFS,
    /// hackmd, project page, blog post... Max 120 characters. Must be
    /// valid agains Go url.ParseRequestURI. Required.
    #[prost(string, tag = "9")]
    pub url: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateSaleResponse {
    #[prost(uint64, tag = "1")]
    pub sale_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubscribe {
    /// sender is an account address adding a deposit
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// ID of an existing sale.
    #[prost(uint64, tag = "2")]
    pub sale_id: u64,
    /// number of sale.token_in staked by a user.
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdraw {
    /// sender is an account address subscribed to the sale_id
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// ID of a sale.
    #[prost(uint64, tag = "2")]
    pub sale_id: u64,
    /// amount of tokens_in to withdraw. Must be at most the amount of not spent
    /// tokens, unless set to null - then all remaining balance will be withdrawn.
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExitSale {
    /// sender is an account address exiting a sale
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// ID of an existing sale.
    #[prost(uint64, tag = "2")]
    pub sale_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExitSaleResponse {
    /// Purchased amount of "out" tokens withdrawn to the user.
    #[prost(string, tag = "1")]
    pub purchased: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFinalizeSale {
    /// sender is an account signing the message and triggering the finalization.
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// ID of an existing sale.
    #[prost(uint64, tag = "2")]
    pub sale_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFinalizeSaleResponse {
    /// Income amount of token_in sent to the sale recipient.
    #[prost(string, tag = "1")]
    pub income: ::prost::alloc::string::String,
}
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
        #[doc = " CreateSale creates new token sale. Anyone can create a new sale."]
        #[doc = " params.SaleBond OSMO will be charged as a bond (returned in FinalizeSale)"]
        #[doc = " to avoid spams."]
        #[doc = " The sale follows the streamswap functionality explained in the"]
        #[doc = " x/launchapd/spec"]
        pub async fn create_sale(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateSale>,
        ) -> Result<tonic::Response<super::MsgCreateSaleResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.streamswap.v1.Msg/CreateSale");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Subscribe to a token sale. Any use at any time before the sale end can join"]
        #[doc = " the sale by sending `token_in` to the Sale through the Subscribe msg."]
        #[doc = " During the sale, user `token_in` will be automatically charged every"]
        #[doc = " epoch to purchase `token_out`."]
        pub async fn subscribe(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSubscribe>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.streamswap.v1.Msg/Subscribe");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Withdraw sends back `amount` of unspent tokens_in to the user."]
        #[doc = " If `amount` is empty, it will default to all unspent tokens."]
        #[doc = " User can do it any time unless his deposit is empty."]
        pub async fn withdraw(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWithdraw>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.streamswap.v1.Msg/Withdraw");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ExitSale withdraws (by a user who subscribed to the sale) purchased"]
        #[doc = " tokens_out from the pool and remained tokens_in. Must be called before"]
        #[doc = " the sale end."]
        pub async fn exit_sale(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgExitSale>,
        ) -> Result<tonic::Response<super::MsgExitSaleResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.streamswap.v1.Msg/ExitSale");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " FinalizeSale clean ups the sale and sends income (earned tokens_in) to the"]
        #[doc = " Sale recipient. Returns error if called before the Sale end. Anyone can"]
        #[doc = " call this method."]
        pub async fn finalize_sale(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgFinalizeSale>,
        ) -> Result<tonic::Response<super::MsgFinalizeSaleResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.streamswap.v1.Msg/FinalizeSale");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sale {
    /// Destination for the earned token_in
    #[prost(string, tag = "1")]
    pub treasury: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub id: u64,
    /// token_out is a token denom to be bootstraped. May be referred as base
    /// currency, or a sale token.
    #[prost(string, tag = "3")]
    pub token_out: ::prost::alloc::string::String,
    /// token_in is a token denom used to buy sale tokens (`token_out`). May be
    /// referred as quote_currency or payment token.
    #[prost(string, tag = "4")]
    pub token_in: ::prost::alloc::string::String,
    /// total number of `tokens_out` to be sold during the continuous sale.
    #[prost(string, tag = "5")]
    pub token_out_supply: ::prost::alloc::string::String,
    /// start time when the token emission starts.
    #[prost(message, optional, tag = "6")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// end time when the token emission ends. Can't be bigger than start +
    /// 139years (to avoid round overflow)
    #[prost(message, optional, tag = "7")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Round number when the sale was last time updated.
    #[prost(int64, tag = "8")]
    pub round: i64,
    /// Last round of the Sale;
    #[prost(int64, tag = "9")]
    pub end_round: i64,
    /// amout of remaining token_out to sell
    #[prost(string, tag = "10")]
    pub out_remaining: ::prost::alloc::string::String,
    /// amount of token_out sold
    #[prost(string, tag = "11")]
    pub out_sold: ::prost::alloc::string::String,
    /// out token per share
    #[prost(string, tag = "12")]
    pub out_per_share: ::prost::alloc::string::String,
    /// total amount of currently staked coins (token_in) but not spent coins.
    #[prost(string, tag = "13")]
    pub staked: ::prost::alloc::string::String,
    /// total amount of earned coins (token_in)
    #[prost(string, tag = "14")]
    pub income: ::prost::alloc::string::String,
    /// total amount of shares
    #[prost(string, tag = "15")]
    pub shares: ::prost::alloc::string::String,
    /// Name for the sale.
    #[prost(string, tag = "20")]
    pub name: ::prost::alloc::string::String,
    /// URL with sale and project details.
    #[prost(string, tag = "21")]
    pub url: ::prost::alloc::string::String,
}
/// UserPosition represents user account in a sale
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserPosition {
    #[prost(string, tag = "1")]
    pub shares: ::prost::alloc::string::String,
    /// total number of currently staked tokens
    #[prost(string, tag = "2")]
    pub staked: ::prost::alloc::string::String,
    /// last token/share ratio
    #[prost(string, tag = "3")]
    pub out_per_share: ::prost::alloc::string::String,
    /// amount of token_in spent
    #[prost(string, tag = "4")]
    pub spent: ::prost::alloc::string::String,
    /// Amount of accumulated, not withdrawn, purchased tokens (token_out)
    #[prost(string, tag = "5")]
    pub purchased: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySales {
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySalesResponse {
    #[prost(message, repeated, tag = "1")]
    pub sales: ::prost::alloc::vec::Vec<Sale>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// Request type for Query/Sale
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySale {
    /// Sale ID
    #[prost(uint64, tag = "1")]
    pub sale_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySaleResponse {
    #[prost(message, optional, tag = "1")]
    pub sale: ::core::option::Option<Sale>,
}
/// Request type for Query/Sale
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUserPosition {
    /// ID of the Sale
    #[prost(uint64, tag = "1")]
    pub sale_id: u64,
    /// user account address
    #[prost(string, tag = "2")]
    pub user: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUserPositionResponse {
    #[prost(message, optional, tag = "1")]
    pub user_position: ::core::option::Option<UserPosition>,
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
        #[doc = " Returns list of Sales ordered by the creation time"]
        pub async fn sales(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySales>,
        ) -> Result<tonic::Response<super::QuerySalesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.streamswap.v1.Query/Sales");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the specific Sale object"]
        pub async fn sale(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySale>,
        ) -> Result<tonic::Response<super::QuerySaleResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.streamswap.v1.Query/Sale");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn user_position(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryUserPosition>,
        ) -> Result<tonic::Response<super::QueryUserPositionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.streamswap.v1.Query/UserPosition");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateSale {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub token_out: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSubscribe {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub sale_id: u64,
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventWithdraw {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub sale_id: u64,
    /// amount of staked token_in withdrawn by user.
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventExit {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub sale_id: u64,
    /// amount of purchased token_out sent to the user
    #[prost(string, tag = "3")]
    pub purchased: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventFinalizeSale {
    #[prost(uint64, tag = "1")]
    pub sale_id: u64,
    /// amount of earned tokens_in
    #[prost(string, tag = "3")]
    pub income: ::prost::alloc::string::String,
}
/// Params holds parameters for the streamswap module
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// fee charged when creating a new sale. The fee will go to the
    /// sale_fee_recipient unless it is not defined (empty).
    #[prost(message, repeated, tag = "1")]
    pub sale_creation_fee:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// bech32 address of the fee recipient
    #[prost(string, tag = "2")]
    pub sale_creation_fee_recipient: ::prost::alloc::string::String,
    /// minimum amount duration of time between the sale creation and the sale
    /// start time.
    #[prost(message, optional, tag = "3")]
    pub min_duration_until_start_time: ::core::option::Option<::prost_types::Duration>,
    /// minimum duration for every new sale.
    #[prost(message, optional, tag = "4")]
    pub min_sale_duration: ::core::option::Option<::prost_types::Duration>,
}
/// GenesisState defines the streamswap module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub sales: ::prost::alloc::vec::Vec<Sale>,
    #[prost(message, repeated, tag = "2")]
    pub user_positions: ::prost::alloc::vec::Vec<UserPositionKv>,
    #[prost(uint64, tag = "3")]
    pub next_sale_id: u64,
    #[prost(message, optional, tag = "4")]
    pub params: ::core::option::Option<Params>,
}
/// UserPositionKV is a record in genesis representing acc_address user position
/// of a sale_id sale.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserPositionKv {
    /// user account address
    #[prost(string, tag = "1")]
    pub acc_address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub sale_id: u64,
    #[prost(message, optional, tag = "3")]
    pub user_position: ::core::option::Option<UserPosition>,
}
