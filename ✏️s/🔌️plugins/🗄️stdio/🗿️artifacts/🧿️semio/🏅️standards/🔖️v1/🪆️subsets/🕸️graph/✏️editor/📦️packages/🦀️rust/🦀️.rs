//! ✏️ editor of `semio-s-artifact-stdio-semio-graph` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
        pub mod mutations;
    }
    #[path = "."]
    pub mod text {
        pub use semio_s_artifact_stdio_semio_graph_snapshot::io::text::snapshot;
        #[path = "../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
        pub mod mutations;
    }
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_graph_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_graph_snapshot::schema::diff;
    #[path = "."]
    pub mod mutations {
        #[path = "../../../🧬️schema/🧬️mutations/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod create_node {
            #[path = "../../../🧬️schema/🧬️mutations/🏗️create-node/🦀️.rs"]
            mod component;
            #[path = "../../../🧬️schema/🧬️mutations/🏗️create-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🧬️schema/🧬️mutations/🏗️create-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_node {
            #[path = "../../../🧬️schema/🧬️mutations/🗑️delete-node/🦀️.rs"]
            mod component;
            #[path = "../../../🧬️schema/🧬️mutations/🗑️delete-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🧬️schema/🧬️mutations/🗑️delete-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        pub use semio_s_artifact_stdio_semio_graph_inferences::schema::mutations::change_node_kind;
        pub use semio_s_artifact_stdio_semio_graph_inferences::schema::mutations::change_node_label;
        pub use semio_s_artifact_stdio_semio_graph_inferences::schema::mutations::move_node;
        pub use semio_s_artifact_stdio_semio_graph_inferences::schema::mutations::add_node_port;
        pub use semio_s_artifact_stdio_semio_graph_inferences::schema::mutations::remove_node_port;
        pub use semio_s_artifact_stdio_semio_graph_examples::schema::mutations::add_node_property;
        pub use semio_s_artifact_stdio_semio_graph_examples::schema::mutations::remove_node_property;
        pub use semio_s_artifact_stdio_semio_graph_mut_family::schema::mutations::create_edge;
        pub use semio_s_artifact_stdio_semio_graph_mut_family::schema::mutations::delete_edge;
        #[path = "."]
        pub mod drag_nodes {
            #[path = "../../../🧬️schema/🧬️mutations/✋️drag-nodes/🦀️.rs"]
            mod component;
            #[path = "../../../🧬️schema/🧬️mutations/✋️drag-nodes/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🧬️schema/🧬️mutations/✋️drag-nodes/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        pub use semio_s_artifact_stdio_semio_graph_mut_family::schema::mutations::set_node_property;
        pub use semio_s_artifact_stdio_semio_graph_mut_family::schema::mutations::resize_node;
        pub use semio_s_artifact_stdio_semio_graph_mut_family::schema::mutations::rename_node;
        pub use semio_s_artifact_stdio_semio_graph_mut_family::schema::mutations::set_edge_property;
        pub use semio_s_artifact_stdio_semio_graph_mut_family::schema::mutations::add_edge_property;
        pub use semio_s_artifact_stdio_semio_graph_mut_family::schema::mutations::remove_edge_property;
    }
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
            #[path = "../../🎭️modes/✏️edit/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                pub use semio_s_artifact_stdio_semio_graph_mut_family::editor::modes::edit::windows::*;
                #[path = "."]
                pub mod main {
                    pub use semio_s_artifact_stdio_semio_graph_mut_family::editor::modes::edit::windows::main::*;
                    #[cfg(test)]
                    #[path = "../../🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                }
            }
        }
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
            #[path = "../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                pub use semio_s_artifact_stdio_semio_graph_mut_family::viewer::modes::view::windows::*;
                #[path = "."]
                pub mod main {
                    pub use semio_s_artifact_stdio_semio_graph_mut_family::viewer::modes::view::windows::main::*;
                    #[cfg(test)]
                    #[path = "../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                }
            }
        }
    }
}
