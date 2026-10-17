//! 🧬️ schema of `semio-s-artifact-stdio-tiff` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::TiffSnapshot;
pub mod schema {
    pub use super::standards::v6_0::subsets::document::schema::*;
}
pub use semio_s_artifact_stdio_tiff_snapshot::{STDIO_TIFF_DOCUMENT_SCHEMA};

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
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_tiff_snapshot::standards::v6_0::subsets::document::schema::snapshot;
                    #[path = "."]
                    pub mod diff {
                        #[path = "../../🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod diff {
                            #[path = "../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod diff {
                            #[path = "../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                            mod component;
                            pub use component::*;
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
    pub mod tiff_any {
        #[path = "."]
        pub mod config {
            #[path = "../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod tiff_baseline {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../🧱️baseline/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../🧱️baseline/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
