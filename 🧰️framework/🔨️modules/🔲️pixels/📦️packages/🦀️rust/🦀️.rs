//! 🖼️ Package glue for owned raster image codecs and pixel-buffer utilities.

#[path = "../../🦀️.rs"]
mod component;

pub use component::*;

#[path = "../../✍️editing/🦀️.rs"]
pub mod editing;

#[path = "../../🧩️compositing/🦀️.rs"]
pub mod compositing;

#[path = "../../🖊️coverage/🦀️.rs"]
pub mod coverage;
#[path = "../../🎨️sampling/↗️affine/🦀️.rs"]
pub mod affine_sampling;

#[path="../../♻️retirement/🦀️.rs"]
pub mod retirement;
