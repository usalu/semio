//! 🪆️ The `mesh` subset of the semio artifact: its schema, io codecs, editor and viewer, and the retirement and member-owner
//! implementations of its own types.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "🚪️io/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod schema {
    #[path = "🧬️schema/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod snapshot {
        #[path = "🧬️schema/📸️snapshot/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod diff {
        #[path = "🧬️schema/🔺️diff/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod inferences {
        #[path = "🧬️schema/💡️inferences/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod aabb {
            #[path = "🧬️schema/💡️inferences/📦aabb/🦀️.rs"]
            mod component;
            pub use component::*;
        }
    }
    #[path = "."]
    pub mod mutations {
        #[path = "🧬️schema/🧬️mutations/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod create_mesh {
            #[path = "🧬️schema/🧬️mutations/🕸️create-mesh/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🕸️create-mesh/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🕸️create-mesh/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_mesh {
            #[path = "🧬️schema/🧬️mutations/🗑️delete-mesh/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🗑️delete-mesh/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🗑️delete-mesh/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_primitive {
            #[path = "🧬️schema/🧬️mutations/🔺create-primitive/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🔺create-primitive/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🔺create-primitive/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_primitive {
            #[path = "🧬️schema/🧬️mutations/✂️delete-primitive/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/✂️delete-primitive/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/✂️delete-primitive/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod set_primitive_topology {
            #[path = "🧬️schema/🧬️mutations/🔀set-primitive-topology/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🔀set-primitive-topology/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🔀set-primitive-topology/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod replace_primitive_geometry {
            #[path = "🧬️schema/🧬️mutations/📐replace-primitive-geometry/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/📐replace-primitive-geometry/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/📐replace-primitive-geometry/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod set_primitive_material {
            #[path = "🧬️schema/🧬️mutations/🧲️set-primitive-material/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🧲️set-primitive-material/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🧲️set-primitive-material/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_material {
            #[path = "🧬️schema/🧬️mutations/🎨create-material/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🎨create-material/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🎨create-material/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_material {
            #[path = "🧬️schema/🧬️mutations/🚮delete-material/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🚮delete-material/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🚮delete-material/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod change_material_base_color {
            #[path = "🧬️schema/🧬️mutations/🌈change-material-base/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🌈change-material-base/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🌈change-material-base/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod change_material_metallic {
            #[path = "🧬️schema/🧬️mutations/⚙️change-material-metallic/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/⚙️change-material-metallic/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/⚙️change-material-metallic/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod change_material_roughness {
            #[path = "🧬️schema/🧬️mutations/🧱change-material-roughness/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🧱change-material-roughness/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🧱change-material-roughness/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_texture {
            #[path = "🧬️schema/🧬️mutations/🖼️create-texture/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🖼️create-texture/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🖼️create-texture/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_texture {
            #[path = "🧬️schema/🧬️mutations/🕳️delete-texture/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🕳️delete-texture/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🕳️delete-texture/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod change_texture_mime {
            #[path = "🧬️schema/🧬️mutations/🏷️change-texture-mime/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🏷️change-texture-mime/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🏷️change-texture-mime/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod replace_texture_bytes {
            #[path = "🧬️schema/🧬️mutations/📀replace-texture-bytes/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/📀replace-texture-bytes/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/📀replace-texture-bytes/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod move_vertex {
            #[path = "🧬️schema/🧬️mutations/📍move-vertex/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/📍move-vertex/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/📍move-vertex/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod cube {
        #[path = "📚️examples/🧊️cube/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "✏️editor/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod presence {
        #[path = "✏️editor/👥️presence/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod config {
        #[path = "✏️editor/🎚️config/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            #[path = "✏️editor/🎭️modes/✏️edit/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                #[path = "."]
                pub mod main {
                    #[path = "✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "👁️viewer/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod presence {
        #[path = "👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod config {
        #[path = "👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            #[path = "👁️viewer/🎭️modes/👁️view/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                #[path = "."]
                pub mod main {
                    #[path = "👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}

semio_framework_value::artifact_retire_leaf!(schema::snapshot::SemioTopology);
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioPrimitive { id, topology, positions, normals, uvs, colors, indices, material_id });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioMesh { id, primitives });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioMaterial { id, base_color, metallic, roughness, base_color_texture, metallic_roughness_texture, normal_texture, occlusion_texture, emissive_texture });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioTexture { id, mime, bytes });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioMeshSnapshot { schema, meshes, materials, textures });

semio_s_artifact_stdio_semio_base::semio_snapshot_retirement!(schema::snapshot::SemioMeshSnapshot);
semio_s_artifact_stdio_semio_base::semio_member_owner!(schema::snapshot::SemioMeshSnapshot, schema::mutations::SemioMeshMutation);
