//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

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
                        pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_4::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests;
                        #[path = "."]
                        pub mod add_required_extension {
                            pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::add_required_extension::*;
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod case_t041;
                        }
                        #[path = "."]
                        pub mod add_used_extension {
                            pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::add_used_extension::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../📣️used/➕️add/🧪️tests/🔬️t056/🦀️.rs"]
                            pub mod case_t056;
                        }
                        #[path = "."]
                        pub mod bind_default_scene {
                            pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::bind_default_scene::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🏠️default-scene/🔗️bind/🧪️tests/🎞️moves/🦀️.rs"]
                            pub mod case_moves;
                        }
                        #[path = "."]
                        pub mod bind_morph_target_attribute {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::bind_morph_target_attribute::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎚️morph/🔗️bind/🧪️tests/🔗️binds/🦀️.rs"]
                            pub mod case_binds;
                        }
                        #[path = "."]
                        pub mod bind_node_camera {
                            pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::bind_node_camera::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../📷️node-camera/🔗️bind/🧪️tests/🔗️binds/🦀️.rs"]
                            pub mod case_binds;
                        }
                        #[path = "."]
                        pub mod bind_node_child {
                            pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::bind_node_child::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌿️node-child/🔗️bind/🧪️tests/🔗️adopts/🦀️.rs"]
                            pub mod case_adopts;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌿️node-child/🔗️bind/📜️contract/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod contract;
                        }
                        #[path = "."]
                        pub mod bind_node_mesh {
                            pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::bind_node_mesh::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🏗️node-mesh/🔗️bind/🧪️tests/🔗️binds/🦀️.rs"]
                            pub mod case_binds;
                        }
                        #[path = "."]
                        pub mod bind_node_skin {
                            pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::bind_node_skin::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🩻️node-skin/🔗️bind/🧪️tests/🔗️binds/🦀️.rs"]
                            pub mod case_binds;
                        }
                        #[path = "."]
                        pub mod bind_primitive_attribute {
                            pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::bind_primitive_attribute::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔤️primitive/🔗️bind/🧪️tests/📍️binds/🦀️.rs"]
                            pub mod case_binds_texcoord_0_fc0e9a;
                        }
                        #[path = "."]
                        pub mod bind_primitive_indices {
                            pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::bind_primitive_indices::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔢️primitive/🔗️bind/🧪️tests/🔢️binds/🦀️.rs"]
                            pub mod case_binds;
                        }
                        #[path = "."]
                        pub mod bind_primitive_material {
                            pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::bind_primitive_material::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🧱️primitive/🔗️bind/🧪️tests/🔗️binds/🦀️.rs"]
                            pub mod case_binds;
                        }
                        #[path = "."]
                        pub mod bind_scene_root_node {
                            pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::bind_scene_root_node::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌲️scene-root/🔗️bind/🧪️tests/📥️promotes/🦀️.rs"]
                            pub mod case_promotes;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌲️scene-root/🔗️bind/📜️contract/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod contract;
                        }
                        #[path = "."]
                        pub mod change_asset_descriptive_metadata {
                            pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::change_asset_descriptive_metadata::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🪪️asset/📝️change-description/🧪️tests/🔬️t068/🦀️.rs"]
                            pub mod case_t068;
                        }
                        #[path = "."]
                        pub mod change_asset_extension_data {
                            pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::change_asset_extension_data::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🪪️asset/🧩️change-extensions/🧪️tests/🔬️t069/🦀️.rs"]
                            pub mod case_t069;
                        }
                        #[path = "."]
                        pub mod change_asset_extra_data {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_asset_extra_data::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🪪️asset/🧾️change-extras/🧪️tests/🔬️t070/🦀️.rs"]
                            pub mod case_t070;
                        }
                        #[path = "."]
                        pub mod change_asset_version {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_asset_version::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🪪️asset/🔖️version/🧪️tests/🔖️raises/🦀️.rs"]
                            pub mod case_raises;
                        }
                        #[path = "."]
                        pub mod change_material_alpha_mode {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_material_alpha_mode::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../💎️material/🌫️change-alpha/📜️contract/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod contract;
                        }
                        #[path = "."]
                        pub mod change_material_double_sided {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_material_double_sided::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../💎️material/🪞️change-sides/📜️contract/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod contract;
                        }
                        #[path = "."]
                        pub mod change_node_extra_data {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_node_extra_data::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌳️node/📝️change-extras/📜️contract/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod contract;
                        }
                        #[path = "."]
                        pub mod change_node_name {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_node_name::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌳️node/🏷️rename/📜️contract/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod contract;
                        }
                        #[cfg(test)]
                        #[path = "../../../../../../../🧪️tests/🔬️contract/🦀️.rs"]
                        pub mod contract_tests;
                        #[path = "."]
                        pub mod create_scene {
                            pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::create_scene::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎬️scene/🌱️create/📜️contract/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod contract;
                        }
                        #[path = "."]
                        pub mod delete_scene {
                            pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::delete_scene::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎬️scene/🗑️delete/📜️contract/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod contract;
                        }
                        #[path = "."]
                        pub mod unbind_node_child {
                            pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::unbind_node_child::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌿️node-child/✂️unbind/📜️contract/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod contract;
                        }
                        #[path = "."]
                        pub mod unbind_scene_root_node {
                            pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::unbind_scene_root_node::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌲️scene-root/✂️unbind/📜️contract/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod contract;
                        }
                    }
                }
            }
        }
    }
}
