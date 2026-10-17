//! 🪆️ The `graph` subset of the semio artifact: its schema, io codecs, editor and viewer, and the retirement and member-owner
//! implementations of its own types.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;
extern crate semio_framework_graph_engine as graph_core;
extern crate semio_framework_graph_manifest as graph_manifest;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_graph_schema::schema::*;

    pub use semio_s_artifact_stdio_semio_graph_examples::schema::operators_internals;
    pub use semio_s_artifact_stdio_semio_graph_examples::schema::traversal_internals;
    #[path = "."]
    pub mod normal_internals {
        pub use semio_s_artifact_stdio_semio_graph_examples::schema::normal_internals::directed;
        pub use semio_s_artifact_stdio_semio_graph_examples::schema::normal_internals::undirected;
    }
    #[path = "."]
    pub mod ports_internals {
        pub use semio_s_artifact_stdio_semio_graph_examples::schema::ports_internals::undirected;
        pub use semio_s_artifact_stdio_semio_graph_examples::schema::ports_internals::directed;
    }
    pub use semio_s_artifact_stdio_semio_graph_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_graph_snapshot::schema::diff;
    pub use semio_s_artifact_stdio_semio_graph_inferences::schema::inferences;
    #[path = "."]
    pub mod mutations {
        pub use semio_s_artifact_stdio_semio_graph_editor::schema::mutations::*;

        pub use semio_s_artifact_stdio_semio_graph_editor::schema::mutations::create_node;
        pub use semio_s_artifact_stdio_semio_graph_editor::schema::mutations::delete_node;
        pub use semio_s_artifact_stdio_semio_graph_inferences::schema::mutations::change_node_kind;
        pub use semio_s_artifact_stdio_semio_graph_inferences::schema::mutations::change_node_label;
        pub use semio_s_artifact_stdio_semio_graph_inferences::schema::mutations::move_node;
        pub use semio_s_artifact_stdio_semio_graph_inferences::schema::mutations::add_node_port;
        pub use semio_s_artifact_stdio_semio_graph_inferences::schema::mutations::remove_node_port;
        pub use semio_s_artifact_stdio_semio_graph_examples::schema::mutations::add_node_property;
        pub use semio_s_artifact_stdio_semio_graph_examples::schema::mutations::remove_node_property;
        pub use semio_s_artifact_stdio_semio_graph_mut_family::schema::mutations::create_edge;
        pub use semio_s_artifact_stdio_semio_graph_mut_family::schema::mutations::delete_edge;
        pub use semio_s_artifact_stdio_semio_graph_editor::schema::mutations::drag_nodes;
        pub use semio_s_artifact_stdio_semio_graph_mut_family::schema::mutations::set_node_property;
        pub use semio_s_artifact_stdio_semio_graph_mut_family::schema::mutations::resize_node;
        pub use semio_s_artifact_stdio_semio_graph_mut_family::schema::mutations::rename_node;
        pub use semio_s_artifact_stdio_semio_graph_mut_family::schema::mutations::set_edge_property;
        pub use semio_s_artifact_stdio_semio_graph_mut_family::schema::mutations::add_edge_property;
        pub use semio_s_artifact_stdio_semio_graph_mut_family::schema::mutations::remove_edge_property;
    }
}
pub use semio_s_artifact_stdio_semio_graph_examples::examples;
#[path = "."]
pub mod editor {
    pub use semio_s_artifact_stdio_semio_graph_editor::editor::*;

    pub use semio_s_artifact_stdio_semio_graph_examples::editor::presence;
    pub use semio_s_artifact_stdio_semio_graph_examples::editor::config;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            pub use semio_s_artifact_stdio_semio_graph_editor::editor::modes::edit::*;

            pub use semio_s_artifact_stdio_semio_graph_mut_family::editor::modes::edit::windows;
        }
    }
}
#[path = "."]
pub mod viewer {
    pub use semio_s_artifact_stdio_semio_graph_editor::viewer::*;

    pub use semio_s_artifact_stdio_semio_graph_examples::viewer::presence;
    pub use semio_s_artifact_stdio_semio_graph_examples::viewer::config;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            pub use semio_s_artifact_stdio_semio_graph_editor::viewer::modes::view::*;

            pub use semio_s_artifact_stdio_semio_graph_mut_family::viewer::modes::view::windows;
        }
    }
}

semio_framework_value::artifact_retire_leaf!(schema::snapshot::SemioGraphPortKind);
semio_framework_value::artifact_retire_struct!(schema::snapshot::GraphNodeId { value });
semio_framework_value::artifact_retire_struct!(schema::snapshot::GraphEdgeId { value });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioGraphPort { name, kind, category, properties });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioGraphNode { id, kind, label, position, width, height, ports, properties });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioGraphEdge { id, source, target, kind, label, source_port, target_port, properties });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioGraphSnapshot { schema, nodes, edges });

semio_s_artifact_stdio_semio_base::semio_snapshot_retirement!(schema::snapshot::SemioGraphSnapshot);
semio_s_artifact_stdio_semio_base::semio_member_owner!(schema::snapshot::SemioGraphSnapshot, schema::mutations::SemioGraphMutation);

#[cfg(test)]
#[path = "🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_0;
#[cfg(test)]
#[path = "🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_1;
