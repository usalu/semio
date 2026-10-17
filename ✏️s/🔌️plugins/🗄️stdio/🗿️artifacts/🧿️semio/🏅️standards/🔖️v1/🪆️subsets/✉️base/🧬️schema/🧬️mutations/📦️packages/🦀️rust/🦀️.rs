//! 🧬️ mut-family of `semio-s-artifact-stdio-semio` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    #[path = "."]
    pub mod mutations {
        #[path = "../../🔊apply-audio/🦀️.rs"]
        pub mod apply_audio;
        #[path = "../../🧱apply-brep/🦀️.rs"]
        pub mod apply_brep;
        #[path = "../../📐apply-cad/🦀️.rs"]
        pub mod apply_cad;
        #[path = "../../📃apply-document/🦀️.rs"]
        pub mod apply_document;
        #[path = "../../🖊️apply-drawing/🦀️.rs"]
        pub mod apply_drawing;
        #[path = "../../🔀apply-flow/🦀️.rs"]
        pub mod apply_flow;
        #[path = "../../🌐apply-graph/🦀️.rs"]
        pub mod apply_graph;
        #[path = "../../🖼️apply-image/🦀️.rs"]
        pub mod apply_image;
        #[path = "../../🧰apply-kit/🦀️.rs"]
        pub mod apply_kit;
    }
    pub use semio_s_artifact_stdio_semio_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_inferences::schema::diff;
}
