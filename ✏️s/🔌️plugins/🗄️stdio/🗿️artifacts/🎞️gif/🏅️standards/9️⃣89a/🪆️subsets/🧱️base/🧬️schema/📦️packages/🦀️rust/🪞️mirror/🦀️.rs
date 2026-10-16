//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

pub mod io_registry {
    use crate::standards::v87a::subsets::any::io::io_registry as v87a;
    use crate::standards::v89a::subsets::any::io::io_registry as v89a;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    /// 🎹️ Both standards' entries, dialect-keyed (`writes.standard == "87a"` vs `"89a"`) — this is
    /// how a caller reaches 89a's real multi-frame codec: `compose` below picks the entry whose
    /// `writes` matches the requested `Dialect`, so 87a and 89a coexist without either shadowing
    /// the other (unlike the flat schema/document-codec registries, which are standard-agnostic
    /// pre-D4 and would collide — see `standards::v89a::engine::register`'s doc comment).
    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v87a::entries().iter().chain(v89a::entries().iter()).collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("GifComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v87a::entries()).expect("static Stdio registration must be available and conflict-free");
        register_composer_entries(v89a::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v87a {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_gif_schema::standards::v87a::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_gif_inferences::standards::v87a::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_gif_inferences::standards::v87a::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_gif_inferences::standards::v87a::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_gif_mutations::standards::v87a::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../../7️⃣87a/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
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
                                            #[path = "../../../../../../../7️⃣87a/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_stdio_gif_fixture_tests::standards::v87a::subsets::any::io::binary::*;
                        pub use semio_s_artifact_stdio_gif_inferences::standards::v87a::subsets::any::io::binary::snapshot;
                        pub use semio_s_artifact_stdio_gif_mutations::standards::v87a::subsets::any::io::binary::mutations;
                        pub use semio_s_artifact_stdio_gif_inferences::standards::v87a::subsets::any::io::binary::diff;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_gif_fixture_tests::standards::v87a::subsets::any::io::text::*;
                        pub use semio_s_artifact_stdio_gif_inferences::standards::v87a::subsets::any::io::text::snapshot;
                        pub use semio_s_artifact_stdio_gif_mutations::standards::v87a::subsets::any::io::text::mutations;
                        pub use semio_s_artifact_stdio_gif_inferences::standards::v87a::subsets::any::io::text::diff;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod v89a {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "../../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_gif_snapshot::standards::v89a::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_gif_schema::standards::v89a::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_gif_mutations::standards::v89a::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_gif_mutations_2::standards::v89a::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_stdio_gif_fixture_tests::standards::v89a::subsets::any::io::binary::*;
                        pub use semio_s_artifact_stdio_gif_snapshot::standards::v89a::subsets::any::io::binary::snapshot;
                        pub use semio_s_artifact_stdio_gif_mutations_2::standards::v89a::subsets::any::io::binary::mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_gif_fixture_tests::standards::v89a::subsets::any::io::text::*;
                        pub use semio_s_artifact_stdio_gif_snapshot::standards::v89a::subsets::any::io::text::snapshot;
                        pub use semio_s_artifact_stdio_gif_mutations_2::standards::v89a::subsets::any::io::text::mutations;
                        pub use semio_s_artifact_stdio_gif_mutations::standards::v89a::subsets::any::io::text::diff;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod dancing {
        #[path = "../../../../../../../7️⃣87a/🪆️subsets/✳️any/📚️examples/💃️dancing/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
