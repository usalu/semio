//! 📸️ snapshot of `semio-s-artifact-stdio-svg` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::SvgSnapshot;
pub mod schema {
    pub use super::standards::v1_1::subsets::base::schema::*;
}
pub use crate::standards::v1_1::subsets::tiny::schema::conformance::artifact_items::{STDIO_SVG_DOCUMENT_SCHEMA};
pub use crate::standards::v1_1::subsets::tiny::schema::conformance::artifact_items_2::{SVG_ARTIFACT_SCHEMA_ID};
pub use crate::standards::v1_1::subsets::tiny::schema::conformance::artifact_items_3::{ARTIFACT_DEFINITION_SCHEMA};
pub use crate::standards::v1_1::subsets::tiny::schema::conformance::artifact_items_4::{artifact_kind};
pub use crate::standards::v1_1::subsets::tiny::schema::conformance::artifact_items_8::{formats};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "../../../🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "../../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                }
            }
            #[path = "."]
            pub mod tiny {
                #[path = "."]
                pub mod schema {
                    #[path="../../../../../🔬️tiny/🧬️schema/🏅️conformance/🦀️.rs"]
                    pub mod conformance;
                }
            }
            #[path = "."]
            pub mod basic {
                #[path = "."]
                pub mod schema {
                    #[path="../../../../../🔰️basic/🧬️schema/🏅️conformance/🦀️.rs"]
                    pub mod conformance;
                }
            }
        }
    }
}
