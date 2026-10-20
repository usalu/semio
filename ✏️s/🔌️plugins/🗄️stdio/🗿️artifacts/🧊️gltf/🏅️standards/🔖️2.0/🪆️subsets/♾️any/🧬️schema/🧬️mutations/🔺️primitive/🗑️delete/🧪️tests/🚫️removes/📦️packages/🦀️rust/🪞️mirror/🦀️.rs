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
                        pub mod delete_primitive {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::delete_primitive::*;
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod case_removes_the_8d7de9;
                            #[cfg(test)]
                            #[path = "../../../../🔬️middle-row/🦀️.rs"]
                            pub mod case_middle_row;
                        }
                        #[path = "."]
                        pub mod delete_sampler {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::delete_sampler::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎛️sampler/🗑️delete/🧪️tests/🚫️removes/🦀️.rs"]
                            pub mod case_removes;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎛️sampler/🗑️delete/🧪️tests/🔬️middle-row/🦀️.rs"]
                            pub mod case_middle_row;
                        }
                        #[path = "."]
                        pub mod delete_scene {
                            pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::delete_scene::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎬️scene/🗑️delete/🧪️tests/🚫️removes/🦀️.rs"]
                            pub mod case_removes;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎬️scene/🗑️delete/🧪️tests/🔬️middle-row/🦀️.rs"]
                            pub mod case_middle_row;
                        }
                        #[path = "."]
                        pub mod delete_skin {
                            pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::delete_skin::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🦴️skin/🗑️delete/🧪️tests/🚫️removes/🦀️.rs"]
                            pub mod case_removes;
                            #[cfg(test)]
                            #[path = "../../../../../../../🦴️skin/🗑️delete/🧪️tests/🔬️middle-row/🦀️.rs"]
                            pub mod case_middle_row;
                        }
                        #[path = "."]
                        pub mod delete_texture {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::delete_texture::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎨️texture/🗑️delete/🧪️tests/🔬️t049/🦀️.rs"]
                            pub mod case_t049;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎨️texture/🗑️delete/🧪️tests/🔬️middle-row/🦀️.rs"]
                            pub mod case_middle_row;
                        }
                        #[path = "."]
                        pub mod move_accessor {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_accessor::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../📐️accessor/🚚️move/🧪️tests/🚚️swaps/🦀️.rs"]
                            pub mod case_swaps;
                        }
                        #[path = "."]
                        pub mod move_animation {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_animation::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎞️animation/🚚️move/🧪️tests/⏫️promotes/🦀️.rs"]
                            pub mod case_promotes;
                        }
                        #[path = "."]
                        pub mod move_buffer {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_buffer::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../💿️buffer/🚚️move/🧪️tests/🚚️swaps/🦀️.rs"]
                            pub mod case_swaps;
                        }
                        #[path = "."]
                        pub mod move_buffer_view {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_buffer_view::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🪟️buffer-view/🚚️move/🧪️tests/🚚️swaps/🦀️.rs"]
                            pub mod case_swaps;
                        }
                        #[path = "."]
                        pub mod move_camera {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_camera::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎥️camera/🚚️move/🧪️tests/🚚️swaps/🦀️.rs"]
                            pub mod case_swaps;
                        }
                        #[path = "."]
                        pub mod move_image {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_image::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🖼️image/🚚️move/🧪️tests/🖼️swaps/🦀️.rs"]
                            pub mod case_swaps;
                        }
                        #[path = "."]
                        pub mod move_material {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_material::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../💎️material/🚚️move/🧪️tests/🚚️swaps/🦀️.rs"]
                            pub mod case_swaps;
                        }
                        #[path = "."]
                        pub mod move_mesh {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_mesh::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🕸️mesh/🚚️move/🧪️tests/🚚️swaps/🦀️.rs"]
                            pub mod case_swaps;
                        }
                        #[path = "."]
                        pub mod move_morph_target {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_morph_target::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🧬️morph/🚚️move/🧪️tests/🚚️swaps/🦀️.rs"]
                            pub mod case_swaps;
                        }
                        #[path = "."]
                        pub mod move_morph_target_attribute {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_morph_target_attribute::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎚️morph/🚚️move/🧪️tests/🔬️t047/🦀️.rs"]
                            pub mod case_t047;
                        }
                        #[path = "."]
                        pub mod move_node {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_node::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌳️node/🚚️move/🧪️tests/🚚️swaps/🦀️.rs"]
                            pub mod case_swaps;
                        }
                        #[path = "."]
                        pub mod move_node_child {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_node_child::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌿️node-child/🚚️move/🧪️tests/🔬️t045/🦀️.rs"]
                            pub mod case_t045;
                        }
                        #[path = "."]
                        pub mod move_node_parent {
                            pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::move_node_parent::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌳️node/🌿️reparent/🧪️tests/🌿️adopts/🦀️.rs"]
                            pub mod case_adopts;
                        }
                        #[path = "."]
                        pub mod move_primitive {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_primitive::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🚚️move/🧪️tests/🚚️moves/🦀️.rs"]
                            pub mod case_moves;
                        }
                        #[path = "."]
                        pub mod move_primitive_attribute {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_primitive_attribute::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔤️primitive/🚚️move/🧪️tests/🔬️t060/🦀️.rs"]
                            pub mod case_t060;
                        }
                        #[path = "."]
                        pub mod move_required_extension {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_required_extension::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../✅️required/🚚️move/🧪️tests/🔬️t043/🦀️.rs"]
                            pub mod case_t043;
                        }
                        #[path = "."]
                        pub mod move_sampler {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_sampler::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎛️sampler/🚚️move/🧪️tests/🚚️swaps/🦀️.rs"]
                            pub mod case_swaps;
                        }
                    }
                }
            }
        }
    }
}
