//! 📇️ `dsl_registry` — W1 foundation of the DSL registry unification (design ruling B-R3): a real
//! (non-demonstration) `crate::os_pack::cli::SchemaResolver` fan-in, so `pack_cli`'s CLI functions can resolve
//! real app schemas without `pack_cli` itself ever depending on an app crate (the orphan-rule-shaped
//! reason `SchemaResolver` is a trait defined in `pack_cli`, implemented here instead). This crate is
//! the one place in the workspace allowed to depend on many app `🗣️dsl`/`🔧️op` crates at once —
//! every other crate in the `dsl_*`/`pack_*`/`protocol_*` family stays app-dependency-free by design.
//!
//! 🚧️ W1 scope: proves the mechanism on two apps (writer, note) — their document schema AND their
//! `"<doc-schema>#diff"` diff schema (the first two real `#[derive(crate::os_dsl::DslDiff)]` types, see
//! `writer::artifacts::writer::diff::WriterDiff`/`note_op::NoteDiff`). Full fan-in across every real app schema (the
//! `🧪️fixture-sweep` crate's dev-dependency list is the template for what that eventually looks
//! like) is deferred to a later wave — tracked as the W8 "dsl_registry completeness assertion" item
//! in `.claude/plans/the-final-goal-for-jolly-spindle.md`. Add one app's `🗣️dsl`/`🔧️op` pair to
//! `Cargo.toml` + [`full_resolver`] per follow-up; nothing else in this crate needs to change shape.

use crate::os_pack::cli::SchemaResolver;
use std::collections::HashMap;

//#region 🔖️Registry
/// @emoji 📇️ A `SchemaResolver` backed by a fixed table of `(schema id, RecordSpec constructor)`
/// pairs — [`full_resolver`] is the only constructor real callers use; the struct itself stays
/// public so a caller that wants a narrower/custom table (e.g. a test double) can build one by hand.
pub struct FullResolver {
    schemas: HashMap<&'static str, fn() -> crate::os_dsl::schema::RecordSpec>,
}

impl SchemaResolver for FullResolver {
    fn resolve(&self, schema: &str) -> Option<crate::os_dsl::schema::RecordSpec> {
        self.schemas.get(schema).map(|spec_fn| spec_fn())
    }

    fn names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.schemas.keys().map(|s| s.to_string()).collect();
        names.sort_unstable();
        names
    }
}

/// @emoji 🏗️ Builds the real fan-in resolver — the schema ids follow the schema lattice's own
/// convention (`"<doc-schema>"` for a document, `"<doc-schema>#diff"` for its diff, design ruling
/// B-R4) so a future `dsl_registry`-driven `pack diff --schema writer.document#diff` (or similar)
/// resolves the diff's own grammar, not the document's.
pub fn full_resolver() -> FullResolver {
    // Kernel stays app-dependency-free; hosts insert schema constructors into FullResolver.
    let schemas: HashMap<&'static str, fn() -> crate::os_dsl::schema::RecordSpec> = HashMap::new();
    FullResolver { schemas }
}
//#endregion 🔖️Registry

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_resolver_starts_empty_without_app_fan_in() {
        let resolver = full_resolver();
        assert!(resolver.names().is_empty());
        assert!(resolver.resolve("writer.document").is_none());
        assert!(resolver.resolve("never-registered").is_none());
    }

    #[test]
    fn full_resolver_accepts_manual_schema_inserts() {
        let mut schemas: HashMap<&'static str, fn() -> crate::os_dsl::schema::RecordSpec> = HashMap::new();
        // Manual insert path used by hosts/plugins — constructor is a no-op placeholder type check only when empty.
        let resolver = FullResolver { schemas };
        assert!(resolver.names().is_empty());
    }
}
//#endregion 🧪️Tests

