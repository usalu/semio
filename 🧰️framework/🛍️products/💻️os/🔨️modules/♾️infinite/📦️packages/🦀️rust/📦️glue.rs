//! ♾️ OS infinite family glue.

extern crate self as infinite_world;
extern crate self as infinite_canvas;
extern crate self as infinite_board;

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
