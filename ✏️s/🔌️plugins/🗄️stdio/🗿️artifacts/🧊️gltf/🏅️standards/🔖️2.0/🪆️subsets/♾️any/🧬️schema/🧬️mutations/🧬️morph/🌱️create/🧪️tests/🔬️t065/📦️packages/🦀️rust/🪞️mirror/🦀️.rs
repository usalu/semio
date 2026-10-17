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
                        pub mod create_morph_target {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::create_morph_target::*;
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod case_t065;
                        }
                        #[path = "."]
                        pub mod create_node {
                            pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::create_node::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌳️node/🌱️create/🧪️tests/🌱️inserts-an-empty-e75eec/🦀️.rs"]
                            pub mod case_inserts_an_empty_e75eec;
                        }
                        #[path = "."]
                        pub mod create_primitive {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::create_primitive::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔺️primitive/🌱️create/🧪️tests/🔬️t061/🦀️.rs"]
                            pub mod case_t061;
                        }
                        #[path = "."]
                        pub mod create_sampler {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::create_sampler::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎛️sampler/🌱️create/🧪️tests/🎛️inserts-a-repeat-512dbe/🦀️.rs"]
                            pub mod case_inserts_a_repeat_512dbe;
                        }
                        #[path = "."]
                        pub mod create_scene {
                            pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::create_scene::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎬️scene/🌱️create/🧪️tests/🎞️inserts-an-empty-3bc9ce/🦀️.rs"]
                            pub mod case_inserts_an_empty_3bc9ce;
                        }
                        #[path = "."]
                        pub mod create_skin {
                            pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::create_skin::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🦴️skin/🌱️create/🧪️tests/🦴️inserts-an-empty-5b0798/🦀️.rs"]
                            pub mod case_inserts_an_empty_5b0798;
                        }
                        #[path = "."]
                        pub mod create_texture {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::create_texture::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎨️texture/🌱️create/🧪️tests/🎨️inserts-an-empty-736467/🦀️.rs"]
                            pub mod case_inserts_an_empty_736467;
                        }
                        #[path = "."]
                        pub mod delete_accessor {
                            pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::delete_accessor::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../📐️accessor/🗑️delete/🧪️tests/🔬️t055/🦀️.rs"]
                            pub mod case_t055;
                            #[cfg(test)]
                            #[path = "../../../../../../../📐️accessor/🗑️delete/🧪️tests/🔬️middle-row/🦀️.rs"]
                            pub mod case_middle_row;
                        }
                        #[path = "."]
                        pub mod delete_animation {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::delete_animation::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎞️animation/🗑️delete/🧪️tests/🚫️removes-the-bob-e4e346/🦀️.rs"]
                            pub mod case_removes_the_bob_e4e346;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎞️animation/🗑️delete/🧪️tests/🔬️middle-row/🦀️.rs"]
                            pub mod case_middle_row;
                        }
                        #[path = "."]
                        pub mod delete_buffer {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::delete_buffer::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../💿️buffer/🗑️delete/🧪️tests/🚫️removes-the-two-156f9d/🦀️.rs"]
                            pub mod case_removes_the_two_156f9d;
                            #[cfg(test)]
                            #[path = "../../../../../../../💿️buffer/🗑️delete/🧪️tests/🔬️middle-row/🦀️.rs"]
                            pub mod case_middle_row;
                        }
                        #[path = "."]
                        pub mod delete_buffer_view {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::delete_buffer_view::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🪟️buffer-view/🗑️delete/🧪️tests/🔬️t067/🦀️.rs"]
                            pub mod case_t067;
                            #[cfg(test)]
                            #[path = "../../../../../../../🪟️buffer-view/🗑️delete/🧪️tests/🔬️middle-row/🦀️.rs"]
                            pub mod case_middle_row;
                        }
                        #[path = "."]
                        pub mod delete_camera {
                            pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::delete_camera::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎥️camera/🗑️delete/🧪️tests/🚫️removes-the-2257c3/🦀️.rs"]
                            pub mod case_removes_the_2257c3;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎥️camera/🗑️delete/🧪️tests/🔬️middle-row/🦀️.rs"]
                            pub mod case_middle_row;
                        }
                        #[path = "."]
                        pub mod delete_image {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::delete_image::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🖼️image/🗑️delete/🧪️tests/🖼️removes-the-6557fc/🦀️.rs"]
                            pub mod case_removes_the_6557fc;
                            #[cfg(test)]
                            #[path = "../../../../../../../🖼️image/🗑️delete/🧪️tests/🔬️middle-row/🦀️.rs"]
                            pub mod case_middle_row;
                        }
                        #[path = "."]
                        pub mod delete_material {
                            pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::delete_material::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../💎️material/🗑️delete/🧪️tests/🔬️t051/🦀️.rs"]
                            pub mod case_t051;
                            #[cfg(test)]
                            #[path = "../../../../../../../💎️material/🗑️delete/🧪️tests/🔬️middle-row/🦀️.rs"]
                            pub mod case_middle_row;
                        }
                        #[path = "."]
                        pub mod delete_mesh {
                            pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::delete_mesh::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🕸️mesh/🗑️delete/🧪️tests/🔗️removes-the-hull-b7a141/🦀️.rs"]
                            pub mod case_removes_the_hull_b7a141;
                            #[cfg(test)]
                            #[path = "../../../../../../../🕸️mesh/🗑️delete/🧪️tests/🔬️middle-row/🦀️.rs"]
                            pub mod case_middle_row;
                        }
                        #[path = "."]
                        pub mod delete_morph_target {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::delete_morph_target::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🗑️delete/🧪️tests/🚫️removes-the-db72f4/🦀️.rs"]
                            pub mod case_removes_the_db72f4;
                            #[cfg(test)]
                            #[path = "../../../../../../🗑️delete/🧪️tests/🔬️middle-row/🦀️.rs"]
                            pub mod case_middle_row;
                        }
                        #[path = "."]
                        pub mod delete_node {
                            pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::delete_node::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌳️node/🗑️delete/🧪️tests/🚫️removes-the-root-516263/🦀️.rs"]
                            pub mod case_removes_the_root_516263;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌳️node/🗑️delete/🧪️tests/🔬️middle-row/🦀️.rs"]
                            pub mod case_middle_row;
                        }
                    }
                }
            }
        }
    }
}
