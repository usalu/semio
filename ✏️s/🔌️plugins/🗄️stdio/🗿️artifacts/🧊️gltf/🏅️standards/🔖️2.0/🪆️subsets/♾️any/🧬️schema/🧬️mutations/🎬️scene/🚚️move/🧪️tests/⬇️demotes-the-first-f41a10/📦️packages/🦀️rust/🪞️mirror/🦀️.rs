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
                            #[path = "../../../../../../../🌲️scene-root/🚚️move/🧪️tests/🛑️refuses-to-move-ba235e/🦀️.rs"]
                            pub mod case_refuses_to_move_ba235e;
                        }
                        #[path = "."]
                        pub mod move_skin {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::move_skin::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🦴️skin/🚚️move/🧪️tests/🚚️swaps-the-two-skins-dddb3f/🦀️.rs"]
                            pub mod case_swaps_the_two_skins_dddb3f;
                        }
                        #[path = "."]
                        pub mod move_texture {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::move_texture::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎨️texture/🚚️move/🧪️tests/🚚️swaps-the-albedo-e77287/🦀️.rs"]
                            pub mod case_swaps_the_albedo_e77287;
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
                            #[path = "../../../../../../../📐️accessor/🔀️reorder/🧪️tests/🔀️flips-the-two-e56161/🦀️.rs"]
                            pub mod case_flips_the_two_e56161;
                        }
                        #[path = "."]
                        pub mod reorder_animations {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_animations::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎞️animation/🔀️reorder/🧪️tests/🔀️flips-the-spin-5be673/🦀️.rs"]
                            pub mod case_flips_the_spin_5be673;
                        }
                        #[path = "."]
                        pub mod reorder_buffer_views {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_buffer_views::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🪟️buffer-view/🔀️reorder/🧪️tests/🔀️flips-the-two-a73017/🦀️.rs"]
                            pub mod case_flips_the_two_a73017;
                        }
                        #[path = "."]
                        pub mod reorder_buffers {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_buffers::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../💿️buffer/🔀️reorder/🧪️tests/🔀️flips-the-two-1bb32a/🦀️.rs"]
                            pub mod case_flips_the_two_1bb32a;
                        }
                        #[path = "."]
                        pub mod reorder_cameras {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_cameras::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎥️camera/🔀️reorder/🧪️tests/🔀️flips-the-0609c1/🦀️.rs"]
                            pub mod case_flips_the_0609c1;
                        }
                        #[path = "."]
                        pub mod reorder_images {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_images::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🖼️image/🔀️reorder/🧪️tests/🖼️flips-the-albedo-17dec9/🦀️.rs"]
                            pub mod case_flips_the_albedo_17dec9;
                        }
                        #[path = "."]
                        pub mod reorder_materials {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_materials::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../💎️material/🔀️reorder/🧪️tests/🔀️flips-the-glass-98d5db/🦀️.rs"]
                            pub mod case_flips_the_glass_98d5db;
                        }
                        #[path = "."]
                        pub mod reorder_meshs {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_meshs::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🕸️mesh/🔀️reorder/🧪️tests/🔀️flips-the-hull-ebfffa/🦀️.rs"]
                            pub mod case_flips_the_hull_ebfffa;
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
                            #[path = "../../../../../../../🧬️morph/🔀️reorder/🧪️tests/🔀️flips-the-two-14d32f/🦀️.rs"]
                            pub mod case_flips_the_two_14d32f;
                        }
                        #[path = "."]
                        pub mod reorder_node_children {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_node_children::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌿️node-child/🔀️reorder/🧪️tests/🚫️refuses-an-order-dd1195/🦀️.rs"]
                            pub mod case_refuses_an_order_dd1195;
                        }
                        #[path = "."]
                        pub mod reorder_nodes {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_nodes::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌳️node/🔀️reorder/🧪️tests/🔢️flips-the-two-f25cfb/🦀️.rs"]
                            pub mod case_flips_the_two_f25cfb;
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
                            #[path = "../../../../../../../✅️required/🔀️reorder/🧪️tests/🔌️flips-the-two-c3ca1e/🦀️.rs"]
                            pub mod case_flips_the_two_c3ca1e;
                        }
                        #[path = "."]
                        pub mod reorder_samplers {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_samplers::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎛️sampler/🔀️reorder/🧪️tests/🔀️flips-the-linear-952435/🦀️.rs"]
                            pub mod case_flips_the_linear_952435;
                        }
                        #[path = "."]
                        pub mod reorder_scene_root_nodes {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_scene_root_nodes::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌲️scene-root/🔀️reorder/🧪️tests/🚫️refuses-a-scene-e8c50c/🦀️.rs"]
                            pub mod case_refuses_a_scene_e8c50c;
                        }
                        #[path = "."]
                        pub mod reorder_scenes {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_3::standards::v2_0::subsets::any::schema::mutations::reorder_scenes::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🔀️reorder/🧪️tests/🗺️flips-the-two-71e980/🦀️.rs"]
                            pub mod case_flips_the_two_71e980;
                        }
                    }
                }
            }
        }
    }
}
