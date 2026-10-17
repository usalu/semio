//! 🧬️ schema of `semio-s-artifact-stdio-semio-presentation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    #[path = "../../🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod snapshot {
        pub use semio_s_artifact_stdio_semio_presentation_snapshot::schema::snapshot::*;
        #[cfg(test)]
        #[path = "../../📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
    #[path = "."]
    pub mod diff {
        pub use semio_s_artifact_stdio_semio_presentation_snapshot::schema::diff::*;
        #[cfg(test)]
        #[path = "../../🔺️diff/🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
        pub mod handcrafted_diff_codec_tests;
    }
    #[path = "."]
    pub mod mutations {
        #[path = "../../🧬️mutations/🧰️support/🦀️.rs"]
        pub mod support;
        #[path = "../../🧬️mutations/🧭set-slide-layout/🦀️.rs"]
        pub mod set_slide_layout;
    }
}
#[path = "."]
pub mod examples {
    pub use semio_s_artifact_stdio_semio_presentation_examples::examples::*;
    #[path = "."]
    pub mod deck {
        pub use semio_s_artifact_stdio_semio_presentation_examples::examples::deck::*;
        #[cfg(test)]
        #[path = "../../../📚️examples/📽️deck/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
}
