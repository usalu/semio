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
                        #[path = "."]
                        pub mod change_document_extension_data {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_document_extension_data::*;
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod case_t053;
                        }
                        #[path = "."]
                        pub mod change_document_extra_data {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_document_extra_data::*;
                            #[cfg(test)]
                            #[path = "../../../../../../📝️change-extras/🧪️tests/🔬️t052/🦀️.rs"]
                            pub mod case_t052;
                        }
                        #[path = "."]
                        pub mod change_material_alpha_mode {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_material_alpha_mode::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../💎️material/🌫️change-alpha/🧪️tests/🎭️switches-the-79b834/🦀️.rs"]
                            pub mod case_switches_the_79b834;
                        }
                        #[path = "."]
                        pub mod change_material_double_sided {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_material_double_sided::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../💎️material/🪞️change-sides/🧪️tests/🪞️makes-the-6261b2/🦀️.rs"]
                            pub mod case_makes_the_6261b2;
                        }
                        #[path = "."]
                        pub mod change_mesh_extension_data {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_mesh_extension_data::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🕸️mesh/🧩️change-extensions/🧪️tests/🔌️attaches-an-9e1e51/🦀️.rs"]
                            pub mod case_attaches_an_9e1e51;
                        }
                        #[path = "."]
                        pub mod change_mesh_extra_data {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_mesh_extra_data::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🕸️mesh/📝️change-extras/🧪️tests/✍️attaches-an-8df0ce/🦀️.rs"]
                            pub mod case_attaches_an_8df0ce;
                        }
                        #[path = "."]
                        pub mod change_mesh_morph_weights {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_mesh_morph_weights::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🕸️mesh/⚖️change-weights/🧪️tests/🔬️t064/🦀️.rs"]
                            pub mod case_t064;
                        }
                        #[path = "."]
                        pub mod change_mesh_name {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_mesh_name::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🕸️mesh/🏷️rename/🧪️tests/✏️renames-the-hull-0a49e7/🦀️.rs"]
                            pub mod case_renames_the_hull_0a49e7;
                        }
                        #[path = "."]
                        pub mod change_node_extension_data {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_node_extension_data::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌳️node/🧩️change-extensions/🧪️tests/🔬️t044/🦀️.rs"]
                            pub mod case_t044;
                        }
                        #[path = "."]
                        pub mod change_node_extra_data {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_node_extra_data::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌳️node/📝️change-extras/🧪️tests/📝️attaches-a-unit-5159f9/🦀️.rs"]
                            pub mod case_attaches_a_unit_5159f9;
                        }
                        #[path = "."]
                        pub mod change_node_morph_weights {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_node_morph_weights::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌳️node/⚖️change-weights/🧪️tests/⚖️sets-the-node-1b9600/🦀️.rs"]
                            pub mod case_sets_the_node_1b9600;
                        }
                        #[path = "."]
                        pub mod change_node_name {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_node_name::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌳️node/🏷️rename/🧪️tests/✏️renames-the-root-f1e002/🦀️.rs"]
                            pub mod case_renames_the_root_f1e002;
                        }
                        #[path = "."]
                        pub mod change_node_transform {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_node_transform::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🌳️node/📐️transform/🧪️tests/🔀️replaces-a-f09fa4/🦀️.rs"]
                            pub mod case_replaces_a_f09fa4;
                        }
                        #[path = "."]
                        pub mod change_primitive_extension_data {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_primitive_extension_data::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔺️primitive/🧩️change-extensions/🧪️tests/🔬️t063/🦀️.rs"]
                            pub mod case_t063;
                        }
                        #[path = "."]
                        pub mod change_primitive_extra_data {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_primitive_extra_data::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔺️primitive/📝️change-extras/🧪️tests/📝️attaches-a-lod-698164/🦀️.rs"]
                            pub mod case_attaches_a_lod_698164;
                        }
                        #[path = "."]
                        pub mod change_primitive_topology_mode {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_primitive_topology_mode::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔺️primitive/📐️change-topology/🧪️tests/📐️triangle-81bc44/🦀️.rs"]
                            pub mod case_triangle_81bc44;
                        }
                        #[path = "."]
                        pub mod change_scene_extension_data {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_scene_extension_data::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎬️scene/🧩️change-extensions/🧪️tests/🔬️t050/🦀️.rs"]
                            pub mod case_t050;
                        }
                        #[path = "."]
                        pub mod change_scene_extra_data {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_scene_extra_data::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎬️scene/📝️change-extras/🧪️tests/📏️attaches-a-units-e2653c/🦀️.rs"]
                            pub mod case_attaches_a_units_e2653c;
                        }
                        #[path = "."]
                        pub mod change_scene_name {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::change_scene_name::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎬️scene/🏷️rename/🧪️tests/✏️renames-the-main-4b6c9d/🦀️.rs"]
                            pub mod case_renames_the_main_4b6c9d;
                        }
                        #[path = "."]
                        pub mod create_accessor {
                            pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::create_accessor::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../📐️accessor/🌱️create/🧪️tests/🔬️t054/🦀️.rs"]
                            pub mod case_t054;
                        }
                        #[path = "."]
                        pub mod create_animation {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::create_animation::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎞️animation/🌱️create/🧪️tests/🎞️inserts-an-empty-99ef88/🦀️.rs"]
                            pub mod case_inserts_an_empty_99ef88;
                        }
                        #[path = "."]
                        pub mod create_buffer {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::create_buffer::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../💿️buffer/🌱️create/🧪️tests/💿️inserts-a-two-ab4132/🦀️.rs"]
                            pub mod case_inserts_a_two_ab4132;
                        }
                        #[path = "."]
                        pub mod create_buffer_view {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any::standards::v2_0::subsets::any::schema::mutations::create_buffer_view::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🪟️buffer-view/🌱️create/🧪️tests/🔬️t066/🦀️.rs"]
                            pub mod case_t066;
                        }
                        #[path = "."]
                        pub mod create_camera {
                            pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::create_camera::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🎥️camera/🌱️create/🧪️tests/🔬️t048/🦀️.rs"]
                            pub mod case_t048;
                        }
                        #[path = "."]
                        pub mod create_image {
                            pub use semio_s_artifact_stdio_gltf_mutations_mut_any_2::standards::v2_0::subsets::any::schema::mutations::create_image::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🖼️image/🌱️create/🧪️tests/🖼️inserts-an-empty-e3b07a/🦀️.rs"]
                            pub mod case_inserts_an_empty_e3b07a;
                        }
                        #[path = "."]
                        pub mod create_material {
                            pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::create_material::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../💎️material/🌱️create/🧪️tests/💎️inserts-a-5ae67c/🦀️.rs"]
                            pub mod case_inserts_a_5ae67c;
                        }
                        #[path = "."]
                        pub mod create_mesh {
                            pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::create_mesh::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🕸️mesh/🌱️create/🧪️tests/🕸️inserts-an-empty-49e11c/🦀️.rs"]
                            pub mod case_inserts_an_empty_49e11c;
                        }
                    }
                }
            }
        }
    }
}
