//! ✏️ editor of `semio-s-artifact-stdio-semio-object` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
        pub mod mutations;
    }
    #[path = "."]
    pub mod text {
        #[path = "../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
        pub mod mutations;
    }
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_object_schema::schema::*;
    #[cfg(test)]
    #[path = "../../../🧬️schema/🧪️tests/🪪️document-contract/🦀️.rs"]
    pub mod document_contract;
    pub use semio_s_artifact_stdio_semio_object_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_object_examples::schema::diff;
    #[path = "."]
    pub mod mutations {
        #[path = "../../../🧬️schema/🧬️mutations/🦀️.rs"]
        mod component;
        pub use component::*;
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
#[path = "."]
pub mod editor {
    #[path = "../../🦀️.rs"]
    mod component;
    pub use component::*;
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
    #[path = "../../../👁️viewer/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            pub use semio_s_artifact_stdio_semio_object_examples::viewer::modes::view::*;
            #[path = "."]
            pub mod windows {
                pub use semio_s_artifact_stdio_semio_object_schema::viewer::modes::view::windows::*;
                #[path = "."]
                pub mod main {
                    pub use semio_s_artifact_stdio_semio_object_schema::viewer::modes::view::windows::main::*;
                    #[cfg(test)]
                    #[path = "../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                }
            }
        }
    }
}
