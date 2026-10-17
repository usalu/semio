//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_7 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    #[path = "../../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::schema::graph_source;
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::schema::stream_roles;
                    pub use semio_s_artifact_stdio_pdf_graph_projection::standards::v1_7::subsets::base::schema::graph_projection;
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::schema::content_mapping;
                    #[path = "."]
                    pub mod conformance_support {
                        pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::schema::conformance_support::*;
                        #[cfg(test)]
                        #[path = "../../../🏅️conformance-support/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[cfg(any(test, feature = "testkit"))]
                    pub use semio_s_artifact_stdio_pdf_testkit::standards::v1_7::subsets::base::schema::testkit;
                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::diff::*;
                        pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::mutations::*;
                        #[path = "."]
                        pub mod insert_page {
                            pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::mutations::insert_page::*;
                            #[cfg(test)]
                            #[path = "../../../🧬️mutations/📥️insert-page/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_page {
                            pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::mutations::remove_page::*;
                            #[cfg(test)]
                            #[path = "../../../🧬️mutations/🗑️remove-page/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_page_media_box {
                            pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::schema::mutations::set_page_media_box::*;
                            #[cfg(test)]
                            #[path = "../../../🧬️mutations/📐️set-page-media-box/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_page_crop_box {
                            pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::schema::mutations::set_page_crop_box::*;
                            #[cfg(test)]
                            #[path = "../../../🧬️mutations/✂️set-page-crop-box/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts;
                    #[path = "."]
                    pub mod export {
                        pub use semio_s_artifact_stdio_pdf_testkit::standards::v1_7::subsets::base::io::export::*;
                    }
                }
            }
        }
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
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_pdf_tests_2::editor::pdf17a::modes::edit::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_pdf_tests_2::editor::pdf17a::modes::edit::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../🗄️a/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17 {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17::modes::edit::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17::modes::edit::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
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
                #[path = "../../../../../📐️e/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17e::modes::edit::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17e::modes::edit::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../📐️e/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
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
                #[path = "../../../../../⚕️h/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17h::modes::edit::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17h::modes::edit::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../⚕️h/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
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
                #[path = "../../../../../♿️ua/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17ua::modes::edit::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17ua::modes::edit::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../♿️ua/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
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
                #[path = "../../../../../🧾️vt/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17vt::modes::edit::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17vt::modes::edit::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../🧾️vt/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
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
                #[path = "../../../../../🖨️x/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17x::modes::edit::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17x::modes::edit::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../🖨️x/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod pdf17a {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17a::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17a::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../🗄️a/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17 {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
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
            pub mod view {
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17e::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17e::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../📐️e/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
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
            pub mod view {
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17h::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17h::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../⚕️h/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
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
            pub mod view {
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17ua::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17ua::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../♿️ua/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
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
            pub mod view {
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17vt::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17vt::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../🧾️vt/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
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
            pub mod view {
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17x::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17x::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../🖨️x/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
