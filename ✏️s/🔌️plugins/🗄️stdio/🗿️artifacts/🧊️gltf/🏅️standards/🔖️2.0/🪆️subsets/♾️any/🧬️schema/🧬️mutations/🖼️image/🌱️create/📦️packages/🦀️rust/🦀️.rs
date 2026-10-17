//! 🌱️ mut-any-2 of `semio-s-artifact-stdio-gltf-mutations` — a path-preserving carve of the artifact root mounts.

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
                        pub mod create_image;
                        #[path = "../../../../🧬️morph/🌱️create/🦀️.rs"]
                        pub mod create_morph_target;
                        #[path = "../../../../🔺️primitive/🌱️create/🦀️.rs"]
                        pub mod create_primitive;
                        #[path = "../../../../🎛️sampler/🌱️create/🦀️.rs"]
                        pub mod create_sampler;
                        #[path = "../../../../🎨️texture/🌱️create/🦀️.rs"]
                        pub mod create_texture;
                        #[path = "../../../🗑️delete/🦀️.rs"]
                        pub mod delete_image;
                        #[path = "../../../../🧬️morph/🗑️delete/🦀️.rs"]
                        pub mod delete_morph_target;
                        #[path = "../../../../🔺️primitive/🗑️delete/🦀️.rs"]
                        pub mod delete_primitive;
                        #[path = "../../../../🎛️sampler/🗑️delete/🦀️.rs"]
                        pub mod delete_sampler;
                        #[path = "../../../../🎨️texture/🗑️delete/🦀️.rs"]
                        pub mod delete_texture;
                        #[path = "../../../../📐️accessor/🚚️move/🦀️.rs"]
                        pub mod move_accessor;
                        #[path = "../../../../🎞️animation/🚚️move/🦀️.rs"]
                        pub mod move_animation;
                        #[path = "../../../../💿️buffer/🚚️move/🦀️.rs"]
                        pub mod move_buffer;
                        #[path = "../../../../🪟️buffer-view/🚚️move/🦀️.rs"]
                        pub mod move_buffer_view;
                        #[path = "../../../../🎥️camera/🚚️move/🦀️.rs"]
                        pub mod move_camera;
                        #[path = "../../../🚚️move/🦀️.rs"]
                        pub mod move_image;
                        #[path = "../../../../💎️material/🚚️move/🦀️.rs"]
                        pub mod move_material;
                        #[path = "../../../../🕸️mesh/🚚️move/🦀️.rs"]
                        pub mod move_mesh;
                        #[path = "../../../../🧬️morph/🚚️move/🦀️.rs"]
                        pub mod move_morph_target;
                        #[path = "../../../../🎚️morph/🚚️move/🦀️.rs"]
                        pub mod move_morph_target_attribute;
                        #[path = "../../../../🌳️node/🚚️move/🦀️.rs"]
                        pub mod move_node;
                        #[path = "../../../../🌿️node-child/🚚️move/🦀️.rs"]
                        pub mod move_node_child;
                        #[path = "../../../../🔺️primitive/🚚️move/🦀️.rs"]
                        pub mod move_primitive;
                        #[path = "../../../../🔤️primitive/🚚️move/🦀️.rs"]
                        pub mod move_primitive_attribute;
                        #[path = "../../../../✅️required/🚚️move/🦀️.rs"]
                        pub mod move_required_extension;
                        #[path = "../../../../🎛️sampler/🚚️move/🦀️.rs"]
                        pub mod move_sampler;
                        #[path = "../../../../🎬️scene/🚚️move/🦀️.rs"]
                        pub mod move_scene;
                        pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::remove_used_extension;
                    }
                    #[path = "."]
                    pub mod modules {
                        #[path = "."]
                        pub mod mutation_support {
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
