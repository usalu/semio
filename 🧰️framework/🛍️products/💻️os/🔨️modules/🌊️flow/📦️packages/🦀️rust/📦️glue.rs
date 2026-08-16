//! 🌊️ OS flow family glue — wires document/catalogue/registry/bridge/host/drawing/wasm/vcs, brep geometry, and wasm SDK.
//! Light/draw/brep operator packs are packaged extensions under ✏️s/🔌️plugins/🌊️flow.

extern crate self as flow;
extern crate self as flow_extension_wasm;
extern crate self as flow_extension_sdk;

//#region 🔖️KernelCrateAliases
/// 🧬️ Derive macros (`DslRecord`/`DslArtifact`/`DslOps`) resolve `dsl`/`store`/`pack`/`spr` as crate roots.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as spr;
extern crate semio_framework_os_kernel as protocol;
//#endregion 🔖️KernelCrateAliases

//#region 🔖️KernelModuleAliases
/// 🧬️ Components still use former kernel path names (`crate::os_store` / `os_dsl` / `os_spr`).
pub use semio_framework_os_kernel::os_store;
pub use semio_framework_os_kernel::os_dsl;
pub use semio_framework_os_kernel::os_spr;
pub use semio_framework_os_kernel::os_vcs;
pub use semio_framework_os_kernel::os_pack;
//#endregion 🔖️KernelModuleAliases

//#region 🔖️InfiniteAlias
/// ♾️ Flow components use `crate::infinite::{board,canvas}` paths.
pub use semio_framework_os_infinite as infinite;
//#endregion 🔖️InfiniteAlias

//#region 🔖️DagCanvasNeural
pub use crate::infinite::board::ports::directed_dag as dag;
pub use crate::infinite::canvas as canvas;
pub use neural_engine as neural;
//#endregion 🔖️DagCanvasNeural

//#region 🔖️Playbook
/// 📖️ Playbook domain types used by the forms bridge (`playbook::PlaybookSpec`, …).
#[path = "../../../📖️playbook/🦀️component.rs"]
pub mod playbook;
//#endregion 🔖️Playbook

#[path = "../../📄️artifact/🦀️component.rs"]
pub mod artifact;
pub use artifact::*;

#[path = "../../📚️catalogue/🦀️component.rs"]
pub mod catalogue;
pub use catalogue::*;

#[path = "../../📔️registry/🦀️component.rs"]
pub mod registry;
pub use registry::*;

#[path = "../../🌉️bridge/🦀️component.rs"]
pub mod bridge;
pub use bridge::*;

#[path = "../../🖥️host/🦀️component.rs"]
pub mod host;
pub use host::*;

#[path = "../../🖍️drawing/🦀️component.rs"]
pub mod drawing;
pub use drawing::*;

#[path = "../../🌉️wasm/🦀️component.rs"]
pub mod wasm_session;
pub use wasm_session::*;

#[path = "../../🌿️vcs/🦀️component.rs"]
pub mod vcs;
pub use vcs::*;

#[path = "../../📐️brep-geometry/🦀️component.rs"]
pub mod brep_geometry;
pub use brep_geometry::{
    dispose_geometry, export_solid_json, import_solid_json, retain_geometry_handles, tessellate_geometry,
};

#[path = "."]
pub mod extensions {
  #[path = "../../🧩️extensions/🕸️wasm/🦀️component.rs"]
  pub mod wasm;
}

pub use extensions::wasm::*;
