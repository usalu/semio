//! 🪆️ The `kit` subset of the semio artifact: its schema, io codecs, editor and viewer, and the retirement and member-owner
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
    pub use semio_s_artifact_stdio_semio_kit_schema::schema::*;

    pub use semio_s_artifact_stdio_semio_kit_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_kit_examples::schema::diff;
    pub use semio_s_artifact_stdio_semio_kit_examples::schema::inferences;
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
        pub use semio_s_artifact_stdio_semio_kit_mutations::schema::mutations::add_type;
        pub use semio_s_artifact_stdio_semio_kit_mutations::schema::mutations::remove_type;
        pub use semio_s_artifact_stdio_semio_kit_mutations::schema::mutations::rename_type;
        pub use semio_s_artifact_stdio_semio_kit_mutations::schema::mutations::add_design;
        pub use semio_s_artifact_stdio_semio_kit_mutations::schema::mutations::remove_design;
        pub use semio_s_artifact_stdio_semio_kit_mutations::schema::mutations::edit_design;
    }
}
pub use semio_s_artifact_stdio_semio_kit_examples::examples;
#[path = "."]
pub mod editor {
    pub use semio_s_artifact_stdio_semio_kit_editor::editor::*;

    pub use semio_s_artifact_stdio_semio_kit_examples::editor::presence;
    pub use semio_s_artifact_stdio_semio_kit_examples::editor::config;
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
    pub use semio_s_artifact_stdio_semio_kit_editor::viewer::*;

    pub use semio_s_artifact_stdio_semio_kit_examples::viewer::presence;
    pub use semio_s_artifact_stdio_semio_kit_examples::viewer::config;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            pub use semio_s_artifact_stdio_semio_kit_mutations::viewer::modes::view::*;

            pub use semio_s_artifact_stdio_semio_kit_examples::viewer::modes::view::windows;
        }
    }
}

semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioKitType { id, name, category });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioKitPiece { id, type_id, transform });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioKitConnection { id, connecting_piece_id, connecting_port, connected_piece_id, connected_port });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioKitDesign { id, name, pieces, connections });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioKitSnapshot { schema, types, designs, objects, models, properties, representations });

semio_s_artifact_stdio_semio_base::semio_snapshot_retirement!(schema::snapshot::SemioKitSnapshot);
semio_s_artifact_stdio_semio_base::semio_member_owner!(schema::snapshot::SemioKitSnapshot, schema::mutations::SemioKitMutation);

#[cfg(test)]
#[path = "🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_0;
#[cfg(test)]
#[path = "🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_1;
