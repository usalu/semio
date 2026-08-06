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

// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<SourcingOperation, SourcingCurateConfigOperation>, Fault>`, the exact signature
// `DocumentApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a
// framework-owned error type; boxing it here would diverge from the trait it must satisfy, and the
// lint does not fire on the trait impl itself (only on the free functions the taxonomy split
// creates), so this is a pure artefact of decomposition.
#![allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod curate {
        #[path = "../../🗿️artifacts/🗂️curate/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🗂️curate/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/🗂️curate/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/🗂️curate/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/🗂️curate/🎒️pack/🦀️component.rs"]
        pub mod pack;
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

        #[path = "../../🎛️apps/🗂️curate/🎚️config/🦀️component.rs"]
        pub mod config;
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
semio_framework_plugin::semio_plugin! {
    id: "sourcing", label: "Sourcing", version: "0.1.0",
    setup: artifacts::curate::engine::register,
    apps: [ apps::curate::create_sourcing_curate_app => apps::curate::SourcingCurateApp ],
}
//#endregion 🔖️Plugin
