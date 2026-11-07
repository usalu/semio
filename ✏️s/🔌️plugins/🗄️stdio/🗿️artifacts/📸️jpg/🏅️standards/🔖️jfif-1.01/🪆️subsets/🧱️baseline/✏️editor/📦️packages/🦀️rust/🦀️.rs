//! ✏️ JPEG app and IO assembly owns executable editors, viewers and their window components.
#![allow(async_fn_in_trait)]
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;
pub use schema::snapshot::{JpgImage, JpgSnapshot};
pub use schema::diff::JpgDiff;
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use schema::mutations::JpgMutation;
pub mod schema {
    pub use super::standards::v_jfif_1_01::subsets::document::schema::*;
}
pub use semio_s_artifact_stdio_jpg_operations::{ARTIFACT_DEFINITION_SCHEMA, formats, examples};
pub use semio_s_artifact_stdio_jpg_inferences::STDIO_JPG_DOCUMENT_SCHEMA;
pub use semio_s_artifact_stdio_jpg_mutations::{JPG_ANY_DIALECT, JPG_BASELINE_DIALECT, native_codecs};
pub use editor::jpg_any::artifact_items_7::definition;
pub use editor::jpg_baseline::artifact_items_10::{assembly, declaration, declared_subset_validators, pilot_languages};
pub use editor::jpg_baseline::artifact_items_11::contribution;

pub mod io_registry {
    use crate::standards::v_jfif_1_01::subsets::baseline::io::io_registry as baseline;
    use semio_plugin::io::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, ErasedComposeSource};
    use semio_framework_artifact_reference::Dialect;
    use std::sync::OnceLock;
    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| baseline::entries().iter().collect()).as_slice()
    }
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|entry| entry.writes == target).ok_or_else(|| ComposeError {message:format!("JpgComposer: no entry writes {target:?}"), diagnostics:Vec::new()})?;
        semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }
    pub fn register() {
        register_composer_entries(baseline::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_jfif_1_01 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod document {
                pub mod schema {
                    pub use semio_s_artifact_stdio_jpg_schema::standards::v_jfif_1_01::subsets::document::schema::*;
                    pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::schema::{snapshot, inferences};
                    pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::schema::{diff, mutations};
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::*;
                    #[cfg(test)]
                    #[path = "../../../../🧾️document/🚪️io/🧪️tests/🔬️unit/🦀️.rs"]
                    mod tests;
                    #[path = "../../../../🧾️document/🚪️io/🎹️facets/🦀️.rs"]
                    pub mod facets;
                    pub mod binary {
                        pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::binary::snapshot;
                        pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::io::binary::mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::text::snapshot;
                        pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::io::text::diff;
                        pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::io::text::mutations;
                    }
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
                                        #[path = "../../../../🧾️document/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                        pub mod any;
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
                                        #[path = "../../../../🧾️document/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                        pub mod any;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            #[path = "."]
            pub mod baseline {
                #[path = "../../../🚪️io/🦀️.rs"]
                pub mod io;
                pub use semio_s_artifact_stdio_jpg_schema_2::standards::v_jfif_1_01::subsets::baseline::schema;
            }
        }
    }
}
#[path = "."]
pub mod editor {
    pub use semio_s_artifact_stdio_jpg_schema_2::editor::paint_region_declaration;
    #[path = "."]
    pub mod jpg_any {
        #[path = "../../../../🧾️document/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        pub(crate) use component::paint_region;
        #[cfg(test)]
        #[path = "../../../../🧾️document/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
        mod tests;
        pub use semio_s_artifact_stdio_jpg_operations::editor::jpg_any::edit_rules;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../🧾️document/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../🧾️document/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                    pub mod main;
                }
            }
        }
    }
    #[path = "."]
    pub mod jpg_baseline {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        pub(crate) use component::paint_region;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                    pub mod main;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod jpg_any {
        #[path = "../../../../🧾️document/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "../../../../🧾️document/👁️viewer/🧪️tests/🔬️unit/🦀️.rs"]
        mod tests;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../🧾️document/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../🧾️document/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                    pub mod main;
                }
            }
        }
    }
    #[path = "."]
    pub mod jpg_baseline {
        #[path = "../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "../../../👁️viewer/🧪️tests/🔬️unit/🦀️.rs"]
        mod tests;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                    pub mod main;
                }
            }
        }
    }
}
