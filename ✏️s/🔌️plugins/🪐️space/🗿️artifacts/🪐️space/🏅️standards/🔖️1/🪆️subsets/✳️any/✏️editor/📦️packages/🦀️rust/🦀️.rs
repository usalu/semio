//! ✏️ editor of `semio-s-artifact-space-space` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_space_space_schema::{SPACE_INDEX_DIALECT, S_SPACE_INDEX_DOCUMENT_SCHEMA, artifact_kind};

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
                    pub use semio_s_artifact_space_space_schema::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_space_space_mutations::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod space_index {
        #[path = "../../🦀️.rs"]
                mod component;
        pub use component::*;
        pub use semio_s_artifact_space_space_diff::editor::space_index::config;
        #[path = "."]
        pub mod commands {
            #[path = "../../🎮️commands/🔗copy-invite-link/🦀️.rs"]
            pub mod copy_invite_link;
            #[path = "../../🎮️commands/🌱create-artifact/🦀️.rs"]
            pub mod create_artifact;
            #[path = "../../🎮️commands/🗑️delete-artifact/🦀️.rs"]
            pub mod delete_artifact;
            #[path = "../../🎮️commands/📇fold-directory-events/🦀️.rs"]
            pub mod fold_directory_events;
            #[path = "../../🎮️commands/💌invite-member/🦀️.rs"]
            pub mod invite_member;
            #[path = "../../🎮️commands/🗿️open-artifact/🦀️.rs"]
            pub mod open_artifact;
            #[path = "../../🎮️commands/🗃️open-artifact-with/🦀️.rs"]
            pub mod open_artifact_with;
            #[path = "../../🎮️commands/💓presence-heartbeat/🦀️.rs"]
            pub mod presence_heartbeat;
            #[path = "../../🎮️commands/🚪remove-member/🦀️.rs"]
            pub mod remove_member;
            #[path = "../../🎮️commands/🏷️rename-artifact/🦀️.rs"]
            pub mod rename_artifact;
            #[path = "../../🎮️commands/❔request-delete-artifact/🦀️.rs"]
            pub mod request_delete_artifact;
            #[path = "../../🎮️commands/❕request-invite-member/🦀️.rs"]
            pub mod request_invite_member;
            #[path = "../../🎮️commands/👁️set-visibility/🦀️.rs"]
            pub mod set_visibility;
            #[path = "../../🎮️commands/🕒touch-artifact/🦀️.rs"]
            pub mod touch_artifact;

        #[cfg(test)]
        #[path = "../../🎮️commands/🧪️tests/🔬️unit/🦀️.rs"]
        mod leaf_tests;
        }
        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub mod members {
                #[path = "../../📌️panels/👥️members/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🎭️modes/✏️edit/🪟️windows/🏠️main/🦀️.rs"]
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
    pub mod space_index {
        #[path = "../../../👁️viewer/🦀️.rs"]
                mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_space_space_diff::viewer::space_index::modes::view::*;
                pub use semio_s_artifact_space_space_diff::viewer::space_index::modes::view::windows;
            }
        }
    }
}
pub use semio_s_artifact_space_space_diff::examples;
