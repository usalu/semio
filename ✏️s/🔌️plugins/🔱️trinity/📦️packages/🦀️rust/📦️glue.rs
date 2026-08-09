//! 🔺️ Trinity plugin — Jack and Rewrite apps in one hot-swappable WASM plugin.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]`
//! written in full, relative to THIS file's directory (`📦️packages/🦀️rust/`, per SHAPE V2 —
//! `26/08/05/SHAPE-V2-TREE-PURITY-BROADCAST` — so every leaf path carries a `../../` prefix to reach
//! back out to the owner-root tree). The grouping modules carry `#[path = "."]` so their own names
//! are not spliced into that base directory.

extern crate infinite_canvas as infinite_board_port_directed_normal;
extern crate infinite_canvas as infinite_board_port_directed;
#[allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as schema;
// 📌️ Command-group handler functions (`🎮️commands/<group>/component.rs`) are decomposed out of a
// single `DocumentApp::handle` match, one function per command — the uniform `Result<Emit<_, _>,
// Fault>` signature is dictated by the dispatch call site (some commands in the same group DO fail;
// others never do), so per-function `Ok(...)`-only bodies are intentional, not a mistake to unwrap.
#[allow(clippy::unnecessary_wraps)]

//#region 🔤️Jack kernel
#[path = "../../🌳️ast/🦀️component.rs"]
pub mod ast;
#[path = "../../🔤️lexer/🦀️component.rs"]
pub mod lexer;
#[path = "../../🧮️executor/🦀️component.rs"]
pub mod executor;
#[path = "../../🗣️language-service/🦀️component.rs"]
pub mod language_service;
pub use language_service as core;
//#endregion 🔤️Jack kernel

//#region 🔖️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod jack {
        #[path = "../../🗿️artifacts/🔌️jack/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/🔌️jack/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🔌️jack/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
            pub use schema::*;
        }

        #[path = "."]
        pub mod op {
            #[path = "../../🗿️artifacts/🔌️jack/🔧️op/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_data_property {
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/🎛set-data-property/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/🎛set-data-property/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/🎛set-data-property/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_fixture {
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/🎛set-fixture/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/🎛set-fixture/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/🎛set-fixture/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod clear_data_property {
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/📌clear-data-property/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/📌clear-data-property/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/📌clear-data-property/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod create_edge {
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/📌create-edge/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/📌create-edge/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/📌create-edge/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod create_node {
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/📌create-node/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/📌create-node/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/📌create-node/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod delete_edge {
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/📌delete-edge/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/📌delete-edge/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/📌delete-edge/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod delete_node {
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/📌delete-node/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/📌delete-node/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/📌delete-node/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod rename {
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/📌rename/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/📌rename/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/📌rename/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod reposition {
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/📌reposition/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/📌reposition/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️mutations/📌reposition/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

        }

        #[path = "."]
        pub mod dsl {
            #[path = "../../🗿️artifacts/🔌️jack/🗣️dsl/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/🔌️jack/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;
            #[path = "../../🗿️artifacts/🔌️jack/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
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

        #[path = "../../🗿️artifacts/♻️rewrite/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/♻️rewrite/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/♻️rewrite/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
            pub use schema::*;
        }

        #[path = "."]
        pub mod op {
            #[path = "../../🗿️artifacts/♻️rewrite/🔧️op/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/♻️rewrite/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_state {
                #[path = "../../🗿️artifacts/♻️rewrite/🧬️mutations/🎛set-state/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/♻️rewrite/🧬️mutations/🎛set-state/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/♻️rewrite/🧬️mutations/🎛set-state/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

        }

        #[path = "."]
        pub mod dsl {
            #[path = "../../🗿️artifacts/♻️rewrite/🗣️dsl/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/♻️rewrite/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;
            #[path = "../../🗿️artifacts/♻️rewrite/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
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

            #[path = "../../🎛️apps/🔌️jack/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/🔌️jack/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🔌️jack/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
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

            #[path = "../../🎛️apps/♻️rewrite/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/♻️rewrite/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/♻️rewrite/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
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
/// depending on the artifacts' concrete `Projection`/`Mutation` types.
fn register_trinity_exports() {
    artifacts::jack::engine::register();
    artifacts::rewrite::engine::register();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::jack::TrinityJackPlayApp>(artifacts::jack::TRINITY_GRAPH_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::rewrite::TrinityRewritePlayApp>(artifacts::rewrite::REWRITE_RULE_SCHEMA);
}

//#region 🔖️Plugin
#[path = "../../🔌️plugin/🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/♻️rewrite/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_rewrite_demo;
    #[path = "../../🗿️artifacts/🔌️jack/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_jack_demo;
    #[path = "../../🎛️apps/♻️rewrite/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_rewrite_demo_session;
    #[path = "../../🎛️apps/🔌️jack/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_jack_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
//#endregion 🔖️Bundle
