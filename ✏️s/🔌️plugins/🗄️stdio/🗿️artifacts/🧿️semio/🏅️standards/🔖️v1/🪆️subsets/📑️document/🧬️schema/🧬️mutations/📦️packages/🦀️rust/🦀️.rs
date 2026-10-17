//! 🧬️ mut-family of `semio-s-artifact-stdio-semio-document` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_document_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_document_examples::schema::diff;
    #[path = "."]
    pub mod mutations {
        pub use semio_s_artifact_stdio_semio_document_examples::schema::mutations::support;
        #[path = "../../🧱insert-block/🦀️.rs"]
        pub mod insert_block;
        #[path = "../../🖼️insert-image/🦀️.rs"]
        pub mod insert_image;
        #[path = "../../🧶insert-style/🦀️.rs"]
        pub mod insert_style;
        #[path = "../../🪓remove-block/🦀️.rs"]
        pub mod remove_block;
        #[path = "../../🪦remove-image/🦀️.rs"]
        pub mod remove_image;
        #[path = "../../🧽️remove-style/🦀️.rs"]
        pub mod remove_style;
        #[path = "../../📦set-block-content/🦀️.rs"]
        pub mod set_block_content;
        #[path = "../../📐set-heading-level/🦀️.rs"]
        pub mod set_heading_level;
        #[path = "../../📷set-image-block/🦀️.rs"]
        pub mod set_image_block;
    }
}
