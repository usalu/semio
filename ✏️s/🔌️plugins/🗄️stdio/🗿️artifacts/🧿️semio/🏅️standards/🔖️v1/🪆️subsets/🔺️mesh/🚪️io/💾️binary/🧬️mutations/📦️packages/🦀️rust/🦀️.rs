//! 🧬️ mutations of `semio-s-artifact-stdio-semio-mesh` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../../🦀️.rs"]
        pub mod mutations;
    }
    #[path = "."]
    pub mod text {
        pub use semio_s_artifact_stdio_semio_mesh_snapshot::io::text::snapshot;
        #[path = "../../../../📝️text/🧬️mutations/🦀️.rs"]
        pub mod mutations;
    }
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_mesh_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_mesh_examples::schema::diff;
    #[path = "."]
    pub mod mutations {
        #[path = "../../../../../🧬️schema/🧬️mutations/🦀️.rs"]
        mod component;
        pub use component::*;
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
        #[path = "."]
        pub mod change_material_metallic {
            #[path = "../../../../../🧬️schema/🧬️mutations/⚙️change-material-metallic/🦀️.rs"]
            mod component;
            #[path = "../../../../../🧬️schema/🧬️mutations/⚙️change-material-metallic/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../../../🧬️schema/🧬️mutations/⚙️change-material-metallic/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod change_material_roughness {
            #[path = "../../../../../🧬️schema/🧬️mutations/🧱change-material-roughness/🦀️.rs"]
            mod component;
            #[path = "../../../../../🧬️schema/🧬️mutations/🧱change-material-roughness/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../../../🧬️schema/🧬️mutations/🧱change-material-roughness/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_texture {
            #[path = "../../../../../🧬️schema/🧬️mutations/🖼️create-texture/🦀️.rs"]
            mod component;
            #[path = "../../../../../🧬️schema/🧬️mutations/🖼️create-texture/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../../../🧬️schema/🧬️mutations/🖼️create-texture/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_texture {
            #[path = "../../../../../🧬️schema/🧬️mutations/🕳️delete-texture/🦀️.rs"]
            mod component;
            #[path = "../../../../../🧬️schema/🧬️mutations/🕳️delete-texture/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../../../🧬️schema/🧬️mutations/🕳️delete-texture/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod change_texture_mime {
            #[path = "../../../../../🧬️schema/🧬️mutations/🏷️change-texture-mime/🦀️.rs"]
            mod component;
            #[path = "../../../../../🧬️schema/🧬️mutations/🏷️change-texture-mime/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../../../🧬️schema/🧬️mutations/🏷️change-texture-mime/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod replace_texture_bytes {
            #[path = "../../../../../🧬️schema/🧬️mutations/📀replace-texture-bytes/🦀️.rs"]
            mod component;
            #[path = "../../../../../🧬️schema/🧬️mutations/📀replace-texture-bytes/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../../../🧬️schema/🧬️mutations/📀replace-texture-bytes/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod move_vertex {
            #[path = "../../../../../🧬️schema/🧬️mutations/📍move-vertex/🦀️.rs"]
            mod component;
            #[path = "../../../../../🧬️schema/🧬️mutations/📍move-vertex/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../../../🧬️schema/🧬️mutations/📍move-vertex/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
    }
}
semio_s_artifact_stdio_semio_base::semio_member_owner!(schema::snapshot::SemioMeshSnapshot, schema::mutations::SemioMeshMutation);
