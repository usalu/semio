//! 🧬️ schema of `semio-s-artifact-stdio-semio-graph` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;
extern crate semio_framework_graph_engine as graph_core;
extern crate semio_framework_graph_manifest as graph_manifest;

#[path = "."]
pub mod schema {
    #[path = "../../🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod operators_internals {
        pub use semio_s_artifact_stdio_semio_graph_examples::schema::operators_internals::*;
        #[cfg(test)]
        #[path = "../../🔧️operators-internals/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
    #[path = "."]
    pub mod traversal_internals {
        pub use semio_s_artifact_stdio_semio_graph_examples::schema::traversal_internals::*;
        #[cfg(test)]
        #[path = "../../🚶️traversal-internals/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
    #[path = "."]
    pub mod normal_internals {
        #[path = "."]
        pub mod directed {
            pub use semio_s_artifact_stdio_semio_graph_examples::schema::normal_internals::directed::*;
            #[cfg(test)]
            #[path = "../../➕️normal-internals/➡️directed/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[path = "."]
        pub mod undirected {
            pub use semio_s_artifact_stdio_semio_graph_examples::schema::normal_internals::undirected::*;
            #[cfg(test)]
            #[path = "../../➕️normal-internals/↔️undirected/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
    }
    #[path = "."]
    pub mod ports_internals {
        #[path = "."]
        pub mod undirected {
            pub use semio_s_artifact_stdio_semio_graph_examples::schema::ports_internals::undirected::*;
            #[cfg(test)]
            #[path = "../../🔌️ports-internals/↔️undirected/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
    }
    pub use semio_s_artifact_stdio_semio_graph_snapshot::schema::snapshot;
}
#[path = "."]
pub mod io {
    #[path = "../../../🚪️io/💾️binary/🦀️.rs"]
    pub mod binary;
    #[path = "../../../🚪️io/📝️text/🦀️.rs"]
    pub mod text;
}
