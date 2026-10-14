//! 📚️ remodeling examples: the demo document and the synthetic-orbit capture.

extern crate semio_framework_os_kernel as protocol;

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "../../🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "../../🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
    #[path = "."]
    pub mod synthetic_orbit {
        #[path = "../../🛰️synthetic-orbit/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
