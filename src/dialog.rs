use skw_lib_shared::prelude::iggy::{HeaderKey, ReceivedMessage};

use super::*;

/// Every dialog method is signed: the dialog owner is the request's API-KEY (auth entity id).
pub mod signed {
    use super::*;

    #[derive(IntoStaticStr, EnumIter, EnumProperty, Deserialize, Serialize, Debug, Clone)]
    #[serde(tag = "Method", content = "Params", rename_all = "PascalCase")]
    pub enum RpcMethod {
        /// `Model` defaults to `/llm/defaultModel`; `SystemPrompt` is stored as the first (System) message.
        #[strum(props(topic = "signed"))]
        #[serde(rename_all = "PascalCase")]
        CreateDialog {
            model: Option<String>,
            title: Option<String>,
            system_prompt: Option<String>,
        },

        /// Most recently active first; `Before` is the `UpdatedAt` of the last dialog of the previous page.
        #[strum(props(topic = "signed"))]
        #[serde(rename_all = "PascalCase")]
        ListDialogs { limit: Option<u32>, before: Option<DateTime<Utc>> },

        /// `AfterMessageId` returns only newer messages (polling for a pending answer).
        #[strum(props(topic = "signed"))]
        #[serde(rename_all = "PascalCase")]
        GetDialog { id: Uuid, after_message_id: Option<Uuid> },

        /// Stores the question and a Pending assistant message; the answer is produced by dialog-mq-service.
        #[strum(props(topic = "signed"))]
        #[serde(rename_all = "PascalCase")]
        Ask { dialog_id: Uuid, content: String },

        #[strum(props(topic = "signed"))]
        #[serde(rename_all = "PascalCase")]
        RenameDialog { id: Uuid, title: Option<String> },

        #[strum(props(topic = "signed"))]
        #[serde(rename_all = "PascalCase")]
        DeleteDialog { id: Uuid },
    }
    crate::impl_try_from_iggy_message!(RpcMethod);
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, EnumString, IntoStaticStr)]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, EnumString, IntoStaticStr)]
pub enum MessageStatus {
    Pending,
    Done,
    Failed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Dialog {
    pub id: Uuid,
    pub title: Option<String>,
    pub model: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Message {
    pub id: Uuid,
    pub role: MessageRole,
    pub status: MessageStatus,
    pub content: String,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum RpcPayload {
    CreateDialog(Dialog),

    ListDialogs(Vec<Dialog>),

    #[serde(rename_all = "PascalCase")]
    GetDialog { dialog: Dialog, messages: Vec<Message> },

    #[serde(rename_all = "PascalCase")]
    Ask { user_message_id: Uuid, assistant_message_id: Uuid },

    RenameDialog(Dialog),

    #[serde(rename_all = "PascalCase")]
    DeleteDialog {},
}
crate::impl_try_from_string!(RpcPayload);
