//! 📜️ Imperative plugin — declarative imperative play app bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to the owner root (this file itself now lives two levels deeper, in
//! `📦️packages/🦀️rust/`, so every path carries a `../../` prefix back out to the owner root). The
//! grouping modules carry `#[path = "."]` so their own names are not spliced into that base
//! directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as schema;
extern crate imperative_extension_sdk;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<ImperativeMutation, ImperativeConfigMutation>, Fault>`, the exact signature
// `DocumentApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned
// error type; boxing it here would diverge from the trait it must satisfy, and the lint does not fire on
// the trait impl itself (only on the free functions the taxonomy split creates), so this is a pure
// artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod imperative {
        #[path = "../../🗿️artifacts/📜️imperative/🦀️component.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod schema {
            #[path = "../../🗿️artifacts/📜️imperative/🧬️schema/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod snapshot {
                #[path = "../../🗿️artifacts/📜️imperative/🧬️schema/📸️snapshot/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/📜️imperative/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/📜️imperative/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod diff {
                #[path = "../../🗿️artifacts/📜️imperative/🧬️schema/🔺️diff/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/📜️imperative/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/📜️imperative/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod mutations {
                #[path = "../../🗿️artifacts/📜️imperative/🧬️schema/🧬️mutations/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/📜️imperative/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/📜️imperative/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                pub mod binary;
                #[path = "."]
                pub mod step_collection {
                    #[path = "../../🗿️artifacts/📜️imperative/🧬️schema/🧬️mutations/✂️step-collection/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/📜️imperative/🧬️schema/🧬️mutations/✂️step-collection/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/📜️imperative/🧬️schema/🧬️mutations/✂️step-collection/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
            }
        }
        pub mod op { pub use crate::artifacts::imperative::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::imperative::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::imperative::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::imperative::schema::diff::*; pub use crate::artifacts::imperative::schema::diff::text::*; pub mod schema { pub use crate::artifacts::imperative::schema::diff::*; } pub mod text { pub use crate::artifacts::imperative::schema::diff::text::*; } }
        pub mod mutations { pub use crate::artifacts::imperative::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::imperative::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::imperative::schema::snapshot::binary::*; } }
        #[path = "../../🗿️artifacts/📜️imperative/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📜️imperative/🪓️decomposer/🦀️component.rs"]
        pub mod decomposer;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/📜️imperative/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod import {
                #[path = "."]
                pub mod deserializers {
                    #[path = "."]
                    pub mod artifacts {
                        #[path = "."]
                        pub mod csv {
                            #[path = "../../🗿️artifacts/📜️imperative/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/📜️imperative/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod md {
                            #[path = "../../🗿️artifacts/📜️imperative/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod export {
                #[path = "."]
                pub mod serializers {
                    #[path = "."]
                    pub mod artifacts {
                        #[path = "."]
                        pub mod csv {
                            #[path = "../../🗿️artifacts/📜️imperative/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/📜️imperative/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod md {
                            #[path = "../../🗿️artifacts/📜️imperative/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::imperative::io::export::serializers::artifacts::csv::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::imperative::io::import::deserializers::artifacts::csv::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::imperative::io::export::serializers::artifacts::json::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::imperative::io::import::deserializers::artifacts::json::*;
                }
            }
            #[path = "."]
            pub mod md {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::imperative::io::export::serializers::artifacts::md::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::imperative::io::import::deserializers::artifacts::md::*;
                }
            }
        }
        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/📜️imperative/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;
        }
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod imperative {
        #[path = "../../🎛️apps/📜️imperative/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/📜️imperative/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/📜️imperative/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/📜️imperative/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/📜️imperative/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
        #[path = "../../🎛️apps/📜️imperative/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/📜️imperative/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📜️imperative/🎮️commands/🔧️step/🦀️component.rs"]
            pub mod step;
            #[path = "../../🎛️apps/📜️imperative/🎮️commands/👁️view/🦀️component.rs"]
            pub mod view;
            #[path = "../../🎛️apps/📜️imperative/🎮️commands/🧩️contribution/🦀️component.rs"]
            pub mod contribution;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📜️imperative/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📜️imperative/🎭️modes/✏️edit/🪟️windows/📋️main/🦀️component.rs"]
                    pub mod main;
                    #[path = "../../🎛️apps/📜️imperative/🎭️modes/✏️edit/🪟️windows/📝️script/🦀️component.rs"]
                    pub mod script;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📜️imperative/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📜️imperative/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📜️imperative/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }
}
//#endregion 🎛️Apps

//#region 🧩️Extensions
#[path = "."]
pub mod extensions {
    #[path = "../../🧩️extensions/📣️effect/🦀️component.rs"]
    pub mod effect;
    #[path = "../../🧩️extensions/🧮️math/🦀️component.rs"]
    pub mod math;
    #[path = "../../🧩️extensions/📝️text/🦀️component.rs"]
    pub mod text;
    #[path = "../../🧩️extensions/🧠️logic/🦀️component.rs"]
    pub mod logic;
    #[path = "../../🧩️extensions/🎮️control/🦀️component.rs"]
    pub mod control;
}
//#endregion 🧩️Extensions

//#region 🕸️Wasm
#[cfg(target_arch = "wasm32")]
pub use apps::imperative::wasm::ImperativeSession;
//#endregion 🕸️Wasm

//#region 🔖️Plugin
#[path = "../../🔌️plugin/🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/📜️imperative/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_imperative_demo;
    #[path = "../../🎛️apps/📜️imperative/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_imperative_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
