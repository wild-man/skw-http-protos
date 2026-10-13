use skw_lib_shared::prelude::iggy::{HeaderKey, ReceivedMessage};

use super::*;

pub const DEFAULT_SERVICE_URL: &str = "http://skw-ping-http-service";

pub mod open {
    use super::*;

    #[derive(IntoStaticStr, EnumIter, EnumProperty, Deserialize, Serialize, Debug, Clone)]
    #[serde(tag = "Method", content = "Params", rename_all = "PascalCase")]
    pub enum RpcMethod {
        #[serde(rename_all = "PascalCase")]
        Public {},

        #[serde(rename_all = "PascalCase")]
        PingOpen {},
    }
    crate::impl_try_from_iggy_message!(RpcMethod);
}

pub mod signed {
    use super::*;
    #[derive(IntoStaticStr, EnumIter, EnumProperty, Deserialize, Serialize, Debug, Clone)]
    #[serde(tag = "Method", content = "Params", rename_all = "PascalCase")]
    pub enum RpcMethod {
        #[strum(props(topic = "signed"))]
        #[serde(rename_all = "PascalCase")]
        PingSigned {},
    }
    crate::impl_try_from_iggy_message!(RpcMethod);
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum RpcPayload {
    #[serde(rename_all = "PascalCase")]
    Public {},

    #[serde(rename_all = "PascalCase")]
    Ping { ts: DateTime<Utc>, count: u64 },
}
crate::impl_try_from_string!(RpcPayload);
