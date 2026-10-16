//! 🧬️ mutations of `semio-s-artifact-stdio-png` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::PngSnapshot;
pub use schema::diff::PngDiff;
pub use schema::mutations::PngMutation;
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use semio_s_artifact_stdio_contract::apply_mutation_checked;
pub use semio_s_artifact_stdio_contract::mutation_from_payload_json;
pub use semio_s_artifact_stdio_contract::mutation_inverse;
pub use semio_s_artifact_stdio_contract::mutation_payload_json;
pub use semio_s_artifact_stdio_contract::MutationRefusal;
pub mod schema {
    pub use super::standards::v1_2::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_png_snapshot::{STDIO_PNG_DOCUMENT_SCHEMA};
pub use crate::standards::v1_2::subsets::any::io::artifact_items_5::{native_codec};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_2 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_png_schema::standards::v1_2::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_png_snapshot::standards::v1_2::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_png_diff::standards::v1_2::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🦀️.rs"]
                        mod top_level;
                        pub use top_level::*;
                        #[path = "../../🌗️change-gamma/🦀️.rs"]
                        pub mod change_gamma;
                        #[path = "../../🩹️patch-pixels/🦀️.rs"]
                        pub mod patch_pixels;
                        #[path = "../../🎨️paint-native-samples/🦀️.rs"]
                        pub mod paint_native_samples;
                        #[path = "../../🔄️replace-image/🦀️.rs"]
                        pub mod replace_image;
                        #[path = "../../🎚️set-gamma/🦀️.rs"]
                        pub mod set_gamma;
                        #[path = "../../🧩️replace-samples/🦀️.rs"]
                        pub mod replace_samples;
                    }
                    pub use semio_s_artifact_stdio_png_diff::standards::v1_2::subsets::any::schema::operations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_png_diff::standards::v1_2::subsets::any::io::sqlite;
                }
            }
        }
    }
}
