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
                    pub use semio_s_artifact_playbook_playbook_schema::standards::v1::subsets::any::schema::*;
                    #[cfg(test)]
                    #[path = "../../../../🧪️tests/🧱️block-defaults/🦀️.rs"]
                    mod block_defaults_tests;
                    pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::schema::snapshot;
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
                    pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::schema::mutations::central_apply;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_playbook_playbook_schema::standards::v1::subsets::any::io::binary;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_playbook_playbook_schema::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_playbook_playbook_schema::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_playbook_playbook_schema::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_playbook_playbook_schema::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_playbook_playbook_snapshot::op;
pub use semio_s_artifact_playbook_playbook_examples::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod playbook {
        #[path = "../../../../../✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_playbook_playbook_examples::editor::playbook::engine;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_playbook_playbook_examples::editor::playbook::config::*;
            pub use semio_s_artifact_playbook_playbook_viewer::editor::playbook::config::schema;
        }
        #[path = "."]
        pub mod terminology {
            pub use semio_s_artifact_playbook_playbook_viewer::editor::playbook::terminology::*;
            #[cfg(test)]
            #[path = "../../../../../✏️editor/🗣️terminology/🧪️tests/🔬️unit/🦀️.rs"]
            mod tests;
        }
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../✏️editor/🎮️commands/🧱️add-block/🦀️.rs"]
            pub mod add_block;
            #[path = "../../../../../✏️editor/🎮️commands/🪜️add-step/🦀️.rs"]
            pub mod add_step;
            #[path = "../../../../../✏️editor/🎮️commands/🚚️move-block/🦀️.rs"]
            pub mod move_block;
            #[path = "../../../../../✏️editor/🎮️commands/↔️move-step/🦀️.rs"]
            pub mod move_step;
            #[path = "../../../../../✏️editor/🎮️commands/🚮️remove-block/🦀️.rs"]
            pub mod remove_block;
            #[path = "../../../../../✏️editor/🎮️commands/➖️remove-step/🦀️.rs"]
            pub mod remove_step;
            #[path = "../../../../../✏️editor/🎮️commands/🧬️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            pub use semio_s_artifact_playbook_playbook_examples::editor::playbook::commands::set_contributions;
            pub use semio_s_artifact_playbook_playbook_examples::editor::playbook::commands::update_playbook;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod builder {
                #[path = "../../../../../✏️editor/🎭️modes/🏗️builder/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_playbook_playbook_examples::editor::playbook::modes::builder::windows::builder;
                    pub use semio_s_artifact_playbook_playbook_viewer::editor::playbook::modes::builder::windows::changes;
                    pub use semio_s_artifact_playbook_playbook_viewer::editor::playbook::modes::builder::windows::steps;
                    pub use semio_s_artifact_playbook_playbook_viewer::editor::playbook::modes::builder::windows::activity;
                    #[path = "."]
                    pub mod source {
                        pub use semio_s_artifact_playbook_playbook_viewer::editor::playbook::modes::builder::windows::source::*;
                        #[cfg(test)]
                        #[path = "../../../../../✏️editor/🎭️modes/🏗️builder/🪟️windows/📜️source/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                    #[path = "."]
                    pub mod files {
                        #[path = "../../../../../✏️editor/🎭️modes/🏗️builder/🪟️windows/🗂️files/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
        pub use semio_s_artifact_playbook_playbook_examples::editor::playbook::schema;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod playbook {
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
                    pub use semio_s_artifact_playbook_playbook_viewer::viewer::playbook::modes::view::windows::*;
                    #[path = "."]
                    pub mod steps {
                        pub use semio_s_artifact_playbook_playbook_viewer::viewer::playbook::modes::view::windows::steps::*;
                        #[cfg(test)]
                        #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🌳️steps/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                }
            }
        }
    }
}
