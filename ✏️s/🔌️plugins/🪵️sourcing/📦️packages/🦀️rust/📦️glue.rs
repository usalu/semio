//! 🪵️ Sourcing plugin — declarative curate app bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every leaf `mod` below points at exactly one taxonomy component file with a `#[path]`
//! written in full from the plugin root, prefixed with `../../` since THIS file now lives two levels
//! below the plugin root (`📦️packages/🦀️rust/`, moved here for Shape V2 tree purity — see ticket
//! `26/08/05/SOURCING-SHAPE-V2-TREE-PURITY-RETROFIT`). The grouping modules keep plain `#[path = "."]`
//! (unprefixed): `#[path]` values on nested inline `mod` blocks compose by concatenation against the
//! immediately enclosing mod's already-resolved directory, so a `../../` correction applied at every
//! nesting level would stack and over-correct — applying it exactly once, at each leaf, is both correct
//! and sufficient; the grouping modules' own names are still kept out of the base by `.` so leaf paths
//! stay writable in full from the plugin root, unchanged from before the move. Do not inline any
//! component file back into this one: the taxonomy validator and the `TaxonomyLibShape` policy lint both
//! fail on it (see master ticket `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`,
//! Single-File-Repo hazard ruling).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault>`, the exact signature
// `DocumentApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a
// framework-owned error type; boxing it here would diverge from the trait it must satisfy, and the
// lint does not fire on the trait impl itself (only on the free functions the taxonomy split
// creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod curate {
        #[path = "../../🗿️artifacts/🗂️curate/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🗂️curate/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/🗂️curate/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;

            #[path = "../../🗿️artifacts/🗂️curate/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/🗂️curate/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🗂️curate/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
            pub use schema::*;
        }
        #[path = "../../🗿️artifacts/🗂️curate/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/🗂️curate/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/🗂️curate/🧬️mutations/📸️set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🗂️curate/🧬️mutations/📸️set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🗂️curate/🧬️mutations/📸️set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/🗂️curate/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/🗂️curate/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/🗂️curate/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod curate {
        #[path = "../../🎛️apps/🗂️curate/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/🗂️curate/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🗂️curate/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/🗂️curate/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🗂️curate/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
        #[path = "../../🎛️apps/🗂️curate/🗣️terminology/🦀️component.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🗂️curate/🎮️commands/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🗂️curate/🎮️commands/🧺️curation/🦀️component.rs"]
            pub mod curation;
            #[path = "../../🎛️apps/🗂️curate/🎮️commands/🔍️filter/🦀️component.rs"]
            pub mod filter;
            #[path = "../../🎛️apps/🗂️curate/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🗂️curate/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
            #[path = "../../🎛️apps/🗂️curate/🎮️commands/🧩️contribution/🦀️component.rs"]
            pub mod contribution;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod curate {
                #[path = "../../🎛️apps/🗂️curate/🎭️modes/🗂️curate/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/🗂️curate/🎭️modes/🗂️curate/🪟️windows/🏊️pool/🦀️component.rs"]
                    pub mod pool;
                    #[path = "../../🎛️apps/🗂️curate/🎭️modes/🗂️curate/🪟️windows/🧺️curated/🦀️component.rs"]
                    pub mod curated;
                    #[path = "../../🎛️apps/🗂️curate/🎭️modes/🗂️curate/🪟️windows/👁️preview/🦀️component.rs"]
                    pub mod preview;
                    #[path = "../../🎛️apps/🗂️curate/🎭️modes/🗂️curate/🪟️windows/🔢️grid/🦀️component.rs"]
                    pub mod grid;
                }
            }
        }
    }
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
#[path = "../../🔌️plugin/🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/🗂️curate/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_curate_demo;
    #[path = "../../🎛️apps/🗂️curate/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_curate_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
