//! 🧱️ Block plugin — 2D, 3D, and 5D single-kind-definition editors in one hot-swappable WASM plugin.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<...Operation, ...ConfigOperation>, Fault>`, the exact signature `DocumentApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🫀️Core
/// 🤝️ Record types shared by all three artifacts' document entities (non-constitutional cross-artifact
/// kernel — see the constitutional-split recipe's "shared code used by ≥2 artifacts" rule).
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
    pub mod block2d {
        #[path = "../../🗿️artifacts/◻2d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/◻2d/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/◻2d/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/◻2d/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/◻2d/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/◻2d/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/◻2d/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod block3d {
        #[path = "../../🗿️artifacts/🧊️3d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🧊️3d/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/🧊️3d/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/🧊️3d/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/🧊️3d/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/🧊️3d/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/🧊️3d/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod block5d {
        #[path = "../../🗿️artifacts/🖐️5d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🖐️5d/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/🖐️5d/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/🖐️5d/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/🖐️5d/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/🖐️5d/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/🖐️5d/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod block2d {
        #[path = "../../🎛️apps/◻2d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/◻2d/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/◻2d/🗣️terminology/🦀️component.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/◻2d/🎮️commands/🏷️kind/🦀️component.rs"]
            pub mod kind;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🔘️handle-kind/🦀️component.rs"]
            pub mod handle_kind;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🌱️handle/🦀️component.rs"]
            pub mod handle;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🔗️compatibility/🦀️component.rs"]
            pub mod compatibility;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🎨️example/🦀️component.rs"]
            pub mod example;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/📋️board/🦀️component.rs"]
                    pub mod board;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/◻2d/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/◻2d/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod block3d {
        #[path = "../../🎛️apps/🧊️3d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/🧊️3d/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/🧊️3d/🌍️world/🦀️component.rs"]
        pub mod world;
        #[path = "../../🎛️apps/🧊️3d/🗣️terminology/🦀️component.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🏷️kind/🦀️component.rs"]
            pub mod kind;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🧱️representation/🦀️component.rs"]
            pub mod representation;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🔘️vortex-kind/🦀️component.rs"]
            pub mod vortex_kind;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🌀️vortex/🦀️component.rs"]
            pub mod vortex;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🎨️example/🦀️component.rs"]
            pub mod example;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🪟️window/🦀️component.rs"]
            pub mod window;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🖌️brush/🦀️component.rs"]
            pub mod brush;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod world {
                        #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🌐️world/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🌐️world/🎚️options/🧱️representations/🦀️component.rs"]
                            pub mod representations;
                            #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🌐️world/🎚️options/🔀️quick-representation/🦀️component.rs"]
                            pub mod quick_representation;
                            #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🌐️world/🎚️options/↔️arrangement/🦀️component.rs"]
                            pub mod arrangement;
                            #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🌐️world/🎚️options/📏️spacing/🦀️component.rs"]
                            pub mod spacing;
                            #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🌐️world/🎚️options/🖌️brush/🦀️component.rs"]
                            pub mod brush;
                        }
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🧊️3d/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🧊️3d/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod block5d {
        #[path = "../../🎛️apps/🖐️5d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/🖐️5d/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/🖐️5d/🗣️terminology/🦀️component.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🏷️kind/🦀️component.rs"]
            pub mod kind;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🔘️grip-kind/🦀️component.rs"]
            pub mod grip_kind;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🌱️grip/🦀️component.rs"]
            pub mod grip;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🎨️example/🦀️component.rs"]
            pub mod example;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/📋️board/🦀️component.rs"]
                    pub mod board;
                    #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/🌐️world/🦀️component.rs"]
                    pub mod world;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🖐️5d/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🖐️5d/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
fn register_block_exports() {
    artifacts::block2d::engine::register();
    artifacts::block3d::engine::register();
    artifacts::block5d::engine::register();
}

semio_framework_plugin::semio_plugin! {
    id: "block",
    label: "Block",
    version: "0.1.0",
    setup: register_block_exports,
    apps: [
        apps::block2d::create_block2d_app => apps::block2d::Block2dPlayApp,
        apps::block3d::create_block3d_app => apps::block3d::Block3dPlayApp,
        apps::block5d::create_block5d_app => apps::block5d::Block5dPlayApp,
    ],
}
//#endregion 🔖️Plugin
