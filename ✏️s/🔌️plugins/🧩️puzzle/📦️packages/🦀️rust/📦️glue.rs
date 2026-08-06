//! 🧩️ Puzzle plugin — the 2d/3d/5d play apps bundled as one hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that
//! is written in full, relative to THIS file's directory (`📦️packages/🦀️rust/`, Shape V2 — see ticket
//! `26/08/05/SHAPE-V2-TREE-PURITY-BROADCAST`), prefixed with `../../` to reach back up to the plugin
//! root. The grouping modules carry `#[path = "."]` so their own names are not spliced into that base
//! directory — without it, Rust resolves an inline module's children under
//! `<file dir>/<inline mod name>/…` and every leaf path dangles. Do not inline any component file back
//! into this one: the taxonomy validator and the `TaxonomyLibShape` policy lint both fail on it (see
//! master ticket `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard
//! ruling).

extern crate infinite_canvas as infinite_board_port_directed_normal;
extern crate infinite_canvas as infinite_board_port_directed;

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
// 🧯️ `clippy::result_large_err` — `DocumentApp::handle` and `import_media` return
// `Result<Emit<Puzzle2dOperation, Puzzle2dConfigOperation>, Fault>`/`…, MediaError>`, the exact
// signatures the trait requires. `Fault` is a framework-owned error type; boxing it here would
// diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself (only
// on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod puzzle2d {
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

        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/◻2d/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/◻2d/⚙️engine/🔣️icons/🦀️component.rs"]
            pub mod icons;
            #[path = "../../🗿️artifacts/◻2d/⚙️engine/🎲️board-host/🦀️component.rs"]
            pub mod board_host;
            #[path = "../../🗿️artifacts/◻2d/⚙️engine/🔗️linking/🦀️component.rs"]
            pub mod linking;
            #[path = "../../🗿️artifacts/◻2d/⚙️engine/🖌️brush/🦀️component.rs"]
            pub mod brush;
            #[path = "../../🗿️artifacts/◻2d/⚙️engine/📐️layout/🦀️component.rs"]
            pub mod layout;
        }
    }

    #[path = "."]
    pub mod puzzle3d {
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

        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/🧊️3d/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🧊️3d/⚙️engine/📐️geometry/🦀️component.rs"]
            pub mod geometry;
            #[path = "../../🗿️artifacts/🧊️3d/⚙️engine/🖌️brush/🦀️component.rs"]
            pub mod brush;
            #[path = "../../🗿️artifacts/🧊️3d/⚙️engine/🪣️fill/🦀️component.rs"]
            pub mod fill;
            #[path = "../../🗿️artifacts/🧊️3d/⚙️engine/⏳️session/🦀️component.rs"]
            pub mod session;
        }
    }

    #[path = "."]
    pub mod puzzle5d {
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

        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/🖐️5d/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🖐️5d/⚙️engine/✂️transfer/🦀️component.rs"]
            pub mod transfer;
            #[path = "../../🗿️artifacts/🖐️5d/⚙️engine/🌉️compose/🦀️component.rs"]
            pub mod compose;
        }
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod puzzle2d {
        #[path = "../../🎛️apps/◻2d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/◻2d/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/◻2d/🗣️terminology/🦀️component.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/◻2d/🎮️commands/🕸️node/🦀️component.rs"]
            pub mod node;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🖌️brush/🦀️component.rs"]
            pub mod brush;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🌐️grid/🦀️component.rs"]
            pub mod grid;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🤝️engagement/🦀️component.rs"]
            pub mod engagement;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🔭️lod/🦀️component.rs"]
            pub mod lod;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🛍️example/🦀️component.rs"]
            pub mod example;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🎲️board/🦀️component.rs"]
            pub mod board;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🧰️utility/🦀️component.rs"]
            pub mod utility;
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/◻2d/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/◻2d/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/◻2d/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod options {
                    #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🎚️options/🔭️lod/🦀️component.rs"]
                    pub mod lod;
                    #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🎚️options/🖌️brush/🦀️component.rs"]
                    pub mod brush;
                }

                #[path = "."]
                pub mod tools {
                    #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️component.rs"]
                    pub mod fill;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod overview {
                        #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/👁️overview/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod utilities {
                            #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️component.rs"]
                            pub mod select;
                            #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖌️brush/🦀️component.rs"]
                            pub mod brush;
                        }
                    }

                    #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🔍️detail/🦀️component.rs"]
                    pub mod detail;
                    #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🎯️selection/🦀️component.rs"]
                    pub mod selection;
                }
            }
        }
    }

    #[path = "."]
    pub mod puzzle3d {
        #[path = "../../🎛️apps/🧊️3d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/🧊️3d/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/🧊️3d/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/🧊️3d/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🛍️example/🦀️component.rs"]
            pub mod example;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/👆️hover/🦀️component.rs"]
            pub mod hover;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🧊️object/🦀️component.rs"]
            pub mod object;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🔗️attraction/🦀️component.rs"]
            pub mod attraction;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🧊️volume/🦀️component.rs"]
            pub mod volume;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/☀️sun/🦀️component.rs"]
            pub mod sun;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🔭️lod/🦀️component.rs"]
            pub mod lod;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🌐️grid/🦀️component.rs"]
            pub mod grid;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/⚙️settings/🦀️component.rs"]
            pub mod settings;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🔄️transform/🦀️component.rs"]
            pub mod transform;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🖌️brush/🦀️component.rs"]
            pub mod brush;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🪣️fill/🦀️component.rs"]
            pub mod fill;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🤝️engagement/🦀️component.rs"]
            pub mod engagement;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🧰️utility/🦀️component.rs"]
            pub mod utility;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🧊️3d/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🧊️3d/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/🧊️3d/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
            #[path = "../../🎛️apps/🧊️3d/📌️panels/⚙️settings/🦀️component.rs"]
            pub mod settings;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod options {
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🎚️options/🎥️projection/🦀️component.rs"]
                    pub mod projection;
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🎚️options/🌀️vortex/🦀️component.rs"]
                    pub mod vortex;
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🎚️options/🔭️lod/🦀️component.rs"]
                    pub mod lod;
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🎚️options/🌐️grid/🦀️component.rs"]
                    pub mod grid;
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🎚️options/🎯️select/🦀️component.rs"]
                    pub mod select;
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🎚️options/☀️sun/🦀️component.rs"]
                    pub mod sun;
                }

                #[path = "."]
                pub mod tools {
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️component.rs"]
                    pub mod fill;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod utilities {
                            #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🔄️transform/🦀️component.rs"]
                            pub mod transform;
                            #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🖌️brush/🦀️component.rs"]
                            pub mod brush;
                            #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🧊️volume-brush/🦀️component.rs"]
                            pub mod volume_brush;
                            #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🚚️world-relocate/🦀️component.rs"]
                            pub mod world_relocate;
                        }
                    }
                }
            }
        }
    }

    #[path = "."]
    pub mod puzzle5d {
        #[path = "../../🎛️apps/🖐️5d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/🖐️5d/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/🖐️5d/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/🖐️5d/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🛍️example/🦀️component.rs"]
            pub mod example;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🧩️part/🦀️component.rs"]
            pub mod part;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/✏️patch/🦀️component.rs"]
            pub mod patch;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/👆️hover/🦀️component.rs"]
            pub mod hover;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/☀️sun/🦀️component.rs"]
            pub mod sun;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🔭️lod/🦀️component.rs"]
            pub mod lod;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🌐️grid/🦀️component.rs"]
            pub mod grid;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🖌️brush/🦀️component.rs"]
            pub mod brush;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🪣️fill/🦀️component.rs"]
            pub mod fill;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🤝️engagement/🦀️component.rs"]
            pub mod engagement;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🧰️utility/🦀️component.rs"]
            pub mod utility;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🔄️transform/🦀️component.rs"]
            pub mod transform;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🎲️board/🦀️component.rs"]
            pub mod board;
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🖐️5d/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🖐️5d/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/🖐️5d/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod options {
                    #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🎚️options/🖌️brush/🦀️component.rs"]
                    pub mod brush;
                    #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🎚️options/🪣️fill/🦀️component.rs"]
                    pub mod fill;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod board2d {
                        #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/◻2d/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/◻2d/🎬️actions/🦀️component.rs"]
                        pub mod actions;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/◻2d/🎚️options/🔭️lod/🦀️component.rs"]
                            pub mod lod;
                        }

                        #[path = "."]
                        pub mod utilities {
                            #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/◻2d/🪛️utilities/🖱️select/🦀️component.rs"]
                            pub mod select;
                            #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/◻2d/🪛️utilities/🖌️brush/🦀️component.rs"]
                            pub mod brush;
                            #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/◻2d/🪛️utilities/🪣️fill/🦀️component.rs"]
                            pub mod fill;
                        }
                    }

                    #[path = "."]
                    pub mod world3d {
                        #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/🧊️3d/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/🧊️3d/🎬️actions/🦀️component.rs"]
                        pub mod actions;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/🧊️3d/🎚️options/☀️sun/🦀️component.rs"]
                            pub mod sun;
                        }

                        #[path = "."]
                        pub mod utilities {
                            #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/🧊️3d/🪛️utilities/🔄️transform/🦀️component.rs"]
                            pub mod transform;
                            #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/🧊️3d/🪛️utilities/🚚️world-relocate/🦀️component.rs"]
                            pub mod world_relocate;
                        }
                    }
                }
            }
        }
    }
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
semio_framework_plugin::semio_plugin! {
    id: "puzzle", label: "Puzzle", version: "0.1.0",
    setup: artifacts::puzzle2d::engine::register,
    apps: [
        apps::puzzle2d::create_puzzle2d_app => apps::puzzle2d::Puzzle2dPlayApp,
        apps::puzzle3d::create_puzzle3d_app => apps::puzzle3d::Puzzle3dPlayApp,
        apps::puzzle5d::create_puzzle5d_app => apps::puzzle5d::Puzzle5dPlayApp,
    ],
}
//#endregion 🔖️Plugin
