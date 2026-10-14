//! 🧰️ remodeling editor kit: shared editor plumbing, window transient state and terminology.

extern crate semio_framework_os_kernel as store;

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod app_remodeling_demo_session {
        #[path = "../../../📚️examples/🎬️demo-session/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "../../../📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod remodeling {
        #[path = "../../../📚️examples/🦀️.rs"]
        pub mod examples;
        #[path = "../../../🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "../../🦀️.rs"]
        pub mod kit;
    }
}
