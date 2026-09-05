//! 🌊️ OS flow family glue — wires document/catalogue/registry/bridge/host/drawing/wasm/vcs, brep geometry, and wasm SDK.
//! Light/draw/brep operator packs are packaged extensions under ✏️s/🔌️plugins/🌊️flow.

extern crate self as flow_extension_sdk;

//#region 🔖️KernelCrateAliases
/// 🧬️ Derive macros (`DslRecord`/`DslArtifact`/`DslOps`) resolve `dsl`/`store`/`pack`/`spr` as crate roots.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
//#endregion 🔖️KernelCrateAliases

//#region 🔖️KernelModuleAliases
pub use semio_framework_os_kernel::os_dsl;
pub use semio_framework_os_kernel::os_pack;
pub use semio_framework_os_kernel::os_spr;
/// 🧬️ Components still use former kernel path names (`crate::os_store` / `os_dsl` / `os_spr`).
pub use semio_framework_os_kernel::os_store;
pub use semio_framework_os_kernel::os_vcs;
pub use protocol::value::ordered::{OrderedMap, OrderedSet};
//#endregion 🔖️KernelModuleAliases

//#region 🔖️InfiniteAlias
/// ♾️ Flow components use `crate::infinite::{board,canvas}` paths.
pub use semio_framework_os_infinite as infinite;
//#endregion 🔖️InfiniteAlias

//#region 🔖️DagCanvasNeural
pub use crate::infinite::board::ports::directed_dag as dag;
pub use crate::infinite::canvas;
pub use neural_engine as neural;
//#endregion 🔖️DagCanvasNeural

//#region 🔖️Playbook
/// 📖️ Playbook domain types used by the forms bridge (`playbook::PlaybookSpec`, …).
#[path = "../../../📖️playbook/🦀️.rs"]
pub mod playbook;
//#endregion 🔖️Playbook

#[path = "../../🗿️artifact/🦀️.rs"]
pub mod artifact;
pub use artifact::*;

//#region 🎚️GraphParameterIntent
#[path = "../../🎚️parameter/📨️intent/🦀️.rs"]
pub mod graph_parameter;
//#endregion 🎚️GraphParameterIntent

#[path = "../../🧵️retained/🦀️.rs"]
pub mod retained;

#[path = "../../🗂️catalogue/🦀️.rs"]
pub mod catalogue;
pub use catalogue::*;

#[path = "../../📔️registry/🦀️.rs"]
pub mod registry;
pub use registry::*;

#[path = "../../🌉️bridge/🦀️.rs"]
pub mod bridge;

#[path = "../../🖥️host/🦀️.rs"]
pub mod host;
pub use host::*;

#[path = "../../🖍️drawing/🦀️.rs"]
pub mod drawing;
pub use drawing::*;

// 🌉️ `wasm_session`'s `flow_bridge_*` linear-memory ABI is the browser wasm-pack SDK entry point
// (its own `ui_webgpu::{CanvasMetrics, SurfaceId, SurfaceGeneration}` imports are canvas-surface
// types); nothing in `semio-s-plugin-flow`'s own `plugin_exports!` guest dispatch calls it
// (confirmed by repo-wide grep). `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too, so a
// bare arch gate would not exclude the shipped WASI component target — narrowed to keep native
// (this file's own `#[cfg(test)]` suite) and browser wasm, drop only wasip2, matching
// `os-kernel-host-crates-split.md`'s target-table shape. RUNTIME-DEPENDENCY-ELIMINATION ticket
// 26/09/01.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[path = "../../🕸️wasm/🦀️component.rs"]
pub mod wasm_session;

#[path = "../../🌿️vcs/🦀️.rs"]
pub mod vcs;
pub use vcs::*;

#[path = "../../📐️brep-geometry/🦀️.rs"]
pub mod brep_geometry;
pub use brep_geometry::{dispose_geometry, export_solid_json, import_solid_json, retain_geometry_handles, tessellate_geometry};

#[path = "."]
pub mod extensions {
    #[path = "../../🧩️extensions/🕸️wasm/🦀️.rs"]
    pub mod wasm;
}

pub use extensions::wasm::*;
