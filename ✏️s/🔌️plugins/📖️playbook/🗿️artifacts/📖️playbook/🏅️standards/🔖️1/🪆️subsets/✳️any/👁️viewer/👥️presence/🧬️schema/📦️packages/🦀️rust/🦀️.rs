//! 🧬️ viewer of `semio-s-artifact-playbook-playbook` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::playbook::PlaybookSpec;
use semio_framework_artifact_playbook_playbook as playbook;
pub use crate::playbook::PlaybookBlock;
pub use crate::playbook::PlaybookBlockOption;
pub use crate::playbook::PlaybookExpr;
pub use crate::playbook::PlaybookStep;
pub use crate::playbook::PlaybookVectorField;
pub use crate::playbook::PLAYBOOK_BUILTIN_KINDS;
pub use crate::playbook::PLAYBOOK_DOCUMENT_SCHEMA;
#[cfg(test)]
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}

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
                    pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::schema::mutations::central_apply;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod mutations {
                            pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::io::binary::mutations::*;
                            #[cfg(test)]
                            #[path = "../../../../../🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                            mod tests;
                        }
                        #[path = "../../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod playbook {
        #[path = "."]
        pub mod engine {
            pub use semio_s_artifact_playbook_playbook_examples::editor::playbook::engine::*;
            #[cfg(test)]
            #[path = "../../../../../✏️editor/⚙️engine/🧪️tests/🔬️unit/🦀️.rs"]
            mod tests;
        }
        #[path = "."]
        pub mod config {
            #[path = "."]
            pub mod io {
                #[path = "../../../../../✏️editor/🎚️config/🚪️io/💾️binary/🦀️.rs"]
                pub mod binary;
                #[path = "../../../../../✏️editor/🎚️config/🚪️io/📝️text/🦀️.rs"]
                pub mod text;
            }
            #[path = "../../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "../../../../../✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod builder {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod changes {
                        #[path = "../../../../../✏️editor/🎭️modes/🏗️builder/🪟️windows/🔺️changes/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod steps {
                        #[path = "../../../../../✏️editor/🎭️modes/🏗️builder/🪟️windows/📋️steps/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod activity {
                        #[path = "../../../../../✏️editor/🎭️modes/🏗️builder/🪟️windows/📰️activity/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod source {
                        #[path = "../../../../../✏️editor/🎭️modes/🏗️builder/🪟️windows/📜️source/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod playbook {
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
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod steps {
                        #[path = "../../../../🎭️modes/👁️view/🪟️windows/🌳️steps/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
