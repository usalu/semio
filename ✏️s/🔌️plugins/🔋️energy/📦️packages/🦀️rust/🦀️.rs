//! ⚡️ Energy plugin — headless building energy model (BEM) engine: typed Rust API for transient
//! whole-building simulation (EnergyPlus-class predictor-corrector kernel), no IDF/epJSON, templates,
//! scripting, or language bindings. See `AGENTS.md` for the domain overview.
//!
//! WIRING ONLY. Every `pub mod` below points at exactly one taxonomy/module component file with a
//! `#[path]` written in full, relative to the owner root (this file itself lives two levels deeper,
//! in `📦️packages/🦀️rust/`, so every path carries a `../../` prefix back out to the owner root) —
//! do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).
//!
//! 🔄️ 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES: these 50 mounts moved out of
//! `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/<domain>/` into a plugin-level module,
//! `🔨️modules/⚡️simulation/⚙️engine/<domain>/` — a `💡️inferences` family member must satisfy
//! `Inference<Snapshot>` (total, deterministic, pure over a snapshot); this fallible, on-demand BEM
//! simulation kernel (air/HVAC/plant solves, curve fits, sizing, economics, …) does not, so it was
//! never a legitimate inference. An artifact is a schema + io system, never an engine; a MODULE may
//! still have one (`taxonomyLeafParentDirs` already lists `⚙️engine` globally — see the
//! `🏗️fem`/`✏️s/🔨️modules/🏗️fem/⚙️engine/` precedent under this same ticket). Energy has no document
//! app (see the "Shape note" below), so this is a module engine, not an app engine. Mount NAMEs are
//! unchanged (`crate::air_exchange`, `crate::kernel`, …), only the `#[path]` TARGET moved, so every
//! existing `crate::<domain>::X`-style call site elsewhere in this crate is unaffected — declared flat
//! at the crate root, one file per domain, with the flat `pub use` re-export surface preserved so
//! `crate::props::…`/`crate::units::…`-style internal references and any external
//! `semio_s_plugin_energy::<Type>` usage both keep working unchanged.
//!
//! 🧭️ Shape note: energy is a headless library plugin — no document app, no DSL/pack/spr wire
//! codec of its own, no command surface. There is no app to receive "behaviour", which is why the
//! 50 domain modules below relocated to a plugin-level `🔨️modules/` engine rather than an app engine.

#![allow(clippy::too_many_arguments)]

use semio_s_artifact_energy_model::*;
mod editor {
    pub use semio_s_artifact_energy_model::editor::*;
}
mod viewer {
    pub use semio_s_artifact_energy_model::viewer::*;
}

//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
pub mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin, plugin::EnergyApps);

//#endregion 🔖️Plugin
