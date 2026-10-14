//! 🗂️ Applied entropy measures: streaming estimators, machine-learning uncertainty, spatial and graph entropies and the feature registry.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

#[path = "."]
pub mod schema {
    #[path = "."]
    pub mod entropy_internals {
        #[path = "../../🗺️spatial/🦀️.rs"]
        pub mod spatial;
        #[path = "../../🕸️graph/🦀️.rs"]
        pub mod graph;
        #[path = "../../🤖️ml/🦀️.rs"]
        pub mod ml;
        #[path = "../../🚰️streaming/🦀️.rs"]
        pub mod streaming;
        #[path = "../../🏷️features/🦀️.rs"]
        pub mod features;
    }
}
