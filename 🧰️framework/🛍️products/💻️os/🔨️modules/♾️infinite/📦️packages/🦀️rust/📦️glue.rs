//! ♾️ OS infinite family glue.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as spr;

extern crate self as infinite_world;
extern crate self as infinite_canvas;
extern crate self as infinite_board;
extern crate self as infinite;

//#region 🔖️KernelModuleAliases
/// 🧬️ Components still use former kernel path names (`crate::os_store` / `os_dsl` / `os_spr`).
pub use semio_framework_os_kernel::os_store;
pub use semio_framework_os_kernel::os_dsl;
pub use semio_framework_os_kernel::os_spr;
pub use semio_framework_os_kernel::os_vcs;
pub use semio_framework_os_kernel::os_pack;
//#endregion 🔖️KernelModuleAliases

//#region 🔖️TerrainSession
/// 🏔️ Terrain session core path-mounted (surface crate depends on infinite — avoid cargo cycle).
#[path = "../../../../../../🔨️modules/🗺️surface/🏔️terrain/🦀️component.rs"]
pub mod framework_surface_terrain;
//#endregion 🔖️TerrainSession

#[path = "../../🦀️component.rs"]
mod component;
pub use component::*;

#[path = "../../🌍️world/🦀️component.rs"]
pub mod world;
pub use world::*;

#[path = "../../🖼️canvas/🦀️component.rs"]
pub mod canvas;
pub use canvas::*;

#[path = "."]
pub mod board {
  #[path = "../../🎲️board/🦀️component.rs"]
  mod component;
  pub use component::*;

  #[path = "."]
  pub mod ports {
    #[path = "../../🎲️board/🔌️ports/🦀️component.rs"]
    mod component;
    pub use component::*;

    #[path = "../../🎲️board/🔌️ports/➡️directed/🦀️component.rs"]
    pub mod directed;

    #[path = "../../🎲️board/🔌️ports/➡️directed/➕️normal/🦀️component.rs"]
    pub mod directed_normal;

    #[path = "../../🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs"]
    pub mod directed_dag;

    #[path = "../../🎲️board/🔌️ports/↔undirected/🦀️component.rs"]
    pub mod undirected;
  }

  #[path = "."]
  pub mod normal {
    #[path = "../../🎲️board/➕️normal/➡️directed/🦀️component.rs"]
    pub mod directed;

    #[path = "../../🎲️board/➕️normal/↔undirected/🦀️component.rs"]
    pub mod undirected;
  }
}
