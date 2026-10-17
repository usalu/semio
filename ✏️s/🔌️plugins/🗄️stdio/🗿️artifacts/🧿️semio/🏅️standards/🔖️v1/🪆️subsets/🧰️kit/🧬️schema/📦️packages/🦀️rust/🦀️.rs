//! 🧬️ schema of `semio-s-artifact-stdio-semio-kit` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

#[path = "."]
pub mod schema {
    #[path = "../../🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod snapshot {
        pub use semio_s_artifact_stdio_semio_kit_snapshot::schema::snapshot::*;
        pub use semio_s_artifact_stdio_semio_kit_snapshot::schema::snapshot::*;
        #[cfg(test)]
        #[path = "../../📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
    pub use semio_s_artifact_stdio_semio_kit_examples::schema::diff;
    #[path = "."]
    pub mod inferences {
        pub use semio_s_artifact_stdio_semio_kit_examples::schema::inferences::*;
        #[path = "."]
        pub mod entries {
            pub use semio_s_artifact_stdio_semio_kit_examples::schema::inferences::entries::*;
            #[cfg(test)]
            //#region 🧪️Tests
            #[path = "../../💡️inferences/🗃️entries/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
    }
    #[path = "."]
    pub mod mutations {
        pub use semio_s_artifact_stdio_semio_kit_mutations::schema::mutations::*;
        pub use semio_s_artifact_stdio_semio_kit_mut_family::schema::mutations::create_object;
        pub use semio_s_artifact_stdio_semio_kit_mut_family::schema::mutations::delete_object;
        pub use semio_s_artifact_stdio_semio_kit_mut_family::schema::mutations::create_model;
        pub use semio_s_artifact_stdio_semio_kit_mut_family::schema::mutations::delete_model;
        pub use semio_s_artifact_stdio_semio_kit_mut_family::schema::mutations::create_properties;
        pub use semio_s_artifact_stdio_semio_kit_mut_family::schema::mutations::delete_properties;
        pub use semio_s_artifact_stdio_semio_kit_mut_family::schema::mutations::bind_representation;
        pub use semio_s_artifact_stdio_semio_kit_mut_family::schema::mutations::unbind_representation;
        pub use semio_s_artifact_stdio_semio_kit_mut_family::schema::mutations::change_representation_pin;
        #[cfg(test)]
        #[path = "../../🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        pub use semio_s_artifact_stdio_semio_kit_mutations::schema::mutations::add_type;
        pub use semio_s_artifact_stdio_semio_kit_mutations::schema::mutations::remove_type;
        pub use semio_s_artifact_stdio_semio_kit_mutations::schema::mutations::rename_type;
        pub use semio_s_artifact_stdio_semio_kit_mutations::schema::mutations::add_design;
        pub use semio_s_artifact_stdio_semio_kit_mutations::schema::mutations::remove_design;
        pub use semio_s_artifact_stdio_semio_kit_mutations::schema::mutations::edit_design;
    }
}
#[path = "."]
pub mod io {
    #[path = "../../../🚪️io/💾️binary/🦀️.rs"]
    pub mod binary;
    #[path = "../../../🚪️io/📝️text/🦀️.rs"]
    pub mod text;
}
