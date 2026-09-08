//! 🧬️ Schema leaf: canonical Rust mirror of `🔣️.json` for the 🕹️interaction module.
//! Every type here is `pub use`d from the module root (`super`) rather than redefined — `super`
//! itself re-exports `PresenceInteraction`/`PresenceDomain`/`InteractionState`/friends from
//! `semio-framework-replication`, where `PresencePeer.interaction: Option<PresenceInteraction>`
//! is wired directly beside `PresencePeer` in `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs` without this
//! leaf owning the type.

pub use super::{
    DomainHover, DomainSelection, DomainTopology, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionState, InteractionTarget, InteractionTopology, MergeMode, PresenceDomain, PresenceInteraction,
    SelectionMethod, SelectionMode, SelectionSpec, TopologyNode,
};

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
mod tests {
    use super::*;

    #[test]
    fn every_named_export_resolves_in_every_declared_format() {
        register_scope_exports();
        for export in EXPORTS {
            for format in [semio_framework_schema::SchemaFormat::JsonSchema, semio_framework_schema::SchemaFormat::Rust, semio_framework_schema::SchemaFormat::Typescript, semio_framework_schema::SchemaFormat::Graphql] {
                let leaf = semio_framework_schema::resolve_schema_export("framework.interaction", export.id, format).expect("resolve");
                assert!(!leaf.is_empty());
            }
            assert!(semio_framework_schema::resolve_schema_export("framework.interaction", export.id, semio_framework_schema::SchemaFormat::Protobuf).is_err());
            assert!(LEAVES.json_schema.contains(export.id));
        }
        assert!(semio_framework_schema::scope_schema_exports_registered("framework.interaction"));
    }
}

//#endregion 🔖️Tests
