//! ✏️ editor of `semio-s-artifact-stdio-semio-mesh` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_mesh_snapshot::schema::snapshot;
    #[path = "."]
    pub mod mutations {
        pub use semio_s_artifact_stdio_semio_mesh_mutations::schema::mutations::*;
        pub use semio_s_artifact_stdio_semio_mesh_mutations::schema::mutations::move_vertex;
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
                #[path = "."]
                pub mod main {
                    #[path = "../../🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
    pub use semio_s_artifact_stdio_semio_mesh_io::editor::edit_rules;
}
#[path = "."]
pub mod viewer {
    pub use semio_s_artifact_stdio_semio_mesh_io::viewer::*;
}
