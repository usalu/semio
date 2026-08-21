//! 📇️ `dsl_registry` — W1 foundation of the DSL registry unification (design ruling B-R3): a real
//! (non-demonstration) `crate::os_pack::cli::SchemaResolver` fan-in, so `pack_cli`'s CLI functions can resolve
//! real app schemas without `pack_cli` itself ever depending on an app crate (the orphan-rule-shaped
//! reason `SchemaResolver` is a trait defined in `pack_cli`, implemented here instead). This crate is
//! the one place in the workspace allowed to depend on many app `🗣️dsl`/`🔧️op` crates at once —
//! every other crate in the `dsl_*`/`pack_*`/`protocol_*` family stays app-dependency-free by design.
//!
//! 🌐️ P2-M3: the insertion API this module's own doc comment used to call missing —
//! [`register_schema_spec`] is a process-global `OnceLock<Mutex<HashMap<...>>>` registry mirroring
//! `crate::os_dsl::register_language`'s exact shape/thread-safety/hot-reload-overwrite semantics
//! (`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️component.rs`'s `LANGUAGE_REGISTRY`). Any host or
//! plugin — typically an artifact's `⚙️engine::register()` — calls it once per schema id at init,
//! covering both a document's own schema id and its `"<doc-schema>#diff"` diff schema (design ruling
//! B-R4), making that convention genuinely resolvable for the first time (previously zero live
//! consumers per the P2-W0 recon). [`full_resolver`] now reads a live snapshot of that global
//! registry instead of returning a hardcoded empty map. Full fan-in across every real app schema
//! (the `🧪️fixture-sweep` crate's dev-dependency list is the template for what that eventually looks
//! like) is still tracked as the W8 "dsl_registry completeness assertion" item in
//! `.claude/plans/the-final-goal-for-jolly-spindle.md` — this wave builds the mechanism, not the
//! full 32-standard fan-in.

use crate::os_pack::cli::SchemaResolver;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

//#region 🔖️Registry
/// @emoji 🌐️ Process-global `(schema id, RecordSpec constructor)` table — see module doc for the
/// `register_language` precedent this mirrors. Not `pub`: reached only through
/// [`register_schema_spec`] (write) and [`full_resolver`] (read-a-snapshot), same access shape as
/// `crate::os_dsl`'s `LANGUAGE_REGISTRY`/`IDIOM_REGISTRY`.
static SCHEMA_REGISTRY: OnceLock<Mutex<HashMap<&'static str, fn() -> crate::os_dsl::schema::RecordSpec>>> = OnceLock::new();

async fn schema_registry() -> &'static Mutex<HashMap<&'static str, fn() -> crate::os_dsl::schema::RecordSpec>> {
    SCHEMA_REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

/// @emoji 📌️ Registers one schema id's `RecordSpec` constructor — called once per schema at
/// host/plugin init (typically inside an artifact's `⚙️engine::register()`), for both a document's
/// own schema id (`"stdio.gif"`) and its diff schema (`"stdio.gif#diff"`, B-R4). Overwrites on
/// re-registration rather than erroring, matching `register_language`'s hot-reload-safe behavior —
/// a re-run dev build never deadlocks or panics on re-registering the same id.
pub async fn register_schema_spec(id: &'static str, spec: fn() -> crate::os_dsl::schema::RecordSpec) {
    let mut registry = schema_registry().await.lock().unwrap_or_else(|poison| poison.into_inner());
    registry.insert(id, spec);
}

/// @emoji 📇️ A `SchemaResolver` backed by a fixed table of `(schema id, RecordSpec constructor)`
/// pairs — [`full_resolver`] is the real-callers constructor (a live snapshot of the process-global
/// registry); [`FullResolver::from_map`] stays available for a caller that wants a narrower/custom
/// table (e.g. a test double) built by hand, independent of global registration state.
pub struct FullResolver {
    schemas: HashMap<&'static str, fn() -> crate::os_dsl::schema::RecordSpec>,
}

impl FullResolver {
    /// @emoji 🧪️ Builds a resolver from an explicit table, bypassing the process-global registry
    /// entirely — for tests/test-doubles that want an isolated, narrower set.
    pub async fn from_map(schemas: HashMap<&'static str, fn() -> crate::os_dsl::schema::RecordSpec>) -> Self {
        Self { schemas }
    }
}

impl SchemaResolver for FullResolver {
    async fn resolve(&self, schema: &str) -> Option<crate::os_dsl::schema::RecordSpec> {
        self.schemas.get(schema).map(|spec_fn| spec_fn())
    }

    async fn names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.schemas.keys().map(|s| s.to_string()).collect();
        names.sort_unstable();
        names
    }
}

/// @emoji 🏗️ Builds the real fan-in resolver as a live snapshot of everything registered via
/// [`register_schema_spec`] so far (call again after new registrations to see them — this is a
/// point-in-time copy of `&'static str`/`fn` pointers, not a live view). Schema ids follow the
/// schema lattice's own convention (`"<doc-schema>"` for a document, `"<doc-schema>#diff"` for its
/// diff, design ruling B-R4) so a future `dsl_registry`-driven `pack diff --schema
/// writer.document#diff` (or similar) resolves the diff's own grammar, not the document's.
pub async fn full_resolver() -> FullResolver {
    let registry = schema_registry().await.lock().unwrap_or_else(|poison| poison.into_inner());
    FullResolver { schemas: registry.clone() }
}
//#endregion 🔖️Registry

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::os_dsl::schema::{FieldSpec, RecordLayout, RecordSpec, Shape};

    /// @emoji 🧬️ A real (not mocked) minimal `RecordSpec` — one `Int` field under `Inline` layout.
    // 🚫️async: E4 fn-pointer slot — passed to `register_schema_spec`/`FullResolver::from_map`,
    // both `fn() -> RecordSpec` (sync, unnameable if async) — see R9/E4.
    fn sample_spec() -> RecordSpec {
        RecordSpec::new(Some("p2m3-sample"), RecordLayout::Inline, vec![FieldSpec::new(0, "value", Shape::Int)])
    }

    #[semio_framework_async_macros::async_test]
    async fn full_resolver_resolves_a_registered_schema_and_none_for_an_unregistered_one() {
        register_schema_spec("p2m3.registry-test.schema", sample_spec).await;
        let resolver = full_resolver().await;
        assert!(resolver.names().await.contains(&"p2m3.registry-test.schema".to_string()));
        let resolved = resolver.resolve("p2m3.registry-test.schema").await.expect("registered schema must resolve");
        assert_eq!(resolved.keyword.as_deref(), Some("p2m3-sample"));
        assert_eq!(resolved.fields.len(), 1);
        assert!(resolver.resolve("p2m3.registry-test.never-registered").await.is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn full_resolver_resolves_the_diff_schema_id_separately_from_its_document_id() {
        register_schema_spec("p2m3.diff-test.doc", sample_spec).await;
        register_schema_spec("p2m3.diff-test.doc#diff", sample_spec).await;
        let resolver = full_resolver().await;
        assert!(resolver.resolve("p2m3.diff-test.doc").await.is_some());
        assert!(resolver.resolve("p2m3.diff-test.doc#diff").await.is_some());
        assert!(resolver.resolve("p2m3.diff-test.doc#nonsense").await.is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn full_resolver_from_map_bypasses_the_global_registry() {
        let mut schemas: HashMap<&'static str, fn() -> RecordSpec> = HashMap::new();
        schemas.insert("p2m3.isolated.schema", sample_spec as fn() -> RecordSpec);
        let resolver = FullResolver::from_map(schemas).await;
        assert_eq!(resolver.names().await, vec!["p2m3.isolated.schema".to_string()]);
        assert!(resolver.resolve("p2m3.isolated.schema").await.is_some());
        // Not in THIS table (even though another test registers it globally) — proves from_map
        // is a genuinely isolated table, not a view into the process-global registry.
        assert!(resolver.resolve("p2m3.registry-test.schema").await.is_none());
    }
}
//#endregion 🧪️Tests
