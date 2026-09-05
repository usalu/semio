//! 🕹️ First-class hover/selection mechanism: declarative `InteractionDefinition` types (mirroring
//! the manifest's action/utility/tool/command family) — the human-facing declaration layer.
//!
//! 🧬️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (crate-layering blocker fix): the
//! pure, render-independent hover/selection state machine (`next_selection`/`next_hover`/
//! `validate_state`), its runtime/topology state, and the `PresenceInteraction` broadcast payload
//! now live in `semio-framework-replication` (`🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs`,
//! `🔖️Interaction` region) so kernel-side code (`store`, `sync`, wire codec) can name
//! `InteractionState`/`PresenceInteraction` without a `semio-framework` → os-kernel → `semio-framework`
//! cargo cycle (`semio-framework` depends on the kernel crate, never the reverse). `HierarchyProvider`/
//! `HoverSpec`/`SelectionSpec`/`SelectionMode`/`SelectionMethod`/`MergeMode` moved with it — only
//! `InteractionDefinition`/`GranularityDefinition` stay here, since their `label`/`icon_id` fields pull
//! in `ui_wgpu::LocalizedLabel`/`IconName`, which the wasm-safe kernel crate does not depend on. This
//! module `pub use`s the moved surface below so every existing consumer (`crate::InteractionState`
//! etc., re-exported crate-root-wide via `🦀️.rs`'s `pub use interaction::*;`) keeps resolving
//! unchanged. Handcrafted TS parity lives in `🟦️.ts` (unmoved — TypeScript has no crate
//! graph); schema leaves live in `🧬️schema/`.

use dsl::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use ui_wgpu::wgpu::LocalizedLabel;

use crate::IconName;

//#region 🔖️Definition
pub use protocol::{next_hover, next_selection, validate_state, HoverInput, PresenceDomain, PresenceInteraction, SelectionInput};
pub use protocol::{DomainHover, DomainSelection, DomainTopology, HierarchyProvider, HoverSpec, InteractionOutline, InteractionState, InteractionTarget, InteractionTopology, MergeMode, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode};

/// 🕹️ One interaction domain an app declares (e.g. "graph", "mesh", "ast", "world"): the target
/// universe/hierarchy shared by both its hover and selection sub-specs. `AppDefinition.interactions`
/// holds these; `WindowKindDefinition.interactions` references them via `InteractionRef`.
// 🌱️ `ui_wgpu::LocalizedLabel`/`IconName` gained `ToValue`/`FromValue` (ticket
// 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, see
// 📓️ui-wgpu-keystone-seven-value-derive-2026-09-02.md) — this type is unblocked. Kept additive
// (Serialize/Deserialize alongside ToValue/FromValue): `🛂️manifest/🦀️.rs`'s `AppDefinition`
// embeds `Vec<InteractionDefinition>` and is itself consumed outside this pass by
// `🛍️products/💻️os` (plugin/renderer modules) and `✏️s/🔌️plugins/**` while still serde-deriving.
// 🌱️ RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS: the `HierarchyProvider`/`HoverSpec`
// /`SelectionSpec` fields below have completed their move to `ToValue`/`FromValue`, so the additive
// serde half of this declaration no longer has impls to stand on and comes off with them.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(rename_all = "camelCase")]
pub struct InteractionDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    /// 🪜️ Non-empty; the first entry is the domain's default granularity.
    pub granularities: Vec<GranularityDefinition>,
    pub hierarchy: HierarchyProvider,
    pub hover: HoverSpec,
    pub selection: SelectionSpec,
}

impl InteractionDefinition {
    /// 🪞️ Projects this declaration down to the label/icon-free `InteractionOutline` `validate_state`
    /// consumes — `semio-framework-os-kernel` cannot name `InteractionDefinition` itself (see this
    /// file's header comment), so callers build one of these per declared domain before validating.
    pub async fn outline(&self) -> InteractionOutline {
        InteractionOutline { id: self.id.clone(), granularity_ids: self.granularities.iter().map(|granularity| granularity.id.clone()).collect(), selection: self.selection.clone() }
    }
}

/// 🔬️ One selectable/hoverable level of detail within a domain (e.g. mesh's object/face/edge/vertex).
// 🌱️ Unblocked alongside `InteractionDefinition` above (same `LocalizedLabel`/`IconName` fields,
// both now `ToValue`/`FromValue`). Kept additive for the same reason: transitively embedded in
// `AppDefinition`/`InteractionDefinition.granularities`, still consumed with plain serde outside
// this pass.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GranularityDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    pub icon_id: IconName,
}

/// 📇️ A validated reference into an app's `AppDefinition.interactions` registry — mirrors
/// `ActionRef`/`UtilityRef` exactly.
// 🚧️ BLOCKED (26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS): `Serialize`/
// `Deserialize` restored here ADDITIVELY, not removed — `🛂️manifest/🦀️.rs`'s
// `WindowKindDefinition.interactions: Vec<InteractionRef>` is itself still serde-only (blocked on
// `ui_wgpu::LocalizedLabel`/`IconName`/`SurfaceKind`/`WindowOptions`, none owned by this pass), so
// this type still needs the serde half regardless of its own `ToValue`/`FromValue`. Revisit once
// `WindowKindDefinition` converts.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(transparent)]
#[value(transparent)]
pub struct InteractionRef(String);

impl InteractionRef {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for InteractionRef {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for InteractionRef {
    fn from(value: String) -> Self {
        Self(value)
    }
}
//#endregion 🔖️Definition

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn interaction_definition_round_trips_through_json() {
        let def = InteractionDefinition {
            id: "graph".into(),
            label: LocalizedLabel::native("Graph", "Graph"),
            granularities: vec![
                GranularityDefinition { id: "node".into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle".into() },
                GranularityDefinition { id: "edge".into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "minus".into() },
            ],
            hierarchy: HierarchyProvider::Topology,
            hover: HoverSpec::default(),
            selection: SelectionSpec {
                modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                methods: vec![SelectionMethod::Pick],
                merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
                transitive: false,
                broadcast: true,
            },
        };
        let json = serde_json::to_string(&def).expect("serializes");
        assert!(json.contains("\"iconId\""), "{json}");
        assert!(json.contains("\"granularities\""), "{json}");
        let parsed: InteractionDefinition = serde_json::from_str(&json).expect("deserializes");
        assert_eq!(parsed, def);
    }

    #[semio_framework_async_macros::async_test]
    async fn outline_projects_id_granularity_ids_and_selection_only() {
        let def = InteractionDefinition {
            id: "graph".into(),
            label: LocalizedLabel::native("Graph", "Graph"),
            granularities: vec![
                GranularityDefinition { id: "node".into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle".into() },
                GranularityDefinition { id: "edge".into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "minus".into() },
            ],
            hierarchy: HierarchyProvider::Flat,
            hover: HoverSpec::default(),
            selection: SelectionSpec { modes: vec![SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
        };
        let outline = def.outline().await;
        assert_eq!(outline.id, "graph");
        assert_eq!(outline.granularity_ids, vec!["node".to_string(), "edge".to_string()]);
        assert_eq!(outline.selection, def.selection);
    }
}
