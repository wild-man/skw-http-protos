use skw_lib_shared::prelude::iggy::ReceivedMessage;

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

        /// Queues a `PingTask` and replies `Created` (201); ping-mq-service sends the final reply
        /// (`Ping`, 200) under the same signature once the task is done.
        #[serde(rename_all = "PascalCase")]
        PingCreated {},

        /// Queues a `PingTask` and replies `Accepted` (202); ping-mq-service sends the final reply
        /// (`Ping`, 200) under the same signature once the task is done.
        #[serde(rename_all = "PascalCase")]
        PingAccepted {},

        /// Test helper: replies `Ping` (200) like `PingSigned`, but only after `PING_SLOW_DELAY`
        /// (ping-stream-service), so the request is reliably still pending meanwhile.
        #[serde(rename_all = "PascalCase")]
        PingSlow {},
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

    /// provisional reply to `PingCreated`; the final one is `Ping`
    #[serde(rename_all = "PascalCase")]
    PingCreated {},

    /// provisional reply to `PingAccepted`; the final one is `Ping`
    #[serde(rename_all = "PascalCase")]
    PingAccepted {},
}
crate::impl_try_from_string!(RpcPayload);

impl RpcReply for RpcPayload {
    fn status(&self) -> Option<SkwServiceStatus> {
        match self {
            RpcPayload::PingCreated {} => Some(SkwServiceStatus::Created),
            RpcPayload::PingAccepted {} => Some(SkwServiceStatus::Accepted),
            RpcPayload::Public {} | RpcPayload::Ping { .. } => None,
        }
    }
}
