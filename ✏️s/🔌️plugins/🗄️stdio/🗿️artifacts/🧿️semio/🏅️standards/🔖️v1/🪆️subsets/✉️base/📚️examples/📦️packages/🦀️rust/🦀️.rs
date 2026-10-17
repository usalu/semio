//! 📚️ The stdio semio family's shared examples: one demo source per subset domain, each a real printed snapshot.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

#[path = "."]
pub mod examples {
    #[path = "../../🏢️building/🦀️.rs"]
    pub mod building;
    #[path = "../../📽️deck/🦀️.rs"]
    pub mod deck;
    #[path = "../../📐️drawing/🦀️.rs"]
    pub mod drawing;
    #[path = "../../🕸️graph/🦀️.rs"]
    pub mod graph;
    #[path = "../../📃️note/🦀️.rs"]
    pub mod note;
    #[path = "../../🖍️sketch/🦀️.rs"]
    pub mod sketch;
    #[path = "../../🖼️swatch/🦀️.rs"]
    pub mod swatch;
    #[path = "../../🚶️walk/🦀️.rs"]
    pub mod walk;
    pub use semio_s_artifact_stdio_semio_base_examples_2::examples::{clip, demo, envelope, memo, pipeline, solid, tone};
}
