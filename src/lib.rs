#![allow(unused_imports)]
use skw_lib_shared::{
    AppError,
    prelude::{
        chrono::*,
        consts::*,
        iggy::{RpcMessageTrait, RpcReply},
        jsonrpc::*,
        serde::*,
        strum::*,
        uuid::*,
    },
};
use std::{fmt::Debug, str::FromStr};

pub mod auth;
pub mod dialog;
pub mod ping;

#[derive(Debug, Display)]
pub enum InvalidMessageError {
    InvalidHeaders,
    Serde(serde_json::Error),
}

/// запрос который приходит сервису по Iggy шине
/// параметризируется структурами http-potos::[mod]::HttpMethod
#[derive(Debug)]
pub struct JsonRpcServiceRequest<T: Deserialize<'static> + Debug> {
    pub http_method: String,
    pub http_path: String,
    pub api_key: Option<String>,
    pub signature: String,
    pub payload: Option<T>,
    pub err: Option<InvalidMessageError>,
}

impl<T> RpcMessageTrait for JsonRpcServiceRequest<T>
where
    T: Deserialize<'static> + Debug,
{
    type RpcMethod = T;

    fn signature(&self) -> &str {
        &self.signature
    }

    fn http_method(&self) -> &str {
        &self.http_method
    }

    fn http_path(&self) -> &str {
        &self.http_path
    }

    fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    fn into_rpc_method(self) -> Result<T, AppError> {
        self.payload
            .ok_or(AppError::Service(SkwServiceHttpError::BadRequest))
    }
}

macro_rules! impl_try_from_iggy_message {
    ($($ty:ty),* $(,)?) => {
        $(
            impl TryInto<JsonRpcServiceRequest<$ty>> for ReceivedMessage {
                type Error = InvalidMessageError;
                fn try_into(self) -> Result<JsonRpcServiceRequest<$ty>, Self::Error> {
                    use InvalidMessageError::*;

                    let headers = self
                        .message
                        .user_headers_map()
                        .map_err(|_| InvalidHeaders)?
                        .ok_or(InvalidHeaders)?;

                    let http_method = headers
                        .get(&HeaderKey::from_str(IGGY_HEADER_HTTP_METHOD).unwrap())
                        .ok_or(InvalidHeaders)?
                        .as_str()
                        .map_err(|_| InvalidHeaders)?
                        .to_string();

                    let http_path = headers
                        .get(&HeaderKey::from_str(IGGY_HEADER_HTTP_PATH).unwrap())
                        .ok_or(InvalidHeaders)?
                        .as_str()
                        .map_err(|_| InvalidHeaders)?
                        .to_string();

                    let signature = headers
                        .get(&HeaderKey::from_str(IGGY_HEADER_SIGNATURE).unwrap())
                        .ok_or(InvalidHeaders)?
                        .as_str()
                        .map_err(|_| InvalidHeaders)?
                        .to_string();

                    let api_key = headers
                        .get(&HeaderKey::from_str(IGGY_HEADER_API_KEY).unwrap())
                        .map(|m| m.as_str().map(|s| s.to_string()))
                        .transpose()
                        .map_err(|_| InvalidHeaders)?;

                    // dbg!(&http_method, &signature);
                    // dbg!(&self.message.payload);
                    let mut rpc = JsonRpcServiceRequest {
                        http_method,
                        api_key,
                        http_path,
                        signature,
                        payload: None,
                        err: None
                    };

                    match serde_json::from_slice(&self.message.payload) {
                        Ok(p) => rpc.payload = Some(p),
                        Err(e) => rpc.err = Some(InvalidMessageError::Serde(e))
                    };

                    Ok(rpc)
                }
            }
        )*
    };
}

pub(crate) use impl_try_from_iggy_message;

macro_rules! impl_try_from_string {
    ($($ty:ty),* $(,)?) => {
        $(
            impl TryFrom<String> for $ty {
                type Error = serde_json::Error;
                fn try_from(value: String) -> Result<Self, Self::Error> {
                    serde_json::from_str(&value)
                }
            }
        )*
    };
}
pub(crate) use impl_try_from_string;
