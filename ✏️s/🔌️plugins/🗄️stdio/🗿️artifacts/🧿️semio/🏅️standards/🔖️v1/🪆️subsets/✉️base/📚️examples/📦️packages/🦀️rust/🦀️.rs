//! 📚️ The stdio semio family's shared examples: one demo source per subset domain, each a real printed snapshot.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

#[path = "."]
pub mod examples {
    #[path = "../../🏢️building/🦀️.rs"]
    pub mod building;
    #[path = "../../🎥️clip/🦀️.rs"]
    pub mod clip;
    #[path = "../../📽️deck/🦀️.rs"]
    pub mod deck;
    #[path = "../../🎬️demo/🦀️.rs"]
    pub mod demo;
    #[path = "../../📐️drawing/🦀️.rs"]
    pub mod drawing;
    #[path = "../../🌐️envelope/🦀️.rs"]
    pub mod envelope;
    #[path = "../../🕸️graph/🦀️.rs"]
    pub mod graph;
    #[path = "../../🗒️memo/🦀️.rs"]
    pub mod memo;
    #[path = "../../📃️note/🦀️.rs"]
    pub mod note;
    #[path = "../../🌊️pipeline/🦀️.rs"]
    pub mod pipeline;
    #[path = "../../🖍️sketch/🦀️.rs"]
    pub mod sketch;
    #[path = "../../🧊️solid/🦀️.rs"]
    pub mod solid;
    #[path = "../../🖼️swatch/🦀️.rs"]
    pub mod swatch;
    #[path = "../../🎵️tone/🦀️.rs"]
    pub mod tone;
    #[path = "../../🚶️walk/🦀️.rs"]
    pub mod walk;
}
