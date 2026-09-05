//! 💻️ Semio framework OS kernel — wasm-safe document model (store, spr, dsl, pack).
//!
//! Infinite/flow component files exist under 🔨️modules/ but are unwired pending dep-DAG cleanup.

#![allow(
    unused_extern_crates,
    ambiguous_glob_reexports,
    unused_imports,
    dead_code,
    async_fn_in_trait,
    clippy::approx_constant,
    clippy::await_holding_lock,
    clippy::double_must_use,
    clippy::drop_non_drop,
    clippy::empty_line_after_doc_comments,
    clippy::explicit_counter_loop,
    clippy::extra_unused_type_parameters,
    clippy::field_reassign_with_default,
    clippy::len_without_is_empty,
    clippy::let_underscore_future,
    clippy::map_unwrap_or,
    clippy::needless_pass_by_value,
    clippy::needless_range_loop,
    clippy::result_large_err,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::unnecessary_get_then_check,
    clippy::unnecessary_wraps,
    clippy::vec_init_then_push
)]

extern crate self as dsl;
extern crate self as dsl_grammar;
extern crate self as dsl_notation;
pub extern crate self as semio_format;
extern crate self as semio_framework_os_kernel;
extern crate self as spr;
extern crate self as store;
extern crate self as vcs;
// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; this alias only feeds the browser
// wasm-bindgen async-fn codegen in this crate's session/transport bridges, so it is narrowed to
// exclude the WASI component target.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
extern crate semio_framework_async as wasm_bindgen_futures;

// 🏷️ Former standalone crate names — proc-macros (`dsl_derive`) and in-tree `use store::` /
// `use protocol::` style paths resolve through these aliases to this crate root.

#[path = "."]
pub mod os_dsl {
    #[path = "../../🔨️modules/🗣️dsl/🦀️.rs"]
    mod component;
    pub use component::*;

    // 📡️ `span`/`diagnostic` are owned by `🧰️framework/🔨️modules/⚠️diagnostic` and reach the tree
    // through the replication crate, which mounts them once — every `crate::os_dsl::Severity` /
    // `TextSpan` / `Fault` path below resolves through these re-exports unchanged.
    pub use protocol::diagnostic;
    pub use protocol::span;

    #[path = "../../🔨️modules/🗣️dsl/🔤️token/🦀️.rs"]
    pub mod token;

    #[path = "../../🔨️modules/🗣️dsl/🔍️lexer/🦀️.rs"]
    pub mod lexer;

    #[path = "../../🔨️modules/🗣️dsl/🎖️trust/🦀️.rs"]
    pub mod trust;

    pub use self::diagnostic::*;
    pub use self::lexer::*;
    pub use self::span::*;
    pub use self::token::*;
    pub use self::trust::*;

    #[path = "."]
    pub mod family {
        #[path = "../../🔨️modules/🗣️dsl/👪️family/🗂️catalog/🦀️.rs"]
        pub mod catalog;

        #[path = "../../🔨️modules/🗣️dsl/👪️family/📎️embed/🦀️.rs"]
        pub mod embed;

        #[path = "../../🔨️modules/🗣️dsl/👪️family/🌍️geo/🦀️.rs"]
        pub mod geo;

        #[path = "../../🔨️modules/🗣️dsl/👪️family/🕸️graph/🦀️.rs"]
        pub mod graph;

        #[path = "../../🔨️modules/🗣️dsl/👪️family/🧑‍🍳recipe/🦀️.rs"]
        pub mod recipe;

        #[path = "../../🔨️modules/🗣️dsl/👪️family/🎬️scene/🦀️.rs"]
        pub mod scene;

        #[path = "../../🔨️modules/🗣️dsl/👪️family/📊️sheet/🦀️.rs"]
        pub mod sheet;
    }

    #[path = "../../🔨️modules/🗣️dsl/🧹️fixture-sweep/🦀️.rs"]
    pub mod fixture_sweep;

    #[path = "../../🔨️modules/🗣️dsl/📖️grammar/🦀️.rs"]
    pub mod grammar;

    #[path = "../../🔨️modules/🗣️dsl/🧠️lsp/🦀️.rs"]
    pub mod lsp;

    #[path = "../../🔨️modules/🗣️dsl/🖋️notation/🦀️.rs"]
    pub mod notation;

    #[cfg(not(target_arch = "wasm32"))]
    #[path = "../../🔨️modules/🗣️dsl/📇️registry/🦀️.rs"]
    pub mod registry;

    #[path = "../../🔨️modules/🗣️dsl/🧬️schema/🦀️.rs"]
    pub mod schema;
}

#[path = "."]
pub mod os_pack {
    #[path = "../../🔨️modules/🎒️pack/🦀️.rs"]
    mod component;
    pub use component::*;

    #[cfg(not(target_arch = "wasm32"))]
    #[path = "../../🔨️modules/🎒️pack/⌨️cli/🦀️.rs"]
    pub mod cli;

    // 🎒️ The `.spk` container (header/footer/segments/manifest/recovery/sources) is owned by
    // `🧰️framework/🔨️modules/🎒️pack`, and its codec floor by the replication crate. What stays os-side
    // below is only the schema-driven half: the record value codec and the arbitrary/law testkit.
    pub use pack::async_;
    pub use pack::codec;
    pub use pack::codec::ids;
    pub use pack::format;
    pub use pack::http;
    #[cfg(not(target_arch = "wasm32"))]
    pub use pack::io;
    // 🌉️ `DslValue ↔ pack::json::Value` bridge + `to_json_string`/`from_json_str` — ticket
    // `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`'s `TopicContribution`
    // seam and every plugin converting off `serde_json` route JSON text through this.
    pub use pack::json;
    pub use pack::source;

    // 🎾️ The flat codec/ids/source surface arrives through `component`'s `pub use pack::*` above —
    // re-exporting it a second time here would make every primitive an ambiguous glob.

    #[path = "../../🔨️modules/🎒️pack/🧪️testkit/🦀️.rs"]
    pub mod testkit;

    #[path = "../../🔨️modules/🎒️pack/🌱️value/🦀️.rs"]
    pub mod value;
}

#[path = "."]
pub mod os_spr {
    #[path = "../../🔨️modules/📡️spr/🦀️.rs"]
    mod component;
    pub use component::*;

    // 📡️ The replication contract itself (frames, envelopes, mutation traits, conflict vocabulary,
    // `.spr` format) lives in `🧰️framework/🔨️modules/📡️replication`; the kernel speaks it but no
    // longer owns it. This facade keeps every historical `protocol::`/`os_spr::` path working.
    pub use protocol::causal;
    pub use protocol::conflict;
    pub use protocol::crypto;
    pub use protocol::dictionary;
    pub use protocol::format;
    pub use protocol::ids;
    pub use protocol::scalar;
    pub use protocol::wire;

    #[path = "../../🔨️modules/📡️spr/🧵️channel/🦀️.rs"]
    pub mod channel;

    #[cfg(not(target_arch = "wasm32"))]
    #[path = "../../🔨️modules/📡️spr/⌨️cli/🦀️.rs"]
    pub mod cli;

    // 🎞️ The os authoring half of the command layer (inference, semantics, diff kit, descriptor
    // registry, composite planner). It re-exports `protocol::mutation`'s contract from its own file,
    // so `os_spr::command::Mutation` and friends still resolve here.
    #[path = "../../🔨️modules/📡️spr/🎮️command/🦀️.rs"]
    pub mod command;

    pub use self::crypto::*;
    pub use self::dictionary::*;
    pub use self::ids::*;
    pub use self::wire::*;

    #[path = "../../🔨️modules/📡️spr/📜️history/🦀️.rs"]
    pub mod history;

    #[cfg(not(target_arch = "wasm32"))]
    #[path = "../../🔨️modules/📡️spr/🔌️io/🦀️.rs"]
    pub mod io;

    #[path = "../../🔨️modules/📡️spr/💎️materialize/🦀️.rs"]
    pub mod materialize;

    #[path = "../../🔨️modules/📡️spr/🧪️testkit/🦀️.rs"]
    pub mod testkit;
}

#[path = "../../🔨️modules/🌿️vcs/🦀️.rs"]
pub mod os_vcs;

#[path = "../../🔨️modules/🪪️identity/🦀️.rs"]
pub mod os_identity;

// 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS C1: directory event log
// schema + pure read model (`DirectoryEvent`/`DirectoryReadModel`/`fold`) — plain serde data, no
// cross-crate dependency, so it mounts cleanly unlike `🔁️workflow` below.
#[path = "../../🔨️modules/📇️directory/🦀️.rs"]
pub mod os_directory;

// 🔁️ ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W1: `🔁️workflow/🦀️.rs`
// is NOT mounted here (tried, reverted — see this file's own header comment "pending dep-DAG
// cleanup"): its `use semio_framework::{AppDefinition, MediaClass, MediaType, ConfigSpec,
// Terminology, Locale, …}` lines need the FULL framework crate's surface, which this wasm-safe
// kernel crate cannot depend on without an actual `semio-framework` → `semio-framework-os-kernel`
// →(back to)→ `semio-framework` cargo dependency CYCLE (`semio-framework` already depends on this
// crate — see its Cargo.toml). It is mounted in `🧰️framework/📦️packages/🦀️rust/🦀️.rs`
// (the `semio-framework` crate) instead, where all of those symbols already live — see that
// file's own `os_workflow` mount for the real fix, and the run crate's glue.rs for the matching
// `extern crate semio_framework as workflow;` alias change.

// 🚪️ `io`'s FULL registry file (`ComposerEntry`/`IoKey`/`io_dispatch`/`SubsetValidator`/…) is
// still mounted independently here AND in `semio-framework`'s own glue (as `io`) — that half of
// the double-mount is recorded debt D2 (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM),
// cleaned up wholesale at W6 alongside the old registry itself. This mount exists solely so
// `store::ArtifactEnvelope` can carry a persisted `dialect`/`migrated_from` coordinate (26/08/10
// D4 evolution slice); a kernel-side dependency on the full `semio-framework` crate (to reuse ITS
// `io` mount instead) would be circular — see the `os_workflow`/`workflow` comment above.
#[path = "../../../../🔨️modules/🚪️io/🦀️.rs"]
pub mod os_io;

// 🪡 Crate-root aliases the `compose_thunk!`/`io_run_thunk!`/`io_sniff_thunk!` macros need.
// Those macros live in the shared io file and refer to `$crate::io`, `$crate::io_schema`,
// `$crate::ErasedComposeSource` and `$crate::ComposeFuture`. `$crate` expands to whichever crate
// is compiling the file — in `semio-framework` those names sit at its root, but here the module is
// mounted as `os_io` and the root `io` name is free (this crate's `pub use pack::io` is nested
// inside `os_pack`), so the macros could not resolve and every thunk-using row failed to compile.
// `io_schema` above is already root-mounted for exactly the same "resolves in whichever crate
// compiles the shared file" reason; these two lines complete that contract rather than inventing a
// new one. Part of recorded debt D2 — the double-mount itself still goes away wholesale at W6.
pub use crate::os_io as io;
pub use crate::os_io::{ComposeFuture, ErasedComposeSource};

// 🧬️ `io`'s pure `StandardId`/`SubsetId`/`Dialect`/`ArtifactDialect`/`ArtifactKindId`/`ArtifactRef`
// vocabulary (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1-A task 1) is mounted
// ONCE, here — it has no `store::`/registry dependency, so unlike `os_io` above it does not need
// double-mounting. `semio-framework`'s own glue re-exports THIS module (`pub use
// semio_framework_os_kernel::io_schema;`) instead of remounting the schema file a second time; the
// registry file (`os_io`/`io`, both still mounted) references it uniformly via `crate::io_schema`,
// which resolves correctly whichever crate compiles that shared file.
#[path = "../../../../🔨️modules/🚪️io/🧬️schema/🦀️.rs"]
pub mod io_schema;

#[path = "../../../../🔨️modules/🧬️schema/🧩️composition/🦀️.rs"]
pub mod os_schema_composition;

#[path = "."]
pub mod os_store {
    #[path = "../../🔨️modules/🏪️store/🦀️.rs"]
    mod component;
    pub use component::*;

    // 🌉️ Also excluded from `wasm32-wasip2`, not just gated on the `sync` feature. This module's
    // `use tokio::sync::{broadcast, mpsc}` is unconditional, and `tokio` is deliberately absent from
    // wasip2 (see this crate's `Cargo.toml`: the dependency is
    // `cfg(not(all(target_arch = "wasm32", target_env = "p2")))`). Cargo feature unification can still
    // switch `sync` on inside a wasip2 plugin's graph — `🌉️mcp` and the wgpu renderer both request it
    // — and the module would then fail to resolve `tokio`. Excluding it here matches what the feature's
    // own docstring already states: WASI-P2 guest plugins never link the sync actor's transport.
    #[cfg(all(feature = "sync", not(all(target_arch = "wasm32", target_env = "p2"))))]
    #[path = "../../🔨️modules/🏪️store/🔄️sync/🦀️.rs"]
    pub mod sync;

    // 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; `👷️worker/🦀️.rs` is an
    // unconditional `wasm_bindgen` Web Worker `postMessage` bridge with no internal target split of
    // its own, so the mount itself is narrowed to exclude the WASI component target (which has no
    // "Web Worker" concept for an in-guest plugin).
    #[cfg(all(feature = "worker", target_arch = "wasm32", not(target_env = "p2")))]
    #[path = "../../🔨️modules/🏪️store/👷️worker/🦀️.rs"]
    pub mod worker;
}

#[path = "."]
pub mod os_engine {
    #[path = "../../🔨️modules/⚙️engine/🦀️.rs"]
    mod component;
    pub use component::*;
}

#[path = "."]
pub mod os_inference {
    #[path = "../../🔨️modules/💡️inference/🦀️.rs"]
    mod component;
    pub use component::*;
}

pub use crate::os_dsl::*;
pub use crate::os_inference::*;
pub use crate::os_pack::*;
pub use crate::os_spr::*;
pub use crate::os_store::test_support;
pub use crate::os_store::*;
#[path = "../../🔨️modules/🧬️semio/🦀️.rs"]
pub mod os_semio;

// 🧩️ `.sxt` extension package pack/unpack/verify is host-only: installing a runtime extension is
// native-host tooling (only caller repo-wide: `semio-framework-os`'s host crate), never something a
// guest component does to itself. `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too, so this
// mount is narrowed to exclude the WASI component target with the full
// `not(all(target_arch = "wasm32", target_env = "p2"))` form — a bare arch gate would also exclude
// the browser wasm32 target, which does not need to lose this capability. See
// `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml`'s matching `zip` target-gate.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[path = "../../🔨️modules/🧩️extension/🦀️.rs"]
pub mod os_extension;

pub use crate::os_engine::*;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub use crate::os_extension as extension;
pub use crate::os_semio::*;
pub use crate::os_vcs::*;

// Former dsl_notation crate root surface
pub use crate::os_dsl::grammar::*;
pub use crate::os_dsl::notation::*;
pub use crate::os_dsl::{diagnostic::*, lexer::*, span::*, token::*, trust::*};

/// 🌱️ Crate-root re-export of `os_dsl::schema`'s `ToValue`/`FromValue`/`DslValue`/`ValueError` —
/// `#[derive(ToValue, FromValue)]` (`semio-framework-value-derive`) generates fully-qualified
/// `::semio_framework_os_kernel::ToValue`/`FromValue` paths, so every plugin depending on this
/// crate under that literal name needs them reachable at the crate root, not only as
/// `crate::schema::ToValue`.
pub use crate::os_dsl::schema::{DslValue, FromValue, ToValue, ValueError};

/// 🌿️ Crate-root re-export of the `#[derive(ToValue, FromValue)]` proc-macros themselves (distinct
/// Rust namespace from the trait re-export directly above — a derive macro and a trait can share an
/// identifier with zero conflict). `semio_framework_plugin::app_commands!` (`🔌️plugin/🦀️.rs`)
/// spells these as `$crate::ToValue`/`$crate::FromValue` in its generated `#[derive(...)]` line so the
/// path is robust regardless of what the *invoking* plugin crate has imported — `macro_rules!` gives
/// bare (non-`$crate`) identifiers def-site hygiene only for local bindings, not for macro/item paths,
/// so relying on every one of the ~190 `app_commands!` call sites to already `use
/// semio_framework_value_derive::{ToValue, FromValue}` would be fragile; `$crate::` sidesteps that.
pub use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧪️Tests
/// 🚨️ Every `#[path]` in this file must point at a file that exists. A mount whose target moved
/// turns into "os-kernel does not compile" for every session in the tree, with an error that names
/// a path rather than a cause; this turns it into one named failing test in the owning crate.
#[test]
fn every_path_mount_in_this_glue_resolves_to_an_existing_file() {
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = include_str!("🦀️.rs");
    let mut missing = Vec::new();
    for line in source.lines() {
        let Some(rest) = line.trim().strip_prefix("#[path = \"") else { continue };
        let Some(target) = rest.split('"').next() else { continue };
        if target == "." {
            continue;
        }
        if !here.join(target).exists() {
            missing.push(target.to_string());
        }
    }
    assert!(missing.is_empty(), "glue.rs mounts files that do not exist: {missing:?}");
}
//#endregion 🧪️Tests
