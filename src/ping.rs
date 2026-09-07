use skw_lib_shared::prelude::iggy::{HeaderKey, ReceivedMessage};

use super::*;

pub const DEFAULT_SERVICE_URL: &str = "http://skw-ping-http-service";

#[derive(IntoStaticStr, Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "Method", content = "Params", rename_all = "PascalCase")]
pub enum RpcMethod {
    #[serde(rename_all = "PascalCase")]
    Public {},

    #[serde(rename_all = "PascalCase")]
    Ping {},
}
crate::impl_try_from_iggy_message!(RpcMethod);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum RpcPayload {
    #[serde(rename_all = "PascalCase")]
    Ping { ts: DateTime<Utc>, count: u64 },

    #[serde(rename_all = "PascalCase")]
    Public {},
}
crate::impl_try_from_string!(RpcPayload);
