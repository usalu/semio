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

pub use semio_framework_os_kernel::{os_dsl, os_engine, os_pack, os_semio, os_spr, os_store, os_vcs};

// 🌐️ World/3D host depends on surface terrain (which depends on this crate) — optional to avoid the cycle.
#[cfg(feature = "world3d")]
#[path = "../../🦀️component.rs"]
mod component;
#[cfg(feature = "world3d")]
pub use component::*;

#[cfg(feature = "world3d")]
#[path = "../../🌍️world/🦀️component.rs"]
pub mod world;

#[path = "../../🖼️canvas/🦀️component.rs"]
pub mod canvas;

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

/// ♾️ Components address the family as `crate::infinite::{canvas,board,...}` (former mega-crate layout).
pub mod infinite {
  #[cfg(feature = "world3d")]
  pub use crate::world;
  pub use crate::{board, canvas};
}
