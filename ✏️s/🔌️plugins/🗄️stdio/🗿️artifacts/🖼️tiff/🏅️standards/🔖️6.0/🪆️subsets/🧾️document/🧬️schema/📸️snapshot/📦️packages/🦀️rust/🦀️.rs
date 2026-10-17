//! 📸️ snapshot of `semio-s-artifact-stdio-tiff` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub mod schema {
    pub use super::standards::v6_0::subsets::document::schema::*;
}
pub use crate::editor::tiff_any::edit_rules::artifact_items::{STDIO_TIFF_DOCUMENT_SCHEMA};
pub use crate::editor::tiff_any::edit_rules::artifact_items_2::{TIFF_ARTIFACT_SCHEMA_ID};
pub use crate::editor::tiff_any::edit_rules::artifact_items_3::{ARTIFACT_DEFINITION_SCHEMA};
pub use crate::editor::tiff_any::edit_rules::artifact_items_4::{artifact_kind};
pub use crate::editor::tiff_any::edit_rules::artifact_items_8::{formats};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v6_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod document {
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
                pub mod schema {
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "../../../⚙️operations/🦀️.rs"]
                    pub mod operations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod snapshot {
                            #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod inferences {
                            #[path = "../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod snapshot {
                            #[path = "../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod inferences {
                            #[path = "../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "."]
                        pub mod snapshot {
                            #[path = "../../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path="../../../../🚪️io/🗂️ordering/🦀️.rs"] pub mod controlled_ordering;
                    #[path="../../../../🚪️io/💾️binary/📸️snapshot/👁️observations/🦀️.rs"]
                    pub mod native_observations;
                }
            }
            #[path = "."]
            pub mod baseline {
                #[path = "."]
                pub mod schema {
                    #[path="../../../../../🧱️baseline/🧬️schema/🏅️conformance/🦀️.rs"]
                    pub mod conformance;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod tiff_any {
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
    #[path = "."]
    pub mod tiff_baseline {
        #[path = "../../../../../🧱️baseline/✏️editor/🧭️edit-rules/🦀️.rs"]
        pub mod edit_rules;
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../🧱️baseline/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../🧱️baseline/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod tiff_any {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
