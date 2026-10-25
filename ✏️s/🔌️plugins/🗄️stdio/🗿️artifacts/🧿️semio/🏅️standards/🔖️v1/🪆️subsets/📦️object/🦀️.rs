//! 🪆️ The `object` subset of the semio artifact: its schema, io codecs, editor and viewer, and the retirement and member-owner
//! implementations of its own types.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_object_schema::schema::*;

    pub use semio_s_artifact_stdio_semio_object_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_object_examples::schema::diff;
    pub use semio_s_artifact_stdio_semio_object_examples::schema::inferences;
    #[path = "."]
    pub mod mutations {
        pub use semio_s_artifact_stdio_semio_object_editor::schema::mutations::*;

        pub use semio_s_artifact_stdio_semio_object_mut_family::schema::mutations::move_object;
        pub use semio_s_artifact_stdio_semio_object_mut_family::schema::mutations::rotate_object;
        pub use semio_s_artifact_stdio_semio_object_mut_family::schema::mutations::scale_object;
        pub use semio_s_artifact_stdio_semio_object_mut_family::schema::mutations::create_brep;
        pub use semio_s_artifact_stdio_semio_object_mut_family::schema::mutations::delete_brep;
        pub use semio_s_artifact_stdio_semio_object_mut_family::schema::mutations::create_mesh;
        pub use semio_s_artifact_stdio_semio_object_mut_family::schema::mutations::delete_mesh;
        pub use semio_s_artifact_stdio_semio_object_mut_family::schema::mutations::create_properties;
        pub use semio_s_artifact_stdio_semio_object_mut_family::schema::mutations::delete_properties;
    }
}
pub use semio_s_artifact_stdio_semio_object_examples::examples;
#[path = "."]
pub mod editor {
    pub use semio_s_artifact_stdio_semio_object_editor::editor::*;

    pub use semio_s_artifact_stdio_semio_object_examples::editor::presence;
    pub use semio_s_artifact_stdio_semio_object_examples::editor::config;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            pub use semio_s_artifact_stdio_semio_object_schema::editor::modes::edit::*;

            pub use semio_s_artifact_stdio_semio_object_schema::editor::modes::edit::windows;
        }
    }
}
#[path = "."]
pub mod viewer {
    pub use semio_s_artifact_stdio_semio_object_editor::viewer::*;

    pub use semio_s_artifact_stdio_semio_object_examples::viewer::presence;
    pub use semio_s_artifact_stdio_semio_object_examples::viewer::config;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            pub use semio_s_artifact_stdio_semio_object_examples::viewer::modes::view::*;

            pub use semio_s_artifact_stdio_semio_object_schema::viewer::modes::view::windows;
        }
    }
}



#[cfg(test)]
#[path = "🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_0;
#[cfg(test)]
#[path = "🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_1;
