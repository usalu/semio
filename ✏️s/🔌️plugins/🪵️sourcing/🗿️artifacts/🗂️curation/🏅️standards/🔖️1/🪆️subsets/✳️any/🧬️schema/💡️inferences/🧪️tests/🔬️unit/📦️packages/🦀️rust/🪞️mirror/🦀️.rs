//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        pub use semio_s_artifact_sourcing_curation_examples::examples::demo::*;
        #[cfg(test)]
        #[path = "../../../../../../../📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
        pub mod example;
    }
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "../../../../../../../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../../../../../../🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_sourcing_curation_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_sourcing_curation_inferences::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_sourcing_curation_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod entries {
                            pub use semio_s_artifact_sourcing_curation_inferences::standards::v1::subsets::any::schema::inferences::entries::*;
                            #[cfg(test)]
                            //#region 🧪️Tests
                            #[path = "../../../../../🗃️entries/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../../🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_sourcing_curation_schema::standards::v1::subsets::any::io::binary::*;
                        pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::io::binary::snapshot;
                        pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::io::binary::mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_sourcing_curation_schema::standards::v1::subsets::any::io::text::*;
                        pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::io::text::snapshot;
                        pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::io::text::mutations;
                        pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::io::text::diff;
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::io::import::deserializers::artifacts::zip;
                                pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::io::export::serializers::artifacts::zip;
                                pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod sourcing {
        #[path = "../../../../../../../✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_sourcing_curation_operations::editor::sourcing::editor_vocabulary;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_sourcing_curation_examples::editor::sourcing::config::*;
            pub use semio_s_artifact_sourcing_curation_examples::editor::sourcing::config::schema;
        }
        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_sourcing_curation_domain::editor::sourcing::presence::*;
        }
        pub use semio_s_artifact_sourcing_curation_domain::editor::sourcing::terminology;
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::curation_add;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::curation_remove;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::curation_set_count;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::drop_on_curated;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::drop_on_pool;
            pub use semio_s_artifact_sourcing_curation_schema::editor::sourcing::commands::set_active_example;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::load_document_json;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::set_contributions;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::set_filter_min_availability;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::set_filter_module;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::set_filter_query;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::set_filter_typology;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::sort_table;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::stock_from_catalogue;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::set_grid_instance_display;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_sourcing_curation_operations::editor::sourcing::modes::edit::windows::curated;
                    #[path = "."]
                    pub mod grid {
                        pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::modes::edit::windows::grid::*;
                        pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::modes::edit::windows::grid::config;
                    }
                    pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::modes::edit::windows::pool;
                    #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs"]
                    pub mod preview;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod sourcing {
        pub use semio_s_artifact_sourcing_curation_inferences::viewer::sourcing::*;
        #[cfg(test)]
        #[path = "../../../../../../../👁️viewer/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
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
