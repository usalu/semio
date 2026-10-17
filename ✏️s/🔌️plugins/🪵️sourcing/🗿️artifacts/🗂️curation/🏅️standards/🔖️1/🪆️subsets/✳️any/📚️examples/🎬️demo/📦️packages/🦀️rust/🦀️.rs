//! 🎬️ examples of `semio-s-artifact-sourcing-curation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::editor::sourcing::config::artifact_items_5::{CuratedItem, Filters, SortDirection, TableSort};

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
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🗑️delete-curated-item/🦀️.rs"]
                            pub mod delete_curated_item;
                            #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🌱create-curated-item/🦀️.rs"]
                            pub mod create_curated_item;
                            #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🔢change-curated-item/🦀️.rs"]
                            pub mod change_curated_item_count;
                        }
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../🚪️io/📝️text/🧬️mutations/🗑️delete-curated-item/🦀️.rs"]
                            pub mod delete_curated_item;
                            #[path = "../../../../🚪️io/📝️text/🧬️mutations/🌱create-curated-item/🦀️.rs"]
                            pub mod create_curated_item;
                            #[path = "../../../../🚪️io/📝️text/🧬️mutations/🔢change-curated-item/🦀️.rs"]
                            pub mod change_curated_item_count;
                        }
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
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod sourcing {
        pub use semio_s_artifact_sourcing_curation_tests::editor::sourcing::*;
        #[path = "../../../../✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
        pub mod demo_session;
        #[cfg(test)]
        #[path = "../../../../✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
        pub mod example;
        #[path = "."]
        pub mod config {
            #[path = "../../../../✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
