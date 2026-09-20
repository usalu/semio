//#region 🧊️Gltf20Standard
//! 🫙️ The standard-level component leaf, carrying no declarations of its own: `🦀️.rs` builds
//! `pub mod standards { pub mod v2_0 { … } }` as an inline barrel (`#[path = "."]`, ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES: "real code now lives in
//! `subsets::any::{io,schema}`"), and this file is mounted INTO that barrel as its
//! `mod component` — the shape 29 of the 31 standard-level leaves in `✏️s/🔌️plugins` already use
//! (e.g. `✒️writer/🗿️artifacts/✒️writer/🦀️.rs:235`). It was the only one of the 31 left unmounted,
//! which is why `plugin-registry check` reported it unreachable from the Cargo manifest.
//#endregion 🧊️Gltf20Standard
