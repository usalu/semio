//! 🕸️ examples of `semio-s-artifact-stdio-semio-graph` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;
extern crate semio_framework_graph_engine as graph_core;
extern crate semio_framework_graph_manifest as graph_manifest;

#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
        pub mod inferences;
    }
    #[path = "."]
    pub mod text {
        #[path = "../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
        pub mod inferences;
    }
    #[path = "../../../../🚪️io/🪶️sqlite/🦀️.rs"]
    pub mod sqlite;
}
#[path = "."]
pub mod schema {
    #[path = "../../../../🧬️schema/🔧️operators-internals/🦀️.rs"]
    pub mod operators_internals;
    #[path = "../../../../🧬️schema/🚶️traversal-internals/🦀️.rs"]
    pub mod traversal_internals;
    #[path = "."]
    pub mod normal_internals {
        #[path = "../../../../🧬️schema/➕️normal-internals/➡️directed/🦀️.rs"]
        pub mod directed;
        #[path = "../../../../🧬️schema/➕️normal-internals/↔️undirected/🦀️.rs"]
        pub mod undirected;
    }
    #[path = "."]
    pub mod ports_internals {
        #[path = "../../../../🧬️schema/🔌️ports-internals/↔️undirected/🦀️.rs"]
        pub mod undirected;
        #[path = "."]
        pub mod directed {
            #[path = "../../../../🧬️schema/🔌️ports-internals/➡️directed/➕️normal/🦀️.rs"]
            pub mod normal;
        }
    }
    #[path = "."]
    pub mod snapshot {
        pub use semio_s_artifact_stdio_semio_graph_snapshot::schema::snapshot::*;
        #[path="../../../../🧬️schema/📸️snapshot/🔣️json/🦀️.rs"]
        pub mod declared_json;
    }
    pub use semio_s_artifact_stdio_semio_graph_snapshot::schema::diff;
    #[path = "."]
    pub mod mutations {
        #[path = "."]
        pub mod add_node_property {
            #[path = "../../../../🧬️schema/🧬️mutations/➕add-node-property/🦀️.rs"]
            mod component;
            #[path = "../../../../🧬️schema/🧬️mutations/➕add-node-property/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../../🧬️schema/🧬️mutations/➕add-node-property/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod remove_node_property {
            #[path = "../../../../🧬️schema/🧬️mutations/➖remove-node-property/🦀️.rs"]
            mod component;
            #[path = "../../../../🧬️schema/🧬️mutations/➖remove-node-property/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../../🧬️schema/🧬️mutations/➖remove-node-property/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod wires {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod presence {
        #[path = "../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod config {
        #[path = "../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod presence {
        #[path = "../../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod config {
        #[path = "../../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
}
