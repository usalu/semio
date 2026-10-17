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
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../../🧬️mutations/🔀️move-page/🦀️.rs"]
                        pub mod move_page;
                        #[path = "../../../../../../🧬️mutations/📐️resize-page/🦀️.rs"]
                        pub mod resize_page;
                        #[path = "../../../../../../🧬️mutations/♻️replace-page-text/🦀️.rs"]
                        pub mod replace_page_text;
                    }
                    pub use semio_s_artifact_stdio_pdf_snapshot::standards::v1_4::subsets::base::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_stdio_pdf_schema::standards::v1_4::subsets::base::schema::inferences::*;
                        pub use semio_s_artifact_stdio_pdf_schema::standards::v1_4::subsets::base::schema::inferences::*;
                        #[path = "."]
                        pub mod outline {
                            pub use semio_s_artifact_stdio_pdf_schema::standards::v1_4::subsets::base::schema::inferences::outline::*;
                            #[cfg(test)]
                            //#region 🧪️Tests
                            #[path = "../../../../../🧾outline/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../../🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_stdio_pdf_diff::standards::v1_4::subsets::base::schema::diff::*;
                        pub use semio_s_artifact_stdio_pdf_diff::standards::v1_4::subsets::base::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../../../../🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
            #[path = "."]
            pub mod a {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../../../../🗄️a/🧬️schema/🧬️mutations/📝️set-page-text/🦀️.rs"]
                        pub mod set_page_text;
                    }
                }
            }
            #[path = "."]
            pub mod x {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../../../../🖨️x/🧬️schema/🧬️mutations/📐️set-page-size/🦀️.rs"]
                        pub mod set_page_size;
                    }
                }
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
                pub mod schema {
                    #[path = "."]
                    pub mod snapshot {
                        pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::schema::snapshot::model;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::io::sqlite::snapshot::number;
                            #[path = "../../../../../../../../../../7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧰️reader/🦀️.rs"]
                            pub mod reader;
                            #[path = "../../../../../../../../../../7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🛂️admission/🦀️.rs"]
                            pub mod admission;
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        pub use semio_s_artifact_stdio_pdf_model::examples::demo::*;
        pub use semio_s_artifact_stdio_pdf_model::examples::demo::*;
        #[cfg(test)]
        #[path = "../../../../../../../📚️examples/🎬️demo/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod pdf14a {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../../../../🗄️a/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../../../🗄️a/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf14 {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf14x {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../../../🖨️x/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
