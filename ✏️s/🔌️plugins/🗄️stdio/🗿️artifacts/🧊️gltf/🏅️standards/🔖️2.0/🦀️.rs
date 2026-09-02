//#region 🧊️Gltf20Standard
//! 🫙️ Deliberately empty. `🦀️.rs` builds `pub mod standards { pub mod v_2_0 { … } }` inline
//! (`#[path = "."]`, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES: "real code now
//! lives in `subsets::any::{io,schema}`; this stays an inline barrel") rather than `#[path]`-ing
//! into this file — every stdio sibling with more than one `🏅️standards` version (`📄️pdf`
//! 1.4/1.7, `🖊️dwg` ac1018/ac1024, `🎞️gif` 87a/89a, `🏗️ifc` 2x3/4) leaves this same position
//! unmounted too. Not part of any `mod` tree; `crate::artifacts::gltf` resolves through
//! `../../🦀️.rs` instead.
//#endregion 🧊️Gltf20Standard
