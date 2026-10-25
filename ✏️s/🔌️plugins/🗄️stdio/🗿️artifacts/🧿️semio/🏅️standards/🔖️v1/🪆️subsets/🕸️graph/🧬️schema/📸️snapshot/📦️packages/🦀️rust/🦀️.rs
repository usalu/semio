//! 📸️ snapshot of `semio-s-artifact-stdio-semio-graph` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
        #[path = "../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
        pub mod diff;
    }
    #[path = "."]
    pub mod text {
        #[path = "../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
        #[path = "../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
        pub mod diff;
    }
    #[path = "."]
    pub mod sqlite {
        #[path = "../../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
    }
}
#[path = "."]
pub mod schema {
    #[path = "."]
    pub mod snapshot {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod diff {
        #[path = "../../../🔺️diff/🦀️.rs"]
        mod component;
        pub use component::*;
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
