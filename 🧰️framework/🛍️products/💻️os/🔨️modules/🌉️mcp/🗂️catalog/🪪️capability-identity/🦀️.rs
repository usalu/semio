//! 🪪️ Capability identity of the catalog: the capability reference, owner, kind and audience types with their conversions and audience constants. See [`super`].

use super::*;

//#region 🔖️CapabilityRef
/// 🪪️ A capability's full identity string — `<plugin_id>.<app_id>.<action_id>`, `framework.*`,
/// `os.*`, `ui.*`, or a bare gateway verb (`context.resolve`, `capabilities.search`, …). Never a
/// bare action id (D3).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, ToValue, FromValue)]
#[serde(transparent)]
#[value(transparent)]
pub struct CapabilityRef(pub String);

impl CapabilityRef {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CapabilityRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
//#endregion 🔖️CapabilityRef

//#region 🔖️CapabilityOwner
/// 📍️ Who declared a capability — `app_id`/`window_kind_id`/`mode_id` are `None` for a plugin-scope
/// (not app-scope) command, matching `PluginManifest.commands` (a real, populated field with no
/// owning app) — a deliberate widening of `📋️master.md` §3.1's literal `Plugin{plugin_id, app_id,
/// window_kind_id, mode_id}` shape, documented in `📓️terra-P2-report.md`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum CapabilityOwner {
    Os,
    Framework,
    Shell,
    Plugin {
        plugin_id: String,
        /// 🏷️ The declaring plugin's own display name (`PluginManifest.label`, e.g. "Draw") — indexed
        /// by `🔎️search` so a query can find a verb by the product a human would name, not only by
        /// the lowercase id embedded in the capability ref.
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        app_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        window_kind_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        mode_id: Option<String>,
    },
    Extension {
        extension_id: String,
    },
    Gateway,
}

/// 🌉️ Hand-written, not derived: `#[value(...)]` has no way to express `skip_serializing_if` on an
/// enum VARIANT's own named field (the derive's internally-tagged codegen never reads that
/// attribute there — struct-field position only) — a derive here would silently start writing
/// `appId`/`windowKindId`/`modeId` as `null` instead of omitting them, changing the wire shape.
/// Mirrors `#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]`
/// exactly, field-for-field with the omit-if-`None` behavior serde's own `skip_serializing_if`
/// gives it (and the matching implicit "missing `Option` field decodes to `None`" serde gives every
/// `Option<T>` field, `#[serde(default)]` or not).
impl ToValue for CapabilityOwner {
    fn to_value(&self) -> DslValue {
        match self {
            CapabilityOwner::Os => DslValue::object([("kind".to_string(), DslValue::String("os".to_string()))]),
            CapabilityOwner::Framework => DslValue::object([("kind".to_string(), DslValue::String("framework".to_string()))]),
            CapabilityOwner::Shell => DslValue::object([("kind".to_string(), DslValue::String("shell".to_string()))]),
            CapabilityOwner::Plugin { plugin_id, label, app_id, window_kind_id, mode_id } => {
                let mut entries = vec![("kind".to_string(), DslValue::String("plugin".to_string())), ("pluginId".to_string(), plugin_id.to_value())];
                if let Some(label) = label {
                    entries.push(("label".to_string(), label.to_value()));
                }
                if let Some(app_id) = app_id {
                    entries.push(("appId".to_string(), app_id.to_value()));
                }
                if let Some(window_kind_id) = window_kind_id {
                    entries.push(("windowKindId".to_string(), window_kind_id.to_value()));
                }
                if let Some(mode_id) = mode_id {
                    entries.push(("modeId".to_string(), mode_id.to_value()));
                }
                DslValue::Object(entries)
            }
            CapabilityOwner::Extension { extension_id } => DslValue::object([("kind".to_string(), DslValue::String("extension".to_string())), ("extensionId".to_string(), extension_id.to_value())]),
            CapabilityOwner::Gateway => DslValue::object([("kind".to_string(), DslValue::String("gateway".to_string()))]),
        }
    }
}

impl FromValue for CapabilityOwner {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = DslValue::into_object(value)?;
        let tag = entries.iter().find(|(k, _)| k == "kind").map(|(_, v)| v.clone()).ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field `kind`"))?;
        let tag = match tag {
            DslValue::String(s) => s,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string tag, found {other:?}"))),
        };
        let field = |name: &str| entries.iter().find(|(k, _)| k == name).map(|(_, v)| v.clone());
        Ok(match tag.as_str() {
            "os" => CapabilityOwner::Os,
            "framework" => CapabilityOwner::Framework,
            "shell" => CapabilityOwner::Shell,
            "plugin" => CapabilityOwner::Plugin {
                plugin_id: match field("pluginId") {
                    Some(v) => String::from_value(v).map_err(|error| error.under("pluginId"))?,
                    None => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field `pluginId`")),
                },
                label: match field("label") {
                    Some(v) => Option::<String>::from_value(v).map_err(|error| error.under("label"))?,
                    None => None,
                },
                app_id: match field("appId") {
                    Some(v) => Option::<String>::from_value(v).map_err(|error| error.under("appId"))?,
                    None => None,
                },
                window_kind_id: match field("windowKindId") {
                    Some(v) => Option::<String>::from_value(v).map_err(|error| error.under("windowKindId"))?,
                    None => None,
                },
                mode_id: match field("modeId") {
                    Some(v) => Option::<String>::from_value(v).map_err(|error| error.under("modeId"))?,
                    None => None,
                },
            },
            "extension" => CapabilityOwner::Extension {
                extension_id: match field("extensionId") {
                    Some(v) => String::from_value(v).map_err(|error| error.under("extensionId"))?,
                    None => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field `extensionId`")),
                },
            },
            "gateway" => CapabilityOwner::Gateway,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown `kind` variant `{other}`"))),
        })
    }
}

impl CapabilityOwner {
    /// 🔑️ A stable, hashable grouping key for the "no duplicate (owner, title)" conformance rule —
    /// collapses the `Plugin` variant's optional fields into one string rather than requiring
    /// `CapabilityOwner` itself to be `Hash`/`Ord` (it embeds `Option<String>`s that would make that
    /// derive noisy for no benefit outside this one use).
    pub fn dedup_key(&self) -> String {
        match self {
            CapabilityOwner::Os => "os".to_string(),
            CapabilityOwner::Framework => "framework".to_string(),
            CapabilityOwner::Shell => "shell".to_string(),
            CapabilityOwner::Gateway => "gateway".to_string(),
            CapabilityOwner::Extension { extension_id } => format!("extension:{extension_id}"),
            CapabilityOwner::Plugin { plugin_id, app_id, window_kind_id, mode_id, .. } => {
                format!("plugin:{plugin_id}:{}:{}:{}", app_id.as_deref().unwrap_or(""), window_kind_id.as_deref().unwrap_or(""), mode_id.as_deref().unwrap_or(""))
            }
        }
    }
}
//#endregion 🔖️CapabilityOwner

//#region 🔖️CapabilityKind
/// 🏷️ What kind of operation a capability is — the six `manifest::ActionKind` variants plus the
/// three gateway-only kinds `📋️master.md` §3.1 names (`Query`/`Job`/`Ui`) and `Meta` (the gateway's
/// own discovery/context verbs, which are neither an artifact operation nor a UI command).
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum CapabilityKind {
    Mutation,
    View,
    History,
    Clipboard,
    Shell,
    Interaction,
    Query,
    Job,
    Ui,
    Meta,
}

impl From<manifest::ActionKind> for CapabilityKind {
    fn from(kind: manifest::ActionKind) -> Self {
        match kind {
            manifest::ActionKind::Mutation => CapabilityKind::Mutation,
            manifest::ActionKind::View => CapabilityKind::View,
            manifest::ActionKind::History => CapabilityKind::History,
            manifest::ActionKind::Clipboard => CapabilityKind::Clipboard,
            manifest::ActionKind::Shell => CapabilityKind::Shell,
            manifest::ActionKind::Interaction => CapabilityKind::Interaction,
        }
    }
}
//#endregion 🔖️CapabilityKind

//#region 🔖️CapabilityAudience
/// 🎯️ WHO a compiled capability is addressed to — the gateway's own projection of
/// `manifest::CapabilityAudience`, mirroring how [`CapabilityKind`] projects `manifest::ActionKind`
/// (D5: the gateway's wire vocabulary lives HERE, the declaration vocabulary lives in `🛂️manifest`).
/// [`compile`] publishes `Agent` only: an agent choosing a tool must never have to tell
/// `patchLayer` apart from `canvasPointerMove` by reading their ids.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum CapabilityAudience {
    /// 🤖️ An intent-level verb — published to agents and to the UI.
    Agent,
    /// 🖱️ A raw pointer/keyboard/engagement input event — the UI dispatches it, agents never see it.
    Input,
    /// 🪟️ Window/view/session chrome — the UI dispatches it, agents never see it.
    Chrome,
}

impl From<manifest::CapabilityAudience> for CapabilityAudience {
    fn from(audience: manifest::CapabilityAudience) -> Self {
        match audience {
            manifest::CapabilityAudience::Agent => CapabilityAudience::Agent,
            manifest::CapabilityAudience::Input => CapabilityAudience::Input,
            manifest::CapabilityAudience::Chrome => CapabilityAudience::Chrome,
        }
    }
}

/// 🎯️ The audience set [`compile`] publishes by default — agent-meaningful verbs only.
pub const AGENT_AUDIENCES: &[CapabilityAudience] = &[CapabilityAudience::Agent];

/// 🎯️ Every audience, for the callers that want the complete compiled surface (a shell's own
/// command registry, `🧪️conformance`'s duplicate/label checks) rather than the agent projection.
pub const ALL_AUDIENCES: &[CapabilityAudience] = &[CapabilityAudience::Agent, CapabilityAudience::Input, CapabilityAudience::Chrome];
//#endregion 🔖️CapabilityAudience
