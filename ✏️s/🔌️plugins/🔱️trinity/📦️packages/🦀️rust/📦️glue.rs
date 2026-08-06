//! 🔺️ Trinity plugin — Jack and Rewrite apps in one hot-swappable WASM plugin.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]`
//! written in full, relative to THIS file's directory (`📦️packages/🦀️rust/`, per SHAPE V2 —
//! `26/08/05/SHAPE-V2-TREE-PURITY-BROADCAST` — so every leaf path carries a `../../` prefix to reach
//! back out to the owner-root tree). The grouping modules carry `#[path = "."]` so their own names
//! are not spliced into that base directory.

#![allow(clippy::result_large_err)]
// 📌️ Command-group handler functions (`🎮️commands/<group>/component.rs`) are decomposed out of a
// single `DocumentApp::handle` match, one function per command — the uniform `Result<Emit<_, _>,
// Fault>` signature is dictated by the dispatch call site (some commands in the same group DO fail;
// others never do), so per-function `Ok(...)`-only bodies are intentional, not a mistake to unwrap.
#![allow(clippy::unnecessary_wraps)]

//#region 🔖️Core
#[path = "."]
pub mod core {
    #[path = "../../🫀️core/🦀️component.rs"]
    mod component;
    pub use component::*;
}
//#endregion 🔖️Core

//#region 🔖️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod jack {
        #[path = "../../🗿️artifacts/🔌️jack/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/🔌️jack/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod op {
            #[path = "../../🗿️artifacts/🔌️jack/🔧️op/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod dsl {
            #[path = "../../🗿️artifacts/🔌️jack/🗣️dsl/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod pack {
            #[path = "../../🗿️artifacts/🔌️jack/🎒️pack/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod spr {
            #[path = "../../🗿️artifacts/🔌️jack/📡️spr/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/🔌️jack/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;
        }
    }

    #[path = "."]
    pub mod rewrite {
        #[path = "../../🗿️artifacts/♻️rewrite/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/♻️rewrite/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod op {
            #[path = "../../🗿️artifacts/♻️rewrite/🔧️op/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod dsl {
            #[path = "../../🗿️artifacts/♻️rewrite/🗣️dsl/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod pack {
            #[path = "../../🗿️artifacts/♻️rewrite/🎒️pack/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod spr {
            #[path = "../../🗿️artifacts/♻️rewrite/📡️spr/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/♻️rewrite/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;
        }
    }
}
//#endregion 🔖️Artifacts

//#region 🔖️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod jack {
        #[path = "../../🎛️apps/🔌️jack/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/🔌️jack/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod terminology {
            #[path = "../../🎛️apps/🔌️jack/🗣️terminology/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[cfg(target_arch = "wasm32")]
        #[path = "../../🎛️apps/🔌️jack/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub(crate) mod fixture {
                #[path = "../../🎛️apps/🔌️jack/🎮️commands/🗺️fixture/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod query {
                #[path = "../../🎛️apps/🔌️jack/🎮️commands/🔎️query/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod view {
                #[path = "../../🎛️apps/🔌️jack/🎮️commands/👁️view/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
        }

        #[path = "."]
        pub mod windows {
            #[path = "."]
            pub(crate) mod graph {
                #[path = "../../🎛️apps/🔌️jack/🪟️windows/🌐️graph/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod editor {
                #[path = "../../🎛️apps/🔌️jack/🪟️windows/📝️editor/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod results {
                #[path = "../../🎛️apps/🔌️jack/🪟️windows/📊️results/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub(crate) mod document {
                #[path = "../../🎛️apps/🔌️jack/📌️panels/📄️document/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod catalogue {
                #[path = "../../🎛️apps/🔌️jack/📌️panels/📚️catalogue/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod inspection {
                #[path = "../../🎛️apps/🔌️jack/📌️panels/🔍️inspection/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
        }
    }

    #[path = "."]
    pub mod rewrite {
        #[path = "../../🎛️apps/♻️rewrite/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/♻️rewrite/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod terminology {
            #[path = "../../🎛️apps/♻️rewrite/🗣️terminology/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod world {
            #[path = "../../🎛️apps/♻️rewrite/🌍️world/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub(crate) mod rule {
                #[path = "../../🎛️apps/♻️rewrite/🎮️commands/📜️rule/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod view {
                #[path = "../../🎛️apps/♻️rewrite/🎮️commands/👁️view/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
        }

        #[path = "."]
        pub mod windows {
            #[path = "."]
            pub(crate) mod before {
                #[path = "../../🎛️apps/♻️rewrite/🪟️windows/⬅️before/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod after {
                #[path = "../../🎛️apps/♻️rewrite/🪟️windows/➡️after/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod lhs {
                #[path = "../../🎛️apps/♻️rewrite/🪟️windows/⬅️lhs/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod rhs {
                #[path = "../../🎛️apps/♻️rewrite/🪟️windows/➡️rhs/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod jack {
                #[path = "../../🎛️apps/♻️rewrite/🪟️windows/🔎️jack/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod parameters {
                #[path = "../../🎛️apps/♻️rewrite/🪟️windows/🎛️parameters/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub(crate) mod document {
                #[path = "../../🎛️apps/♻️rewrite/📌️panels/📄️document/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod catalogue {
                #[path = "../../🎛️apps/♻️rewrite/📌️panels/📚️catalogue/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod inspection {
                #[path = "../../🎛️apps/♻️rewrite/📌️panels/🔍️inspection/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
        }
    }
}
//#endregion 🔖️Apps

//#region 🔖️Bundle
/// 🗂️ Registers this crate's two document kinds' pack↔dsl codecs so `framework/sync`'s
/// `FolderEndpoint::Pack` (and any other schema-string-keyed caller) can print/parse them without
/// depending on the artifacts' concrete `Projection`/`Operation` types.
fn register_trinity_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::jack::TrinityJackPlayApp>(artifacts::jack::TRINITY_GRAPH_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::rewrite::TrinityRewritePlayApp>(artifacts::rewrite::REWRITE_RULE_SCHEMA);
}

semio_framework_plugin::semio_plugin! {
    id: "trinity",
    label: "Trinity",
    version: "0.1.0",
    setup: register_trinity_exports,
    apps: [
        apps::jack::create_trinity_jack_app => apps::jack::TrinityJackPlayApp,
        apps::rewrite::create_rewrite_app => apps::rewrite::TrinityRewritePlayApp,
    ]
}
//#endregion 🔖️Bundle
