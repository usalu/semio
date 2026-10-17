//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

pub mod io_registry {
    use crate::standards::v_rfc8259::subsets::base::io::io_registry as v_rfc8259;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_rfc8259::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("JsonComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v_rfc8259::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_rfc8259 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_json_schema_2::standards::v_rfc8259::subsets::base::schema::*;
                    pub use semio_s_artifact_stdio_json_schema::standards::v_rfc8259::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_json_schema::standards::v_rfc8259::subsets::base::schema::inferences;
                    pub use semio_s_artifact_stdio_json_mutations::standards::v_rfc8259::subsets::base::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_json_mutations::standards::v_rfc8259::subsets::base::schema::mutations::*;
                        #[path = "."]
                        pub mod set_scalar {
                            pub use semio_s_artifact_stdio_json_mutations::standards::v_rfc8259::subsets::base::schema::mutations::set_scalar::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🧱️base/🧬️schema/🧬️mutations/🔢️set-scalar/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[cfg(test)]
                        #[path = "../../../../../../../🧱️base/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                        #[path = "."]
                        pub mod remove_array_element {
                            pub use semio_s_artifact_stdio_json_mutations::standards::v_rfc8259::subsets::base::schema::mutations::remove_array_element::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🧱️base/🧬️schema/🧬️mutations/📤️remove-array-element/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod insert_array_element {
                            pub use semio_s_artifact_stdio_json_mutations::standards::v_rfc8259::subsets::base::schema::mutations::insert_array_element::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🧱️base/🧬️schema/🧬️mutations/📥️insert-array-element/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_member {
                            pub use semio_s_artifact_stdio_json_mutations::standards::v_rfc8259::subsets::base::schema::mutations::remove_member::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🧱️base/🧬️schema/🧬️mutations/🗑️remove-member/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_member {
                            pub use semio_s_artifact_stdio_json_mutations::standards::v_rfc8259::subsets::base::schema::mutations::set_member::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🧱️base/🧬️schema/🧬️mutations/✏️set-member/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../../🧱️base/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_json_mutations_2::standards::v_rfc8259::subsets::base::io::text::*;
                        pub use semio_s_artifact_stdio_json_schema::standards::v_rfc8259::subsets::base::io::text::snapshot;
                        #[path = "."]
                        pub mod mutations {
                            pub use semio_s_artifact_stdio_json_mutations::standards::v_rfc8259::subsets::base::io::text::mutations::*;
                            #[path = "../../../../../../../🧱️base/🚪️io/📝️text/🧬️mutations/✏️set-member/🦀️.rs"]
                            pub mod set_member;
                            #[path = "../../../../../../../🧱️base/🚪️io/📝️text/🧬️mutations/📤️remove-array-element/🦀️.rs"]
                            pub mod remove_array_element;
                            #[path = "../../../../../../../🧱️base/🚪️io/📝️text/🧬️mutations/🗑️remove-member/🦀️.rs"]
                            pub mod remove_member;
                            #[path = "../../../../../../../🧱️base/🚪️io/📝️text/🧬️mutations/📥️insert-array-element/🦀️.rs"]
                            pub mod insert_array_element;
                            #[path = "../../../../../../../🧱️base/🚪️io/📝️text/🧬️mutations/🔢️set-scalar/🦀️.rs"]
                            pub mod set_scalar;
                        }
                        pub use semio_s_artifact_stdio_json_mutations::standards::v_rfc8259::subsets::base::io::text::diff;
                    }
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_stdio_json_schema::standards::v_rfc8259::subsets::base::io::sqlite::snapshot::*;
                            #[cfg(test)]
                            #[path="../../../../../../../🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🦀️.rs"]
                            pub mod original_receiving_tests;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod geojson {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_json_schema::standards::v_rfc8259::subsets::geojson::schema::*;
                    #[cfg(test)]
                    #[path = "../../../🦀️.rs"]
                    pub mod tests;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod i_json {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_json_schema_2::standards::v_rfc8259::subsets::i_json::schema::*;
                    pub use semio_s_artifact_stdio_json_mutations_2::standards::v_rfc8259::subsets::i_json::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../../🛜️i-json/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
