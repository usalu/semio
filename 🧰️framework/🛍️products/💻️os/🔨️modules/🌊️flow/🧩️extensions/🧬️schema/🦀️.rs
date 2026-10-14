//! 🔌️ Flow extension manifest contracts.
use neural_engine::{ColdRetire, OperatorInfo, Registry, Schema};
use semio_framework_value::{DslValue, FromValue, ToValue};

// #region 🔖️Manifest
/// 📋️ `flow.extension` manifest encoded through the first-party value contract.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct FlowExtensionManifest {
    pub schema: String,
    pub id: String,
    pub name: String,
    pub version: String,
    pub activation_events: Vec<String>,
    pub contributes: FlowExtensionContributes,
}

/// 🎁️ Contributed extension surface.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct FlowExtensionContributes {
    pub schemas: Vec<Schema>,
    pub operators: Vec<OperatorInfo>,
    #[value(default)]
    pub widgets: Vec<FlowExtensionWidget>,
    #[value(default)]
    pub commands: Vec<FlowExtensionCommand>,
    #[value(default)]
    pub settings: Vec<FlowExtensionSetting>,
}

/// 🧩️ Declared widget contribution.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct FlowExtensionWidget {
    pub kind: String,
    pub name: String,
    pub summary: String,
}

/// ⌘️ Declared command contribution.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct FlowExtensionCommand {
    pub id: String,
    pub title: String,
}

/// ⚙️ Declared setting contribution. `default` is a `DslValue` directly (was `serde_json::Value`) —
/// the same tenth-seam pass; no bridge needed since `DslValue` already implements `ToValue`/`FromValue`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct FlowExtensionSetting {
    pub id: String,
    #[value(rename = "type")]
    pub setting_type: String,
    pub default: DslValue,
    pub description: String,
}

/// 🔢️ Builds the `DslValue` for a [`FlowExtensionSetting::default`] of a whole number — factored
/// out so extension crates never need to construct one themselves.
pub fn integer_setting_default(value: i64) -> DslValue {
    value.to_value()
}

/// 📦️ Builds a `flow.extension` JSON manifest from registry catalogue metadata.
#[allow(clippy::too_many_arguments, reason = "manifest needs id+name+version+registry+activation_events+widgets+commands+settings together; a params struct would ripple into every flow/module/*/rs call site outside this ticket's scope")]
pub fn build_manifest_json(id: &str, name: &str, version: &str, registry: &Registry, activation_events: Vec<String>, widgets: Vec<FlowExtensionWidget>, commands: Vec<FlowExtensionCommand>, settings: Vec<FlowExtensionSetting>) -> String {
    let mut manifest = FlowExtensionManifest {
        schema: "flow.extension".into(),
        id: id.into(),
        name: name.into(),
        version: version.into(),
        activation_events,
        contributes: FlowExtensionContributes { schemas: registry.schema_catalogue(), operators: registry.operator_catalogue(), widgets, commands, settings },
    };
    let encoded = semio_framework_pack_json::to_json_string(&manifest);
    std::mem::take(&mut manifest.contributes.schemas).retire_cold();
    std::mem::take(&mut manifest.contributes.operators).retire_cold();
    encoded
}
// #endregion 🔖️Manifest
