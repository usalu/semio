//! 📸️ snapshot of `semio-s-artifact-stdio-md` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::MdSnapshot;
pub mod schema {
    pub use super::standards::v_commonmark::subsets::any::schema::*;
}
pub use crate::standards::v_commonmark::subsets::any::io::import::deserializers::artifact_items::{STDIO_MD_DOCUMENT_SCHEMA};
pub use crate::standards::v_commonmark::subsets::any::io::import::deserializers::artifact_items_2::{MD_ARTIFACT_SCHEMA_ID};
pub use crate::standards::v_commonmark::subsets::any::io::import::deserializers::artifact_items_3::{ARTIFACT_DEFINITION_SCHEMA};
pub use crate::standards::v_commonmark::subsets::any::io::import::deserializers::artifact_items_8::{formats};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_commonmark {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../🦀️.rs"]
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
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "../../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "../../../../🚪️io/📥️import/🧩️deserializers/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "../../../../🚪️io/📤️export/🧵️serializers/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }
    }
}
