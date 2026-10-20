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
                            #[path = "../../../../../../../🌳️node/🌱️create/🧪️tests/🌱️inserts/🦀️.rs"]
                            pub mod case_inserts;
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
                            #[path = "../../../../../../../🎛️sampler/🌱️create/🧪️tests/🎛️inserts/🦀️.rs"]
                            pub mod case_inserts;
                        }
                        #[path = "."]
                        pub mod create_scene {
                            pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::create_scene::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎬️scene/🌱️create/🧪️tests/🎞️inserts/🦀️.rs"]
                            pub mod case_inserts;
                        }
                        #[path = "."]
                        pub mod create_skin {
                            pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::create_skin::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🦴️skin/🌱️create/🧪️tests/🦴️inserts/🦀️.rs"]
                            pub mod case_inserts;
                        }
                        #[path = "."]
                        pub mod create_texture {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::create_texture::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎨️texture/🌱️create/🧪️tests/🎨️inserts/🦀️.rs"]
                            pub mod case_inserts;
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
                            #[path = "../../../../../../../🎞️animation/🗑️delete/🧪️tests/🚫️removes/🦀️.rs"]
                            pub mod case_removes;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎞️animation/🗑️delete/🧪️tests/🔬️middle-row/🦀️.rs"]
                            pub mod case_middle_row;
                        }
                        #[path = "."]
                        pub mod delete_buffer {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::delete_buffer::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../💿️buffer/🗑️delete/🧪️tests/🚫️removes/🦀️.rs"]
                            pub mod case_removes;
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
                            #[path = "../../../../../../../🎥️camera/🗑️delete/🧪️tests/🚫️removes/🦀️.rs"]
                            pub mod case_removes;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎥️camera/🗑️delete/🧪️tests/🔬️middle-row/🦀️.rs"]
                            pub mod case_middle_row;
                        }
                        #[path = "."]
                        pub mod delete_image {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::delete_image::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🖼️image/🗑️delete/🧪️tests/🖼️removes/🦀️.rs"]
                            pub mod case_removes;
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
                            #[path = "../../../../../../../🕸️mesh/🗑️delete/🧪️tests/🔗️removes/🦀️.rs"]
                            pub mod case_removes;
                            #[cfg(test)]
                            #[path = "../../../../../../../🕸️mesh/🗑️delete/🧪️tests/🔬️middle-row/🦀️.rs"]
                            pub mod case_middle_row;
                        }
                        #[path = "."]
                        pub mod delete_morph_target {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::delete_morph_target::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🗑️delete/🧪️tests/🚫️removes/🦀️.rs"]
                            pub mod case_removes;
                            #[cfg(test)]
                            #[path = "../../../../../../🗑️delete/🧪️tests/🔬️middle-row/🦀️.rs"]
                            pub mod case_middle_row;
                        }
                        #[path = "."]
                        pub mod delete_node {
                            pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::delete_node::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌳️node/🗑️delete/🧪️tests/🚫️removes/🦀️.rs"]
                            pub mod case_removes;
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
