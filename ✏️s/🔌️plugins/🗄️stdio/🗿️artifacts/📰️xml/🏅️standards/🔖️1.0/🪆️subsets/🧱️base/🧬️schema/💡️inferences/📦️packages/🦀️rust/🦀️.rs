//! 💡️ inferences of `semio-s-artifact-stdio-xml` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::XmlSnapshot;
pub mod schema {
    pub use super::standards::v1_0::subsets::base::schema::*;
}
pub use semio_s_artifact_stdio_xml_snapshot::{STDIO_XML_DOCUMENT_SCHEMA};
pub use crate::editor::xml_valid::edit_rules::artifact_items_2::{XML_ARTIFACT_SCHEMA_ID};
pub use crate::editor::xml_valid::edit_rules::artifact_items_3::{ARTIFACT_DEFINITION_SCHEMA};
pub use crate::editor::xml_valid::edit_rules::artifact_items_4::{artifact_kind};
pub use crate::editor::xml_valid::edit_rules::artifact_items_8::{formats};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod examples {
                    #[path = "."]
                    pub mod demo {
                        #[path = "../../../../📚️examples/🎬️demo/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_xml_snapshot::standards::v1_0::subsets::base::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod outline {
                            #[path = "../../🧾outline/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
            }
            #[path = "."]
            pub mod valid {
                #[path = "."]
                pub mod examples {
                    #[path = "."]
                    pub mod demo {
                        #[path = "../../../../../✅️valid/📚️examples/🎬️demo/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod no_doctype {
                        #[path = "../../../../../✅️valid/📚️examples/🚫️no-doctype/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod xml_any {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod xml_valid {
        #[path = "../../../../../✅️valid/✏️editor/🧭️edit-rules/🦀️.rs"]
        pub mod edit_rules;
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../✅️valid/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../✅️valid/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod xml_any {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod xml_valid {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../✅️valid/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../✅️valid/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
