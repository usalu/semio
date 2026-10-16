//! 🧬️ viewer of `semio-s-artifact-imperative-procedure` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_schema as framework_schema;

pub use crate::schema::snapshot::ProcedureSnapshot;
pub use imperative_engine::Step;
pub use imperative_engine::Path;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_imperative_procedure_schema::{ProcedureScene};

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
                    pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "../../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
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
        #[path = "."]
        pub mod config {
            #[path = "../../../../../✏️editor/🎚️config/🚪️io/🦀️.rs"]
            pub mod io;
            #[path = "../../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "../../../../../✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "../../../../../✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/📋️main/🦀️.rs"]
                    pub mod main;
                }
            }
        }
        #[path = "../../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
        pub mod schema;
        #[path = "../../../../../✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
        pub mod demo_session;
        #[cfg(test)]
        #[path = "../../../../../✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
        mod example;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod procedure {
        #[path = "."]
        pub mod presence {
            #[path = "../../🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
