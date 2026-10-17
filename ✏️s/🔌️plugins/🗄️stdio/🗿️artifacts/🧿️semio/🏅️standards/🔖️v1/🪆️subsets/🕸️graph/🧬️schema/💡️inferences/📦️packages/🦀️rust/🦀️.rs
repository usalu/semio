//! 💡️ inferences of `semio-s-artifact-stdio-semio-graph` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;
extern crate semio_framework_graph_engine as graph_core;
extern crate semio_framework_graph_manifest as graph_manifest;

#[path = "."]
pub mod io {
    #[path = "."]
    pub mod text {
        pub use semio_s_artifact_stdio_semio_graph_snapshot::io::text::snapshot;
    }
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_graph_examples::schema::traversal_internals;
    #[path = "."]
    pub mod normal_internals {
        pub use semio_s_artifact_stdio_semio_graph_examples::schema::normal_internals::undirected;
    }
    #[path = "."]
    pub mod ports_internals {
        #[path = "."]
        pub mod directed {
            pub use semio_s_artifact_stdio_semio_graph_examples::schema::ports_internals::directed::*;
            #[path = "."]
            pub mod normal {
                pub use semio_s_artifact_stdio_semio_graph_examples::schema::ports_internals::directed::normal::*;
                #[cfg(test)]
                #[path = "../../../🔌️ports-internals/➡️directed/➕️normal/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
    }
    #[path = "."]
    pub mod snapshot {
        pub use semio_s_artifact_stdio_semio_graph_snapshot::schema::snapshot::*;
        #[cfg(test)]
        #[path = "../../../📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
    #[path = "."]
    pub mod diff {
        pub use semio_s_artifact_stdio_semio_graph_snapshot::schema::diff::*;
        #[cfg(test)]
        #[path = "../../../🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
    #[path = "."]
    pub mod inferences {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod topology {
            #[path = "../../🧭topology/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod connectivity {
            #[path = "../../🕸️connectivity/🦀️.rs"]
            mod component;
            pub use component::*;
        }
    }
    #[path = "."]
    pub mod mutations {
        #[path = "."]
        pub mod change_node_kind {
            #[path = "../../../🧬️mutations/🔧change-node-kind/🦀️.rs"]
            mod component;
            #[path = "../../../🧬️mutations/🔧change-node-kind/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🧬️mutations/🔧change-node-kind/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod change_node_label {
            #[path = "../../../🧬️mutations/🖍️change-node-label/🦀️.rs"]
            mod component;
            #[path = "../../../🧬️mutations/🖍️change-node-label/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🧬️mutations/🖍️change-node-label/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod move_node {
            #[path = "../../../🧬️mutations/📍move-node/🦀️.rs"]
            mod component;
            #[path = "../../../🧬️mutations/📍move-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🧬️mutations/📍move-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod add_node_port {
            #[path = "../../../🧬️mutations/🔌add-node-port/🦀️.rs"]
            mod component;
            #[path = "../../../🧬️mutations/🔌add-node-port/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🧬️mutations/🔌add-node-port/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod remove_node_port {
            #[path = "../../../🧬️mutations/🔚remove-node-port/🦀️.rs"]
            mod component;
            #[path = "../../../🧬️mutations/🔚remove-node-port/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🧬️mutations/🔚remove-node-port/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
    }
}
