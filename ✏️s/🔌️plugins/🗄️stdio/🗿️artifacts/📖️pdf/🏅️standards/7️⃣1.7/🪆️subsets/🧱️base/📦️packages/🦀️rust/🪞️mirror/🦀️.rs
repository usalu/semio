//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_4 {
        #[path = "../../../../../../4️⃣1.4/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_pdf_graph_projection::standards::v1_4::subsets::base::io::*;
                }
            }
            #[path = "."]
            pub mod a {
                pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_4::subsets::a::io;
            }
            #[path = "."]
            pub mod x {
                pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_4::subsets::x::io;
            }
        }
    }
    #[path = "."]
    pub mod v1_7 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod modules {
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::lexer;
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::filters;
                    pub use semio_s_artifact_stdio_pdf_graph_projection::standards::v1_7::subsets::base::modules::xref;
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::encryption;
                    pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::modules::colour;
                    #[path = "../../../🔨️modules/🖼️images/🦀️.rs"]
                    pub mod images;
                    #[path = "."]
                    pub mod writer {
                        pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::modules::writer::*;
                        #[cfg(test)]
                        #[path = "../../../🔨️modules/📝️writer/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::schema::graph_source;
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::schema::stream_roles;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::mutations::remove_page;
                        pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::schema::mutations::set_page_media_box;
                        pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::schema::mutations::set_page_crop_box;
                        #[path = "../../../🧬️schema/🧬️mutations/➕️append-page-content/🦀️.rs"]
                        pub mod append_page_content;
                        pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::schema::mutations::insert_object;
                        pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::schema::mutations::remove_object;
                        #[path = "../../../🧬️schema/🧬️mutations/🔧️set-object-value/🦀️.rs"]
                        pub mod set_object_value;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::move_page;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_page_rotation;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_page_user_unit;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::remove_content;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_annotation;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_image;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_form;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::remove_ext_g_state;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::remove_shading;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_pattern;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::remove_pattern;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_color_space;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_named_destination;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_viewer_preferences;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_open_action;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_language;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_mark_info;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_catalog_entry;
                    }
                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::diff;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::error;
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts;
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/🌈️set-color-space/🦀️.rs"]
                            pub mod set_color_space;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/📐️set-page-media-box/🦀️.rs"]
                            pub mod set_page_media_box;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/🧩️set-pattern/🦀️.rs"]
                            pub mod set_pattern;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/🌄️remove-shading/🦀️.rs"]
                            pub mod remove_shading;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/🔄️set-page-rotation/🦀️.rs"]
                            pub mod set_page_rotation;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/🔀️move-page/🦀️.rs"]
                            pub mod move_page;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/🚪️set-open-action/🦀️.rs"]
                            pub mod set_open_action;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/🪡️remove-pattern/🦀️.rs"]
                            pub mod remove_pattern;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/🗑️remove-page/🦀️.rs"]
                            pub mod remove_page;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/📦️insert-object/🦀️.rs"]
                            pub mod insert_object;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/🛠️set-viewer-preferences/🦀️.rs"]
                            pub mod set_viewer_preferences;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/📏️set-page-user-unit/🦀️.rs"]
                            pub mod set_page_user_unit;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/🗣️set-language/🦀️.rs"]
                            pub mod set_language;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/✂️set-page-crop-box/🦀️.rs"]
                            pub mod set_page_crop_box;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/🏞️set-image/🦀️.rs"]
                            pub mod set_image;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/🎯️set-named-destination/🦀️.rs"]
                            pub mod set_named_destination;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/📄️set-form/🦀️.rs"]
                            pub mod set_form;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/🗂️set-catalog-entry/🦀️.rs"]
                            pub mod set_catalog_entry;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/🔏️set-mark-info/🦀️.rs"]
                            pub mod set_mark_info;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/🎚️remove-ext-g-state/🦀️.rs"]
                            pub mod remove_ext_g_state;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/📝️set-annotation/🦀️.rs"]
                            pub mod set_annotation;
                        }
                    }
                }
            }
        }
    }
}
