//! 💻️ Semio framework OS kernel — wasm-safe document model (store, spr, dsl, pack).
//!
//! Infinite/flow component files exist under 🔨️modules/ but are unwired pending dep-DAG cleanup.

extern crate self as dsl;
extern crate self as store;
extern crate self as protocol;
extern crate self as pack;
extern crate self as spr;
extern crate self as vcs;


// 🏷️ Former standalone crate names — proc-macros (`dsl_derive`) and in-tree `use store::` /
// `use protocol::` style paths resolve through these aliases to this crate root.


#[path = "."]
pub mod os_dsl {
  #[path = "../../🔨️modules/🗣️dsl/🦀️component.rs"]
  mod component;
  pub use component::*;

  #[path = "../../🔨️modules/🗣️dsl/🫀️core/🦀️component.rs"]
  pub mod core;

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

  #[path = "../../🔨️modules/🎒️pack/🫀️core/🦀️component.rs"]
  pub mod core;

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

  #[path = "../../🔨️modules/📡️spr/🫀️core/🦀️component.rs"]
  pub mod core;

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

  #[path = "../../🔨️modules/📡️spr/📡️wire/🦀️component.rs"]
  pub mod wire;

}

#[path = "../../🔨️modules/🌿️vcs/🦀️component.rs"]
pub mod os_vcs;

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


pub use crate::os_dsl::*;
pub use crate::os_store::*;
pub use crate::os_spr::*;
pub use crate::os_pack::*;
pub use crate::os_vcs::*;
