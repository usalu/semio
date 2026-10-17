//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_7 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "../../../../../🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod modules {
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::lexer;
                    pub use semio_s_artifact_stdio_pdf_graph_projection::standards::v1_7::subsets::base::modules::xref;
                    #[path = "."]
                    pub mod fonts {
                        pub use semio_s_artifact_stdio_pdf_fonts::standards::v1_7::subsets::base::modules::fonts::*;
                        pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::modules::fonts::truetype;
                    }
                    pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::base::modules::content;
                    #[path = "."]
                    pub mod colour {
                        pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::modules::colour::*;
                        #[cfg(test)]
                        #[path = "../../../../../🔨️modules/🎨️colour/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod images {
                        pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::modules::images::*;
                        #[cfg(test)]
                        #[path = "../../../../../🔨️modules/🖼️images/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    pub use semio_s_artifact_stdio_pdf_lift::standards::v1_7::subsets::base::modules::lift;
                    pub use semio_s_artifact_stdio_pdf_lower::standards::v1_7::subsets::base::modules::lower;
                    pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::modules::writer;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::schema::graph_source;
                    #[path = "."]
                    pub mod stream_roles {
                        pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::schema::stream_roles::*;
                        #[cfg(test)]
                        #[path = "../../../../🪪️stream-roles/🧪️tests/🦀️.rs"]
                        pub mod tests;
                    }
                    pub use semio_s_artifact_stdio_pdf_graph_projection::standards::v1_7::subsets::base::schema::graph_projection;
                    pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::schema::conformance_support;
                    #[cfg(any(test, feature = "testkit"))]
                    #[path = "../../../🦀️.rs"]
                    pub mod testkit;
                    #[path = "."]
                    pub mod snapshot {
                        pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::*;
                        pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::schema::snapshot::model;
                    }
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::schema::inferences;
                    pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::mutations::*;
                        pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::mutations::*;
                        pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::schema::mutations::set_object_value;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_shading;
                        #[cfg(test)]
                        #[path = "../../../../🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_pdf_tests_2::standards::v1_7::subsets::base::io::*;
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::io::binary::mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::io::text::snapshot;
                        pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::io::text::mutations;
                    }
                    pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::error;
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts;
                    #[path = "."]
                    pub mod import {
                        pub use semio_s_artifact_stdio_pdf_mut_family_11::standards::v1_7::subsets::base::io::import::*;
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "../../../../../🚪️io/📤️export/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod binary {
                                    #[path = "."]
                                    pub mod v_raw {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod deflate {
                                    #[path = "."]
                                    pub mod v_rfc1950 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
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
            pub mod ua {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_family_9::standards::v1_7::subsets::ua::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../../../../../♿️ua/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../♿️ua/🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "../../../../../../♿️ua/🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                }
            }
            #[path = "."]
            pub mod vt {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_mut_family_10::standards::v1_7::subsets::vt::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod mutations {
                            pub use semio_s_artifact_stdio_pdf_mut_family_10::standards::v1_7::subsets::vt::io::text::mutations::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🧾️vt/🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                    #[path = "../../../../../../🧾️vt/🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod bachelor_thesis {
        #[path = "../../../../../../../../4️⃣1.4/🪆️subsets/🧱️base/📚️examples/🎓️bachelor-thesis/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "../../../../../../../../4️⃣1.4/🪆️subsets/🧱️base/📚️examples/🎓️bachelor-thesis/🧪️tests/🧩️example/🦀️.rs"]
        pub mod bachelor_thesis_tests;
    }
}
#[path = "."]
pub mod editor {
    pub use semio_s_artifact_stdio_pdf_tests_2::editor::page;
    #[path = "."]
    pub mod pdf17a {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../../../🗄️a/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                pub use semio_s_artifact_stdio_pdf_tests_2::editor::pdf17a::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17 {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17e {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../📐️e/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17h {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../⚕️h/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17ua {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../♿️ua/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17vt {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../🧾️vt/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17x {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../🖨️x/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
