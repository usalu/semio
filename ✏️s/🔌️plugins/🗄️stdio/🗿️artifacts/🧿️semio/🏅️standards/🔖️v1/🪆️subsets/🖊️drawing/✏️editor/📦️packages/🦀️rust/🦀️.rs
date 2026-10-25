//! ✏️ editor of `semio-s-artifact-stdio-semio-drawing` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
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
        pub use semio_s_artifact_stdio_semio_drawing_snapshot::io::text::snapshot;
        #[path = "../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
        pub mod mutations;
    }
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_drawing_schema::schema::geometry;
    pub use semio_s_artifact_stdio_semio_drawing_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_drawing_inferences::schema::diff;
    #[path = "."]
    pub mod mutations {
        #[path = "../../../🧬️schema/🧬️mutations/🦀️.rs"]
        mod component;
        pub use component::*;
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
    #[path = "../../🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            pub use semio_s_artifact_stdio_semio_drawing_schema::editor::modes::edit::*;
            pub use semio_s_artifact_stdio_semio_drawing_schema::editor::modes::edit::windows;
        }
    }
    pub use semio_s_artifact_stdio_semio_drawing_inferences::editor::edit_rules;
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
            pub use semio_s_artifact_stdio_semio_drawing_mut_family_2::viewer::modes::view::*;
            pub use semio_s_artifact_stdio_semio_drawing_schema::viewer::modes::view::windows;
        }
    }
}
semio_s_artifact_stdio_semio_base::semio_member_owner!(schema::snapshot::SemioDrawingSnapshot, schema::mutations::SemioDrawingMutation);
