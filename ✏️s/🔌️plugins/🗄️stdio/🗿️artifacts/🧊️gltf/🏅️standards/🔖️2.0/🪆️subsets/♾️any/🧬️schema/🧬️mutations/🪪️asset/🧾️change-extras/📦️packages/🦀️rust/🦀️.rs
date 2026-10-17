//! 🧾️ mut-any of `semio-s-artifact-stdio-gltf-mutations` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v2_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🦀️.rs"]
                        pub mod change_asset_extra_data;
                        #[path = "../../../🔖️version/🦀️.rs"]
                        pub mod change_asset_version;
                        #[path = "../../../../📃️document/🧩️change-extensions/🦀️.rs"]
                        pub mod change_document_extension_data;
                        #[path = "../../../../📃️document/📝️change-extras/🦀️.rs"]
                        pub mod change_document_extra_data;
                        #[path = "../../../../💎️material/🌫️change-alpha/🦀️.rs"]
                        pub mod change_material_alpha_mode;
                        #[path = "../../../../💎️material/🪞️change-sides/🦀️.rs"]
                        pub mod change_material_double_sided;
                        #[path = "../../../../🕸️mesh/🧩️change-extensions/🦀️.rs"]
                        pub mod change_mesh_extension_data;
                        #[path = "../../../../🕸️mesh/📝️change-extras/🦀️.rs"]
                        pub mod change_mesh_extra_data;
                        #[path = "../../../../🕸️mesh/⚖️change-weights/🦀️.rs"]
                        pub mod change_mesh_morph_weights;
                        #[path = "../../../../🕸️mesh/🏷️rename/🦀️.rs"]
                        pub mod change_mesh_name;
                        #[path = "../../../../🌳️node/🧩️change-extensions/🦀️.rs"]
                        pub mod change_node_extension_data;
                        #[path = "../../../../🌳️node/📝️change-extras/🦀️.rs"]
                        pub mod change_node_extra_data;
                        #[path = "../../../../🌳️node/⚖️change-weights/🦀️.rs"]
                        pub mod change_node_morph_weights;
                        #[path = "../../../../🌳️node/🏷️rename/🦀️.rs"]
                        pub mod change_node_name;
                        #[path = "../../../../🌳️node/📐️transform/🦀️.rs"]
                        pub mod change_node_transform;
                        #[path = "../../../../🔺️primitive/🧩️change-extensions/🦀️.rs"]
                        pub mod change_primitive_extension_data;
                        #[path = "../../../../🔺️primitive/📝️change-extras/🦀️.rs"]
                        pub mod change_primitive_extra_data;
                        #[path = "../../../../🔺️primitive/📐️change-topology/🦀️.rs"]
                        pub mod change_primitive_topology_mode;
                        #[path = "../../../../🎬️scene/🧩️change-extensions/🦀️.rs"]
                        pub mod change_scene_extension_data;
                        #[path = "../../../../🎬️scene/📝️change-extras/🦀️.rs"]
                        pub mod change_scene_extra_data;
                        #[path = "../../../../🎬️scene/🏷️rename/🦀️.rs"]
                        pub mod change_scene_name;
                        #[path = "../../../../🎞️animation/🌱️create/🦀️.rs"]
                        pub mod create_animation;
                        #[path = "../../../../💿️buffer/🌱️create/🦀️.rs"]
                        pub mod create_buffer;
                        #[path = "../../../../🪟️buffer-view/🌱️create/🦀️.rs"]
                        pub mod create_buffer_view;
                        #[path = "../../../../🎞️animation/🗑️delete/🦀️.rs"]
                        pub mod delete_animation;
                        #[path = "../../../../💿️buffer/🗑️delete/🦀️.rs"]
                        pub mod delete_buffer;
                        #[path = "../../../../🪟️buffer-view/🗑️delete/🦀️.rs"]
                        pub mod delete_buffer_view;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::remove_used_extension;
                    }
                    #[path = "."]
                    pub mod modules {
                        #[path = "."]
                        pub mod mutation_support {
                            pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::modules::mutation_support::material_animation;
                            pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::modules::mutation_support::structure_geometry;
                            pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level;
                            pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections;
                        }
                    }
                }
            }
        }
    }
}
