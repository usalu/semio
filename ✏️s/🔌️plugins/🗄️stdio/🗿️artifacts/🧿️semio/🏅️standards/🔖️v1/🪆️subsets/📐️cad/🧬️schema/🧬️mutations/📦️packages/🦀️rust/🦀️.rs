//! 🧬️ mut-family of `semio-s-artifact-stdio-semio-cad` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_cad_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_cad_examples::schema::diff;
    #[path = "."]
    pub mod mutations {
        pub use semio_s_artifact_stdio_semio_cad_examples::schema::mutations::support;
        #[path = "../../🧱add-block/🦀️.rs"]
        pub mod add_block;
        #[path = "../../🧩add-block-entity/🦀️.rs"]
        pub mod add_block_entity;
        #[path = "../../🔷add-entity/🦀️.rs"]
        pub mod add_entity;
        #[path = "../../🗂️add-layer/🦀️.rs"]
        pub mod add_layer;
        #[path = "../../🚫remove-block/🦀️.rs"]
        pub mod remove_block;
        #[path = "../../✂️remove-block-entity/🦀️.rs"]
        pub mod remove_block_entity;
        #[path = "../../🗑️remove-entity/🦀️.rs"]
        pub mod remove_entity;
        #[path = "../../🧹remove-layer/🦀️.rs"]
        pub mod remove_layer;
        #[path = "../../🔺set-block-entity-geometry/🦀️.rs"]
        pub mod set_block_entity_geometry;
    }
}
