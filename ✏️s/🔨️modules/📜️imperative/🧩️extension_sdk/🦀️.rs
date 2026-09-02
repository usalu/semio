//! 🔌️ Shared manifest + evaluate helpers for imperative path extensions.

use neural_engine::{inject_channel_defaults, Dictionary, OperatorInfo, Registry};
use semio_framework::{ProgramContributionEntry, TopicContribution};
use semio_framework_os_kernel::{DslValue, FromValue, ToValue};

// #region 🔖️Manifest
/// 📋️ `imperative.extension` manifest document embedded in the `"imperative.module"` topic
/// contribution's `manifestJson` field. No `serde` at all, not even test-gated
/// (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01, tenth-seam pass): this
/// crate has no oracle test needing it, and `OperatorInfo: Serialize/Deserialize` (needed for
/// `operators: Vec<OperatorInfo>`) is itself `#[cfg(test)]`-gated INSIDE `neural_engine`'s own
/// compilation unit, which stays inactive here even under this crate's own `cfg(test)` — cfg(test)
/// never crosses a crate boundary. See `📓️orderedmap-tenth-seam.md`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ImperativeExtensionManifest {
    pub schema: String,
    pub id: String,
    pub name: String,
    pub version: String,
    pub contributes: ImperativeExtensionContributes,
}

/// 🎁️ Contributed imperative surface (operators + optional catalogue fragment). No `serde` — see
/// `ImperativeExtensionManifest` above.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ImperativeExtensionContributes {
    #[value(default)]
    pub operators: Vec<OperatorInfo>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub catalogue_json: Option<String>,
}

/// 📦️ Builds manifest JSON from a module registry and catalogue fragment.
// 🚫️async: E1 pure — pack::json::to_json_string only, zero suspension points; every caller across
// the 5 imperative-* extensions consumes this synchronously — see R9.
pub fn build_manifest_json(id: &str, name: &str, version: &str, registry: &Registry, catalogue_json: Option<&str>) -> String {
    let manifest = ImperativeExtensionManifest {
        schema: "imperative.extension".into(),
        id: id.into(),
        name: name.into(),
        version: version.into(),
        contributes: ImperativeExtensionContributes { operators: registry.operator_catalogue(), catalogue_json: catalogue_json.map(str::to_string) },
    };
    semio_framework_os_kernel::os_pack::json::to_json_string(&manifest)
}
// #endregion 🔖️Manifest

// #region 🔖️Evaluate
/// 🧮️ Evaluates an operator and returns JSON dictionary or `{ "error": ... }`.
// 🚫️async: E1 pure — in-memory registry dispatch only, zero suspension points — see R9.
pub fn evaluate_json(registry: &Registry, kind_id: &str, input_json: &str) -> String {
    let input: Dictionary = match semio_framework_os_kernel::os_pack::json::from_json_str(input_json) {
        Ok(d) => d,
        Err(err) => return semio_framework_os_kernel::os_pack::json::to_json_string(&DslValue::object([("error".to_string(), err.to_string().to_value())])),
    };
    let input = match registry.operator_info(kind_id) {
        Some(info) => inject_channel_defaults(input, info),
        None => input,
    };
    match registry.dispatch(kind_id, &input) {
        Ok(out) => semio_framework_os_kernel::os_pack::json::to_json_string(&out),
        Err(err) => semio_framework_os_kernel::os_pack::json::to_json_string(&DslValue::object([("error".to_string(), err.to_string().to_value())])),
    }
}

/// 🔀️ WIT `extension::invoke` payload for `imperative.module/evaluate`.
// 🚫️async: E1 pure — serde_json only, zero suspension points — see R9.
pub fn evaluate_request_json(kind_id: &str, input_json: &str) -> String {
    let input: serde_json::Value = serde_json::from_str(input_json).unwrap_or(serde_json::json!({}));
    serde_json::json!({ "kindId": kind_id, "input": input }).to_string()
}

/// 🔀️ Parses an evaluate invoke request and runs it against `registry`.
// 🚫️async: E1 pure — delegates to `evaluate_json` (sync), zero suspension points — see R9.
pub fn evaluate_invoke(registry: &Registry, request: &[u8]) -> Result<Vec<u8>, String> {
    let body: serde_json::Value = serde_json::from_slice(request).map_err(|err| err.to_string())?;
    let kind_id = body.get("kindId").and_then(|v| v.as_str()).ok_or_else(|| "missing kindId".to_string())?;
    let input_json = body.get("input").map(|v| v.to_string()).unwrap_or_else(|| "{}".to_string());
    Ok(evaluate_json(registry, kind_id, &input_json).into_bytes())
}
// #endregion 🔖️Evaluate

// #region 🔖️Constants
/// 🎯️ Imperative play host app id for the `"imperative.module"` topic contribution's `appId` field.
pub const IMPERATIVE_PLAY_APP_ID: &str = "imperative-play";

/// 🔀️ Capability topic handled by extension bundles for operator dispatch.
pub const IMPERATIVE_MODULE_EVALUATE_CAPABILITY: &str = "imperative.module/evaluate";

/// 🧩️ Builds one `ProgramContributionEntry` carrying the `"imperative.module"` topic contribution.
// 🚫️async: E1 pure — struct literal over `imperative_module_topic_contribution` (sync); every one of
// the 5 imperative-* extensions' own wrapper fns consumes this synchronously (unawaited) — see R9.
pub fn imperative_module_contribution(extension_id: &str, module_id: &str, label: &str, icon_id: &str, manifest_id: &str, manifest_name: &str, version: &str, registry: &Registry, catalogue_json: Option<&str>) -> ProgramContributionEntry {
    ProgramContributionEntry { plugin_id: extension_id.into(), topic_contribution: Some(imperative_module_topic_contribution(module_id, label, icon_id, manifest_id, manifest_name, version, registry, catalogue_json)) }
}
// #endregion 🔖️Constants

// #region 🔖️TopicContribution
/// 🗺️ Builds the `"imperative.module"` `TopicContribution` payload consumed by
/// [`imperative_module_contribution`] — see
/// `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs::TopicContribution`.
pub fn imperative_module_topic_contribution(module_id: &str, label: &str, icon_id: &str, manifest_id: &str, manifest_name: &str, version: &str, registry: &Registry, catalogue_json: Option<&str>) -> TopicContribution {
    let manifest_json = build_manifest_json(manifest_id, manifest_name, version, registry, catalogue_json);
    TopicContribution::new(
        "imperative.module",
        DslValue::object([
            ("appId".to_string(), DslValue::String(IMPERATIVE_PLAY_APP_ID.to_string())),
            ("moduleId".to_string(), DslValue::String(module_id.to_string())),
            ("label".to_string(), DslValue::String(label.to_string())),
            ("iconId".to_string(), DslValue::String(icon_id.to_string())),
            ("manifestJson".to_string(), DslValue::String(manifest_json)),
        ]),
    )
}
// #endregion 🔖️TopicContribution
