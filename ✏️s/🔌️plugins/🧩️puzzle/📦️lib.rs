//! 🧩️ Puzzle plugin — the 2d/3d/5d play apps bundled as one hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that
//! is written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

// 🧯️ `clippy::result_large_err` — `DocumentApp::handle` and `import_media` return
// `Result<Emit<Puzzle2dOperation, Puzzle2dConfigOperation>, Fault>`/`…, MediaError>`, the exact
// signatures the trait requires. `Fault` is a framework-owned error type; boxing it here would
// diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself (only
// on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#![allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod puzzle2d {
        #[path = "🗿️artifacts/◻2d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "🗿️artifacts/◻2d/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "🗿️artifacts/◻2d/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "🗿️artifacts/◻2d/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "🗿️artifacts/◻2d/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "🗿️artifacts/◻2d/📡️spr/🦀️component.rs"]
        pub mod spr;

        #[path = "."]
        pub mod engine {
            #[path = "🗿️artifacts/◻2d/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "🗿️artifacts/◻2d/⚙️engine/🦀️icons.rs"]
            pub mod icons;
            #[path = "🗿️artifacts/◻2d/⚙️engine/🦀️board_host.rs"]
            pub mod board_host;
            #[path = "🗿️artifacts/◻2d/⚙️engine/🦀️linking.rs"]
            pub mod linking;
            #[path = "🗿️artifacts/◻2d/⚙️engine/🦀️brush.rs"]
            pub mod brush;
            #[path = "🗿️artifacts/◻2d/⚙️engine/🦀️layout.rs"]
            pub mod layout;
        }
    }

    // 🚧️ The 🧊️3d and 🖐️5d artifact regions land here, in the same shape as `puzzle2d` above.
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod puzzle2d {
        #[path = "🎛️apps/◻2d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "🎛️apps/◻2d/🦀️config.rs"]
        pub mod config;
        #[path = "🎛️apps/◻2d/🦀️terminology.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "🎛️apps/◻2d/🎮️commands/🕸️node/🦀️component.rs"]
            pub mod node;
            #[path = "🎛️apps/◻2d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "🎛️apps/◻2d/🎮️commands/🖌️brush/🦀️component.rs"]
            pub mod brush;
            #[path = "🎛️apps/◻2d/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "🎛️apps/◻2d/🎮️commands/🌐️grid/🦀️component.rs"]
            pub mod grid;
            #[path = "🎛️apps/◻2d/🎮️commands/🤝️engagement/🦀️component.rs"]
            pub mod engagement;
            #[path = "🎛️apps/◻2d/🎮️commands/🔭️lod/🦀️component.rs"]
            pub mod lod;
            #[path = "🎛️apps/◻2d/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
            #[path = "🎛️apps/◻2d/🎮️commands/🛍️example/🦀️component.rs"]
            pub mod example;
            #[path = "🎛️apps/◻2d/🎮️commands/🎲️board/🦀️component.rs"]
            pub mod board;
            #[path = "🎛️apps/◻2d/🎮️commands/🧰️utility/🦀️component.rs"]
            pub mod utility;
        }

        #[path = "."]
        pub mod panels {
            #[path = "🎛️apps/◻2d/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "🎛️apps/◻2d/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "🎛️apps/◻2d/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🎛️apps/◻2d/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod options {
                    #[path = "🎛️apps/◻2d/🎭️modes/✏️edit/🎚️options/🔭️lod/🦀️component.rs"]
                    pub mod lod;
                    #[path = "🎛️apps/◻2d/🎭️modes/✏️edit/🎚️options/🖌️brush/🦀️component.rs"]
                    pub mod brush;
                }

                #[path = "."]
                pub mod tools {
                    #[path = "🎛️apps/◻2d/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️component.rs"]
                    pub mod fill;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod overview {
                        #[path = "🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/👁️overview/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod utilities {
                            #[path = "🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️component.rs"]
                            pub mod select;
                            #[path = "🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖌️brush/🦀️component.rs"]
                            pub mod brush;
                        }
                    }

                    #[path = "🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🔍️detail/🦀️component.rs"]
                    pub mod detail;
                    #[path = "🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🎯️selection/🦀️component.rs"]
                    pub mod selection;
                }
            }
        }
    }

    // 🚧️ The 🧊️3d and 🖐️5d app regions land here, in the same shape as `puzzle2d` above; each adds its
    // own `create_puzzle<n>d_app => Puzzle<n>dPlayApp` row to `semio_plugin!`'s `apps:` list below and
    // its own `register_puzzle<n>d_exports()` call inside `artifacts::puzzle2d::engine::register`.
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
semio_framework_plugin::semio_plugin! {
    id: "puzzle", label: "Puzzle", version: "0.1.0",
    setup: artifacts::puzzle2d::engine::register,
    apps: [ apps::puzzle2d::create_puzzle2d_app => apps::puzzle2d::Puzzle2dPlayApp ],
}
//#endregion 🔖️Plugin
