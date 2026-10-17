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
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                pub use semio_s_artifact_stdio_pdf_testkit::standards::v1_7::subsets::base::*;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_schema_4::standards::v1_7::subsets::base::schema::*;
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::schema::stream_roles;
                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::diff;
                    pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_pdf_tests_2::standards::v1_7::subsets::base::io::*;
                    #[path = "."]
                    pub mod sqlite {
                        pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::sqlite::snapshot;
                    }
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts;
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
            pub mod x {
                pub use semio_s_artifact_stdio_pdf_mut_family_12::standards::v1_7::subsets::x::io;
            }
            #[path = "."]
            pub mod ua {
                pub use semio_s_artifact_stdio_pdf_mut_family_14::standards::v1_7::subsets::ua::io;
            }
            #[path = "."]
            pub mod vt {
                #[path = "../../../../🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "../../../🦀️.rs"]
                pub mod schema;
            }
            #[path = "."]
            pub mod h {
                #[path = "../../../../../⚕️h/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
        }
    }
}
#[path = "."]
pub mod editor {
    pub use semio_s_artifact_stdio_pdf_tests_2::editor::page;
    #[path = "."]
    pub mod pdf14a {
        #[path = "../../../../../../../4️⃣1.4/🪆️subsets/🗄️a/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
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
        #[path = "../../../../../../../4️⃣1.4/🪆️subsets/🧱️base/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
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
        #[path = "../../../../../../../4️⃣1.4/🪆️subsets/🖨️x/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
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
        #[path = "../../../../../🗄️a/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
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
        #[path = "../../../../../🧱️base/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
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
        #[path = "../../../../../📐️e/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
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
        #[path = "../../../../../⚕️h/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
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
        #[path = "../../../../../♿️ua/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
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
    pub mod pdf17x {
        #[path = "../../../../../🖨️x/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
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
        #[path = "../../../../../../../4️⃣1.4/🪆️subsets/🗄️a/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
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
        #[path = "../../../../../../../4️⃣1.4/🪆️subsets/🧱️base/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_tests::viewer::pdf14::modes::view::*;
                pub use semio_s_artifact_stdio_pdf_tests::viewer::pdf14::modes::view::windows;
            }
        }
    }
}
