//! ♾️ OS infinite family glue.

// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; this alias only feeds the browser
// wasm-bindgen async-fn codegen in this crate's session bridges, so it is narrowed to exclude the
// WASI component target.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
extern crate semio_framework_async as wasm_bindgen_futures;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

extern crate self as infinite;

//#region 🔖️KernelModuleAliases
pub use semio_framework_os_kernel::os_dsl;
pub use semio_framework_os_kernel::os_pack;
pub use semio_framework_os_kernel::os_spr;
/// 🧬️ Components still use former kernel path names (`crate::os_store` / `os_dsl` / `os_spr`).
pub use semio_framework_os_kernel::os_store;
pub use semio_framework_os_kernel::os_vcs;
//#endregion 🔖️KernelModuleAliases

//#region 🔖️TerrainSession
/// 🏔️ Terrain session core path-mounted (surface crate depends on infinite — avoid cargo cycle).
#[path = "../../../../../../🔨️modules/🗺️surface/🏔️terrain/🦀️.rs"]
pub mod framework_surface_terrain;
//#endregion 🔖️TerrainSession

#[path = "../../🦀️.rs"]
mod component;
pub use component::*;

#[path = "../../🌍️world/🦀️.rs"]
pub mod world;

#[path = "../../🖼️canvas/🦀️.rs"]
pub mod canvas;
pub use canvas::*;

#[path = "."]
pub mod board {
    #[path = "../../🎲️board/🦀️.rs"]
    mod component;
    pub use component::*;

    #[path = "."]
    pub mod ports {
        #[path = "../../🎲️board/🔌️ports/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎲️board/🔌️ports/➡️directed/🦀️.rs"]
        pub mod directed;

        #[path = "../../🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs"]
        pub mod directed_normal;

        #[path = "../../🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs"]
        pub mod directed_dag;

        #[path = "../../🎲️board/🔌️ports/↔️undirected/🦀️.rs"]
        pub mod undirected;
    }

    #[path = "."]
    pub mod normal {
        #[path = "../../🎲️board/➕️normal/➡️directed/🦀️.rs"]
        pub mod directed;

        #[path = "../../🎲️board/➕️normal/↔️undirected/🦀️.rs"]
        pub mod undirected;
    }
}

//#region 🔖️DirectedNormalSurface
pub use board::ports::directed::force_graph;
/// ♾️ Crate-root surface for plugins that `extern crate infinite_canvas as infinite_board_port_directed(_normal)`.
/// `directed_normal` already re-exports `ports::directed::*` (layouts, `GraphExtension`, `BoardEngine`, …).
pub use board::ports::directed_normal::*;
pub use board::HandleRole;
//#endregion 🔖️DirectedNormalSurface

//#region 🔖️DirectedDagSurface
pub use board::ports::directed_dag::{
    dag_document_from_fixture, dag_fixture_from_document, dag_fixture_to_wire_literal, dag_node_kind_tag, default_dag_document, fit_node_size, note_widget_size, preview_widget_size, would_create_cycle, DagCamera, DagDiff, DagEdgePatch, DagFixture,
    DagFixtureEdge, DagHost, DagLayoutOptions, DagLayoutOrientation, DagMutation, DagNodeKind, DagNodePatch, DagNodeSpec, DagPreviewContent, DagSnapshot, EdgeRouteStyle, IoPortSpec, PortShape, DAG_DOCUMENT_SCHEMA,
};
//#endregion 🔖️DirectedDagSurface
