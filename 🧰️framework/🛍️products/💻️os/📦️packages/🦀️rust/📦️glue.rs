//! 💻️ Semio framework OS kernel — wasm-safe document model (store, spr, dsl, pack).
//!
//! Infinite/flow component files exist under 🔨️modules/ but are unwired pending dep-DAG cleanup.

extern crate self as dsl;
extern crate self as dsl_grammar;
extern crate self as dsl_notation;
extern crate self as store;
extern crate self as protocol;
extern crate self as pack;
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

  #[path = "../../🔨️modules/🗣️dsl/📍️span/🦀️component.rs"]
  pub mod span;

  #[path = "../../🔨️modules/🗣️dsl/⚠️diagnostic/🦀️component.rs"]
  pub mod diagnostic;

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

  #[path = "../../🔨️modules/🎒️pack/⏳️async/🦀️component.rs"]
  pub mod async_;

  #[cfg(not(target_arch = "wasm32"))]
  #[path = "../../🔨️modules/🎒️pack/⌨️cli/🦀️component.rs"]
  pub mod cli;

  #[path = "../../🔨️modules/🎒️pack/🆔️ids/🦀️component.rs"]
  pub mod ids;

  #[path = "../../🔨️modules/🎒️pack/🧾️codec/🦀️component.rs"]
  pub mod codec;

  #[path = "../../🔨️modules/🎒️pack/🚰️source/🦀️component.rs"]
  pub mod source;

  pub use self::ids::*;
  pub use self::source::*;
  // 🎾️ Re-export codec primitives without PackSource/PackSink (those live in `source`).
  pub use self::codec::{
    ByteReader, ByteWriter, CompressionCodec, NoCompression, PackError, PackLimits, crc32c,
    is_minimal_varint, read_varint_i64, read_varint_u64, write_varint_i64, write_varint_u64,
  };

  #[path = "../../🔨️modules/🎒️pack/📐️format/🦀️component.rs"]
  pub mod format;

  #[path = "../../🔨️modules/🎒️pack/🌐️http/🦀️component.rs"]
  pub mod http;

  #[path = "../../🔨️modules/🎒️pack/🔢️index/🦀️component.rs"]
  pub mod index;

  #[cfg(not(target_arch = "wasm32"))]
  #[path = "../../🔨️modules/🎒️pack/🔌️io/🦀️component.rs"]
  pub mod io;

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

  #[path = "../../🔨️modules/📡️spr/🔗️causal/🦀️component.rs"]
  pub mod causal;

  #[path = "../../🔨️modules/📡️spr/🧵️channel/🦀️component.rs"]
  pub mod channel;

  #[cfg(not(target_arch = "wasm32"))]
  #[path = "../../🔨️modules/📡️spr/⌨️cli/🦀️component.rs"]
  pub mod cli;

  #[path = "../../🔨️modules/📡️spr/🎮️command/🦀️component.rs"]
  pub mod command;

  #[path = "../../🔨️modules/📡️spr/🆔️ids/🦀️component.rs"]
  pub mod ids;

  #[path = "../../🔨️modules/📡️spr/🔢️scalar/🦀️component.rs"]
  pub mod scalar;

  #[path = "../../🔨️modules/📡️spr/📖️dictionary/🦀️component.rs"]
  pub mod dictionary;

  #[path = "../../🔨️modules/📡️spr/🔐️crypto/🦀️component.rs"]
  pub mod crypto;

  #[path = "."]
  pub mod wire {
    #[path = "../../🔨️modules/📡️spr/🧾️wire/🦀️component.rs"]
    mod codec;
    pub use codec::*;

    #[path = "../../🔨️modules/📡️spr/📡️wire/🦀️component.rs"]
    mod hub;
    pub use hub::*;

    // 🧬️ Historical protocol facade re-exported ids/crypto/dictionary through `wire::`.
    pub use super::ids::*;
    pub use super::crypto::*;
    pub use super::dictionary::*;
  }

  pub use self::ids::*;
  pub use self::dictionary::*;
  pub use self::crypto::*;
  pub use self::wire::*;

  #[path = "../../🔨️modules/📡️spr/🔀️crdt/🦀️component.rs"]
  pub mod crdt;

  #[path = "../../🔨️modules/📡️spr/📐️format/🦀️component.rs"]
  pub mod format;

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

// 🚪️ `io`'s `ArtifactDialect`/`Dialect` vocabulary is mounted independently in every crate that
// needs it (`semio-framework`, each plugin's own glue.rs) rather than depended on, because
// `semio-framework` itself depends on `semio-framework-os-kernel` — a kernel-side dependency on
// `semio-framework` would be circular. This mount exists solely so `store::ArtifactEnvelope` can
// carry a persisted `dialect`/`migrated_from` coordinate (26/08/10 D4 evolution slice); it is the
// same file, same nominal-type-per-compilation-unit tradeoff every other mount already accepts.
#[path = "../../../../🔨️modules/🚪️io/🦀️component.rs"]
pub mod os_io;

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


pub use crate::os_dsl::*;
pub use crate::os_store::*;
pub use crate::os_spr::*;
pub use crate::os_pack::*;
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
