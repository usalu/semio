//! ✏️ editor of `semio-s-artifact-stdio-semio-document` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_document_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_document_mutations::schema::mutations;
}
#[path = "."]
pub mod editor {
    #[path = "../../🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            pub use semio_s_artifact_stdio_semio_document_examples::editor::modes::edit::*;
            pub use semio_s_artifact_stdio_semio_document_examples::editor::modes::edit::windows;
        }
    }
    pub use semio_s_artifact_stdio_semio_document_mutations::editor::edit_rules;
}
#[path = "."]
pub mod viewer {
    pub use semio_s_artifact_stdio_semio_document_io::viewer::*;
    #[cfg(test)]
    #[path = "../../../👁️viewer/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
