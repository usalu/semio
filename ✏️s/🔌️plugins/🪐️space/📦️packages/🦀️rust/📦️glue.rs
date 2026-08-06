extern crate infinite_canvas as infinite_board_port_directed_dag;
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
//! `PluginBundle` builder + `plugin_exports!` invocation the pre-migration bundle crate already used.
//! `🪐️space`'s own app owns no document type at all (wraps the kernel-owned `WorkflowDocument`), so
//! there is only ONE `🗿️artifacts` node in this crate (`🏠️home`) — see `apps::space::🦀️component.rs`'s
//! module doc for the full rationale.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as vcs;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<Operation, ConfigOperation>, Fault>`, the exact signature `DocumentApp::handle` and
// `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#![allow(clippy::result_large_err)]

//#region 🫀️Core
#[path = "."]
pub mod core {
    #[path = "../../🫀️core/🦀️component.rs"]
    mod component;
    pub use component::*;
}
//#endregion 🫀️Core

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod home {
        #[path = "../../🗿️artifacts/🏠️home/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🏠️home/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/🏠️home/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/🏠️home/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/🏠️home/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/🏠️home/📡️spr/🦀️component.rs"]
        pub mod spr;
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

        #[path = "../../🎛️apps/🏠️home/🎚️config/🦀️component.rs"]
        pub mod config;
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

        #[path = "../../🎛️apps/🪐️space/🎚️config/🦀️component.rs"]
        pub mod config;
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
/// documents without depending on this crate's concrete `Projection`/`Operation` types.
fn register_s_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::home::HomeApp>("s.home");
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::space::SpaceApp>(semio_framework_os::OS_SPACE_SCHEMA);
}
//#endregion 🔖️DocumentCodecs

//#region 🔖️Manifest
fn bundle() -> semio_framework_plugin::PluginBundle {
    register_s_exports();
    semio_framework_plugin::PluginBundle::new("s", "S Studio", "0.1.0")
        .local_backbone_storage()
        .register_document_app(apps::home::create_home_app(), || apps::home::HomeApp)
        .register_document_app(apps::space::create_space_app(), || apps::space::SpaceApp)
}
semio_framework_plugin::plugin_exports!(bundle);
//#endregion 🔖️Manifest
