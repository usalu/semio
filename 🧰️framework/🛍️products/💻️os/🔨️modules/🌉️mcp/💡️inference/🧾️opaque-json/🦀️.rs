//! 🧾️ Opaque json of the foundation crate: Gateway vocabulary: errors, handles, audit lane, reports, local session files and delegated credentials.

//#region 💡️InferenceJobWire

pub const INFERENCE_RESPONSE_MAX_BYTES:usize=64*1024;

//#endregion 💡️InferenceJobWire

pub mod dsl_json {
    use serde::{Deserialize,Serialize};
    pub fn serialize<S:serde::Serializer>(value:&semio_framework_value::DslValue,serializer:S)->Result<S::Ok,S::Error> {
        semio_framework_os_kernel::os_directory::client::document_http::CompiledDocumentHttpPortV1::validate_payload(value,super::INFERENCE_RESPONSE_MAX_BYTES).map_err(|code|serde::ser::Error::custom(format!("opaque payload rejected: {code:?}")))?;
        let json:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(value)).map_err(serde::ser::Error::custom)?;json.serialize(serializer)
    }
    pub fn deserialize<'de,D:serde::Deserializer<'de>>(deserializer:D)->Result<semio_framework_value::DslValue,D::Error> {
        let json=serde_json::Value::deserialize(deserializer)?;semio_framework_pack_json::from_json_str(&json.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(serde::de::Error::custom)
    }
}

pub mod dsl_json_optional {
    use serde::{Deserialize,Serialize};
    pub fn serialize<S:serde::Serializer>(value:&Option<semio_framework_value::DslValue>,serializer:S)->Result<S::Ok,S::Error> {
        let json=value.as_ref().map(|value|serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(value))).transpose().map_err(serde::ser::Error::custom)?;json.serialize(serializer)
    }
    pub fn deserialize<'de,D:serde::Deserializer<'de>>(deserializer:D)->Result<Option<semio_framework_value::DslValue>,D::Error> {
        Option::<serde_json::Value>::deserialize(deserializer)?.map(|json|semio_framework_pack_json::from_json_str(&json.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(serde::de::Error::custom)).transpose()
    }
}
