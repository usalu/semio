//! 🚪️ Native JSON admission for typed config mutations.

use crate::opening_config::mutations::{OpeningConfigMutation, UiPreferencesConfigMutation, IdentityConfigMutation, LocalCatalogConfigMutation, LocalFoldersConfigMutation, MergePolicyConfigMutation};

/// 📥️ Decodes the internally tagged opening-config JSON projection.
pub fn decode_opening_config_mutation_json(text: &str) -> Result<OpeningConfigMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📥️ Decodes the internally tagged UI-preferences mutation JSON projection.
pub fn decode_ui_preferences_config_mutation_json(text: &str) -> Result<UiPreferencesConfigMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📥️ Decodes the internally tagged local-folders mutation JSON projection.
pub fn decode_local_folders_config_mutation_json(text: &str) -> Result<LocalFoldersConfigMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📥️ Decodes the internally tagged local-catalog mutation JSON projection.
pub fn decode_local_catalog_config_mutation_json(text: &str) -> Result<LocalCatalogConfigMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📥️ Decodes the internally tagged merge-policy mutation JSON projection.
pub fn decode_merge_policy_config_mutation_json(text: &str) -> Result<MergePolicyConfigMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📥️ Decodes the internally tagged identity mutation JSON projection.
pub fn decode_identity_config_mutation_json(text: &str) -> Result<IdentityConfigMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
