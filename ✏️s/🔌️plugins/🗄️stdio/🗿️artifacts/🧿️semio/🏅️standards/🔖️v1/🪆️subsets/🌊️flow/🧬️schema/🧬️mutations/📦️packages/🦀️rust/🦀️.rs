//! 🧬️ mut-family of `semio-s-artifact-stdio-semio-flow` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_flow_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_flow_examples::schema::diff;
    #[path = "."]
    pub mod mutations {
        pub use semio_s_artifact_stdio_semio_flow_examples::schema::mutations::support;
        #[path = "../../🌉️insert-edge/🦀️.rs"]
        pub mod insert_edge;
        #[path = "../../➕️insert-node/🦀️.rs"]
        pub mod insert_node;
        #[path = "../../✂️remove-edge/🦀️.rs"]
        pub mod remove_edge;
        #[path = "../../🗑️remove-node/🦀️.rs"]
        pub mod remove_node;
        #[path = "../../🧹️remove-node-param/🦀️.rs"]
        pub mod remove_node_param;
        #[path = "../../🎨️set-edge-kind/🦀️.rs"]
        pub mod set_edge_kind;
        #[path = "../../🏷️set-node-kind/🦀️.rs"]
        pub mod set_node_kind;
        #[path = "../../🔤️set-node-label/🦀️.rs"]
        pub mod set_node_label;
        #[path = "../../🎛️set-node-param/🦀️.rs"]
        pub mod set_node_param;
    }
}
