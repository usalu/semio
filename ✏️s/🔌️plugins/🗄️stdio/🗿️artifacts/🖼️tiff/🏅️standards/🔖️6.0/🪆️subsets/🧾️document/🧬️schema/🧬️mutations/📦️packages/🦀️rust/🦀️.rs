//! 🧬️ mutations of `semio-s-artifact-stdio-tiff` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::TiffSnapshot;
#[cfg(test)]
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub mod schema {
    pub use super::standards::v6_0::subsets::document::schema::*;
}

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v6_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod document {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_tiff_schema::standards::v6_0::subsets::document::schema::*;
                    pub use semio_s_artifact_stdio_tiff_snapshot::standards::v6_0::subsets::document::schema::snapshot;
                    pub use semio_s_artifact_stdio_tiff_schema::standards::v6_0::subsets::document::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🦀️.rs"]
                        mod top_level;
                        pub use top_level::*;
                        #[path = "../../🧮️replace-samples/🦀️.rs"]
                        pub mod replace_samples;
                        #[path = "../../📥️insert-ifd/🦀️.rs"]
                        pub mod insert_ifd;
                        #[path = "../../📤️remove-ifd/🦀️.rs"]
                        pub mod remove_ifd;
                        #[path = "../../🗑️remove-tag/🦀️.rs"]
                        pub mod remove_tag;
                        #[path = "../../🏷️replace-tag/🦀️.rs"]
                        pub mod replace_tag;
                        #[path = "../../🎨️paint-region/🦀️.rs"]
                        pub mod paint_region;

                    #[cfg(test)]
                    #[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
                    mod leaf_tests;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        pub use semio_s_artifact_stdio_tiff_schema::standards::v6_0::subsets::document::io::binary::diff;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        pub use semio_s_artifact_stdio_tiff_schema::standards::v6_0::subsets::document::io::text::diff;
                    }
                }
            }
            #[path = "."]
            pub mod baseline {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_tiff_schema_2::standards::v6_0::subsets::baseline::schema::*;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_tiff_schema_2::standards::v6_0::subsets::baseline::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../../../../🧱️baseline/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
