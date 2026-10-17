//! 🪜️ The stdio semio family's shared examples: one demo source per subset domain, each a real printed snapshot (part 2).

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

#[path = "."]
pub mod examples {
    #[path = "../🎥️clip/🦀️.rs"]
    pub mod clip;
    #[path = "../🎬️demo/🦀️.rs"]
    pub mod demo;
    #[path = "../🌐️envelope/🦀️.rs"]
    pub mod envelope;
    #[path = "../🗒️memo/🦀️.rs"]
    pub mod memo;
    #[path = "../🌊️pipeline/🦀️.rs"]
    pub mod pipeline;
    #[path = "../🧊️solid/🦀️.rs"]
    pub mod solid;
    #[path = "../🎵️tone/🦀️.rs"]
    pub mod tone;
}
