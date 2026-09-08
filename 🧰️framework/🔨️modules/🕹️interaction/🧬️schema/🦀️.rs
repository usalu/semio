//! 🧬️ Schema leaf: canonical Rust mirror of `🔣️.json` for the 🕹️interaction module.
//! Every type here is published from the module root (`super`) rather than redefined — `super`
//! itself re-exports `PresenceInteraction`/`PresenceDomain`/`InteractionState`/friends from
//! `semio-framework-replication`, where `PresencePeer.interaction: Option<PresenceInteraction>`
//! is wired directly beside `PresencePeer` in `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs` without this
//! leaf owning the type.
//!
//! Each of the module's sixteen `$defs` exports is published as its own named `pub type`, because
//! execution contract §A defines Rust export presence as a same-named `pub struct|enum|type` in the
//! format leaf: a grouped `pub use` publishes the same items but names none of them, so a reader —
//! and the `schema check` gate — cannot tell which exports this leaf claims to carry.
//! `InteractionRef`/`InteractionTopology` are NOT `$defs` exports and stay a plain re-export.

pub use super::{InteractionRef, InteractionTopology};

pub type InteractionDefinition = super::InteractionDefinition;
pub type GranularityDefinition = super::GranularityDefinition;
pub type HierarchyProvider = super::HierarchyProvider;
pub type HoverSpec = super::HoverSpec;
pub type SelectionSpec = super::SelectionSpec;
pub type SelectionMode = super::SelectionMode;
pub type SelectionMethod = super::SelectionMethod;
pub type MergeMode = super::MergeMode;
pub type InteractionTarget = super::InteractionTarget;
pub type DomainSelection = super::DomainSelection;
pub type DomainHover = super::DomainHover;
pub type InteractionState = super::InteractionState;
pub type TopologyNode = super::TopologyNode;
pub type DomainTopology = super::DomainTopology;
pub type PresenceDomain = super::PresenceDomain;
pub type PresenceInteraction = super::PresenceInteraction;

//#region 🔖️ScopeSchemaExports

use semio_framework_schema::{register_scope_schema_exports as register_exports, FacetLeaves, SchemaExport, ScopeSchemaExports};

/// 🍃 The four format leaves this module publishes; every named export resolves to the same
/// documents because one `🧬️schema/` module carries them all. No proto leaf exists here.
const LEAVES: FacetLeaves = FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: "" };

/// 🏷️ `$defs` of `🔣️.json`, in declaration order.
const EXPORTS: [SchemaExport; 16] = [
    SchemaExport { id: "InteractionDefinition", leaves: LEAVES },
    SchemaExport { id: "GranularityDefinition", leaves: LEAVES },
    SchemaExport { id: "HierarchyProvider", leaves: LEAVES },
    SchemaExport { id: "HoverSpec", leaves: LEAVES },
    SchemaExport { id: "SelectionSpec", leaves: LEAVES },
    SchemaExport { id: "SelectionMode", leaves: LEAVES },
    SchemaExport { id: "SelectionMethod", leaves: LEAVES },
    SchemaExport { id: "MergeMode", leaves: LEAVES },
    SchemaExport { id: "InteractionTarget", leaves: LEAVES },
    SchemaExport { id: "DomainSelection", leaves: LEAVES },
    SchemaExport { id: "DomainHover", leaves: LEAVES },
    SchemaExport { id: "InteractionState", leaves: LEAVES },
    SchemaExport { id: "TopologyNode", leaves: LEAVES },
    SchemaExport { id: "DomainTopology", leaves: LEAVES },
    SchemaExport { id: "PresenceDomain", leaves: LEAVES },
    SchemaExport { id: "PresenceInteraction", leaves: LEAVES },
];

/// 📌️ Registers `framework.interaction`'s named exports into the OS-wide export catalog.
/// See `📋️execution-contract.md` §C and `semio_framework_schema::resolve_schema_export`.
// 🚫️async: E1 pure registration helper (no I/O) — see R9
pub fn register_scope_exports() {
    register_exports(ScopeSchemaExports { scope: "framework.interaction", exports: &EXPORTS }).expect("framework.interaction scope schema exports");
}

//#endregion 🔖️ScopeSchemaExports

//#region 🔖️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🔖️Tests
