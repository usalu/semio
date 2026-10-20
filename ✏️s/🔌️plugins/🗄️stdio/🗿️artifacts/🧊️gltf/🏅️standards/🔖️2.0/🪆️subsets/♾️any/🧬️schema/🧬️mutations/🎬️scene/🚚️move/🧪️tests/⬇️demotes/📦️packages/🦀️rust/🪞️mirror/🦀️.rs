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
                        pub mod move_scene {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::move_scene::*;
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod case_demotes_the_first_f41a10;
                        }
                        #[path = "."]
                        pub mod move_scene_root_node {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::move_scene_root_node::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌲️scene-root/🚚️move/🧪️tests/🛑️refuses/🦀️.rs"]
                            pub mod case_refuses;
                        }
                        #[path = "."]
                        pub mod move_skin {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::move_skin::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🦴️skin/🚚️move/🧪️tests/🚚️swaps/🦀️.rs"]
                            pub mod case_swaps;
                        }
                        #[path = "."]
                        pub mod move_texture {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::move_texture::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎨️texture/🚚️move/🧪️tests/🚚️swaps/🦀️.rs"]
                            pub mod case_swaps;
                        }
                        #[path = "."]
                        pub mod move_used_extension {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::move_used_extension::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../📣️used/🚚️move/🧪️tests/🔬️t058/🦀️.rs"]
                            pub mod case_t058;
                        }
                        #[path = "."]
                        pub mod remove_required_extension {
                            pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::remove_required_extension::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../✅️required/➖️remove/🧪️tests/🔬️t042/🦀️.rs"]
                            pub mod case_t042;
                            #[cfg(test)]
                            #[path = "../../../../../../../✅️required/➖️remove/🧪️tests/🔬️middle-row/🦀️.rs"]
                            pub mod case_middle_row;
                        }
                        #[path = "."]
                        pub mod remove_used_extension {
                            pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../📣️used/➖️remove/🧪️tests/🔬️t057/🦀️.rs"]
                            pub mod case_t057;
                            #[cfg(test)]
                            #[path = "../../../../../../../📣️used/➖️remove/🧪️tests/🔬️middle-row/🦀️.rs"]
                            pub mod case_middle_row;
                        }
                        #[path = "."]
                        pub mod reorder_accessors {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_accessors::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../📐️accessor/🔀️reorder/🧪️tests/🔀️flips/🦀️.rs"]
                            pub mod case_flips;
                        }
                        #[path = "."]
                        pub mod reorder_animations {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_animations::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎞️animation/🔀️reorder/🧪️tests/🔀️flips/🦀️.rs"]
                            pub mod case_flips;
                        }
                        #[path = "."]
                        pub mod reorder_buffer_views {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_buffer_views::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🪟️buffer-view/🔀️reorder/🧪️tests/🔀️flips/🦀️.rs"]
                            pub mod case_flips;
                        }
                        #[path = "."]
                        pub mod reorder_buffers {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_buffers::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../💿️buffer/🔀️reorder/🧪️tests/🔀️flips/🦀️.rs"]
                            pub mod case_flips;
                        }
                        #[path = "."]
                        pub mod reorder_cameras {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_cameras::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎥️camera/🔀️reorder/🧪️tests/🔀️flips/🦀️.rs"]
                            pub mod case_flips;
                        }
                        #[path = "."]
                        pub mod reorder_images {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_images::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🖼️image/🔀️reorder/🧪️tests/🖼️flips/🦀️.rs"]
                            pub mod case_flips;
                        }
                        #[path = "."]
                        pub mod reorder_materials {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_materials::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../💎️material/🔀️reorder/🧪️tests/🔀️flips/🦀️.rs"]
                            pub mod case_flips;
                        }
                        #[path = "."]
                        pub mod reorder_meshs {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_meshs::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🕸️mesh/🔀️reorder/🧪️tests/🔀️flips/🦀️.rs"]
                            pub mod case_flips;
                        }
                        #[path = "."]
                        pub mod reorder_morph_target_attributes {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_morph_target_attributes::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎚️morph/🔀️reorder/🧪️tests/🔬️t046/🦀️.rs"]
                            pub mod case_t046;
                        }
                        #[path = "."]
                        pub mod reorder_morph_targets {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_morph_targets::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🧬️morph/🔀️reorder/🧪️tests/🔀️flips/🦀️.rs"]
                            pub mod case_flips;
                        }
                        #[path = "."]
                        pub mod reorder_node_children {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_node_children::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌿️node-child/🔀️reorder/🧪️tests/🚫️refuses/🦀️.rs"]
                            pub mod case_refuses;
                        }
                        #[path = "."]
                        pub mod reorder_nodes {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_nodes::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌳️node/🔀️reorder/🧪️tests/🔢️flips/🦀️.rs"]
                            pub mod case_flips;
                        }
                        #[path = "."]
                        pub mod reorder_primitive_attributes {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_primitive_attributes::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔤️primitive/🔀️reorder/🧪️tests/🔬️t059/🦀️.rs"]
                            pub mod case_t059;
                        }
                        #[path = "."]
                        pub mod reorder_primitives {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_primitives::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔺️primitive/🔀️reorder/🧪️tests/🔬️t062/🦀️.rs"]
                            pub mod case_t062;
                        }
                        #[path = "."]
                        pub mod reorder_required_extensions {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_required_extensions::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../✅️required/🔀️reorder/🧪️tests/🔌️flips/🦀️.rs"]
                            pub mod case_flips;
                        }
                        #[path = "."]
                        pub mod reorder_samplers {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_samplers::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎛️sampler/🔀️reorder/🧪️tests/🔀️flips/🦀️.rs"]
                            pub mod case_flips;
                        }
                        #[path = "."]
                        pub mod reorder_scene_root_nodes {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_scene_root_nodes::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌲️scene-root/🔀️reorder/🧪️tests/🚫️refuses/🦀️.rs"]
                            pub mod case_refuses;
                        }
                        #[path = "."]
                        pub mod reorder_scenes {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_scenes::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🔀️reorder/🧪️tests/🗺️flips/🦀️.rs"]
                            pub mod case_flips;
                        }
                    }
                }
            }
        }
    }
}
