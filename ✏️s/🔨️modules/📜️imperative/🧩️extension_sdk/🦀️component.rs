//! 🔌️ Shared manifest + evaluate helpers for imperative path extensions.

use neural_engine::{inject_channel_defaults, Dictionary, OperatorInfo, Registry};
use semio_framework::{Contribution, ProgramContributionEntry, TopicContribution};
use serde::{Deserialize, Serialize};

// #region 🔖️Manifest
/// 📋️ `imperative.extension` manifest document embedded in `Contribution::ImperativeModule::manifest_json`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImperativeExtensionManifest {
    pub schema: String,
    pub id: String,
    pub name: String,
    pub version: String,
    pub contributes: ImperativeExtensionContributes,
}

/// 🎁️ Contributed imperative surface (operators + optional catalogue fragment).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImperativeExtensionContributes {
    #[serde(default)]
    pub operators: Vec<OperatorInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalogue_json: Option<String>,
}

/// 📦️ Builds manifest JSON from a module registry and catalogue fragment.
pub fn build_manifest_json(id: &str, name: &str, version: &str, registry: &Registry, catalogue_json: Option<&str>) -> String {
    let manifest = ImperativeExtensionManifest {
        schema: "imperative.extension".into(),
        id: id.into(),
        name: name.into(),
        version: version.into(),
        contributes: ImperativeExtensionContributes {
            operators: registry.operator_catalogue(),
            catalogue_json: catalogue_json.map(str::to_string),
        },
    };
    serde_json::to_string(&manifest).unwrap_or_else(|_| "{}".into())
}
// #endregion 🔖️Manifest

// #region 🔖️Evaluate
/// 🧮️ Evaluates an operator and returns JSON dictionary or `{ "error": ... }`.
pub fn evaluate_json(registry: &Registry, kind_id: &str, input_json: &str) -> String {
    let input: Dictionary = match serde_json::from_str(input_json) {
        Ok(d) => d,
        Err(err) => return serde_json::json!({ "error": err.to_string() }).to_string(),
    };
    let input = match registry.operator_info(kind_id) {
        Some(info) => inject_channel_defaults(input, info),
        None => input,
    };
    match registry.dispatch(kind_id, &input) {
        Ok(out) => serde_json::to_string(&out).unwrap_or_else(|_| "{}".into()),
        Err(err) => serde_json::json!({ "error": err.to_string() }).to_string(),
    }
}

/// 🔀️ WIT `extension::invoke` payload for `imperative.module/evaluate`.
pub fn evaluate_request_json(kind_id: &str, input_json: &str) -> String {
    let input: serde_json::Value = serde_json::from_str(input_json).unwrap_or(serde_json::json!({}));
    serde_json::json!({ "kindId": kind_id, "input": input }).to_string()
}

/// 🔀️ Parses an evaluate invoke request and runs it against `registry`.
pub fn evaluate_invoke(registry: &Registry, request: &[u8]) -> Result<Vec<u8>, String> {
    let body: serde_json::Value = serde_json::from_slice(request).map_err(|err| err.to_string())?;
    let kind_id = body.get("kindId").and_then(|v| v.as_str()).ok_or_else(|| "missing kindId".to_string())?;
    let input_json = body.get("input").map(|v| v.to_string()).unwrap_or_else(|| "{}".to_string());
    Ok(evaluate_json(registry, kind_id, &input_json).into_bytes())
}
// #endregion 🔖️Evaluate

// #region 🔖️Constants
/// 🎯️ Imperative play host app id for `Contribution::ImperativeModule::app_id`.
pub const IMPERATIVE_PLAY_APP_ID: &str = "imperative-play";

/// 🔀️ Capability topic handled by extension bundles for operator dispatch.
pub const IMPERATIVE_MODULE_EVALUATE_CAPABILITY: &str = "imperative.module/evaluate";

/// 🧩️ Builds one `ProgramContributionEntry` for `Contribution::ImperativeModule`.
pub fn imperative_module_contribution(
    extension_id: &str,
    module_id: &str,
    label: &str,
    icon_id: &str,
    manifest_id: &str,
    manifest_name: &str,
    version: &str,
    registry: &Registry,
    catalogue_json: Option<&str>,
) -> ProgramContributionEntry {
    let manifest_json = build_manifest_json(manifest_id, manifest_name, version, registry, catalogue_json);
    ProgramContributionEntry {
        plugin_id: extension_id.into(),
        contribution: Contribution::ImperativeModule {
            app_id: IMPERATIVE_PLAY_APP_ID.into(),
            module_id: module_id.into(),
            label: label.into(),
            icon_id: icon_id.into(),
            manifest_json,
        },
    }
}
// #endregion 🔖️Constants

// #region 🔖️TopicContribution
/// 🗺️ Open-registry twin of [`imperative_module_contribution`] — builds the same data under the
/// `"imperative.module"` topic (reuses this crate's own `contributes = ["imperative.module"]` Cargo
/// metadata vocabulary) instead of the closed `Contribution::ImperativeModule` variant. Additive: the
/// closed-enum producer above is unchanged and still the one wired into `ProgramContributionEntry`;
/// this sibling exists for open-registry consumers to adopt going forward — see
/// `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs::TopicContribution`.
pub fn imperative_module_topic_contribution(
    module_id: &str,
    label: &str,
    icon_id: &str,
    manifest_id: &str,
    manifest_name: &str,
    version: &str,
    registry: &Registry,
    catalogue_json: Option<&str>,
) -> TopicContribution {
    let manifest_json = build_manifest_json(manifest_id, manifest_name, version, registry, catalogue_json);
    TopicContribution::new(
        "imperative.module",
        serde_json::json!({
            "appId": IMPERATIVE_PLAY_APP_ID,
            "moduleId": module_id,
            "label": label,
            "iconId": icon_id,
            "manifestJson": manifest_json,
        }),
    )
}
// #endregion 🔖️TopicContribution
