//! ⚡️ Energy plugin composition — wiring only: mounts the plugin root (`../../🦀️.rs`), which assembles the energy
//! model artifact (`semio_s_artifact_energy_model`) with its editor and viewer crates. The simulation engine and the
//! artifact layers live in their own crates under `✏️s/🔌️plugins/🔋️energy`.

#![allow(clippy::too_many_arguments)]


//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
pub mod plugin;

//#endregion 🔖️Plugin
