//! ✏️ editor of `semio-s-artifact-stdio-semio-kit` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_kit_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_kit_examples::schema::diff;
    #[path = "."]
    pub mod mutations {
        pub use semio_s_artifact_stdio_semio_kit_mutations::schema::mutations::*;
        pub use semio_s_artifact_stdio_semio_kit_mut_family::schema::mutations::unbind_representation;
        #[cfg(test)]
        #[path = "../../../🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs"]
        pub mod fixture_tests;
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
            pub use semio_s_artifact_stdio_semio_kit_mutations::editor::modes::edit::*;
            pub use semio_s_artifact_stdio_semio_kit_examples::editor::modes::edit::windows;
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
            pub use semio_s_artifact_stdio_semio_kit_mutations::viewer::modes::view::*;
            pub use semio_s_artifact_stdio_semio_kit_examples::viewer::modes::view::windows;
        }
    }
}
