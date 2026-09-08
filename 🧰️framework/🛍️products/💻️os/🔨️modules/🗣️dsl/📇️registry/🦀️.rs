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
//! (`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs`'s `LANGUAGE_REGISTRY`). Any host or
//! plugin — typically an artifact's `⚙️engine::register()` — calls it once per schema id at init,
//! covering both a document's own schema id and its `"<doc-schema>#diff"` diff schema (design ruling
//! B-R4), making that convention genuinely resolvable for the first time (previously zero live
//! consumers per the P2-W0 recon). [`full_resolver`] now reads a live snapshot of that global
//! registry instead of returning a hardcoded empty map. Full fan-in across every real app schema
//! (the `🧹️fixture-sweep` crate's dev-dependency list is the template for what that eventually looks
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
