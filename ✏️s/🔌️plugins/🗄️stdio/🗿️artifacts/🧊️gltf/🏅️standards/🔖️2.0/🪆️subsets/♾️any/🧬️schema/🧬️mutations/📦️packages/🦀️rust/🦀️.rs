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
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "../../✅️required/➕️add/🦀️.rs"]
                        pub mod add_required_extension;
                        #[path = "../../📣️used/➕️add/🦀️.rs"]
                        pub mod add_used_extension;
                        #[path = "../../🏠️default-scene/🔗️bind/🦀️.rs"]
                        pub mod bind_default_scene;
                        #[path = "../../🎚️morph/🔗️bind/🦀️.rs"]
                        pub mod bind_morph_target_attribute;
                        #[path = "../../📷️node-camera/🔗️bind/🦀️.rs"]
                        pub mod bind_node_camera;
                        #[path = "../../🌿️node-child/🔗️bind/🦀️.rs"]
                        pub mod bind_node_child;
                        #[path = "../../🏗️node-mesh/🔗️bind/🦀️.rs"]
                        pub mod bind_node_mesh;
                        #[path = "../../🩻️node-skin/🔗️bind/🦀️.rs"]
                        pub mod bind_node_skin;
                        #[path = "../../🔤️primitive/🔗️bind/🦀️.rs"]
                        pub mod bind_primitive_attribute;
                        #[path = "../../🔢️primitive/🔗️bind/🦀️.rs"]
                        pub mod bind_primitive_indices;
                        #[path = "../../🧱️primitive/🔗️bind/🦀️.rs"]
                        pub mod bind_primitive_material;
                        #[path = "../../🌲️scene-root/🔗️bind/🦀️.rs"]
                        pub mod bind_scene_root_node;
                        #[path = "../../🪪️asset/📝️change-description/🦀️.rs"]
                        pub mod change_asset_descriptive_metadata;
                        #[path = "../../🪪️asset/🧩️change-extensions/🦀️.rs"]
                        pub mod change_asset_extension_data;
                        #[path = "../../🪪️asset/🧾️change-extras/🦀️.rs"]
                        pub mod change_asset_extra_data;
                        #[path = "../../🪪️asset/🔖️version/🦀️.rs"]
                        pub mod change_asset_version;
                        #[path = "../../📃️document/🧩️change-extensions/🦀️.rs"]
                        pub mod change_document_extension_data;
                        #[path = "../../📃️document/📝️change-extras/🦀️.rs"]
                        pub mod change_document_extra_data;
                        #[path = "../../💎️material/🌫️change-alpha/🦀️.rs"]
                        pub mod change_material_alpha_mode;
                        #[path = "../../💎️material/🪞️change-sides/🦀️.rs"]
                        pub mod change_material_double_sided;
                        #[path = "../../🕸️mesh/🧩️change-extensions/🦀️.rs"]
                        pub mod change_mesh_extension_data;
                        #[path = "../../🕸️mesh/📝️change-extras/🦀️.rs"]
                        pub mod change_mesh_extra_data;
                        #[path = "../../🕸️mesh/⚖️change-weights/🦀️.rs"]
                        pub mod change_mesh_morph_weights;
                        #[path = "../../🕸️mesh/🏷️rename/🦀️.rs"]
                        pub mod change_mesh_name;
                        #[path = "../../🌳️node/🧩️change-extensions/🦀️.rs"]
                        pub mod change_node_extension_data;
                        #[path = "../../🌳️node/📝️change-extras/🦀️.rs"]
                        pub mod change_node_extra_data;
                        #[path = "../../🌳️node/⚖️change-weights/🦀️.rs"]
                        pub mod change_node_morph_weights;
                        #[path = "../../🌳️node/🏷️rename/🦀️.rs"]
                        pub mod change_node_name;
                        #[path = "../../🌳️node/📐️transform/🦀️.rs"]
                        pub mod change_node_transform;
                        #[path = "../../🔺️primitive/🧩️change-extensions/🦀️.rs"]
                        pub mod change_primitive_extension_data;
                        #[path = "../../🔺️primitive/📝️change-extras/🦀️.rs"]
                        pub mod change_primitive_extra_data;
                        #[path = "../../🔺️primitive/📐️change-topology/🦀️.rs"]
                        pub mod change_primitive_topology_mode;
                        #[path = "../../🎬️scene/🧩️change-extensions/🦀️.rs"]
                        pub mod change_scene_extension_data;
                        #[path = "../../🎬️scene/📝️change-extras/🦀️.rs"]
                        pub mod change_scene_extra_data;
                        #[path = "../../🎬️scene/🏷️rename/🦀️.rs"]
                        pub mod change_scene_name;
                        #[cfg(test)]
                        #[path = "../../🧪️tests/🔬️contract/🦀️.rs"]
                        pub(crate) mod contract_tests;
                        #[path = "../../📐️accessor/🌱️create/🦀️.rs"]
                        pub mod create_accessor;
                        #[path = "../../🎞️animation/🌱️create/🦀️.rs"]
                        pub mod create_animation;
                        #[path = "../../💿️buffer/🌱️create/🦀️.rs"]
                        pub mod create_buffer;
                        #[path = "../../🪟️buffer-view/🌱️create/🦀️.rs"]
                        pub mod create_buffer_view;
                        #[path = "../../🎥️camera/🌱️create/🦀️.rs"]
                        pub mod create_camera;
                        #[path = "../../🖼️image/🌱️create/🦀️.rs"]
                        pub mod create_image;
                        #[path = "../../💎️material/🌱️create/🦀️.rs"]
                        pub mod create_material;
                        #[path = "../../🕸️mesh/🌱️create/🦀️.rs"]
                        pub mod create_mesh;
                        #[path = "../../🧬️morph/🌱️create/🦀️.rs"]
                        pub mod create_morph_target;
                        #[path = "../../🌳️node/🌱️create/🦀️.rs"]
                        pub mod create_node;
                        #[path = "../../🔺️primitive/🌱️create/🦀️.rs"]
                        pub mod create_primitive;
                        #[path = "../../🎛️sampler/🌱️create/🦀️.rs"]
                        pub mod create_sampler;
                        #[path = "../../🎬️scene/🌱️create/🦀️.rs"]
                        pub mod create_scene;
                        #[path = "../../🦴️skin/🌱️create/🦀️.rs"]
                        pub mod create_skin;
                        #[path = "../../🎨️texture/🌱️create/🦀️.rs"]
                        pub mod create_texture;
                        #[path = "../../📐️accessor/🗑️delete/🦀️.rs"]
                        pub mod delete_accessor;
                        #[path = "../../🎞️animation/🗑️delete/🦀️.rs"]
                        pub mod delete_animation;
                        #[path = "../../💿️buffer/🗑️delete/🦀️.rs"]
                        pub mod delete_buffer;
                        #[path = "../../🪟️buffer-view/🗑️delete/🦀️.rs"]
                        pub mod delete_buffer_view;
                        #[path = "../../🎥️camera/🗑️delete/🦀️.rs"]
                        pub mod delete_camera;
                        #[path = "../../🖼️image/🗑️delete/🦀️.rs"]
                        pub mod delete_image;
                        #[path = "../../💎️material/🗑️delete/🦀️.rs"]
                        pub mod delete_material;
                        #[path = "../../🕸️mesh/🗑️delete/🦀️.rs"]
                        pub mod delete_mesh;
                        #[path = "../../🧬️morph/🗑️delete/🦀️.rs"]
                        pub mod delete_morph_target;
                        #[path = "../../🌳️node/🗑️delete/🦀️.rs"]
                        pub mod delete_node;
                        #[path = "../../🔺️primitive/🗑️delete/🦀️.rs"]
                        pub mod delete_primitive;
                        #[path = "../../🎛️sampler/🗑️delete/🦀️.rs"]
                        pub mod delete_sampler;
                        #[path = "../../🎬️scene/🗑️delete/🦀️.rs"]
                        pub mod delete_scene;
                        #[path = "../../🦴️skin/🗑️delete/🦀️.rs"]
                        pub mod delete_skin;
                        #[path = "../../🎨️texture/🗑️delete/🦀️.rs"]
                        pub mod delete_texture;
                        #[path = "../../📐️accessor/🚚️move/🦀️.rs"]
                        pub mod move_accessor;
                        #[path = "../../🎞️animation/🚚️move/🦀️.rs"]
                        pub mod move_animation;
                        #[path = "../../💿️buffer/🚚️move/🦀️.rs"]
                        pub mod move_buffer;
                        #[path = "../../🪟️buffer-view/🚚️move/🦀️.rs"]
                        pub mod move_buffer_view;
                        #[path = "../../🎥️camera/🚚️move/🦀️.rs"]
                        pub mod move_camera;
                        #[path = "../../🖼️image/🚚️move/🦀️.rs"]
                        pub mod move_image;
                        #[path = "../../💎️material/🚚️move/🦀️.rs"]
                        pub mod move_material;
                        #[path = "../../🕸️mesh/🚚️move/🦀️.rs"]
                        pub mod move_mesh;
                        #[path = "../../🧬️morph/🚚️move/🦀️.rs"]
                        pub mod move_morph_target;
                        #[path = "../../🎚️morph/🚚️move/🦀️.rs"]
                        pub mod move_morph_target_attribute;
                        #[path = "../../🌳️node/🚚️move/🦀️.rs"]
                        pub mod move_node;
                        #[path = "../../🌿️node-child/🚚️move/🦀️.rs"]
                        pub mod move_node_child;
                        #[path = "../../🌳️node/🌿️reparent/🦀️.rs"]
                        pub mod move_node_parent;
                        #[path = "../../🔺️primitive/🚚️move/🦀️.rs"]
                        pub mod move_primitive;
                        #[path = "../../🔤️primitive/🚚️move/🦀️.rs"]
                        pub mod move_primitive_attribute;
                        #[path = "../../✅️required/🚚️move/🦀️.rs"]
                        pub mod move_required_extension;
                        #[path = "../../🎛️sampler/🚚️move/🦀️.rs"]
                        pub mod move_sampler;
                        #[path = "../../🎬️scene/🚚️move/🦀️.rs"]
                        pub mod move_scene;
                        #[path = "../../🌲️scene-root/🚚️move/🦀️.rs"]
                        pub mod move_scene_root_node;
                        #[path = "../../🦴️skin/🚚️move/🦀️.rs"]
                        pub mod move_skin;
                        #[path = "../../🎨️texture/🚚️move/🦀️.rs"]
                        pub mod move_texture;
                        #[path = "../../📣️used/🚚️move/🦀️.rs"]
                        pub mod move_used_extension;
                        #[path = "../../✅️required/➖️remove/🦀️.rs"]
                        pub mod remove_required_extension;
                        #[path = "../../📣️used/➖️remove/🦀️.rs"]
                        pub mod remove_used_extension;
                        #[path = "../../📐️accessor/🔀️reorder/🦀️.rs"]
                        pub mod reorder_accessors;
                        #[path = "../../🎞️animation/🔀️reorder/🦀️.rs"]
                        pub mod reorder_animations;
                        #[path = "../../🪟️buffer-view/🔀️reorder/🦀️.rs"]
                        pub mod reorder_buffer_views;
                        #[path = "../../💿️buffer/🔀️reorder/🦀️.rs"]
                        pub mod reorder_buffers;
                        #[path = "../../🎥️camera/🔀️reorder/🦀️.rs"]
                        pub mod reorder_cameras;
                        #[path = "../../🖼️image/🔀️reorder/🦀️.rs"]
                        pub mod reorder_images;
                        #[path = "../../💎️material/🔀️reorder/🦀️.rs"]
                        pub mod reorder_materials;
                        #[path = "../../🕸️mesh/🔀️reorder/🦀️.rs"]
                        pub mod reorder_meshs;
                        #[path = "../../🎚️morph/🔀️reorder/🦀️.rs"]
                        pub mod reorder_morph_target_attributes;
                        #[path = "../../🧬️morph/🔀️reorder/🦀️.rs"]
                        pub mod reorder_morph_targets;
                        #[path = "../../🌿️node-child/🔀️reorder/🦀️.rs"]
                        pub mod reorder_node_children;
                        #[path = "../../🌳️node/🔀️reorder/🦀️.rs"]
                        pub mod reorder_nodes;
                        #[path = "../../🔤️primitive/🔀️reorder/🦀️.rs"]
                        pub mod reorder_primitive_attributes;
                        #[path = "../../🔺️primitive/🔀️reorder/🦀️.rs"]
                        pub mod reorder_primitives;
                        #[path = "../../✅️required/🔀️reorder/🦀️.rs"]
                        pub mod reorder_required_extensions;
                        #[path = "../../🎛️sampler/🔀️reorder/🦀️.rs"]
                        pub mod reorder_samplers;
                        #[path = "../../🌲️scene-root/🔀️reorder/🦀️.rs"]
                        pub mod reorder_scene_root_nodes;
                        #[path = "../../🎬️scene/🔀️reorder/🦀️.rs"]
                        pub mod reorder_scenes;
                        #[path = "../../🦴️skin/🔀️reorder/🦀️.rs"]
                        pub mod reorder_skins;
                        #[path = "../../🎨️texture/🔀️reorder/🦀️.rs"]
                        pub mod reorder_textures;
                        #[path = "../../📣️used/🔀️reorder/🦀️.rs"]
                        pub mod reorder_used_extensions;
                        #[path = "../../🏠️default-scene/✂️unbind/🦀️.rs"]
                        pub mod unbind_default_scene;
                        #[path = "../../🎚️morph/✂️unbind/🦀️.rs"]
                        pub mod unbind_morph_target_attribute;
                        #[path = "../../📷️node-camera/✂️unbind/🦀️.rs"]
                        pub mod unbind_node_camera;
                        #[path = "../../🌿️node-child/✂️unbind/🦀️.rs"]
                        pub mod unbind_node_child;
                        #[path = "../../🏗️node-mesh/✂️unbind/🦀️.rs"]
                        pub mod unbind_node_mesh;
                        #[path = "../../🩻️node-skin/✂️unbind/🦀️.rs"]
                        pub mod unbind_node_skin;
                        #[path = "../../🔤️primitive/✂️unbind/🦀️.rs"]
                        pub mod unbind_primitive_attribute;
                        #[path = "../../🔢️primitive/✂️unbind/🦀️.rs"]
                        pub mod unbind_primitive_indices;
                        #[path = "../../🧱️primitive/✂️unbind/🦀️.rs"]
                        pub mod unbind_primitive_material;
                        #[path = "../../🌲️scene-root/✂️unbind/🦀️.rs"]
                        pub mod unbind_scene_root_node;
                    }
                    #[path = "."]
                    pub mod modules {
                        #[path = "."]
                        pub mod mutation_support {
                            #[path = "../../../../🔨️modules/🧬️mutation-support/🎬️create-scene/🦀️.rs"]
                            pub mod create_scene;
                            #[path = "../../../../🔨️modules/🧬️mutation-support/🎞️material-animation/🦀️.rs"]
                            pub mod material_animation;
                            #[path = "../../../../🔨️modules/🧬️mutation-support/🧱️structure-geometry/🦀️.rs"]
                            pub mod structure_geometry;
                            #[path = "../../../../🔨️modules/🧬️mutation-support/📚️top-level/🦀️.rs"]
                            pub mod top_level;
                            #[path = "../../../../🔨️modules/🧬️mutation-support/🗂️top-level-collections/🦀️.rs"]
                            pub mod top_level_collections;
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
