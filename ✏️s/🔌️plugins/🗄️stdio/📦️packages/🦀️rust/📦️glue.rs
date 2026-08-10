//! Stdio plugin glue — zero-app library of well-known file-format artifacts.
//!
//! WIRING ONLY. Every pub mod points at one taxonomy component via #[path].

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as schema;

//#region Plugin
#[path = "../../🔌️plugin/🦀️component.rs"]
pub mod plugin;
pub use plugin::plugin;
//#endregion Plugin

//#region Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod binary {
        #[path = "../../🗿️artifacts/💾️binary/🦀️component.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod schema {
            #[path = "../../🗿️artifacts/💾️binary/🧬️schema/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod snapshot {
                #[path = "../../🗿️artifacts/💾️binary/🧬️schema/📸️snapshot/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/💾️binary/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/💾️binary/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod diff {
                #[path = "../../🗿️artifacts/💾️binary/🧬️schema/🔺️diff/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/💾️binary/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/💾️binary/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod mutations {
                #[path = "../../🗿️artifacts/💾️binary/🧬️schema/🧬️mutations/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/💾️binary/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/💾️binary/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                pub mod binary;
                #[path = "."]
                pub mod set_snapshot {
                    #[path = "../../🗿️artifacts/💾️binary/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/💾️binary/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/💾️binary/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
            }
        }
        #[path = "../../🗿️artifacts/💾️binary/⚙️engine/🦀️component.rs"]
        pub mod engine;
        #[path = "../../🗿️artifacts/💾️binary/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/💾️binary/🪓️decomposer/🦀️component.rs"]
        pub mod decomposer;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/💾️binary/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod import {
                #[path = "."]
                pub mod deserializers {
                    #[path = "."]
                    pub mod artifacts {
                        #[path = "."]
                        pub mod binary {
                            #[path = "../../🗿️artifacts/💾️binary/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🦀️component.rs"]
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
                        pub mod binary {
                            #[path = "../../🗿️artifacts/💾️binary/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }
        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/💾️binary/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]
    pub mod txt {
        #[path = "../../🗿️artifacts/📄txt/🦀️component.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod schema {
            #[path = "../../🗿️artifacts/📄txt/🧬️schema/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod snapshot {
                #[path = "../../🗿️artifacts/📄txt/🧬️schema/📸️snapshot/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/📄txt/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/📄txt/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod diff {
                #[path = "../../🗿️artifacts/📄txt/🧬️schema/🔺️diff/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/📄txt/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/📄txt/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod mutations {
                #[path = "../../🗿️artifacts/📄txt/🧬️schema/🧬️mutations/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/📄txt/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/📄txt/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                pub mod binary;
                #[path = "."]
                pub mod set_snapshot {
                    #[path = "../../🗿️artifacts/📄txt/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/📄txt/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/📄txt/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
            }
        }
        #[path = "../../🗿️artifacts/📄txt/⚙️engine/🦀️component.rs"]
        pub mod engine;
        #[path = "../../🗿️artifacts/📄txt/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📄txt/🪓️decomposer/🦀️component.rs"]
        pub mod decomposer;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/📄txt/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod import {
                #[path = "."]
                pub mod deserializers {
                    #[path = "."]
                    pub mod artifacts {
                        #[path = "."]
                        pub mod binary {
                            #[path = "../../🗿️artifacts/📄txt/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🦀️component.rs"]
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
                        pub mod binary {
                            #[path = "../../🗿️artifacts/📄txt/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }
        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📄txt/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]
    pub mod json {
        #[path = "../../🗿️artifacts/🔣️json/🦀️component.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod schema {
            #[path = "../../🗿️artifacts/🔣️json/🧬️schema/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod snapshot {
                #[path = "../../🗿️artifacts/🔣️json/🧬️schema/📸️snapshot/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🔣️json/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🔣️json/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod diff {
                #[path = "../../🗿️artifacts/🔣️json/🧬️schema/🔺️diff/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🔣️json/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🔣️json/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod mutations {
                #[path = "../../🗿️artifacts/🔣️json/🧬️schema/🧬️mutations/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🔣️json/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🔣️json/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                pub mod binary;
                #[path = "."]
                pub mod set_snapshot {
                    #[path = "../../🗿️artifacts/🔣️json/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🔣️json/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🔣️json/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
            }
        }
        #[path = "../../🗿️artifacts/🔣️json/⚙️engine/🦀️component.rs"]
        pub mod engine;
        #[path = "../../🗿️artifacts/🔣️json/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🔣️json/🪓️decomposer/🦀️component.rs"]
        pub mod decomposer;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/🔣️json/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod import {
                #[path = "."]
                pub mod deserializers {
                    #[path = "."]
                    pub mod artifacts {
                        #[path = "."]
                        pub mod txt {
                            #[path = "../../🗿️artifacts/🔣️json/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🦀️component.rs"]
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
                        pub mod txt {
                            #[path = "../../🗿️artifacts/🔣️json/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }
        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🔣️json/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
//#endregion Artifacts

