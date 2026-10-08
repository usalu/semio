//! 🚪️ Native JSON snapshot admission and emission.

use crate::opening_config::{OpeningPreferences, UiPreferences};
use crate::opening_config::mutations::{IdentitySetting, LocalCatalog, LocalFolderBindings, MergePolicySetting};

/// 📥️ Decodes the canonical opening-preferences JSON projection.
pub fn decode_opening_preferences_json(text: &str) -> Result<OpeningPreferences, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📥️ Decodes the canonical OS UI-preferences JSON projection.
pub fn decode_ui_preferences_json(text: &str) -> Result<UiPreferences, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📤️ Encodes opening preferences to their canonical camel-case JSON projection.
pub fn encode_opening_preferences_json(snapshot: &OpeningPreferences) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📤️ Encodes OS UI preferences to their canonical camel-case JSON projection.
pub fn encode_ui_preferences_json(snapshot: &UiPreferences) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ Decodes the canonical local folder bindings JSON projection.
pub fn decode_local_folder_bindings_json(text: &str) -> Result<LocalFolderBindings, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📤️ Encodes the local folder bindings to their canonical camel-case JSON projection.
pub fn encode_local_folder_bindings_json(snapshot: &LocalFolderBindings) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ Decodes the canonical local catalog JSON projection.
pub fn decode_local_catalog_json(text: &str) -> Result<LocalCatalog, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📤️ Encodes the local catalog to its canonical camel-case JSON projection.
pub fn encode_local_catalog_json(snapshot: &LocalCatalog) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ Decodes the canonical merge-policy setting JSON projection.
pub fn decode_merge_policy_setting_json(text: &str) -> Result<MergePolicySetting, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📤️ Encodes the merge-policy setting to its canonical camel-case JSON projection.
pub fn encode_merge_policy_setting_json(snapshot: &MergePolicySetting) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ Decodes the canonical identity setting JSON projection.
pub fn decode_identity_setting_json(text: &str) -> Result<IdentitySetting, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📤️ Encodes the identity setting to its canonical camel-case JSON projection — a bare
/// session object, or the bare literal `null` when signed out.
pub fn encode_identity_setting_json(snapshot: &IdentitySetting) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}
