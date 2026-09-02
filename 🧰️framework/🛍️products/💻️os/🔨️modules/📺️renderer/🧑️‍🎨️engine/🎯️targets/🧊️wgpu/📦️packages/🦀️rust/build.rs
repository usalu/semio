//! 🏗️ Renderer build package adapter (ASCII-named: Cargo derives an internal
//! `build_script_<stem>` crate identifier from this file's basename, which rejects
//! non-ASCII/emoji stems -- so this one file must keep a plain name. See
//! `🏗️builder/🦀️.rs` (identical content, the catalog-declared adapter) for the
//! canonical kind-only copy.

include!("../../🏗️builder/🦀️.rs");
