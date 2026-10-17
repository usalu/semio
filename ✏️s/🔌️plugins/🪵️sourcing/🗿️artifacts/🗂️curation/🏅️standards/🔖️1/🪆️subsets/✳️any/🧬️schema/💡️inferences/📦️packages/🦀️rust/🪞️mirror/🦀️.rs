//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

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
                    pub use semio_s_artifact_sourcing_curation_schema::standards::v1::subsets::any::schema::*;
                    #[path = "."]
                    pub mod snapshot {
                        pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::schema::snapshot::*;
                        pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::schema::snapshot::*;
                        #[cfg(test)]
                        #[path = "../../../../🧪️tests/🪪️document-contract/🦀️.rs"]
                        pub mod document_contract_tests;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod entries {
                            #[path = "../../../🗃️entries/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod operations {
                        pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::operations::*;
                        #[cfg(test)]
                        #[path = "../../../../⚙️operations/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod create_curated_item {
                            pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::create_curated_item::*;
                            pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::create_curated_item::diff;
                            pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::create_curated_item::inverse;
                        }
                        #[path = "."]
                        pub mod delete_curated_item {
                            pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::delete_curated_item::*;
                            pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::delete_curated_item::diff;
                            pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::delete_curated_item::inverse;
                        }
                        #[path = "."]
                        pub mod change_curated_item_count {
                            pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::change_curated_item_count::*;
                            pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::change_curated_item_count::diff;
                            pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::change_curated_item_count::inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::io::text::snapshot;
                    }
                    pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::io::snapshot;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod sourcing {
        pub use semio_s_artifact_sourcing_curation_operations::editor::sourcing::base;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_sourcing_curation_examples::editor::sourcing::config::*;
        }
        pub use semio_s_artifact_sourcing_curation_domain::editor::sourcing::terminology;
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../✏️editor/🎮️commands/➕️curation-add/🦀️.rs"]
            pub mod curation_add;
            #[path = "../../../../../✏️editor/🎮️commands/➖️curation-remove/🦀️.rs"]
            pub mod curation_remove;
            #[path = "../../../../../✏️editor/🎮️commands/🔢️curation-set-count/🦀️.rs"]
            pub mod curation_set_count;
            #[path = "../../../../../✏️editor/🎮️commands/🧺️drop-on-curated/🦀️.rs"]
            pub mod drop_on_curated;
            #[path = "../../../../../✏️editor/🎮️commands/🏊️drop-on-pool/🦀️.rs"]
            pub mod drop_on_pool;
            #[path = "../../../../../✏️editor/🎮️commands/📥️load-document-json/🦀️.rs"]
            pub mod load_document_json;
            #[path = "../../../../../✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs"]
            pub mod set_contributions;
            #[path = "../../../../../✏️editor/🎮️commands/📉️set-filter-min-availability/🦀️.rs"]
            pub mod set_filter_min_availability;
            #[path = "../../../../../✏️editor/🎮️commands/🧱️set-filter-module/🦀️.rs"]
            pub mod set_filter_module;
            #[path = "../../../../../✏️editor/🎮️commands/🔎️set-filter-query/🦀️.rs"]
            pub mod set_filter_query;
            #[path = "../../../../../✏️editor/🎮️commands/🏛️set-filter-typology/🦀️.rs"]
            pub mod set_filter_typology;
            #[path = "../../../../../✏️editor/🎮️commands/↕️sort-table/🦀️.rs"]
            pub mod sort_table;
            #[path = "../../../../../✏️editor/🎮️commands/📇️stock-from-catalogue/🦀️.rs"]
            pub mod stock_from_catalogue;
            #[path = "../../../../../✏️editor/🎮️commands/🔢️set-grid-instance-display/🦀️.rs"]
            pub mod set_grid_instance_display;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_sourcing_curation_operations::editor::sourcing::modes::edit::windows::curated;
                    #[path = "."]
                    pub mod grid {
                        #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🔢️grid/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🔢️grid/🎚️config/🦀️.rs"]
                        pub mod config;
                        #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🔢️grid/☑️options/🔢️instance-display/🦀️.rs"]
                        pub mod instance_display;
                    }
                    #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🏊️pool/🦀️.rs"]
                    pub mod pool;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod sourcing {
        #[path = "../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_sourcing_curation_operations::viewer::sourcing::modes::view::*;
                pub use semio_s_artifact_sourcing_curation_operations::viewer::sourcing::modes::view::windows;
            }
        }
    }
}
