//! 🧬️ schema-2 of `semio-s-artifact-stdio-zip` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::ZipSnapshot;
pub mod schema {
    pub use super::standards::v2_0::subsets::base::schema::*;
}
pub use semio_s_artifact_stdio_zip_inferences::{STDIO_ZIP_DOCUMENT_SCHEMA};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v2_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_zip_inferences::standards::v2_0::subsets::base::schema::snapshot;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod zip {
        #[path = "."]
        pub mod iso21320 {
            #[path = "."]
            pub mod modes {
                #[path = "."]
                pub mod view {
                    pub use semio_s_artifact_stdio_zip_inferences::viewer::zip::iso21320::modes::view::*;
                    #[cfg(test)]
                    #[path = "../../../../🌐️iso21320/👁️viewer/🎭️modes/👁️view/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                    pub use semio_s_artifact_stdio_zip_inferences::viewer::zip::iso21320::modes::view::windows;
                }
            }
        }
    }
}
