//! 💻️ Semio framework OS kernel — wasm-safe document model (store, spr, dsl, pack).
//!
//! Infinite/flow component files exist under 🔨️modules/ but are unwired pending dep-DAG cleanup.

#![allow(unused_extern_crates, ambiguous_glob_reexports, unused_imports)]

extern crate self as dsl;
extern crate self as semio_framework_os_kernel;
extern crate self as dsl_grammar;
extern crate self as dsl_notation;
extern crate self as store;
extern crate self as spr;
extern crate self as vcs;
pub extern crate self as semio_format;


// 🏷️ Former standalone crate names — proc-macros (`dsl_derive`) and in-tree `use store::` /
// `use protocol::` style paths resolve through these aliases to this crate root.


#[path = "."]
pub mod os_dsl {
  #[path = "../../🔨️modules/🗣️dsl/🦀️component.rs"]
  mod component;
  pub use component::*;

  // 📡️ `span`/`diagnostic` are owned by `🧰️framework/🔨️modules/⚠️diagnostic` and reach the tree
  // through the replication crate, which mounts them once — every `crate::os_dsl::Severity` /
  // `TextSpan` / `Fault` path below resolves through these re-exports unchanged.
  pub use protocol::diagnostic;
  pub use protocol::span;

  #[path = "../../🔨️modules/🗣️dsl/🔤️token/🦀️component.rs"]
  pub mod token;

  #[path = "../../🔨️modules/🗣️dsl/🔍️lexer/🦀️component.rs"]
  pub mod lexer;

  #[path = "../../🔨️modules/🗣️dsl/🎖️trust/🦀️component.rs"]
  pub mod trust;

  pub use self::span::*;
  pub use self::diagnostic::*;
  pub use self::token::*;
  pub use self::lexer::*;
  pub use self::trust::*;

  #[path = "."]
  pub mod family {
    #[path = "../../🔨️modules/🗣️dsl/👪️family/🗂️catalog/🦀️component.rs"]
    pub mod catalog;

    #[path = "../../🔨️modules/🗣️dsl/👪️family/📎️embed/🦀️component.rs"]
    pub mod embed;

    #[path = "../../🔨️modules/🗣️dsl/👪️family/🌍️geo/🦀️component.rs"]
    pub mod geo;

    #[path = "../../🔨️modules/🗣️dsl/👪️family/🕸️graph/🦀️component.rs"]
    pub mod graph;

    #[path = "../../🔨️modules/🗣️dsl/👪️family/🧑‍🍳️recipe/🦀️component.rs"]
    pub mod recipe;

    #[path = "../../🔨️modules/🗣️dsl/👪️family/🎬️scene/🦀️component.rs"]
    pub mod scene;

    #[path = "../../🔨️modules/🗣️dsl/👪️family/📊️sheet/🦀️component.rs"]
    pub mod sheet;

  }

  #[path = "../../🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs"]
  pub mod fixture_sweep;

  #[path = "../../🔨️modules/🗣️dsl/📖️grammar/🦀️component.rs"]
  pub mod grammar;

  #[path = "../../🔨️modules/🗣️dsl/🧠️lsp/🦀️component.rs"]
  pub mod lsp;

  #[path = "../../🔨️modules/🗣️dsl/🖋️notation/🦀️component.rs"]
  pub mod notation;

  #[cfg(not(target_arch = "wasm32"))]
  #[path = "../../🔨️modules/🗣️dsl/📇️registry/🦀️component.rs"]
  pub mod registry;

  #[path = "../../🔨️modules/🗣️dsl/🧬️schema/🦀️component.rs"]
  pub mod schema;

}

#[path = "."]
pub mod os_pack {
  #[path = "../../🔨️modules/🎒️pack/🦀️component.rs"]
  mod component;
  pub use component::*;

  #[cfg(not(target_arch = "wasm32"))]
  #[path = "../../🔨️modules/🎒️pack/⌨️cli/🦀️component.rs"]
  pub mod cli;

  // 🎒️ The `.spk` container (header/footer/segments/manifest/recovery/sources) is owned by
  // `🧰️framework/🔨️modules/🎒️pack`, and its codec floor by the replication crate. What stays os-side
  // below is only the schema-driven half: the record value codec and the arbitrary/law testkit.
  pub use pack::async_;
  pub use pack::codec;
  pub use pack::codec::ids;
  pub use pack::format;
  pub use pack::http;
  pub use pack::source;
  #[cfg(not(target_arch = "wasm32"))]
  pub use pack::io;

  // 🎾️ The flat codec/ids/source surface arrives through `component`'s `pub use pack::*` above —
  // re-exporting it a second time here would make every primitive an ambiguous glob.

  #[path = "../../🔨️modules/🎒️pack/🧪️testkit/🦀️component.rs"]
  pub mod testkit;

  #[path = "../../🔨️modules/🎒️pack/🔢️value/🦀️component.rs"]
  pub mod value;

}

#[path = "."]
pub mod os_spr {
  #[path = "../../🔨️modules/📡️spr/🦀️component.rs"]
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

  #[path = "../../🔨️modules/📡️spr/🧵️channel/🦀️component.rs"]
  pub mod channel;

  #[cfg(not(target_arch = "wasm32"))]
  #[path = "../../🔨️modules/📡️spr/⌨️cli/🦀️component.rs"]
  pub mod cli;

  // 🎞️ The os authoring half of the command layer (inference, semantics, diff kit, descriptor
  // registry, composite planner). It re-exports `protocol::mutation`'s contract from its own file,
  // so `os_spr::command::Mutation` and friends still resolve here.
  #[path = "../../🔨️modules/📡️spr/🎮️command/🦀️component.rs"]
  pub mod command;

  pub use self::ids::*;
  pub use self::dictionary::*;
  pub use self::crypto::*;
  pub use self::wire::*;

  #[path = "../../🔨️modules/📡️spr/📜️history/🦀️component.rs"]
  pub mod history;

  #[cfg(not(target_arch = "wasm32"))]
  #[path = "../../🔨️modules/📡️spr/🔌️io/🦀️component.rs"]
  pub mod io;

  #[path = "../../🔨️modules/📡️spr/💎️materialize/🦀️component.rs"]
  pub mod materialize;

  #[path = "../../🔨️modules/📡️spr/🧪️testkit/🦀️component.rs"]
  pub mod testkit;

}

#[path = "../../🔨️modules/🌿️vcs/🦀️component.rs"]
pub mod os_vcs;

// 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS C1: directory event log
// schema + pure read model (`DirectoryEvent`/`DirectoryReadModel`/`fold`) — plain serde data, no
// cross-crate dependency, so it mounts cleanly unlike `🔁️workflow` below.
#[path = "../../🔨️modules/📇️directory/🦀️component.rs"]
pub mod os_directory;

// 🔁️ ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W1: `🔁️workflow/🦀️component.rs`
// is NOT mounted here (tried, reverted — see this file's own header comment "pending dep-DAG
// cleanup"): its `use semio_framework::{AppDefinition, MediaClass, MediaType, ConfigSpec,
// Terminology, Locale, …}` lines need the FULL framework crate's surface, which this wasm-safe
// kernel crate cannot depend on without an actual `semio-framework` → `semio-framework-os-kernel`
// →(back to)→ `semio-framework` cargo dependency CYCLE (`semio-framework` already depends on this
// crate — see its Cargo.toml). It is mounted in `🧰️framework/📦️packages/🦀️rust/📦️glue.rs`
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
#[path = "../../../../🔨️modules/🚪️io/🦀️component.rs"]
pub mod os_io;

// 🧬️ `io`'s pure `StandardId`/`SubsetId`/`Dialect`/`ArtifactDialect`/`ArtifactKindId`/`ArtifactRef`
// vocabulary (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1-A task 1) is mounted
// ONCE, here — it has no `store::`/registry dependency, so unlike `os_io` above it does not need
// double-mounting. `semio-framework`'s own glue re-exports THIS module (`pub use
// semio_framework_os_kernel::io_schema;`) instead of remounting the schema file a second time; the
// registry file (`os_io`/`io`, both still mounted) references it uniformly via `crate::io_schema`,
// which resolves correctly whichever crate compiles that shared file.
#[path = "../../../../🔨️modules/🚪️io/🧬️schema/🦀️component.rs"]
pub mod io_schema;

#[path = "."]
pub mod os_store {
  #[path = "../../🔨️modules/🏪️store/🦀️component.rs"]
  mod component;
  pub use component::*;

  #[cfg(feature = "sync")]
  #[path = "../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs"]
  pub mod sync;

  #[cfg(all(feature = "worker", target_arch = "wasm32"))]
  #[path = "../../🔨️modules/🏪️store/👷️worker/🦀️component.rs"]
  pub mod worker;
}

#[path = "."]
pub mod os_engine {
  #[path = "../../🔨️modules/⚙️engine/🦀️component.rs"]
  mod component;
  pub use component::*;
}

#[path = "."]
pub mod os_inference {
  #[path = "../../🔨️modules/💡️inference/🦀️component.rs"]
  mod component;
  pub use component::*;
}


pub use crate::os_dsl::*;
pub use crate::os_store::*;
pub use crate::os_store::test_support;
pub use crate::os_spr::*;
pub use crate::os_pack::*;
pub use crate::os_inference::*;
#[path = "../../🔨️modules/🧬️semio/🦀️component.rs"]
pub mod os_semio;

#[path = "../../🔨️modules/🧩️extension/🦀️component.rs"]
pub mod os_extension;

pub use crate::os_vcs::*;
pub use crate::os_engine::*;
pub use crate::os_semio::*;
pub use crate::os_extension as extension;

// Former dsl_notation crate root surface
pub use crate::os_dsl::notation::*;
pub use crate::os_dsl::grammar::*;
pub use crate::os_dsl::{diagnostic::*, lexer::*, span::*, token::*, trust::*};

//#region 🧪️Tests
/// 🚨️ Every `#[path]` in this file must point at a file that exists. A mount whose target moved
/// turns into "os-kernel does not compile" for every session in the tree, with an error that names
/// a path rather than a cause; this turns it into one named failing test in the owning crate.
#[test]
fn every_path_mount_in_this_glue_resolves_to_an_existing_file() {
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = include_str!("📦️glue.rs");
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
