//! 🔬️ mut-fixture-tests of `semio-s-artifact-stdio-semio-graph` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;

#[path = "."]
pub mod io {
    #[path = "."]
    pub mod text {
        pub use semio_s_artifact_stdio_semio_graph_snapshot::io::text::snapshot;
    }
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_graph_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_graph_snapshot::schema::diff;
    #[path = "."]
    pub mod mutations {
        pub use semio_s_artifact_stdio_semio_graph_editor::schema::mutations::*;
        pub use semio_s_artifact_stdio_semio_graph_editor::schema::mutations::drag_nodes;
        #[cfg(test)]
        #[path = "../../🦀️.rs"]
        pub mod fixture_tests;
        pub use semio_s_artifact_stdio_semio_graph_mut_family::schema::mutations::set_node_property;
        pub use semio_s_artifact_stdio_semio_graph_mut_family::schema::mutations::resize_node;
        pub use semio_s_artifact_stdio_semio_graph_mut_family::schema::mutations::rename_node;
        pub use semio_s_artifact_stdio_semio_graph_mut_family::schema::mutations::set_edge_property;
        pub use semio_s_artifact_stdio_semio_graph_mut_family::schema::mutations::add_edge_property;
        pub use semio_s_artifact_stdio_semio_graph_mut_family::schema::mutations::remove_edge_property;
    }
}
