//! 🪆️ The `drawing` subset of the semio artifact: its schema, io codecs, editor and viewer, and the retirement and member-owner
//! implementations of its own types.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub mod io {
    pub use semio_s_artifact_stdio_semio_drawing_io::io::*;
    pub mod sqlite {
        pub use semio_s_artifact_stdio_semio_drawing_inferences::io::sqlite::*;
        pub use semio_s_artifact_stdio_semio_drawing_snapshot::io::sqlite::snapshot;
    }
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_drawing_schema::schema::*;

    pub use semio_s_artifact_stdio_semio_drawing_schema::schema::geometry;
    pub mod snapshot {
    pub use semio_s_artifact_stdio_semio_drawing_snapshot::schema::snapshot::*;
    pub mod native_binding {
        pub use semio_s_artifact_stdio_semio_drawing_inferences::schema::snapshot::native_binding::*;
        pub use semio_s_artifact_stdio_semio_drawing_mut_family::schema::snapshot::native_binding::nodes;
    }
}
    pub use semio_s_artifact_stdio_semio_drawing_inferences::schema::diff;
    pub use semio_s_artifact_stdio_semio_drawing_inferences::schema::inferences;
    #[path = "."]
    pub mod mutations {
        pub use semio_s_artifact_stdio_semio_drawing_editor::schema::mutations::*;

        pub use semio_s_artifact_stdio_semio_drawing_mut_family::schema::mutations::create_node;
        pub use semio_s_artifact_stdio_semio_drawing_mut_family::schema::mutations::delete_node;
        pub use semio_s_artifact_stdio_semio_drawing_mut_family::schema::mutations::create_layer;
        pub use semio_s_artifact_stdio_semio_drawing_inferences::schema::mutations::unflatten_node;
        pub use semio_s_artifact_stdio_semio_drawing_mut_family::schema::mutations::ungroup_node;
        pub use semio_s_artifact_stdio_semio_drawing_mut_family::schema::mutations::move_node;
        pub use semio_s_artifact_stdio_semio_drawing_mut_family::schema::mutations::scale_node;
        pub use semio_s_artifact_stdio_semio_drawing_mut_family_2::schema::mutations::change_stroke_width;
        pub use semio_s_artifact_stdio_semio_drawing_mut_family_2::schema::mutations::reorder_nodes;
        pub use semio_s_artifact_stdio_semio_drawing_mut_family_2::schema::mutations::rotate_node;
        pub use semio_s_artifact_stdio_semio_drawing_mut_family_2::schema::mutations::change_stroke_color;
        pub use semio_s_artifact_stdio_semio_drawing_mut_family_2::schema::mutations::drag_nodes;
        pub use semio_s_artifact_stdio_semio_drawing_mut_family::schema::mutations::delete_layer;
        pub use semio_s_artifact_stdio_semio_drawing_mut_family_2::schema::mutations::replace_path;
        pub use semio_s_artifact_stdio_semio_drawing_mut_family::schema::mutations::group_nodes;
        pub use semio_s_artifact_stdio_semio_drawing_mut_family_2::schema::mutations::replace_fill;
        pub use semio_s_artifact_stdio_semio_drawing_mut_family_2::schema::mutations::flatten_node;
    }
}
#[path = "."]
pub mod editor {
    pub use semio_s_artifact_stdio_semio_drawing_editor::editor::*;

    pub use semio_s_artifact_stdio_semio_drawing_inferences::editor::presence;
    pub use semio_s_artifact_stdio_semio_drawing_inferences::editor::config;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            pub use semio_s_artifact_stdio_semio_drawing_schema::editor::modes::edit::*;

            pub use semio_s_artifact_stdio_semio_drawing_schema::editor::modes::edit::windows;
        }
    }
}
#[path = "."]
pub mod viewer {
    pub use semio_s_artifact_stdio_semio_drawing_editor::viewer::*;

    pub use semio_s_artifact_stdio_semio_drawing_inferences::viewer::presence;
    pub use semio_s_artifact_stdio_semio_drawing_inferences::viewer::config;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            pub use semio_s_artifact_stdio_semio_drawing_mut_family_2::viewer::modes::view::*;

            pub use semio_s_artifact_stdio_semio_drawing_schema::viewer::modes::view::windows;
        }
    }
}

semio_framework_value::artifact_retire_leaf!(schema::snapshot::DrawCanvas);
semio_framework_value::artifact_retire_struct!(schema::snapshot::DrawStyle { name, fill, stroke, stroke_width, opacity });
semio_framework_value::artifact_retire_struct!(schema::snapshot::DrawLayer { id, name, visible, root });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioDrawingSnapshot { schema, canvas, styles, layers });
semio_framework_value::artifact_retire_struct!(schema::diff::NodePath { layer, path });

semio_s_artifact_stdio_semio_base::semio_snapshot_retirement!(schema::snapshot::SemioDrawingSnapshot);
semio_s_artifact_stdio_semio_base::semio_member_owner!(schema::snapshot::SemioDrawingSnapshot, schema::mutations::SemioDrawingMutation);
