//! Stdio plugin glue — zero-app library of well-known file-format artifacts.
//!
//! WIRING ONLY. Every pub mod points at one taxonomy component via #[path].

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as schema;

//#region Plugin
#[path = "../../🦀️component.rs"]
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
        pub mod standards {
            #[path = "."]
            pub mod v_raw {
                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_raw::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_raw::engine::*;
        }
        pub mod io {
            pub use super::standards::v_raw::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/💾️binary/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/💾️binary/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/💾️binary/🎹️composer/🦀️component.rs"]
        pub mod composer;

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
        pub mod standards {
            #[path = "."]
            pub mod v_utf_8 {
                #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_utf_8::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_utf_8::engine::*;
        }
        pub mod io {
            pub use super::standards::v_utf_8::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/📄txt/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📄txt/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📄txt/🎹️composer/🦀️component.rs"]
        pub mod composer;

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
        pub mod standards {
            #[path = "."]
            pub mod v_rfc8259 {
                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_rfc8259::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_rfc8259::engine::*;
        }
        pub mod io {
            pub use super::standards::v_rfc8259::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/🔣️json/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🔣️json/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/🔣️json/🎹️composer/🦀️component.rs"]
        pub mod composer;

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

    #[path = "."]
    pub mod xml {
        #[path = "../../🗿️artifacts/📰xml/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1_0 {
                #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1_0::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1_0::engine::*;
        }
        pub mod io {
            pub use super::standards::v1_0::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/📰xml/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📰xml/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📰xml/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📰xml/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]
    pub mod csv {
        #[path = "../../🗿️artifacts/📊️csv/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_rfc4180 {
                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_rfc4180::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_rfc4180::engine::*;
        }
        pub mod io {
            pub use super::standards::v_rfc4180::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/📊️csv/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📊️csv/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📊️csv/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📊️csv/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]
    pub mod md {
        #[path = "../../🗿️artifacts/📝️md/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_commonmark {
                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_commonmark::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_commonmark::engine::*;
        }
        pub mod io {
            pub use super::standards::v_commonmark::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/📝️md/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📝️md/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📝️md/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📝️md/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]
    pub mod deflate {
        #[path = "../../🗿️artifacts/🗜️deflate/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_rfc1950 {
                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_rfc1950::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_rfc1950::engine::*;
        }
        pub mod io {
            pub use super::standards::v_rfc1950::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/🗜️deflate/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🗜️deflate/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/🗜️deflate/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🗜️deflate/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]
    pub mod zip {
        #[path = "../../🗿️artifacts/🎒️zip/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v2_0 {
                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod deflate {
                                            #[path = "."]
                                            pub mod v_rfc1950 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod deflate {
                                            #[path = "."]
                                            pub mod v_rfc1950 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v2_0::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v2_0::engine::*;
        }
        pub mod io {
            pub use super::standards::v2_0::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/🎒️zip/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🎒️zip/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/🎒️zip/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🎒️zip/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }







    #[path = "."]
    pub mod step {
        #[path = "../../🗿️artifacts/📐️step/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ap214 {
                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_ap214::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_ap214::engine::*;
        }
        pub mod io {
            pub use super::standards::v_ap214::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/📐️step/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📐️step/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📐️step/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📐️step/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod ifc {
        #[path = "../../🗿️artifacts/🏗️ifc/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v4 {
                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v4::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v4::engine::*;
        }
        pub mod io {
            pub use super::standards::v4::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/🏗️ifc/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🏗️ifc/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/🏗️ifc/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🏗️ifc/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod las {
        #[path = "../../🗿️artifacts/☁️las/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1_0 {
                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1_0::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1_0::engine::*;
        }
        pub mod io {
            pub use super::standards::v1_0::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/☁️las/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/☁️las/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/☁️las/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/☁️las/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod gltf {
        #[path = "../../🗿️artifacts/🧊️gltf/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v2_0 {
                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v2_0::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v2_0::engine::*;
        }
        pub mod io {
            pub use super::standards::v2_0::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/🧊️gltf/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🧊️gltf/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/🧊️gltf/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🧊️gltf/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod obj {
        #[path = "../../🗿️artifacts/🧊️obj/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v3_0 {
                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v3_0::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v3_0::engine::*;
        }
        pub mod io {
            pub use super::standards::v3_0::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/🧊️obj/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🧊️obj/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/🧊️obj/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🧊️obj/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod ply {
        #[path = "../../🗿️artifacts/☁️ply/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1_0 {
                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1_0::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1_0::engine::*;
        }
        pub mod io {
            pub use super::standards::v1_0::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/☁️ply/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/☁️ply/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/☁️ply/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/☁️ply/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod dxf {
        #[path = "../../🗿️artifacts/🖊️dxf/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_r12 {
                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_r12::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_r12::engine::*;
        }
        pub mod io {
            pub use super::standards::v_r12::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/🖊️dxf/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🖊️dxf/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/🖊️dxf/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🖊️dxf/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod stl {
        #[path = "../../🗿️artifacts/🟪️stl/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ascii {
                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_ascii::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_ascii::engine::*;
        }
        pub mod io {
            pub use super::standards::v_ascii::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/🟪️stl/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🟪️stl/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/🟪️stl/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🟪️stl/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod svg {
        #[path = "../../🗿️artifacts/🎨️svg/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1_1 {
                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod xml {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📰xml/🔖️1.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                        pub mod xml {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📰xml/🔖️1.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1_1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1_1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1_1::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/🎨️svg/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🎨️svg/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/🎨️svg/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🎨️svg/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod bmp {
        #[path = "../../🗿️artifacts/🖼️bmp/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_v3 {
                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_v3::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_v3::engine::*;
        }
        pub mod io {
            pub use super::standards::v_v3::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/🖼️bmp/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🖼️bmp/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/🖼️bmp/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🖼️bmp/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod dwg {
        #[path = "../../🗿️artifacts/🖊️dwg/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ac1018 {
                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_ac1018::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_ac1018::engine::*;
        }
        pub mod io {
            pub use super::standards::v_ac1018::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/🖊️dwg/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🖊️dwg/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/🖊️dwg/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🖊️dwg/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod png {
        #[path = "../../🗿️artifacts/📷️png/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1_2 {
                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod deflate {
                                            #[path = "."]
                                            pub mod v_rfc1950 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod deflate {
                                            #[path = "."]
                                            pub mod v_rfc1950 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1_2::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1_2::engine::*;
        }
        pub mod io {
            pub use super::standards::v1_2::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/📷️png/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📷️png/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📷️png/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📷️png/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }







    #[path = "."]
    #[path = "."]
    pub mod pdf {
        #[path = "../../🗿️artifacts/📄️pdf/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1_4 {
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod deflate {
                                            #[path = "."]
                                            pub mod v_rfc1950 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod deflate {
                                            #[path = "."]
                                            pub mod v_rfc1950 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1_4::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1_4::engine::*;
        }
        pub mod io {
            pub use super::standards::v1_4::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/📄️pdf/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📄️pdf/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📄️pdf/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📄️pdf/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }







    #[path = "."]
    #[path = "."]
    pub mod jpg {
        #[path = "../../🗿️artifacts/📷️jpg/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_jfif_1_01 {
                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_jfif_1_01::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_jfif_1_01::engine::*;
        }
        pub mod io {
            pub use super::standards::v_jfif_1_01::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/📷️jpg/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📷️jpg/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📷️jpg/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📷️jpg/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]
    #[path = "."]
    pub mod gif {
        #[path = "../../🗿️artifacts/🎞️gif/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v87a {
                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v87a::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v87a::engine::*;
        }
        pub mod io {
            pub use super::standards::v87a::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/🎞️gif/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🎞️gif/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/🎞️gif/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🎞️gif/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]
    #[path = "."]
    pub mod tiff {
        #[path = "../../🗿️artifacts/🖼️tiff/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v6_0 {
                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v6_0::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v6_0::engine::*;
        }
        pub mod io {
            pub use super::standards::v6_0::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/🖼️tiff/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🖼️tiff/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/🖼️tiff/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🖼️tiff/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

    #[path = "."]
    #[path = "."]
    pub mod docx {
        #[path = "../../🗿️artifacts/📜️docx/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ecma_376 {
                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xml {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📰xml/🔖️1.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xml {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📰xml/🔖️1.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_ecma_376::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_ecma_376::engine::*;
        }
        pub mod io {
            pub use super::standards::v_ecma_376::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/📜️docx/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📜️docx/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📜️docx/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📜️docx/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }







    #[path = "."]
    #[path = "."]
    pub mod pptx {
        #[path = "../../🗿️artifacts/🎞️pptx/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ecma_376 {
                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xml {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📰xml/🔖️1.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xml {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📰xml/🔖️1.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_ecma_376::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_ecma_376::engine::*;
        }
        pub mod io {
            pub use super::standards::v_ecma_376::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/🎞️pptx/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🎞️pptx/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/🎞️pptx/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🎞️pptx/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }







    #[path = "."]
    #[path = "."]
    pub mod xlsx {
        #[path = "../../🗿️artifacts/📕️xlsx/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ecma_376 {
                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xml {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📰xml/🔖️1.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xml {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📰xml/🔖️1.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_ecma_376::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_ecma_376::engine::*;
        }
        pub mod io {
            pub use super::standards::v_ecma_376::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/📕️xlsx/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📕️xlsx/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📕️xlsx/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📕️xlsx/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }







    #[path = "."]
    #[path = "."]
    pub mod bcf {
        #[path = "../../🗿️artifacts/💬️bcf/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v2_1 {
                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xml {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📰xml/🔖️1.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xml {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📰xml/🔖️1.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v2_1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v2_1::engine::*;
        }
        pub mod io {
            pub use super::standards::v2_1::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/💬️bcf/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/💬️bcf/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/💬️bcf/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/💬️bcf/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }







    #[path = "."]
    #[path = "."]
    pub mod glb {
        #[path = "../../🗿️artifacts/🧊️glb/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v2_0 {
                #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                            #[path = "."]
                                            pub mod v_raw {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️glb/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v2_0::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v2_0::engine::*;
        }
        pub mod io {
            pub use super::standards::v2_0::subsets::any::io::*;
        }

        #[path = "../../🗿️artifacts/🧊️glb/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🧊️glb/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/🧊️glb/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🧊️glb/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }

}
//#endregion Artifacts

