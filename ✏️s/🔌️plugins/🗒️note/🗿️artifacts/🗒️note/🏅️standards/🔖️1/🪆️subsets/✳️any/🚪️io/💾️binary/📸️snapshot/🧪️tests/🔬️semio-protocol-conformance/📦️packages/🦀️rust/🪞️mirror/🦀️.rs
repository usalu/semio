//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_note_note_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_note_note_mutations::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_note_note_mutations::standards::v1::subsets::any::io::binary::snapshot::*;
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod semio_protocol_conformance;
                        }
                        #[path = "."]
                        pub mod mutations {
                            pub use semio_s_artifact_note_note_mutations::standards::v1::subsets::any::io::binary::mutations::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🧬️mutations/🧪️tests/🔬️semio-protocol-conformance/🦀️.rs"]
                            pub mod semio_protocol_conformance;
                        }
                    }
                    #[path = "../../../../../../../📝️text/🦀️.rs"]
                    pub mod text;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod pdf {
                                    #[path = "."]
                                    pub mod v1_4 {
                                        #[path = "."]
                                        pub mod base {
                                            #[path = "../../../../../../../📥️import/🧩️deserializers/🗿️artifacts/📖️pdf/🔖️1.4/🧱️base/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod png {
                                    #[path = "."]
                                    pub mod v1_2 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dxf {
                                    #[path = "."]
                                    pub mod v_r12 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../📥️import/🧩️deserializers/🗿️artifacts/📐️dxf/🔖️r12/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            #[path = "."]
            pub mod ink {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod change_pencil_width {
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🖋️ink/🧬️schema/🧬️mutations/✏️change-pencil-width/🧪️tests/✏️thickens-pencil/🦀️.rs"]
                            pub mod tests_thickens_pencil;
                        }
                        #[path = "."]
                        pub mod change_eraser_radius {
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🖋️ink/🧬️schema/🧬️mutations/🧽️change-eraser-radius/🧪️tests/🧽️enlarges/🦀️.rs"]
                            pub mod tests_enlarges_eraser;
                        }
                        #[path = "."]
                        pub mod change_block_ink_width {
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🖋️ink/🧬️schema/🧬️mutations/🖊️change-block-ink-width/🧪️tests/🖊️thickens/🦀️.rs"]
                            pub mod tests_thickens_the_sketch_stroke;
                        }
                        #[path = "."]
                        pub mod edit_block_ink_stroke {
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🖋️ink/🧬️schema/🧬️mutations/🎨️edit-block-ink-stroke/🧪️tests/🎨️redraws/🦀️.rs"]
                            pub mod tests_redraws_the_sketch_polyline;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod text {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod edit_block_text {
                            #[cfg(test)]
                            #[path = "../../../../../../../../../📝️text/🧬️schema/🧬️mutations/📝️edit-block-text/🧪️tests/📝️replaces/🦀️.rs"]
                            pub mod tests_replaces_the_intro_paragraphs;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod math {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod edit_block_math {
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🧮️math/🧬️schema/🧬️mutations/🧮️edit-block-math/🧪️tests/📐️replaces/🦀️.rs"]
                            pub mod tests_replaces_the_tex_with_pythagoras;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod table {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod insert_table_row {
                            #[cfg(test)]
                            #[path = "../../../../../../../../../📊️table/🧬️schema/🧬️mutations/⬇️insert-table-row/🧪️tests/⬇️appends/🦀️.rs"]
                            pub mod tests_appends_a_blank_third_row;
                        }
                        #[path = "."]
                        pub mod remove_table_row {
                            #[cfg(test)]
                            #[path = "../../../../../../../../../📊️table/🧬️schema/🧬️mutations/⬆️remove-table-row/🧪️tests/⬆️drops/🦀️.rs"]
                            pub mod tests_drops_the_trailing_blank_row;
                        }
                        #[path = "."]
                        pub mod insert_table_column {
                            #[cfg(test)]
                            #[path = "../../../../../../../../../📊️table/🧬️schema/🧬️mutations/➡️insert-table-column/🧪️tests/➡️appends/🦀️.rs"]
                            pub mod tests_appends_the_lettered_column_c;
                        }
                        #[path = "."]
                        pub mod remove_table_column {
                            #[cfg(test)]
                            #[path = "../../../../../../../../../📊️table/🧬️schema/🧬️mutations/⬅️remove-table-column/🧪️tests/⬅️drops/🦀️.rs"]
                            pub mod tests_drops_the_trailing_column_b;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod asset {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod create_asset {
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🖼️asset/🧬️schema/🧬️mutations/🆕️create-asset/🧪️tests/➕️adds/🦀️.rs"]
                            pub mod tests_adds_a_second_image_asset;
                        }
                        #[path = "."]
                        pub mod replace_asset_payload {
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🖼️asset/🧬️schema/🧬️mutations/🔁️replace-asset-payload/🧪️tests/🔁️swaps/🦀️.rs"]
                            pub mod tests_swaps_logo_payload_for_svg;
                        }
                        #[path = "."]
                        pub mod delete_asset {
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🖼️asset/🧬️schema/🧬️mutations/🗑️delete-asset/🧪️tests/🗑️removes/🦀️.rs"]
                            pub mod tests_removes_the_logo_asset;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod block {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod create_block {
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🧱️block/🧬️schema/🧬️mutations/➕️create-block/🧪️tests/📷️inserts/🦀️.rs"]
                            pub mod tests_inserts_a_photo_block_at_root_index_2;
                        }
                        #[path = "."]
                        pub mod delete_block {
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🧱️block/🧬️schema/🧬️mutations/❌️delete-block/🧪️tests/➖️removes-the-math-block/🦀️.rs"]
                            pub mod tests_removes_the_math_block;
                        }
                        #[path = "."]
                        pub mod delete_blocks {
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🧱️block/🧬️schema/🧬️mutations/🧹️delete-blocks/🧪️tests/🗑️removes/🦀️.rs"]
                            pub mod tests_removes_the_ink_and_image_blocks;
                        }
                        #[path = "."]
                        pub mod duplicate_block {
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🧱️block/🧬️schema/🧬️mutations/📋️duplicate-block/🧪️tests/📋️copies/🦀️.rs"]
                            pub mod tests_copies_the_math_block_right_after_its_source;
                        }
                        #[path = "."]
                        pub mod duplicate_blocks {
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🧱️block/🧬️schema/🧬️mutations/👥️duplicate-blocks/🧪️tests/👥️copies/🦀️.rs"]
                            pub mod tests_copies_ink_and_table_with_shifting_indices;
                        }
                        #[path = "."]
                        pub mod move_block_to_container {
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🧱️block/🧬️schema/🧬️mutations/🚚️move-block-to-container/🧪️tests/📥️reparents/🦀️.rs"]
                            pub mod tests_reparents_ink_into_the_callout_group;
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod note {
        pub use semio_s_artifact_note_note_kinds_catalog::editor::note::dispatch_ctx;
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../../../../✏️editor/🎮️commands/🧲️set-snap-enabled/🦀️.rs"]
            pub mod set_snap_enabled;
        }
    }
}
