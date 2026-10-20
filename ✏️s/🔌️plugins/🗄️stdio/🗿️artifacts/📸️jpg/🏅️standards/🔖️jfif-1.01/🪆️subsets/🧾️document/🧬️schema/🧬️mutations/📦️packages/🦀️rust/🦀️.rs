//! 🧬️ mutations of `semio-s-artifact-stdio-jpg` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::JpgSnapshot;
pub use schema::snapshot::JpgImage;
pub use schema::diff::JpgDiff;
pub use schema::mutations::JpgMutation;
pub mod schema {
    pub use super::standards::v_jfif_1_01::subsets::document::schema::*;
}
pub use semio_s_artifact_stdio_jpg_inferences::{STDIO_JPG_DOCUMENT_SCHEMA};
pub use crate::standards::v_jfif_1_01::subsets::document::schema::diff::artifact_items_2::{JPG_ARTIFACT_SCHEMA_ID};
pub use crate::standards::v_jfif_1_01::subsets::document::schema::diff::artifact_items_4::{artifact_kind};
pub use crate::standards::v_jfif_1_01::subsets::document::schema::diff::artifact_items_5::{native_codec};
pub use crate::standards::v_jfif_1_01::subsets::document::schema::diff::artifact_items_6::{native_codecs};
pub use crate::standards::v_jfif_1_01::subsets::document::schema::diff::artifact_items_9::{JPG_ANY_DIALECT, JPG_BASELINE_DIALECT};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_jfif_1_01 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod document {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_jpg_schema::standards::v_jfif_1_01::subsets::document::schema::*;
                    pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::schema::snapshot;
                    #[path = "."]
                    pub mod diff {
                        #[path = "../../../🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🦀️.rs"]
                        mod top_level;
                        pub use top_level::*;
                        #[path = "../../🪪️change-jfif-header/🦀️.rs"]
                        pub mod change_jfif_header;
                        #[path = "../../📥️insert-other-segment/🦀️.rs"]
                        pub mod insert_other_segment;
                        #[path = "../../🗑️remove-other-segment/🦀️.rs"]
                        pub mod remove_other_segment;
                        #[path = "../../🔲️replace-pixels/🦀️.rs"]
                        pub mod replace_pixels;
                        #[path = "../../🖼️replace-image/🦀️.rs"]
                        pub mod replace_image;
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::*;
                    #[path = "."]
                    pub mod facets {
                        #[cfg(test)]
                        #[path = "../../../../🚪️io/🎹️facets/🧪️tests/🔬️conformance/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::binary::*;
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::*;
                        }
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod diff {
                            #[path = "../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::text::*;
                        pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::text::snapshot;
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod diff {
                            #[path = "../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }
    }
}
