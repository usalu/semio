//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_4 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_snapshot::standards::v1_4::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_pdf_mut_base::standards::v1_4::subsets::base::schema::mutations;
                }
            }
        }
    }
    #[path = "."]
    pub mod v1_7 {
        #[path = "../../../../../../../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                pub use semio_s_artifact_stdio_pdf_testkit::standards::v1_7::subsets::base::*;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_schema_4::standards::v1_7::subsets::base::schema::*;
                    pub use semio_s_artifact_stdio_pdf_testkit::standards::v1_7::subsets::base::schema::*;
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::schema::stream_roles;
                    #[path = "."]
                    pub mod snapshot {
                        pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::*;
                        pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::*;
                        #[cfg(test)]
                        #[path = "../../../🦀️.rs"]
                        pub mod pdf_snapshot_value_tests;
                    }
                    pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::mutations::*;
                        pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::mutations::insert_page;
                        pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::mutations::remove_page;
                        pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::schema::mutations::set_page_media_box;
                        pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::schema::mutations::set_info;
                        pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::schema::mutations::insert_object;
                        pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::schema::mutations::remove_object;
                        pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::schema::mutations::set_object_value;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_trailer_entry;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::remove_trailer_entry;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::move_page;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_page_content;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_page_rotation;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_page_box;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_page_user_unit;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_annotation;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_font;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_image;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_form;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_ext_g_state;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_shading;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_pattern;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_color_space;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_properties;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_embedded_file;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::remove_embedded_file;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_outlines;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_named_destination;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::remove_named_destination;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_page_labels;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_output_intents;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_acro_form;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_optional_content;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_page_layout;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_page_mode;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_viewer_preferences;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_open_action;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_language;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_mark_info;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_metadata;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_document_id;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_encryption;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_catalog_entry;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::remove_catalog_entry;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::replace_page;
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_pdf_tests_2::standards::v1_7::subsets::base::io::*;
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::io::text::snapshot;
                    }
                    #[path = "."]
                    pub mod import {
                        pub use semio_s_artifact_stdio_pdf_mut_family_11::standards::v1_7::subsets::base::io::import::*;
                    }
                    #[path = "."]
                    pub mod export {
                        pub use semio_s_artifact_stdio_pdf_testkit::standards::v1_7::subsets::base::io::export::*;
                    }
                }
            }
            #[path = "."]
            pub mod a {
                pub use semio_s_artifact_stdio_pdf_mut_family_12::standards::v1_7::subsets::a::io;
            }
            #[path = "."]
            pub mod x {
                pub use semio_s_artifact_stdio_pdf_mut_family_12::standards::v1_7::subsets::x::io;
            }
            #[path = "."]
            pub mod e {
                pub use semio_s_artifact_stdio_pdf_mut_family_13::standards::v1_7::subsets::e::io;
            }
            #[path = "."]
            pub mod ua {
                pub use semio_s_artifact_stdio_pdf_mut_family_14::standards::v1_7::subsets::ua::io;
            }
            #[path = "."]
            pub mod vt {
                pub use semio_s_artifact_stdio_pdf_schema_5::standards::v1_7::subsets::vt::io;
            }
            #[path = "."]
            pub mod h {
                pub use semio_s_artifact_stdio_pdf_mut_family_15::standards::v1_7::subsets::h::io;
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod page {
        pub use semio_s_artifact_stdio_pdf_tests_2::editor::page::*;
        #[cfg(test)]
        #[path = "../../../../../../../✏️editor/🖼️page/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
    #[path = "."]
    pub mod pdf14a {
        pub use semio_s_artifact_stdio_pdf_schema_5::editor::pdf14a::*;
        #[cfg(test)]
        #[path = "../../../../../../../../../../4️⃣1.4/🪆️subsets/🗄️a/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_pdf_schema_3::editor::pdf14a::modes::edit::*;
                pub use semio_s_artifact_stdio_pdf_schema_3::editor::pdf14a::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf14 {
        pub use semio_s_artifact_stdio_pdf_schema_5::editor::pdf14::*;
        #[cfg(test)]
        #[path = "../../../../../../../../../../4️⃣1.4/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        pub use semio_s_artifact_stdio_pdf_schema_3::editor::pdf14::page;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_pdf_schema_3::editor::pdf14::modes::edit::*;
                pub use semio_s_artifact_stdio_pdf_schema_3::editor::pdf14::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf14x {
        pub use semio_s_artifact_stdio_pdf_schema_5::editor::pdf14x::*;
        #[cfg(test)]
        #[path = "../../../../../../../../../../4️⃣1.4/🪆️subsets/🖨️x/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_pdf_schema_3::editor::pdf14x::modes::edit::*;
                pub use semio_s_artifact_stdio_pdf_schema_3::editor::pdf14x::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17a {
        pub use semio_s_artifact_stdio_pdf_schema_5::editor::pdf17a::*;
        #[cfg(test)]
        #[path = "../../../../../../../../🗄️a/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17a::modes::edit::*;
                pub use semio_s_artifact_stdio_pdf_tests_2::editor::pdf17a::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17 {
        pub use semio_s_artifact_stdio_pdf_schema_5::editor::pdf17::*;
        #[cfg(test)]
        #[path = "../../../../../../../✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_pdf_schema_4::editor::pdf17::modes::edit::*;
                pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17e {
        pub use semio_s_artifact_stdio_pdf_schema_5::editor::pdf17e::*;
        #[cfg(test)]
        #[path = "../../../../../../../../📐️e/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_pdf_schema_4::editor::pdf17e::modes::edit::*;
                pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17e::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17h {
        pub use semio_s_artifact_stdio_pdf_schema_5::editor::pdf17h::*;
        #[cfg(test)]
        #[path = "../../../../../../../../⚕️h/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_pdf_schema_4::editor::pdf17h::modes::edit::*;
                pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17h::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17ua {
        pub use semio_s_artifact_stdio_pdf_schema_5::editor::pdf17ua::*;
        #[cfg(test)]
        #[path = "../../../../../../../../♿️ua/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_pdf_schema_4::editor::pdf17ua::modes::edit::*;
                pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17ua::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17vt {
        #[path = "../../../../../../../../🧾️vt/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_pdf_schema_4::editor::pdf17vt::modes::edit::*;
                pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17vt::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17x {
        pub use semio_s_artifact_stdio_pdf_schema_5::editor::pdf17x::*;
        #[cfg(test)]
        #[path = "../../../../../../../../🖨️x/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_pdf_schema_4::editor::pdf17x::modes::edit::*;
                pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17x::modes::edit::windows;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod pdf14a {
        pub use semio_s_artifact_stdio_pdf_schema_5::viewer::pdf14a::*;
        #[cfg(test)]
        #[path = "../../../../../../../../../../4️⃣1.4/🪆️subsets/🗄️a/👁️viewer/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_tests::viewer::pdf14a::modes::view::*;
                pub use semio_s_artifact_stdio_pdf_tests::viewer::pdf14a::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf14 {
        pub use semio_s_artifact_stdio_pdf_schema_5::viewer::pdf14::*;
        #[cfg(test)]
        #[path = "../../../../../../../../../../4️⃣1.4/🪆️subsets/🧱️base/👁️viewer/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_tests::viewer::pdf14::modes::view::*;
                pub use semio_s_artifact_stdio_pdf_tests::viewer::pdf14::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf14x {
        pub use semio_s_artifact_stdio_pdf_mut_family_15::viewer::pdf14x::*;
        #[cfg(test)]
        #[path = "../../../../../../../../../../4️⃣1.4/🪆️subsets/🖨️x/👁️viewer/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_schema_2::viewer::pdf14x::modes::view::*;
                pub use semio_s_artifact_stdio_pdf_tests::viewer::pdf14x::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17a {
        #[path = "../../../../../../../../🗄️a/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17a::modes::view::*;
                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17a::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17 {
        #[path = "../../../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17::modes::view::*;
                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17e {
        #[path = "../../../../../../../../📐️e/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17e::modes::view::*;
                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17e::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17h {
        #[path = "../../../../../../../../⚕️h/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_graph_projection::viewer::pdf17h::modes::view::*;
                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17h::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17ua {
        #[path = "../../../../../../../../♿️ua/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_graph_projection::viewer::pdf17ua::modes::view::*;
                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17ua::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17x {
        #[path = "../../../../../../../../🖨️x/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_graph_projection::viewer::pdf17x::modes::view::*;
                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17x::modes::view::windows;
            }
        }
    }
}
