//! 📸️ Remodel plugin — the photogrammetry/videogrammetry play app (video in → watertight mesh out)
//! bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory. Shape V2 (`26/08/05/SHAPE-V2-TREE-PURITY-BROADCAST`)
//! puts this entry file inside `📦️packages/🦀️rust/` — two levels below the plugin root — so every leaf
//! path opens with `../../` to reach back out to the component tree. The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
//#region 🧮️MathInternals
// 🧮️ 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave M3d: crate-root
// aliases onto the compute-internals mounted below in `artifacts::remodeling::…::schema` — every
// `crate::algebra::`/`crate::optimize::`/`crate::lie::`/`crate::signal::`/`crate::spatial::` call
// site (the moved files' own internal references, and the app-engine files that used to say
// `math::algebra::` etc.) resolves through these, exactly as the old `math::` extern-prelude
// name used to. `semio-framework-math` is no longer a dependency of this crate.
pub(crate) use artifacts::remodeling::standards::v1::subsets::any::schema::algebra_internals as algebra;
pub(crate) use artifacts::remodeling::standards::v1::subsets::any::schema::lie_internals as lie;
pub(crate) use artifacts::remodeling::standards::v1::subsets::any::schema::optimize_internals as optimize;
pub(crate) use artifacts::remodeling::standards::v1::subsets::any::schema::signal_internals as signal;
pub(crate) use artifacts::remodeling::standards::v1::subsets::any::schema::spatial_internals as spatial;
//#endregion 🧮️MathInternals

//#region 🗿️Artifacts
mod artifacts { pub use semio_s_artifact_remodel_remodeling as remodeling; }
//#endregion 🗿️Artifacts

//#region ✏️Editor
mod editor { pub use semio_s_artifact_remodel_remodeling::editor::*; }
//#endregion ✏️Editor

//#region 👁️Viewer
mod viewer { pub use semio_s_artifact_remodel_remodeling::viewer::*; }
//#endregion 👁️Viewer

//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
mod plugin;
pub use plugin::RemodelApps;
semio_framework_plugin::plugin_exports!(plugin::plugin, RemodelApps);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
    pub mod app_remodeling_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
