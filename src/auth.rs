use super::*;

pub const DEFAULT_SERVICE_URL: &str = "http://skw-auth-service";

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
