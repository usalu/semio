//! 🚪️ io of `semio-s-artifact-stdio-semio-document` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "."]
pub mod io {
    #[path = "../../🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_document_schema::schema::*;
    pub use semio_s_artifact_stdio_semio_document_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_document_examples::schema::diff;
    pub use semio_s_artifact_stdio_semio_document_schema::schema::inferences;
    #[path = "."]
    pub mod mutations {
        pub use semio_s_artifact_stdio_semio_document_mutations::schema::mutations::*;
        pub use semio_s_artifact_stdio_semio_document_mut_family::schema::mutations::insert_style;
        #[cfg(test)]
        #[path = "../../../🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "../../../👁️viewer/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            pub use semio_s_artifact_stdio_semio_document_examples::viewer::modes::view::*;
            pub use semio_s_artifact_stdio_semio_document_examples::viewer::modes::view::windows;
        }
    }
}
