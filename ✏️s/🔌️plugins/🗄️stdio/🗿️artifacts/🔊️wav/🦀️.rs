//! 🎪 `stdio.wav` artifact — new-format artifact (master plan "New format artifacts" table).

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub use standards::riff_pcm::subsets::any::schema::diff::WavDiff;
pub use standards::riff_pcm::subsets::any::schema::mutations::WavMutation;
pub use standards::riff_pcm::subsets::any::schema::snapshot::WavSnapshot;
pub use standards::riff_pcm::subsets::any::schema::WavArtifact;

pub use semio_s_artifact_stdio_wav_io_registry::{STDIO_WAV_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_wav_schema::{WAV_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_wav_io_registry::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_wav_io_registry::{definition};

pub use semio_s_artifact_stdio_wav_io_registry::{formats};

pub use semio_s_artifact_stdio_wav_schema::{native_codecs};

pub use semio_s_artifact_stdio_wav_io_registry::{contribution};

pub use semio_s_artifact_stdio_wav_io_registry::{WAV_DIALECT};

pub use semio_s_artifact_stdio_wav_io_registry::{artifact_kind, assembly, declaration};
//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_wav_io_registry::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod riff_pcm {
        /// 🗂️ ⚙️️→🚪️ dissolution shim (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
        /// the real codec/`io_registry` moved to `subsets::any::io`; this inline module keeps
        /// only `register()` reachable at its historical path because the plugin root
        /// (`✏️s/🔌️plugins/🗄️stdio/🦀️.rs`) still calls
        /// `standards::riff_pcm::engine::register()` imperatively (one of the ticket's 10
        /// protected `dsl::registry` entrypoints — its call site is explicitly not to be
        /// touched).
        pub use semio_s_artifact_stdio_wav_io_registry::standards::riff_pcm::engine;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub mod io {
    pub use semio_s_artifact_stdio_wav_io_registry::standards::riff_pcm::subsets::any::io::*;
    pub mod binary {
        pub use semio_s_artifact_stdio_wav_schema::standards::riff_pcm::subsets::any::io::binary::*;
        pub use semio_s_artifact_stdio_wav_inferences::standards::riff_pcm::subsets::any::io::binary::diff;
        pub use semio_s_artifact_stdio_wav_inferences::standards::riff_pcm::subsets::any::io::binary::inferences;
        pub use semio_s_artifact_stdio_wav_mutations::standards::riff_pcm::subsets::any::io::binary::mutations;
        pub use semio_s_artifact_stdio_wav_inferences::standards::riff_pcm::subsets::any::io::binary::snapshot;
    }
    pub mod sqlite {
        pub use semio_s_artifact_stdio_wav_schema::standards::riff_pcm::subsets::any::io::sqlite::*;
        pub use semio_s_artifact_stdio_wav_inferences::standards::riff_pcm::subsets::any::io::sqlite::snapshot;
    }
}
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_wav_schema::standards::riff_pcm::subsets::any::schema::*;

                    pub use semio_s_artifact_stdio_wav_inferences::standards::riff_pcm::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_wav_inferences::standards::riff_pcm::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_wav_inferences::standards::riff_pcm::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_wav_mutations::standards::riff_pcm::subsets::any::schema::mutations;
                }
            }
        }
    }
}
pub use semio_s_artifact_stdio_wav_inferences::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod wav {
        pub use semio_s_artifact_stdio_wav_io_registry::editor::wav::*;

        pub use semio_s_artifact_stdio_wav_inferences::editor::wav::edit_rules;
        pub use semio_s_artifact_stdio_wav_inferences::editor::wav::presence;
        pub use semio_s_artifact_stdio_wav_inferences::editor::wav::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_wav_io_registry::editor::wav::modes::edit::*;

                pub use semio_s_artifact_stdio_wav_io_registry::editor::wav::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod wav {
        pub use semio_s_artifact_stdio_wav_io_registry::viewer::wav::*;

        pub use semio_s_artifact_stdio_wav_inferences::viewer::wav::presence;
        pub use semio_s_artifact_stdio_wav_inferences::viewer::wav::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_wav_mutations::viewer::wav::modes::view::*;

                pub use semio_s_artifact_stdio_wav_mutations::viewer::wav::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::riff_pcm::subsets::any::io::{WavBuilderConstruction, WavParts, WavAnalyzerAnalysis, WavBuilderFacets, WavBuilder, WavAnalyzer, WavComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
