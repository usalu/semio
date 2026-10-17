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
                pub mod modules {
                    #[path = "../../../../../🔨️modules/🔤️lexer/🦀️.rs"]
                    pub mod lexer;
                    #[path = "../../../../../🔨️modules/🗜️filters/🦀️.rs"]
                    pub mod filters;
                    #[path = "../../../../../🔨️modules/🔐️encryption/🦀️.rs"]
                    pub mod encryption;
                }
                #[path = "."]
                pub mod schema {
                    #[path = "../../../../🧭️content-mapping/🦀️.rs"]
                    pub mod content_mapping;
                    #[path = "."]
                    pub mod snapshot {
                        pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::*;
                        pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::*;
                        #[cfg(test)]
                        #[path="../../../../📸️snapshot/🧪️tests/♻️retirement/🦀️.rs"]
                        pub mod recursive_retirement_tests;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod outline {
                            #[path = "../../../🧾outline/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "../../../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::error;
                    pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::text_string;
                    #[path="../../../../../🚪️io/📦️foreign-artifacts/🦀️.rs"]
                    pub mod foreign_artifacts;
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::io::text::snapshot::*;
                            #[path = "."]
                            pub mod date {
                                pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::io::text::snapshot::date::*;
                                #[cfg(test)]
                                #[path="../../../../../🚪️io/📝️text/📸️snapshot/📅️date/🧪️tests/🦀️.rs"]
                                pub mod tests;
                            }
                            #[cfg(test)]
                            #[path="../../../../../🚪️io/📝️text/📸️snapshot/🧪️tests/🏭️producer/🦀️.rs"]
                            pub mod producer_tests;
                        }
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
                #[path = "../../../../../../🗄️a/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../🗄️a/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
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
                #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
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
            pub mod view {
                #[path = "../../../../../../📐️e/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../📐️e/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
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
            pub mod view {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../⚕️h/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
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
            pub mod view {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../♿️ua/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
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
            pub mod view {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../🧾️vt/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
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
            pub mod view {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../🖨️x/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
