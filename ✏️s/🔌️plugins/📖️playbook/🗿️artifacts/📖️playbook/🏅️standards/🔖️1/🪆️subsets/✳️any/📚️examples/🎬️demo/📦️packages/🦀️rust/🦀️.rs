//! 🎬️ examples of `semio-s-artifact-playbook-playbook` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework as semio_framework;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::playbook::PLAYBOOK_DOCUMENT_SCHEMA;
pub use crate::schema::snapshot::PlaybookSnapshot;
pub use crate::playbook::PlaybookSpec;
pub use crate::playbook::PLAYBOOK_BUILTIN_KINDS;
use semio_framework_artifact_playbook_playbook as playbook;
pub use crate::playbook::PlaybookBlock;
pub use crate::playbook::PlaybookBlockOption;
pub use crate::playbook::PlaybookExpr;
pub use crate::playbook::PlaybookStep;
pub use crate::playbook::PlaybookVectorField;
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
                    pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::schema::mutations::central_apply;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
            }
        }
    }
}
pub use semio_s_artifact_playbook_playbook_snapshot::op;
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
    pub mod playbook {
        #[path = "../../../../✏️editor/⚙️engine/🦀️.rs"]
        pub mod engine;
        #[path = "."]
        pub mod config {
            #[path = "../../../../✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod io {
                #[path = "."]
                pub mod text {
                    #[path = "../../../../✏️editor/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
                #[path = "."]
                pub mod binary {
                    #[path = "../../../../✏️editor/🎚️config/🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
            }
        }
        #[path = "."]
        pub mod commands {
            #[path = "../../../../✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs"]
            pub mod set_contributions;
            #[path = "../../../../✏️editor/🎮️commands/♻️update-playbook/🦀️.rs"]
            pub mod update_playbook;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod builder {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod builder {
                        #[path = "../../../../✏️editor/🎭️modes/🏗️builder/🪟️windows/🏗️builder/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
        #[path = "../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
        pub mod schema;
        #[path = "../../../../✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
        pub mod demo_session;
        #[cfg(test)]
        #[path = "../../../../✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
        mod example;
    }
}
