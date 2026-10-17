//! 🧬️ schema-2 of `semio-s-artifact-stdio-pdf` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

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
                    pub use semio_s_artifact_stdio_pdf_schema::standards::v1_4::subsets::base::schema::*;
                    pub use semio_s_artifact_stdio_pdf_snapshot::standards::v1_4::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_4::subsets::base::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_base::standards::v1_4::subsets::base::schema::mutations::*;
                        pub use semio_s_artifact_stdio_pdf_mut_base::standards::v1_4::subsets::base::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../../../🧱️base/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../🧱️base/🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "../../../../🧱️base/🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                }
            }
            #[path = "."]
            pub mod a {
                #[path = "../../🦀️.rs"]
                pub mod schema;
            }
            #[path = "."]
            pub mod x {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mutations_2::standards::v1_4::subsets::x::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../../../🖨️x/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
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
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::schema::stream_roles;
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
                            pub use semio_s_artifact_stdio_pdf_tests::standards::v1_7::subsets::base::io::sqlite::snapshot::reader;
                            #[path = "../../../../../../7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🪪️stream-roles/🦀️.rs"]
                            pub mod stream_roles;
                            #[path = "../../../../../../7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/📦️artifact-reference/🦀️.rs"]
                            pub mod artifact_reference;
                            #[path = "../../../../../../7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧩️cos/🦀️.rs"]
                            pub mod cos;
                            #[path = "../../../../../../7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🌈️color/🦀️.rs"]
                            pub mod color;
                            #[path = "../../../../../../7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🔤️font/🦀️.rs"]
                            pub mod font;
                            #[path = "../../../../../../7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🖋️content/🦀️.rs"]
                            pub mod content;
                            #[path = "../../../../../../7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🖼️resource/🦀️.rs"]
                            pub mod resource;
                            #[path = "../../../../../../7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/📇️metadata/🦀️.rs"]
                            pub mod metadata;
                            #[path = "../../../../../../7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🎯️navigation/🦀️.rs"]
                            pub mod navigation;
                            #[path = "../../../../../../7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/📌️annotation/🦀️.rs"]
                            pub mod annotation;
                            #[path = "../../../../../../7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/📝️form/🦀️.rs"]
                            pub mod form;
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
    pub mod pdf14 {
        #[path = "."]
        pub mod page {
            pub use semio_s_artifact_stdio_pdf_schema_3::editor::pdf14::page::*;
            #[cfg(test)]
            #[path = "../../../../🧱️base/✏️editor/🖼️page/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod pdf14x {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../🖨️x/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_pdf_tests::viewer::pdf14x::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_pdf_tests::viewer::pdf14x::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../🖨️x/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
