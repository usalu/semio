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
                    pub use semio_s_artifact_space_home_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_space_home_inferences::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_space_home_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_space_home_inferences::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod operations {
                        pub use semio_s_artifact_space_home_operations::standards::v1::subsets::any::schema::operations::*;
                        #[cfg(test)]
                        #[path = "../../../🦀️.rs"]
                        pub mod kinds_conformance;
                    }
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
                    #[path = "../../../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
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
        #[path = "../../../../../../../✏️editor/🦀️.rs"]
                mod component;
        pub use component::*;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_space_home_examples::editor::home::config::*;
            pub use semio_s_artifact_space_home_examples::editor::home::config::schema;
        }
        #[path = "."]
        pub mod transient {
            pub use semio_s_artifact_space_home_editor::editor::home::transient::*;
            #[cfg(test)]
            #[path = "../../../../../../../✏️editor/🫧️transient/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_space_home_examples::editor::home::presence::*;
        }
        pub use semio_s_artifact_space_home_examples::editor::home::terminology;
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../../../✏️editor/🎮️commands/📬️apply-directory-event-page/🦀️.rs"]
            pub mod apply_directory_event_page;
            #[path = "../../../../../../../✏️editor/🎮️commands/🗃️apply-local-catalog-document/🦀️.rs"]
            pub mod apply_local_catalog_document;
            #[path = "../../../../../../../✏️editor/🎮️commands/📎️bind-space-file/🦀️.rs"]
            pub mod bind_space_file;
            #[path = "../../../../../../../✏️editor/🎮️commands/🏗️create-studio/🦀️.rs"]
            pub mod create_studio;
            #[path = "../../../../../../../✏️editor/🎮️commands/☁️promote-to-hub-space/🦀️.rs"]
            pub mod promote_to_hub_space;
            #[path = "../../../../../../../✏️editor/🎮️commands/💾️persist-locally/🦀️.rs"]
            pub mod persist_locally;
            #[path = "../../../../../../../✏️editor/🎮️commands/🗂️delete-virtual-file-system-node/🦀️.rs"]
            pub mod delete_virtual_file_system_node;
            #[path = "../../../../../../../✏️editor/🎮️commands/🏠️go-home/🦀️.rs"]
            pub mod go_home;
            #[path = "../../../../../../../✏️editor/🎮️commands/📥️import-space/🦀️.rs"]
            pub mod import_space;
            #[path = "../../../../../../../✏️editor/🎮️commands/🧭️navigate-virtual-file-system-node/🦀️.rs"]
            pub mod navigate_virtual_file_system_node;
            #[path = "../../../../../../../✏️editor/🎮️commands/🌌️open-space/🦀️.rs"]
            pub mod open_space;
            // 🐙️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS — additive mounts
            // for the Home overview-table commands, mirroring the shape every sibling mount above uses.
            #[path = "../../../../../../../✏️editor/🎮️commands/📋copy-invite-link/🦀️.rs"]
            pub mod copy_invite_link;
            #[path = "../../../../../../../✏️editor/🎮️commands/🌱create-space/🦀️.rs"]
            pub mod create_space;
            #[path = "../../../../../../../✏️editor/🎮️commands/🗑️delete-space/🦀️.rs"]
            pub mod delete_space;
            #[path = "../../../../../../../✏️editor/🎮️commands/👥️presence-heartbeat/🦀️.rs"]
            pub mod presence_heartbeat;
            #[path = "../../../../../../../✏️editor/🎮️commands/🏷️rename-space/🦀️.rs"]
            pub mod rename_space;
            #[path = "../../../../../../../✏️editor/🎮️commands/🏛️manage-space/🦀️.rs"]
            pub mod manage_space;
            #[path = "../../../../../../../✏️editor/🎮️commands/🔗️share-space/🦀️.rs"]
            pub mod share_space;

        #[cfg(test)]
        #[path = "../../../../../../../✏️editor/🎮️commands/🧪️tests/🔬️unit/🦀️.rs"]
        mod leaf_tests;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod explore {
                pub use semio_s_artifact_space_home_examples::editor::home::modes::explore::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../../✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
