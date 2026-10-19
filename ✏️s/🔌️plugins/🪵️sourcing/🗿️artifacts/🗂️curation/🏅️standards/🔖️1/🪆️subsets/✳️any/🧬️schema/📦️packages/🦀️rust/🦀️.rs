//! 🧬️ schema of `semio-s-artifact-sourcing-curation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::schema::snapshot::CurationSnapshot;
pub use crate::schema::mutations::SourcingMutation;
#[cfg(test)]
pub use crate::schema::diff::CurationDiff;
pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
#[cfg(test)]
pub mod diff {
            pub use crate::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::diff::*;
            }

        }
#[cfg(test)]
pub mod mutations {
            pub use crate::standards::v1::subsets::any::schema::mutations::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::mutations::*;
            }

        }
pub use semio_s_artifact_sourcing_curation_examples::{CuratedItem, Filters};
pub use semio_s_artifact_sourcing_curation_domain::{GeometryRecipe, ObjectKind, ObjectKindExtra, catalog_child_handle};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::schema::domain;
                    pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs"]
                        pub mod structural_correspondence_tests;
                        #[path = "."]
                        pub mod create_curated_item {
                            pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::create_curated_item::*;
                            #[cfg(test)]
                            #[path = "../../🧬️mutations/🌱create-curated-item/🧪️tests/🧲️appends/🦀️.rs"]
                            pub mod tests_appends_a_steel_plate_to_the_curation;
                        }
                        #[path = "."]
                        pub mod delete_curated_item {
                            pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::delete_curated_item::*;
                            #[cfg(test)]
                            #[path = "../../🧬️mutations/🗑️delete-curated-item/🧪️tests/🚫️removes/🦀️.rs"]
                            pub mod tests_removes_the_clt_panel_from_the_curation;
                        }
                        #[path = "."]
                        pub mod change_curated_item_count {
                            pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::change_curated_item_count::*;
                            #[cfg(test)]
                            #[path = "../../🧬️mutations/🔢change-curated-item/🧪️tests/🔢️raises/🦀️.rs"]
                            pub mod tests_raises_the_glulam_beam_count_to_20;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "../../../🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod sourcing {
        pub use semio_s_artifact_sourcing_curation_operations::editor::sourcing::editor_vocabulary;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_sourcing_curation_examples::editor::sourcing::config::*;
        }
        #[path = "."]
        pub mod commands {
            #[path = "../../../✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs"]
            pub mod set_active_example;
        }
    }
}
