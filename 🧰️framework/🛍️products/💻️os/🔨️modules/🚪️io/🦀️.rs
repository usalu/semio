//! 🚪️ OS artifact composition, codec registration and Store-backed IO dispatch.

#[path = "../🏪️store/📦️codec/🪶️snapshot-capability/🧰️codec-operations/🦀️.rs"]
pub mod sqlite_snapshot_operations;
#[path = "📥️lifted/🚪️io/🦀️.rs"]
pub mod lifted_composer_registry;
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_os_store_text_io::ArtifactStoreSnapshot as _;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_diagnostic::Diagnostic;
use std::collections::{BTreeMap, HashMap};
use std::sync::RwLock;

#[path="🧰️testing/🦀️.rs"]
pub mod testing;

use semio_framework_artifact_reference::{ArtifactDialect, Dialect, StandardId, SubsetId};
use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText as _;

//#region 🔖️ComposeTypes
/// 📥 One typed compose source: a foreign or native dialect plus its payload.
#[derive(Clone, Debug)]
pub enum AnalyzeSource<'a> {
    Text(&'a str),
    Binary(&'a [u8]),
}

/// 🎹 One typed source for composition: a foreign or native dialect plus its payload.
pub struct ComposeSource<'a> {
    pub dialect: Dialect,
    pub payload: AnalyzeSource<'a>,
}

/// 🎹 Composition result: one snapshot in the composer's `WRITES` dialect.
#[derive(Clone, Debug)]
pub struct Composition<T> {
    pub snapshot: T,
    pub confidence: Confidence,
    pub diagnostics: Vec<Diagnostic>,
}

//#endregion 🔖️ComposeTypes
//#region 🔖️ErasedRegistry

/// 🌀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (`io-thunks`, R1/R2 E4): wraps an `async fn(&[
/// ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError>` in a macro-generated, non-
/// capturing `fn` thunk that coerces to `AsyncComposeFn` — an `async fn` item's pointer type is
/// unnameable, so `compose: some_async_fn` can never coerce into a `ComposerEntry.compose` row
/// directly. The wrapped function itself keeps the literal `async` keyword the universal-async
/// decree (`📌️important.md` O1) requires; only the macro-generated `__thunk` is sync, and it is
/// invisible in source (R2 E4's "macro-generated" escape hatch — no per-site `// 🚫️async:` tag
/// needed). `$crate::` so this also works from every OTHER crate's `ComposerEntry` construction
/// site once `fleet-codemods` rolls it out fleet-wide (the 163 sites this ticket's `io-thunks`
/// packet does NOT touch — see `compose-thunk-rewrite.py` in the ticket folder).
#[macro_export]
macro_rules! compose_thunk {
    ($f:path) => {{
        // 🚫️async: E4 fn-pointer slot
        fn __thunk<'a>(s: &'a [$crate::ErasedComposeSource]) -> $crate::ComposeFuture<'a> {
            Box::pin($f(s))
        }
        __thunk
    }};
}

#[doc(hidden)]
pub use semio_framework_io_schema as io_schema;

/// 🌀️ `compose_thunk!`'s twin for `io_mechanism::IoEntry.run` — wraps an
/// `async fn(&IoPayload) -> IoResult<IoPayload>` in a macro-generated E4 thunk that resolves it
/// synchronously via `resolve_ready` (unlike `compose_thunk!`, `IoEntry.run` is genuinely
/// non-future-returning: every codec body here runs to completion without suspending, see
/// `resolve_ready`'s own doc comment).
#[macro_export]
macro_rules! io_run_thunk {
    ($f:path) => {{
        // 🚫️async: E4 fn-pointer slot
        fn __thunk(payload: &$crate::io_schema::IoPayload) -> $crate::io_schema::IoResult<$crate::io_schema::IoPayload> {
            ::semio_framework_async::poll::resolve_ready($f(payload))
        }
        __thunk
    }};
}

/// 🌀️ `io_run_thunk!`'s twin for `io_mechanism::IoEntry.sniff` — wraps an
/// `async fn(&IoPayload) -> Confidence`.
#[macro_export]
macro_rules! io_sniff_thunk {
    ($f:path) => {{
        // 🚫️async: E4 fn-pointer slot
        fn __thunk(payload: &$crate::io_schema::IoPayload) -> $crate::io_schema::Confidence {
            ::semio_framework_async::poll::resolve_ready($f(payload))
        }
        __thunk
    }};
}

pub use crate::lifted_composer_registry::{ArtifactAssemblyRegistryError, ArtifactAssemblyRegistryPlan, commit_artifact_assembly_registry_plan, publish_native_snapshot, register_native_document_codec, register_native_snapshot_codec};
pub use semio_framework_os_store_codec_registry::composer_registry::{AsyncComposeFn, ComposeError, ComposeFuture, ComposedArtifact, ComposerEntry, Confidence, ErasedComposeSource, FORMAT_CATALOG, FormatDescriptor, FormatDescriptorIndexes, FormatRegistryConflict, FormatRegistryError, IO_REGISTRY, IoDirection, IoKey, IoRegistryConflict, IoRegistryRegistrationError, IoRegistryUnavailable, SUBSET_VALIDATOR_REGISTRY, SnapshotSubsetValidation, SubsetValidator, SubsetValidatorEntry, SubsetValidatorRegistryConflict, SubsetValidatorRegistryError, bindings, canonicalize_format_descriptor, composer_entries_by_key, format_catalog, format_descriptor_keys, format_mimes, formats_csv, index_format_descriptors, io_registry, same_composer_entry, same_subset_validator_entry, sqlite_snapshot_format_descriptor, subset_validator_registry, validate_composer_entries, validate_format_descriptors, validate_subset_validators};
pub use semio_framework_io_schema::IoPayload;

pub use bindings::{ArtifactCodecBinding, ArtifactCodecBindingChannel, ArtifactCodecBindingError, artifact_codec_bindings, preflight_artifact_codec_bindings, preflight_artifact_codec_bindings_in_assembly};
pub use bindings::catalog::{ArtifactCatalogBinding, ArtifactCatalogBindingError, ArtifactCatalogCapability, ArtifactCatalogTarget, artifact_catalog_bindings, preflight_artifact_catalog_bindings_in_assembly};

#[path = "📇️composer-entries/🦀️.rs"]
mod composer_entries;
pub use composer_entries::*;

#[path = "🌉️compose-dispatch/🦀️.rs"]
mod compose_dispatch;
pub use compose_dispatch::*;

#[path = "🛡️subset-validator/🦀️.rs"]
mod subset_validator;
pub use subset_validator::*;

#[path = "🎚️io-fidelity/🦀️.rs"]
mod io_fidelity;
pub use io_fidelity::*;

#[path = "🔌️wire-framing/🦀️.rs"]
mod wire_framing;
pub use wire_framing::*;

#[path = "📚️format-registry/🦀️.rs"]
mod format_registry;
pub use format_registry::*;

//#endregion 🔖️ErasedRegistry
//#region 🔖️Tests
//#endregion 🔖️Tests

//#region 🔖️IoMechanism
/// 🚪️🆕️ The new io mechanism (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1-A task
/// 2-4, design.md §3): `Serializer`/`Deserializer` typed traits, a type-erased `IoEntry` vtable
/// row, a `(from, into)`-keyed registry, breadth-bounded deterministic routing (`io_route`),
/// route execution (`io_run`), carrier-dialect identification (`io_identify`), and the SDK-facing
/// constructors plugins build entries from. Nested in its OWN module (not flattened into `io`'s
/// top level) so its `IoPayload`/`Confidence` — deliberately DIFFERENT, wider types than the OLD
/// file's `IoPayload`/`Confidence` a few regions up (this one adds `Confidence::None` and is not
/// used by anything above this region) — never collide names with them in the same scope. D2: the
/// OLD registry above (`ComposerEntry`/`IoKey`/`io_dispatch`/`SubsetValidator`/`FormatCatalog`)
/// is untouched and keeps working; this region is purely additive until W6 deletes the old one.
#[path = "🧭️io-mechanism/🦀️.rs"]
pub mod io_mechanism;
//#endregion 🔖️IoMechanism
// #endregion io

/// 📝️ Physical text ownership for framework IO operations.
#[path = "../../../../🔨️modules/🚪️io/📝️text/🦀️.rs"]
pub mod text;

/// ⏱️ Original caller authority for physical route execution.
pub use semio_framework_io_control as control;
pub use semio_framework_os_store_codec_registry::lifted_io::{Analysis, IoFidelityClass, IoResolveError, WireComposeSource, WireComposedArtifact, dialects_for, io_keys_for, register_composer_entry_refs, register_composer_entry_refs_in_assembly, resolve};
