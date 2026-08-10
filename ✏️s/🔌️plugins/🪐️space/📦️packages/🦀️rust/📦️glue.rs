//! 🎛️ S Studio plugin — designer OS shell bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory. Do not inline any
//! component file back into this one: the taxonomy validator and the `TaxonomyLibShape` policy lint
//! both fail on it (see master ticket `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`,
//! Single-File-Repo hazard ruling).
//!
//! 🕳️ Deviation from the usual per-app-plugin shape: `s` is the OS host plugin bundling BOTH the
//! `🏠️home` launcher and `🪐️space` studio apps, so it does NOT use the `semio_plugin!` macro (that
//! macro assumes one document schema and one app-registration path per plugin) — it keeps the manual
//! `Plugin` builder + `plugin_exports!` invocation the pre-migration bundle crate already used.
//! `🪐️space`'s own app owns no document type at all (wraps the kernel-owned `WorkflowSnapshot`), so
//! there is only ONE `🗿️artifacts` node in this crate (`🏠️home`) — see `apps::space::🦀️component.rs`'s
//! module doc for the full rationale.

extern crate infinite_canvas as infinite_board_port_directed_dag;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<Mutation, ConfigMutation>, Fault>`, the exact signature `DocumentApp::handle` and
// `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

#[path = "../../🦀️component.rs"]
mod space_shared;
pub use space_shared::*;

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod home {
        #[path = "../../🗿️artifacts/🏠️home/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_catalog_generation {
                                    #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛set-catalog-generation/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛set-catalog-generation/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛set-catalog-generation/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xlsx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📕️xlsx/🔖️ecma-376/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xlsx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📕️xlsx/🔖️ecma-376/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
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

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::home::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::home::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::home::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::home::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::home::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::home::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::home::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::home::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::home::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::home::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::home::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::home::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::home::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::home::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::home::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::home::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::home::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::home::standards::v1::subsets::any::schema::snapshot::binary::*; } }

        #[path = "../../🗿️artifacts/🏠️home/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🏠️home/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/🏠️home/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🏠️home/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod home {
        #[path = "../../🎛️apps/🏠️home/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/🏠️home/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🏠️home/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/🏠️home/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🏠️home/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "../../🎛️apps/🏠️home/🗣️terminology/🦀️component.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🏠️home/🎮️commands/🏙️studio/🦀️component.rs"]
            pub mod studio;
            #[path = "../../🎛️apps/🏠️home/🎮️commands/🗂️vfs/🦀️component.rs"]
            pub mod vfs;
            #[path = "../../🎛️apps/🏠️home/🎮️commands/⚙️settings/🦀️component.rs"]
            pub mod settings;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod explore {
                #[path = "../../🎛️apps/🏠️home/🎭️modes/🔎️explore/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🎛️apps/🏠️home/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }

    #[path = "."]
    pub mod space {
        #[path = "../../🎛️apps/🪐️space/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/🪐️space/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🪐️space/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/🪐️space/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🪐️space/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "../../🎛️apps/🪐️space/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/🪐️space/⚙️engine/🦀️component.rs"]
        pub mod engine;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🪐️space/🎮️commands/🔢️parameters/🦀️component.rs"]
            pub mod parameters;
            #[path = "../../🎛️apps/🪐️space/🎮️commands/🧩️nodes/🦀️component.rs"]
            pub mod nodes;
            #[path = "../../🎛️apps/🪐️space/🎮️commands/🔗️connections/🦀️component.rs"]
            pub mod connections;
            #[path = "../../🎛️apps/🪐️space/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🪐️space/🎮️commands/🖱️viewport/🦀️component.rs"]
            pub mod viewport;
            #[path = "../../🎛️apps/🪐️space/🎮️commands/💬️engagement/🦀️component.rs"]
            pub mod engagement;
            #[path = "../../🎛️apps/🪐️space/🎮️commands/✏️graph-edit/🦀️component.rs"]
            pub mod graph_edit;
            #[path = "../../🎛️apps/🪐️space/🎮️commands/👥️presence/🦀️component.rs"]
            pub mod presence;
            #[path = "../../🎛️apps/🪐️space/🎮️commands/🖼️media/🦀️component.rs"]
            pub mod media;
            #[path = "../../🎛️apps/🪐️space/🎮️commands/💾️studio-io/🦀️component.rs"]
            pub mod studio_io;
            #[path = "../../🎛️apps/🪐️space/🎮️commands/🔍️instance-nav/🦀️component.rs"]
            pub mod instance_nav;
            #[path = "../../🎛️apps/🪐️space/🎮️commands/🧭️navigation/🦀️component.rs"]
            pub mod navigation;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod main {
                #[path = "../../🎛️apps/🪐️space/🎭️modes/🌐️main/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod workflow {
                        #[path = "../../🎛️apps/🪐️space/🎭️modes/🌐️main/🪟️windows/🕸️workflow/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🎛️apps/🪐️space/🎭️modes/🌐️main/🪟️windows/🕸️workflow/🎚️options/🎯️active-instance/🦀️component.rs"]
                            pub mod active_instance;
                        }
                    }

                    #[path = "."]
                    pub mod media_vfs {
                        #[path = "../../🎛️apps/🪐️space/🎭️modes/🌐️main/🪟️windows/🗂️media-vfs/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }

                    #[path = "."]
                    pub mod compiled_dag {
                        #[path = "../../🎛️apps/🪐️space/🎭️modes/🌐️main/🪟️windows/🕸️compiled-dag/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🪐️space/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/🪐️space/📌️panels/🔢️parameters/🦀️component.rs"]
            pub mod parameters;
            #[path = "../../🎛️apps/🪐️space/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }
}
//#endregion 🎛️Apps

//#region 🔖️DocumentCodecs
/// 🗂️ Registers `s.home`/`s.space`'s pack<->dsl codecs under their real `document_schema()` strings so
/// `framework/sync`'s `FolderEndpoint::Pack` (and any other schema-keyed caller) can print/parse these
/// documents without depending on this crate's concrete `Projection`/`Mutation` types.
fn register_s_exports() {
    crate::artifacts::home::register_pilot_languages();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::home::HomeApp>("s.home");
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::space::SpaceApp>(semio_framework_os::OS_SPACE_SCHEMA);
}
//#endregion 🔖️DocumentCodecs

//#region 🔖️Manifest

#[path = "../../🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);
//#endregion 🔖️Manifest

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/🏠️home/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_home_demo;
    #[path = "../../🎛️apps/🏠️home/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_home_demo_session;
    #[path = "../../🎛️apps/🪐️space/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_space_demo_session;
}
//#endregion 📚️Examples
