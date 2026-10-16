//! 🧬️ schema of `semio-s-artifact-space-space` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::snapshot::{SSpaceSnapshot};
pub use crate::standards::v1::subsets::any::schema::snapshot::{SpaceArtifactRow};
pub use crate::standards::v1::subsets::any::schema::snapshot::{SpaceArtifactDialect};
pub use crate::standards::v1::subsets::any::schema::artifact_items::{SPACE_INDEX_DIALECT, S_SPACE_INDEX_DOCUMENT_SCHEMA, artifact_kind, space_index_topic_contribution};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/🏷️rename-artifact/🦀️.rs"]
                            pub mod rename_artifact;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/🗑️delete-artifact/🦀️.rs"]
                            pub mod delete_artifact;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/🕒touch-artifact/🦀️.rs"]
                            pub mod touch_artifact;
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/🌱create-artifact/🦀️.rs"]
                            pub mod create_artifact;
                        }
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../🚪️io/📝️text/🧬️mutations/🏷️rename-artifact/🦀️.rs"]
                            pub mod rename_artifact;
                            #[path = "../../../🚪️io/📝️text/🧬️mutations/🗑️delete-artifact/🦀️.rs"]
                            pub mod delete_artifact;
                            #[path = "../../../🚪️io/📝️text/🧬️mutations/🕒touch-artifact/🦀️.rs"]
                            pub mod touch_artifact;
                            #[path = "../../../🚪️io/📝️text/🧬️mutations/🌱create-artifact/🦀️.rs"]
                            pub mod create_artifact;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "../../📸️snapshot/🦀️.rs"]
                    pub mod snapshot;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod space_index {
        #[path = "."]
        pub mod presence {
            #[path = "../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
