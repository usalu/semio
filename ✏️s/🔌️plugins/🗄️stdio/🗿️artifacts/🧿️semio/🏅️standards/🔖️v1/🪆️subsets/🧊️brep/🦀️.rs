//! 🪆️ The `brep` subset of the semio artifact: its schema, io codecs, editor and viewer, and the retirement and member-owner
//! implementations of its own types.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub mod io {
    pub use semio_s_artifact_stdio_semio_brep_io::io::*;
    pub mod binary {
        pub use semio_s_artifact_stdio_semio_brep_io::io::binary::*;
        pub use semio_s_artifact_stdio_semio_brep_examples::io::binary::inferences;
    }
    pub mod sqlite {
        pub use semio_s_artifact_stdio_semio_brep_examples::io::sqlite::*;
        pub use semio_s_artifact_stdio_semio_brep_snapshot::io::sqlite::snapshot;
    }
    pub mod text {
        pub use semio_s_artifact_stdio_semio_brep_io::io::text::*;
        pub use semio_s_artifact_stdio_semio_brep_examples::io::text::inferences;
    }
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_brep_schema::schema::*;

    #[path = "."]
    pub mod snapshot {
        pub use semio_s_artifact_stdio_semio_brep_snapshot::schema::snapshot::*;

        pub use semio_s_artifact_stdio_semio_brep_schema::schema::snapshot::body;
    }
    pub use semio_s_artifact_stdio_semio_brep_examples::schema::diff;
    pub use semio_s_artifact_stdio_semio_brep_examples::schema::inferences;
    #[path = "."]
    pub mod mutations {
        pub use semio_s_artifact_stdio_semio_brep_viewer::schema::mutations::*;

        pub use semio_s_artifact_stdio_semio_brep_mut_family::schema::mutations::delete_edge;
        pub use semio_s_artifact_stdio_semio_brep_mut_family::schema::mutations::replace_curve;
        pub use semio_s_artifact_stdio_semio_brep_viewer::schema::mutations::create_vertex;
        pub use semio_s_artifact_stdio_semio_brep_mut_family::schema::mutations::create_shell;
        pub use semio_s_artifact_stdio_semio_brep_mut_family::schema::mutations::delete_shell;
        pub use semio_s_artifact_stdio_semio_brep_mut_family::schema::mutations::move_vertex;
        pub use semio_s_artifact_stdio_semio_brep_mut_family::schema::mutations::create_edge;
        pub use semio_s_artifact_stdio_semio_brep_mut_family::schema::mutations::create_face;
        pub use semio_s_artifact_stdio_semio_brep_viewer::schema::mutations::delete_solid;
        pub use semio_s_artifact_stdio_semio_brep_viewer::schema::mutations::delete_vertex;
        pub use semio_s_artifact_stdio_semio_brep_viewer::schema::mutations::replace_surface;
        pub use semio_s_artifact_stdio_semio_brep_mut_family::schema::mutations::delete_face;
        pub use semio_s_artifact_stdio_semio_brep_viewer::schema::mutations::create_solid;
    }
}
pub use semio_s_artifact_stdio_semio_brep_examples::examples;
#[path = "."]
pub mod editor {
    pub use semio_s_artifact_stdio_semio_brep_io::editor::*;

    pub use semio_s_artifact_stdio_semio_brep_examples::editor::presence;
    pub use semio_s_artifact_stdio_semio_brep_examples::editor::config;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            pub use semio_s_artifact_stdio_semio_brep_io::editor::modes::edit::*;

            pub use semio_s_artifact_stdio_semio_brep_io::editor::modes::edit::windows;
        }
    }
}
#[path = "."]
pub mod viewer {
    pub use semio_s_artifact_stdio_semio_brep_viewer::viewer::*;

    pub use semio_s_artifact_stdio_semio_brep_examples::viewer::presence;
    pub use semio_s_artifact_stdio_semio_brep_examples::viewer::config;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            pub use semio_s_artifact_stdio_semio_brep_viewer::viewer::modes::view::*;

            pub use semio_s_artifact_stdio_semio_brep_viewer::viewer::modes::view::windows;
        }
    }
}
