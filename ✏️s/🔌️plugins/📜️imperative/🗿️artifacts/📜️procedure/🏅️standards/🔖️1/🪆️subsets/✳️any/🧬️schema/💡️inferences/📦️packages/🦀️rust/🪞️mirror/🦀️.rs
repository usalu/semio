//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

pub use semio_s_artifact_imperative_procedure_snapshot::extensions;
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
                    pub use semio_s_artifact_imperative_procedure_schema::standards::v1::subsets::any::schema::*;
                    #[cfg(test)]
                    #[path = "../../../../🧪️tests/🪪️document-contract/🦀️.rs"]
                    mod document_contract_tests;
                    pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod topology {
                            #[path = "../../../🧭topology/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../../🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                    #[path = "."]
                    pub mod operations {
                        pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::schema::operations::*;
                        #[cfg(test)]
                        #[path = "../../../../⚙️operations/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                    pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_imperative_procedure_schema::standards::v1::subsets::any::io::binary;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_imperative_procedure_schema::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_imperative_procedure_schema::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_imperative_procedure_viewer::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_imperative_procedure_schema::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod procedure {
        #[path = "../../../../../✏️editor/⚙️engine/🦀️.rs"]
        pub mod engine;
        #[path = "."]
        pub mod config {
            #[path = "../../../../../✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod io {
                #[path = "."]
                pub mod text {
                    #[path = "../../../../../✏️editor/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
                #[path = "."]
                pub mod binary {
                    #[path = "../../../../../✏️editor/🎚️config/🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
            }
        }
        #[path = "."]
        pub mod terminology {
            pub use semio_s_artifact_imperative_procedure_viewer::editor::procedure::terminology::*;
            #[cfg(test)]
            #[path = "../../../../../✏️editor/🗣️terminology/🧪️tests/🔬️unit/🦀️.rs"]
            mod tests;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod procedure {
        #[path = "../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/📋️main/🦀️.rs"]
                    pub mod main;
                    #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/📝️script/🦀️.rs"]
                    pub mod script;
                }
            }
        }
    }
}
