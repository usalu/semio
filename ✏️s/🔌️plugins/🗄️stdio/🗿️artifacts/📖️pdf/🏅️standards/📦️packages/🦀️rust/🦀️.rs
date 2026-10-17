//! 🏅️ mut-base-2 of `semio-s-artifact-stdio-pdf` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

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
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_4::subsets::base::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_base::standards::v1_4::subsets::base::schema::mutations::*;
                        #[path = "."]
                        pub mod move_page {
                            pub use semio_s_artifact_stdio_pdf_tests::standards::v1_4::subsets::base::schema::mutations::move_page::*;
                            #[cfg(test)]
                            #[path = "../../4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🔀️move-page/🧪️tests/🔄️round/🦀️.rs"]
                            pub mod tests_round_trips_the_concrete_inverse;
                            #[cfg(test)]
                            #[path = "../../4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🔀️move-page/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod resize_page {
                            pub use semio_s_artifact_stdio_pdf_tests::standards::v1_4::subsets::base::schema::mutations::resize_page::*;
                            #[cfg(test)]
                            #[path = "../../4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📐️resize-page/🧪️tests/🔄️round/🦀️.rs"]
                            pub mod tests_round_trips_the_concrete_inverse;
                            #[cfg(test)]
                            #[path = "../../4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📐️resize-page/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod replace_page_text {
                            pub use semio_s_artifact_stdio_pdf_tests::standards::v1_4::subsets::base::schema::mutations::replace_page_text::*;
                            #[cfg(test)]
                            #[path = "../../4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/♻️replace-page-text/🧪️tests/🔄️round/🦀️.rs"]
                            pub mod tests_round_trips_the_concrete_inverse;
                            #[cfg(test)]
                            #[path = "../../4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/♻️replace-page-text/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_page {
                            pub use semio_s_artifact_stdio_pdf_mut_base::standards::v1_4::subsets::base::schema::mutations::remove_page::*;
                            #[cfg(test)]
                            #[path = "../../4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🗑️remove-page/🧪️tests/🔄️round/🦀️.rs"]
                            pub mod tests_round_trips_the_concrete_inverse;
                        }
                        #[path = "."]
                        pub mod insert_page {
                            pub use semio_s_artifact_stdio_pdf_mut_base::standards::v1_4::subsets::base::schema::mutations::insert_page::*;
                            #[cfg(test)]
                            #[path = "../../4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📥️insert-page/🧪️tests/🔄️round/🦀️.rs"]
                            pub mod tests_round_trips_the_concrete_inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_pdf_graph_projection::standards::v1_4::subsets::base::io::*;
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_stdio_pdf_mut_base::standards::v1_4::subsets::base::io::binary::mutations;
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
                pub mod modules {
                    #[path = "."]
                    pub mod fonts {
                        pub use semio_s_artifact_stdio_pdf_fonts::standards::v1_7::subsets::base::modules::fonts::*;
                        #[cfg(test)]
                        #[path = "../../7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔤️fonts/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::schema::stream_roles;
                    pub use semio_s_artifact_stdio_pdf_graph_projection::standards::v1_7::subsets::base::schema::graph_projection;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📥️insert-page/🦀️.rs"]
                        pub mod insert_page;
                        #[path = "../../7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🗑️remove-page/🦀️.rs"]
                        pub mod remove_page;
                    }
                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                    #[path = "."]
                    pub mod diff {
                        #[path = "../../7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../7️⃣1.7/🪆️subsets/🧱️base/🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::error;
                    pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::text_string;
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts;
                }
            }
        }
    }
}
