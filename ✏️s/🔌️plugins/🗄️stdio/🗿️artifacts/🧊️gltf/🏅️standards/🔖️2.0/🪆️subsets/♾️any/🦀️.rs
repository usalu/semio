//#region 🧊️GltfAnySubset
//! 🫙️ The subset-level component leaf, carrying no declarations of its own: `🦀️.rs` builds
//! `pub mod subsets { pub mod any { … } }` as an inline barrel (`#[path = "."]`) and `#[path]`s into
//! the real leaves (`🚪️io/🦀️.rs`, `🧬️schema/🦀️.rs`); this file is mounted into that barrel as its
//! `mod component` — the shape 29 of the 31 subset-level leaves in `✏️s/🔌️plugins` already use
//! (e.g. `✒️writer/🗿️artifacts/✒️writer/🦀️.rs:242`). It was the only one of the 31 left unmounted,
//! which is why `plugin-registry check` reported it unreachable from the Cargo manifest.
//#endregion 🧊️GltfAnySubset
