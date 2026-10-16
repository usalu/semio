//! 🧬️ mutations of `semio-s-artifact-stdio-bcf` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::BcfSnapshot;
pub mod schema {
    pub use super::standards::v2_1::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_bcf_inferences::{ARTIFACT_DEFINITION_SCHEMA, STDIO_BCF_DOCUMENT_SCHEMA, artifact_kind};
pub use crate::editor::bcf::edit_rules::artifact_items_5::{native_codec};
pub use crate::editor::bcf::edit_rules::artifact_items_6::{native_codecs};
pub use crate::editor::bcf::edit_rules::artifact_items_7::{definition};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v2_1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_bcf_inferences::standards::v2_1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_bcf_inferences::standards::v2_1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        pub use semio_s_artifact_stdio_bcf_inferences::standards::v2_1::subsets::any::io::binary::diff;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        pub use semio_s_artifact_stdio_bcf_inferences::standards::v2_1::subsets::any::io::text::diff;
                    }
                    #[path = "../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "../../../../📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bcf {
        #[path = "../../../../✏️editor/🧭️edit-rules/🦀️.rs"]
        pub mod edit_rules;
        #[path = "."]
        pub mod presence {
            #[path = "../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod bcf {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
