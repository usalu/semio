//! 🧬️ The `framework.ui.contract` schema leaf: this scope's named-export registration, the Rust half [Design notes](README.md#source-notes-1).

//#region 🔖️ScopeSchemaExports

use semio_framework_schema_registry::{register_scope_schema_exports, FacetLeaves, SchemaExport, SchemaFormat, ScopeSchemaExports};

/// 🗂️ The formats every export of this scope exists in, which is every format this module carries a [Design notes](README.md#source-notes-18).
pub const DECLARED_FORMATS: [SchemaFormat; 2] = [SchemaFormat::JsonSchema, SchemaFormat::Rust];

/// 🍃 The two leaves `DECLARED_FORMATS` names. `semio-framework-schema-registry` is the os-kernel-free
/// leaf crate (`📋️execution-contract.md` §C), so registering from here does not reintroduce the
/// os-kernel/`dsl` edge this crate's header docstring forbids — which is what blocked WP4b row 40.
/// The other three fields are the registry's spelling of "not provided": no `🟦️.ts`/`🔗️.graphql`/
/// `🛰️.proto` leaf exists in this module, and no export claims one.
const LEAVES: FacetLeaves = FacetLeaves { rust: include_str!("../👥️presence/🦀️.rs"), typescript: "", graphql: "", json_schema: include_str!("🔣️.json"), proto: "" };

/// 🏷️ `$defs` of `🔣️.json`, in declaration order.
const EXPORTS: [SchemaExport; 1] = [SchemaExport { id: "PresenceUpdate", leaves: LEAVES }];

/// 📌️ Registers `framework.ui.contract`'s named exports into the process-wide export catalog.
/// See `📋️execution-contract.md` §C and `semio_framework_schema_registry::resolve_schema_export`.
// 🚫️async: E1 pure registration helper (no I/O) — see R9
pub fn register_scope_exports() {
    register_scope_schema_exports(ScopeSchemaExports { scope: "framework.ui.contract", exports: &EXPORTS }).expect("framework.ui.contract scope schema exports");
}

/// 🔬 Proves the registration at runtime rather than by inspection: it registers, resolves every
/// export in exactly the formats `DECLARED_FORMATS` names and in no other, asserts the scope is
/// visible in the process-wide catalog, and asserts each export's `"x-semio-formats"` annotation is
/// the taxonomy spelling of that same set.
#[cfg(test)]
#[path = "🧪️tests/🔬️scope-schema-export-law-standalone/🦀️.rs"]
mod scope_schema_export_law;

//#endregion 🔖️ScopeSchemaExports

//#region 🔖️Exports

#[cfg(test)]
#[path = "🧪️examples/🦀️.rs"]
pub(crate) mod examples;

/// 📡️ The Rust half of the `PresenceUpdate` export: the crate's own coalesced, TTL-scoped presence
/// message, declared once in `📦️packages/🦀️rust/👥️presence.rs` and re-exported here so the scope's
/// schema module names it (contract §A: a Rust re-export is `pub use`). `🔣️.json`'s `$defs.PresenceUpdate`
/// is the same shape in the normative format, and `presence_update_export_matches_the_wire_shape`
/// asserts the two cannot drift.
pub use crate::PresenceUpdate;

//#endregion 🔖️Exports

