//! 🧬️ `stdio.gltf` mutations: the mutation vocabulary, its dispatch and support modules and its text and binary codecs.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;
#[cfg(test)]
extern crate semio_framework_schema as framework_schema;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v2_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::io::binary;
                    pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::io::text;
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::*;

                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_4::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::add_required_extension;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::add_used_extension;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::bind_default_scene;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::bind_morph_target_attribute;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::bind_node_camera;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::bind_node_child;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::bind_node_mesh;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::bind_node_skin;
                        pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::bind_primitive_attribute;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::bind_primitive_indices;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::bind_primitive_material;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::bind_scene_root_node;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::change_asset_descriptive_metadata;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::change_asset_extension_data;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_asset_extra_data;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_asset_version;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_document_extension_data;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_document_extra_data;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_material_alpha_mode;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_material_double_sided;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_mesh_extension_data;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_mesh_extra_data;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_mesh_morph_weights;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_mesh_name;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_node_extension_data;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_node_extra_data;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_node_morph_weights;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_node_name;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_node_transform;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_primitive_extension_data;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_primitive_extra_data;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_primitive_topology_mode;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_scene_extension_data;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_scene_extra_data;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_scene_name;

                        pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::create_accessor;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::create_animation;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::create_buffer;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::create_buffer_view;
                        pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::create_camera;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::create_image;
                        pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::create_material;
                        pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::create_mesh;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::create_morph_target;
                        pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::create_node;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::create_primitive;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::create_sampler;
                        pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::create_scene;
                        pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::create_skin;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::create_texture;
                        pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::delete_accessor;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::delete_animation;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::delete_buffer;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::delete_buffer_view;
                        pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::delete_camera;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::delete_image;
                        pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::delete_material;
                        pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::delete_mesh;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::delete_morph_target;
                        pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::delete_node;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::delete_primitive;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::delete_sampler;
                        pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::delete_scene;
                        pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::delete_skin;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::delete_texture;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_accessor;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_animation;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_buffer;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_buffer_view;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_camera;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_image;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_material;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_mesh;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_morph_target;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_morph_target_attribute;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_node;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_node_child;
                        pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::move_node_parent;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_primitive;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_primitive_attribute;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_required_extension;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_sampler;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_scene;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::move_scene_root_node;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::move_skin;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::move_texture;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::move_used_extension;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::remove_required_extension;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::remove_used_extension;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_accessors;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_animations;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_buffer_views;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_buffers;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_cameras;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_images;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_materials;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_meshs;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_morph_target_attributes;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_morph_targets;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_node_children;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_nodes;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_primitive_attributes;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_primitives;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_required_extensions;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_samplers;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_scene_root_nodes;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_scenes;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_skins;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_textures;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_used_extensions;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::unbind_default_scene;
                        pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::unbind_morph_target_attribute;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::unbind_node_camera;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::unbind_node_child;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::unbind_node_mesh;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::unbind_node_skin;
                        pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::unbind_primitive_attribute;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::unbind_primitive_indices;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::unbind_primitive_material;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::unbind_scene_root_node;
                    }
                    #[path = "."]
                    pub mod modules {
                        #[path = "."]
                        pub mod mutation_support {
                            pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::modules::mutation_support::create_scene;
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

#[cfg(test)]
#[path = "../../🧪️tests/📜️facet-conformance/🦀️.rs"]
mod facet_conformance_laws;
