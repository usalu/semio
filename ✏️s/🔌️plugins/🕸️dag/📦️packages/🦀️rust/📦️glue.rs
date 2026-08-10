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
        pub use crate::artifacts::dag::schema::snapshot::DagSnapshot;
        pub use crate::artifacts::dag::schema::mutations::DagMutation;
        pub use crate::artifacts::dag::schema::diff::DagDiff;

        #[path = "."]
        pub mod schema {
            #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod snapshot {
                #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/📸️snapshot/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod diff {
                #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🔺️diff/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                pub mod text;
                pub use text::*;
                #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod mutations {
                #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🧬️mutations/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                pub mod binary;
                #[path = "."]
                pub mod edges {
                    #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🧬️mutations/➡️edges/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🧬️mutations/➡️edges/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🧬️mutations/➡️edges/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_snapshot {
                    #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_edges {
                    #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🧬️mutations/📋set-edges/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🧬️mutations/📋set-edges/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🧬️mutations/📋set-edges/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_nodes {
                    #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🧬️mutations/📋set-nodes/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🧬️mutations/📋set-nodes/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🧬️mutations/📋set-nodes/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod nodes {
                    #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🧬️mutations/🔗nodes/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🧬️mutations/🔗nodes/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🕸️dag/🧬️schema/🧬️mutations/🔗nodes/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
            }
        }

        pub mod op { pub use crate::artifacts::dag::schema::mutations::text::*; pub use crate::artifacts::dag::schema::mutations::DagMutation; }
        pub mod dsl { pub use crate::artifacts::dag::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::dag::schema::mutations::binary::*; }
        pub mod pack { pub use crate::artifacts::dag::schema::snapshot::binary::*; }
        pub mod diff { pub use crate::artifacts::dag::schema::diff::*; pub mod schema { pub use crate::artifacts::dag::schema::diff::*; } pub mod text { pub use crate::artifacts::dag::schema::diff::text::*; } }
        pub mod mutations { pub use crate::artifacts::dag::schema::mutations::*; }
        #[path = "."]
        pub mod snapshot {
            pub mod schema { pub use crate::artifacts::dag::schema::snapshot::*; }
            pub mod pack { pub use crate::artifacts::dag::schema::snapshot::binary::*; }
        }
        #[path = "../../🗿️artifacts/🕸️dag/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🕸️dag/🪓️decomposer/🦀️component.rs"]
        pub mod decomposer;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/🕸️dag/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod import {
                #[path = "."]
                pub mod deserializers {
                    #[path = "."]
                    pub mod artifacts {
                        #[path = "."]
                        pub mod csv {
                            #[path = "../../🗿️artifacts/🕸️dag/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🕸️dag/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod md {
                            #[path = "../../🗿️artifacts/🕸️dag/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod png {
                            #[path = "../../🗿️artifacts/🕸️dag/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod svg {
                            #[path = "../../🗿️artifacts/🕸️dag/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🦀️component.rs"]
                            mod component;
                            pub use component::*;
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
                        pub mod csv {
                            #[path = "../../🗿️artifacts/🕸️dag/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🕸️dag/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod md {
                            #[path = "../../🗿️artifacts/🕸️dag/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod png {
                            #[path = "../../🗿️artifacts/🕸️dag/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod svg {
                            #[path = "../../🗿️artifacts/🕸️dag/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::dag::io::export::serializers::artifacts::csv::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::dag::io::import::deserializers::artifacts::csv::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::dag::io::export::serializers::artifacts::json::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::dag::io::import::deserializers::artifacts::json::*;
                }
            }
            #[path = "."]
            pub mod md {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::dag::io::export::serializers::artifacts::md::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::dag::io::import::deserializers::artifacts::md::*;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::dag::io::export::serializers::artifacts::png::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::dag::io::import::deserializers::artifacts::png::*;
                }
            }
            #[path = "."]
            pub mod svg {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::dag::io::export::serializers::artifacts::svg::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::dag::io::import::deserializers::artifacts::svg::*;
                }
            }
        }
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
