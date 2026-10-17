//! 🪆️ The `flow` subset of the semio artifact: its schema, io codecs, editor and viewer, and the retirement and member-owner
//! implementations of its own types.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub mod io {
    pub use semio_s_artifact_stdio_semio_flow_io::io::*;
    pub mod binary {
        pub use semio_s_artifact_stdio_semio_flow_io::io::binary::*;
        pub use semio_s_artifact_stdio_semio_flow_examples::io::binary::inferences;
    }
    pub mod sqlite {
        pub use semio_s_artifact_stdio_semio_flow_examples::io::sqlite::*;
        pub use semio_s_artifact_stdio_semio_flow_snapshot::io::sqlite::snapshot;
    }
    pub mod text {
        pub use semio_s_artifact_stdio_semio_flow_io::io::text::*;
        pub use semio_s_artifact_stdio_semio_flow_examples::io::text::inferences;
    }
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_flow_schema::schema::*;

    pub use semio_s_artifact_stdio_semio_flow_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_flow_examples::schema::diff;
    pub use semio_s_artifact_stdio_semio_flow_schema::schema::inferences;
    pub use semio_s_artifact_stdio_semio_flow_mutations::schema::mutations;
}
pub use semio_s_artifact_stdio_semio_flow_examples::examples;
#[path = "."]
pub mod editor {
    pub use semio_s_artifact_stdio_semio_flow_io::editor::*;

    pub use semio_s_artifact_stdio_semio_flow_examples::editor::presence;
    pub use semio_s_artifact_stdio_semio_flow_examples::editor::config;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            pub use semio_s_artifact_stdio_semio_flow_examples::editor::modes::edit::*;

            pub use semio_s_artifact_stdio_semio_flow_examples::editor::modes::edit::windows;
        }
    }
}
#[path = "."]
pub mod viewer {
    pub use semio_s_artifact_stdio_semio_flow_io::viewer::*;

    pub use semio_s_artifact_stdio_semio_flow_examples::viewer::presence;
    pub use semio_s_artifact_stdio_semio_flow_examples::viewer::config;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            pub use semio_s_artifact_stdio_semio_flow_set_node_position::viewer::modes::view::*;

            pub use semio_s_artifact_stdio_semio_flow_examples::viewer::modes::view::windows;
        }
    }
}

semio_framework_value::artifact_retire_struct!(schema::snapshot::PortRef { node, port });
semio_framework_value::artifact_retire_struct!(schema::snapshot::FlowParam { key, value });
semio_framework_value::artifact_retire_struct!(schema::snapshot::FlowNode { id, kind, label, params, position });
semio_framework_value::artifact_retire_struct!(schema::snapshot::FlowEdge { id, from, to, kind });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioFlowSnapshot { schema, nodes, edges });

semio_s_artifact_stdio_semio_base::semio_snapshot_retirement!(schema::snapshot::SemioFlowSnapshot);
semio_s_artifact_stdio_semio_base::semio_member_owner!(schema::snapshot::SemioFlowSnapshot, schema::mutations::SemioFlowMutation, open = io::binary::snapshot::SemioFlowSnapshotDecode);
