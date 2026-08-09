//! 🔀️ DAG plugin — declarative DAG play app bundled as a hot-swappable WASM component.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_schema as schema;
#[allow(clippy::result_large_err)]

extern crate infinite_canvas as infinite_board_port_directed_dag;

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod dag {
        #[path = "../../🗿️artifacts/🕸️dag/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/🕸️dag/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "../../🗿️artifacts/🕸️dag/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
            pub use schema::*;
        }

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/🕸️dag/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;
            #[path = "../../🗿️artifacts/🕸️dag/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "../../🗿️artifacts/🕸️dag/🔧️op/🦀️component.rs"]
        pub mod op;

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/🕸️dag/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod nodes {
                #[path = "../../🗿️artifacts/🕸️dag/🧬️mutations/🔗nodes/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🕸️dag/🧬️mutations/🔗nodes/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod edges {
                #[path = "../../🗿️artifacts/🕸️dag/🧬️mutations/➡️edges/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🕸️dag/🧬️mutations/➡️edges/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod set_nodes {
                #[path = "../../🗿️artifacts/🕸️dag/🧬️mutations/📋set-nodes/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🕸️dag/🧬️mutations/📋set-nodes/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod set_edges {
                #[path = "../../🗿️artifacts/🕸️dag/🧬️mutations/📋set-edges/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🕸️dag/🧬️mutations/📋set-edges/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/🕸️dag/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🕸️dag/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/🕸️dag/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/🕸️dag/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/🕸️dag/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod dag {
        #[path = "../../🎛️apps/🕸️dag/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/🕸️dag/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🕸️dag/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/🕸️dag/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🕸️dag/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
        #[path = "../../🎛️apps/🕸️dag/🗣️terminology/🦀️component.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🕸️dag/🎮️commands/🔧️nodes/🦀️component.rs"]
            pub mod nodes;
            #[path = "../../🎛️apps/🕸️dag/🎮️commands/🕸️graph/🦀️component.rs"]
            pub mod graph;
            #[path = "../../🎛️apps/🕸️dag/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🕸️dag/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🕸️dag/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/🕸️dag/🎭️modes/✏️edit/🪟️windows/🕸️main/🦀️component.rs"]
                    pub mod main;
                    #[path = "../../🎛️apps/🕸️dag/🎭️modes/✏️edit/🪟️windows/🧬️compiled/🦀️component.rs"]
                    pub mod compiled;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🕸️dag/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🕸️dag/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/🕸️dag/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
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
    #[path = "../../🗿️artifacts/🕸️dag/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_dag_demo;
    #[path = "../../🎛️apps/🕸️dag/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_dag_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
