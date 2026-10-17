//! 🚪️ io of `semio-s-artifact-stdio-semio-flow` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "../../🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_flow_schema::schema::*;
    pub use semio_s_artifact_stdio_semio_flow_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_flow_examples::schema::diff;
    pub use semio_s_artifact_stdio_semio_flow_schema::schema::inferences;
    #[path = "."]
    pub mod mutations {
        pub use semio_s_artifact_stdio_semio_flow_mutations::schema::mutations::*;
        #[cfg(test)]
        #[path = "../../../🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
}
#[path = "."]
pub mod editor {
    #[path = "../../../✏️editor/🦀️.rs"]
    mod component;
    pub use component::*;
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
    #[path = "../../../👁️viewer/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            pub use semio_s_artifact_stdio_semio_flow_set_node_position::viewer::modes::view::*;
            pub use semio_s_artifact_stdio_semio_flow_examples::viewer::modes::view::windows;
        }
    }
}
