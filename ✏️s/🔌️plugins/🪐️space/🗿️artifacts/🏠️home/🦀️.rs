//! 🏠️ S Home launcher artifact — document entity (constitutional: general).

#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::mutations::SHomeMutation;

pub use crate::standards::v1::subsets::any::schema::diff::SHomeDiff;

pub use semio_s_artifact_space_home_inferences::{S_HOME_DOCUMENT_SCHEMA};
pub use crate::standards::v1::subsets::any::schema::SHomeArtifact;
pub use crate::standards::v1::subsets::any::schema::snapshot::SHomeSnapshot;

pub use semio_s_artifact_space_home_kinds_conformance::{HOME_DIALECT};

pub use semio_s_artifact_space_home_inferences::{artifact_kind};

pub use semio_s_artifact_space_home_kinds_conformance::{declaration, definition};

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
                            pub use semio_s_artifact_space_home_schema::standards::v1::subsets::any::schema::*;

                            pub use semio_s_artifact_space_home_inferences::standards::v1::subsets::any::schema::snapshot;
                            pub use semio_s_artifact_space_home_inferences::standards::v1::subsets::any::schema::inferences;
                            pub use semio_s_artifact_space_home_inferences::standards::v1::subsets::any::schema::diff;
                            pub use semio_s_artifact_space_home_operations::standards::v1::subsets::any::schema::operations;
                            #[path = "."]
                            pub mod mutations {
                                pub use semio_s_artifact_space_home_operations::standards::v1::subsets::any::schema::mutations::*;

                                #[path = "."]
                                pub mod change_catalog_generation {
                                    pub use semio_s_artifact_space_home_operations::standards::v1::subsets::any::schema::mutations::change_catalog_generation::*;
                                    pub use semio_s_artifact_space_home_operations::standards::v1::subsets::any::schema::mutations::change_catalog_generation::diff;
                                    pub use semio_s_artifact_space_home_operations::standards::v1::subsets::any::schema::mutations::change_catalog_generation::inverse;

                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            pub use semio_s_artifact_space_home_kinds_conformance::standards::v1::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_space_home_examples::standards::v1::subsets::any::io::binary::*;
    pub use semio_s_artifact_space_home_operations::standards::v1::subsets::any::io::binary::diff;
    pub use semio_s_artifact_space_home_inferences::standards::v1::subsets::any::io::binary::inferences;
    pub use semio_s_artifact_space_home_operations::standards::v1::subsets::any::io::binary::mutations;
    pub use semio_s_artifact_space_home_inferences::standards::v1::subsets::any::io::binary::snapshot;
}
pub mod sqlite {
    pub use semio_s_artifact_space_home_examples::standards::v1::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_space_home_inferences::standards::v1::subsets::any::io::sqlite::snapshot;
}
pub mod text {
    pub use semio_s_artifact_space_home_examples::standards::v1::subsets::any::io::text::*;
    pub use semio_s_artifact_space_home_operations::standards::v1::subsets::any::io::text::diff;
    pub use semio_s_artifact_space_home_inferences::standards::v1::subsets::any::io::text::inferences;
    pub use semio_s_artifact_space_home_operations::standards::v1::subsets::any::io::text::mutations;
    pub use semio_s_artifact_space_home_inferences::standards::v1::subsets::any::io::text::snapshot;
}

                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        pub use semio_s_artifact_space_home_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::zip;
                                        pub use semio_s_artifact_space_home_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                        pub use semio_s_artifact_space_home_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        pub use semio_s_artifact_space_home_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::zip;
                                        pub use semio_s_artifact_space_home_schema::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                        pub use semio_s_artifact_space_home_schema::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                                    }
                                }
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
                pub use semio_s_artifact_space_home_examples::examples::demo::*;

            }
        }

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod home {
pub use semio_s_artifact_space_home_kinds_conformance::editor::home::*;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo_session {
                pub use semio_s_artifact_space_home_examples::editor::home::examples::demo_session::*;

            }
        }

        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_space_home_examples::editor::home::config::*;

            pub use semio_s_artifact_space_home_examples::editor::home::config::schema;
        }

        pub use semio_s_artifact_space_home_editor::editor::home::transient;

        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_space_home_examples::editor::home::presence::*;

            pub use semio_s_artifact_space_home_examples::editor::home::presence::schema;
        }

        pub use semio_s_artifact_space_home_examples::editor::home::terminology;

        pub use semio_s_artifact_space_home_kinds_conformance::editor::home::commands;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod explore {
                pub use semio_s_artifact_space_home_examples::editor::home::modes::explore::*;

                pub use semio_s_artifact_space_home_kinds_conformance::editor::home::modes::explore::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod home {
pub use semio_s_artifact_space_home_viewer::viewer::home::*;

pub use semio_s_artifact_space_home_examples::viewer::home::config;
pub use semio_s_artifact_space_home_examples::viewer::home::presence;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_space_home_examples::viewer::home::modes::view::*;

                pub use semio_s_artifact_space_home_examples::viewer::home::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v1::subsets::any::io::{HomeBuilderConstruction, SHomeParts, SHomeAnalyzerAnalysis, HomeBuilderFacets, HomeBuilder, SHomeAnalyzer, SHomeComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
