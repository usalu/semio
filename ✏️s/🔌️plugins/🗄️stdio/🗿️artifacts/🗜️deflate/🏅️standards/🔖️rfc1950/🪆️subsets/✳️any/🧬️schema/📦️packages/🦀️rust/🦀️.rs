//! 🧬️ schema of `semio-s-artifact-stdio-deflate` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::DeflateSnapshot;
pub mod schema {
    pub use super::standards::v_rfc1950::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_deflate_examples::{ARTIFACT_DEFINITION_SCHEMA};
pub use semio_s_artifact_stdio_deflate_snapshot::{STDIO_DEFLATE_DOCUMENT_SCHEMA};
pub use crate::standards::v_rfc1950::subsets::any::schema::artifact_items_4::{artifact_kind};
pub use crate::standards::v_rfc1950::subsets::any::schema::artifact_items_5::{native_codec};
pub use crate::standards::v_rfc1950::subsets::any::schema::artifact_items_6::{native_codecs};
pub use crate::standards::v_rfc1950::subsets::any::schema::artifact_items_7::{definition};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_rfc1950 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_deflate_snapshot::standards::v_rfc1950::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_deflate_mutations::standards::v_rfc1950::subsets::any::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod zlib {
                        pub use semio_s_artifact_stdio_deflate_zlib::standards::v_rfc1950::subsets::any::io::zlib::*;
                        #[path = "."]
                        pub mod publication {
                            pub use semio_s_artifact_stdio_deflate_examples::standards::v_rfc1950::subsets::any::io::zlib::publication::*;
                            #[cfg(test)]
                            #[path = "../../../🚪️io/📤️publication/🧪️tests/🔬️unit/🦀️.rs"]
                            mod tests;
                        }
                        #[cfg(test)]
                        #[path = "../../../🚪️io/🧪️tests/🔬️codec/🦀️.rs"]
                        mod codec_tests;
                    }
                    #[path = "../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "../../../🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                }
            }
        }
    }
}
