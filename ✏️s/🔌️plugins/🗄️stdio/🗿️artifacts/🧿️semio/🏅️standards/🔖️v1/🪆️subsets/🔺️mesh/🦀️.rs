//! 🪆️ The `mesh` subset of the semio artifact: its schema, io codecs, editor and viewer, and the retirement and member-owner
//! implementations of its own types.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub mod io {
    pub use semio_s_artifact_stdio_semio_mesh_io::io::*;
    pub mod binary {
        pub use semio_s_artifact_stdio_semio_mesh_io::io::binary::*;
        pub use semio_s_artifact_stdio_semio_mesh_examples::io::binary::inferences;
    }
    pub mod sqlite {
        pub use semio_s_artifact_stdio_semio_mesh_examples::io::sqlite::*;
        pub use semio_s_artifact_stdio_semio_mesh_snapshot::io::sqlite::snapshot;
    }
    pub mod text {
        pub use semio_s_artifact_stdio_semio_mesh_io::io::text::*;
        pub use semio_s_artifact_stdio_semio_mesh_examples::io::text::inferences;
    }
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_mesh_schema::schema::*;

    pub use semio_s_artifact_stdio_semio_mesh_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_mesh_examples::schema::diff;
    pub use semio_s_artifact_stdio_semio_mesh_examples::schema::inferences;
    #[path = "."]
    pub mod mutations {
        pub use semio_s_artifact_stdio_semio_mesh_mutations::schema::mutations::*;

        pub use semio_s_artifact_stdio_semio_mesh_mut_family::schema::mutations::create_mesh;
        pub use semio_s_artifact_stdio_semio_mesh_mut_family::schema::mutations::delete_mesh;
        pub use semio_s_artifact_stdio_semio_mesh_mut_family::schema::mutations::create_primitive;
        pub use semio_s_artifact_stdio_semio_mesh_mut_family::schema::mutations::delete_primitive;
        pub use semio_s_artifact_stdio_semio_mesh_mut_family::schema::mutations::set_primitive_topology;
        pub use semio_s_artifact_stdio_semio_mesh_mut_family::schema::mutations::replace_primitive_geometry;
        pub use semio_s_artifact_stdio_semio_mesh_mut_family::schema::mutations::set_primitive_material;
        pub use semio_s_artifact_stdio_semio_mesh_mut_family::schema::mutations::create_material;
        pub use semio_s_artifact_stdio_semio_mesh_mut_family::schema::mutations::delete_material;
        pub use semio_s_artifact_stdio_semio_mesh_schema::schema::mutations::change_material_base_color;
        pub use semio_s_artifact_stdio_semio_mesh_mutations::schema::mutations::change_material_metallic;
        pub use semio_s_artifact_stdio_semio_mesh_mutations::schema::mutations::change_material_roughness;
        pub use semio_s_artifact_stdio_semio_mesh_mutations::schema::mutations::create_texture;
        pub use semio_s_artifact_stdio_semio_mesh_mutations::schema::mutations::delete_texture;
        pub use semio_s_artifact_stdio_semio_mesh_mutations::schema::mutations::change_texture_mime;
        pub use semio_s_artifact_stdio_semio_mesh_mutations::schema::mutations::replace_texture_bytes;
        pub use semio_s_artifact_stdio_semio_mesh_mutations::schema::mutations::move_vertex;
    }
}
pub use semio_s_artifact_stdio_semio_mesh_examples::examples;
#[path = "."]
pub mod editor {
    pub use semio_s_artifact_stdio_semio_mesh_editor::editor::*;

    pub use semio_s_artifact_stdio_semio_mesh_examples::editor::presence;
    pub use semio_s_artifact_stdio_semio_mesh_examples::editor::config;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            pub use semio_s_artifact_stdio_semio_mesh_editor::editor::modes::edit::*;

            pub use semio_s_artifact_stdio_semio_mesh_editor::editor::modes::edit::windows;
        }
    }
}
#[path = "."]
pub mod viewer {
    pub use semio_s_artifact_stdio_semio_mesh_io::viewer::*;

    pub use semio_s_artifact_stdio_semio_mesh_examples::viewer::presence;
    pub use semio_s_artifact_stdio_semio_mesh_examples::viewer::config;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            pub use semio_s_artifact_stdio_semio_mesh_schema::viewer::modes::view::*;

            pub use semio_s_artifact_stdio_semio_mesh_schema::viewer::modes::view::windows;
        }
    }
}
