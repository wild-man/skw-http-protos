use skw_lib_shared::prelude::iggy::{HeaderKey, ReceivedMessage};

use super::*;

pub const DEFAULT_SERVICE_URL: &str = "http://skw-auth-http-service";

#[derive(IntoStaticStr, Deserialize, Serialize, Debug, Clone, EnumProperty, EnumIter)]
#[serde(tag = "Method", content = "Params", rename_all = "PascalCase")]
pub enum RpcMethod {
    #[serde(rename_all = "PascalCase")]
    CreateKey { store_private_key: bool },
}
crate::impl_try_from_iggy_message!(RpcMethod);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum RpcPayload {
    #[serde(rename_all = "PascalCase")]
    CreateKey {
        id: Uuid,
        public_key: String,
        private_key: String,
        enabled: bool,
    },
}
crate::impl_try_from_string!(RpcPayload);
impl RpcReply for RpcPayload {}

pub mod internal {
    use super::*;

    #[derive(IntoStaticStr, Deserialize, Serialize, Debug, Clone)]
    #[serde(tag = "Method", content = "Params", rename_all = "PascalCase")]
    pub enum HttpMethod {
        #[serde(rename_all = "PascalCase")]
        GetKeyById { id: Uuid },
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "PascalCase")]
    pub enum HttpPayload {
        #[serde(rename_all = "PascalCase")]
        GetKeyById {
            id: Uuid,
            public_key: String,
            enabled: bool,
        },
    }
    crate::impl_try_from_string!(HttpPayload);
}
