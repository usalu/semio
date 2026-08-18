//! 📦️ Package glue — wiring only. Domain lives at the owner `🦀️component.rs` files.
//!
//! The crate's `[lib] name` is `protocol`: every replica, authority and plugin crate speaks the
//! replication contract through that one canonical name.

#![allow(ambiguous_glob_reexports, unused_imports)]

#[path = "."]
pub mod codec {
  #[path = "../../⚙️codec/🦀️component.rs"]
  mod component;
  pub use component::*;

  #[path = "../../⚙️codec/🆔️ids/🦀️component.rs"]
  pub mod ids;

  pub use self::ids::*;
}

#[path = "../../🚰️source/🦀️component.rs"]
pub mod source;

#[path = "../../../⚠️diagnostic/🦀️component.rs"]
pub mod diagnostic;

#[path = "../../../⚠️diagnostic/📍️span/🦀️component.rs"]
pub mod span;

#[path = "../../../🌱️value/🦀️component.rs"]
pub mod value;

#[path = "../../🆔️ids/🦀️component.rs"]
pub mod ids;

#[path = "../../🔢️scalar/🦀️component.rs"]
pub mod scalar;

#[path = "../../📖️dictionary/🦀️component.rs"]
pub mod dictionary;

#[path = "../../🔐️crypto/🦀️component.rs"]
pub mod crypto;

#[path = "../../🎮️mutation/🦀️component.rs"]
pub mod mutation;

#[path = "../../🔗️causal/🦀️component.rs"]
pub mod causal;

#[path = "../../⚔️conflict/🦀️component.rs"]
pub mod conflict;

#[path = "."]
pub mod wire {
  #[path = "../../🧾️wire/🦀️component.rs"]
  mod codec;
  pub use codec::*;

  #[path = "../../📡️wire/🦀️component.rs"]
  mod frames;
  pub use frames::*;

  // 🧬️ The facade has always surfaced ids/crypto/dictionary through `wire::`.
  pub use super::crypto::*;
  pub use super::dictionary::*;
  pub use super::ids::*;
}

#[path = "../../📐️format/🦀️component.rs"]
pub mod format;

#[path = "../../🦀️component.rs"]
mod component;
pub use component::*;

pub use crate::causal::*;
pub use crate::causal::{FrontierComparison as RuntimeFrontierComparison, FrontierSummary as RuntimeFrontierSummary};
pub use crate::codec::*;
pub use crate::conflict::*;
pub use crate::diagnostic::*;
pub use crate::format::*;
pub use crate::mutation::*;
pub use crate::scalar::*;
pub use crate::source::*;
pub use crate::span::*;
pub use crate::value::*;
pub use crate::wire::*;
