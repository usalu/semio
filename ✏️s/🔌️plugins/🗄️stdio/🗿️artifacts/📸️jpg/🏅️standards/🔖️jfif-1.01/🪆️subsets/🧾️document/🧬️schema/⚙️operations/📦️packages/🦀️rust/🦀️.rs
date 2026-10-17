//! ⚙️ operations of `semio-s-artifact-stdio-jpg` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_value_derive as value_derive;

pub use crate::editor::jpg_any::edit_rules::artifact_items_3::{ARTIFACT_DEFINITION_SCHEMA};
pub use crate::editor::jpg_any::edit_rules::artifact_items_8::{formats};

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
                    #[path = "../../🦀️.rs"]
                    pub mod operations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../🚪️io/🧮️transforms-and-tables/🦀️.rs"]
                    pub mod transforms_and_tables;
                    #[path = "."]
                    pub mod binary {
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
                        pub mod inferences {
                            #[path = "../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod baseline {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod conformance {
                        #[path="../../../../../🧱️baseline/🧬️schema/🏅️conformance/🧮️facts/🦀️.rs"]
                        pub mod facts;
                    }
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
    pub mod jpg_any {
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
    pub mod jpg_baseline {
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
    pub mod jpg_any {
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
    #[path = "."]
    pub mod jpg_baseline {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../🧱️baseline/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../🧱️baseline/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
