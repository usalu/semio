//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

pub mod io_registry {
    use crate::standards::v_utf_8::subsets::any::io::io_registry as v_utf_8;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_utf_8::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("TxtComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v_utf_8::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_utf_8 {
        #[path = "../../../../../../../../🦀️.rs"]
        mod component;
        pub use component::*;
        pub mod engine {
            pub use super::subsets::any::io::register;
        }
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../../../../../🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_txt_schema::standards::v_utf_8::subsets::any::schema::*;
                    #[cfg(test)]
                    #[path = "../../../🦀️.rs"]
                    mod tests;
                    #[path = "."]
                    pub mod mutation_support {
                        pub use semio_s_artifact_stdio_txt_inferences::standards::v_utf_8::subsets::any::schema::mutation_support::*;
                        #[cfg(test)]
                        #[path = "../../../../../🔨️modules/🧬️mutation-support/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                    pub use semio_s_artifact_stdio_txt_schema::standards::v_utf_8::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_txt_inferences::standards::v_utf_8::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_txt_inferences::standards::v_utf_8::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_txt_mutations::standards::v_utf_8::subsets::any::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../../../../🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                        #[path = "."]
                        pub mod set_line_ending {
                            pub use semio_s_artifact_stdio_txt_mutations::standards::v_utf_8::subsets::any::schema::mutations::set_line_ending::*;
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/🔚️set-line-ending/🧪️tests/🔬️unit/🦀️.rs"]
                            mod tests;
                        }
                        #[path = "."]
                        pub mod set_trailing_newline {
                            pub use semio_s_artifact_stdio_txt_mutations::standards::v_utf_8::subsets::any::schema::mutations::set_trailing_newline::*;
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/↩️set-trailing-newline/🧪️tests/🔬️unit/🦀️.rs"]
                            mod tests;
                        }
                        #[path = "."]
                        pub mod remove_line {
                            pub use semio_s_artifact_stdio_txt_mutations::standards::v_utf_8::subsets::any::schema::mutations::remove_line::*;
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/🗑️remove-line/🧪️tests/🔬️unit/🦀️.rs"]
                            mod tests;
                        }
                        #[path = "."]
                        pub mod set_line {
                            pub use semio_s_artifact_stdio_txt_mutations::standards::v_utf_8::subsets::any::schema::mutations::set_line::*;
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/✏️set-line/🧪️tests/🔬️unit/🦀️.rs"]
                            mod tests;
                        }
                        #[path = "."]
                        pub mod insert_line {
                            pub use semio_s_artifact_stdio_txt_mutations::standards::v_utf_8::subsets::any::schema::mutations::insert_line::*;
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/📥️insert-line/🧪️tests/🔬️unit/🦀️.rs"]
                            mod tests;
                        }
                        #[path = "."]
                        pub mod splice_text {
                            pub use semio_s_artifact_stdio_txt_mutations::standards::v_utf_8::subsets::any::schema::mutations::splice_text::*;
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/✂️splice-text/🧪️tests/🔬️unit/🦀️.rs"]
                            mod tests;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
pub use semio_s_artifact_stdio_txt_examples::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod txt {
        #[path = "../../../../../../✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_stdio_txt_examples::editor::txt::edit_rules;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_txt_inferences::editor::txt::modes::edit::*;
                pub use semio_s_artifact_stdio_txt_inferences::editor::txt::modes::edit::windows;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod txt {
        #[path = "../../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_txt_inferences::viewer::txt::modes::view::*;
                pub use semio_s_artifact_stdio_txt_inferences::viewer::txt::modes::view::windows;
            }
        }
    }
}
