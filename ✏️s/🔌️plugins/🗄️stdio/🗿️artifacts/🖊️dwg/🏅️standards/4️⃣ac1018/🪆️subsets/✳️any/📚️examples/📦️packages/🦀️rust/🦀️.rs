//! 📚️ `stdio.dwg` examples: the demo document and the committed architectural drawing.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "../../🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod architectural {
        #[path = "../../🏛️architectural/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "../../🏛️architectural/🧪️tests/🧩️example/🦀️.rs"]
        mod architectural_tests;
    }
}
