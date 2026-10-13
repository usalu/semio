//! 📤️ The `audio` subset's export conversions: the serializer leaves that carry `s.stdio.semio` `audio` documents out of mp3, wav, and the
//! `CONVERSION` that registers and declares their composer rows.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;

#[path = "."]
pub mod serializers {
    #[path = "."]
    pub mod artifacts {
        #[path = "."]
        pub mod mp3 {
            #[path = "."]
            pub mod v_mpeg1_layer3 {
                #[path = "."]
                pub mod any {
                    #[path = "🧵️serializers/🗿️artifacts/🎵️mp3/🔖️mpeg1-layer3/✳️any/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
        #[path = "."]
        pub mod wav {
            #[path = "."]
            pub mod v_riff_pcm {
                #[path = "."]
                pub mod any {
                    #[path = "🧵️serializers/🗿️artifacts/🔊️wav/🔖️riff-pcm/✳️any/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}

use semio_s_artifact_stdio_semio_base::SemioConversion;
use semio_framework_plugin::{serializer_entry_of, io::{register_composer_entries, ComposerEntry}, app::{ArtifactDeclarationBuilder, DeclarationReady}};

/// 🧾️ This direction's composer rows, built once.
pub fn entries() -> &'static [ComposerEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![serializer_entry_of::<serializers::artifacts::mp3::v_mpeg1_layer3::any::SemioAudioToMp3>(), serializer_entry_of::<serializers::artifacts::wav::v_riff_pcm::any::SemioAudioToWav>()]).as_slice()
}

/// 🔌️ Registers and declares this direction's composer rows; consumers name it in `register` / `declaration`.
pub const CONVERSION: SemioConversion = SemioConversion { declare, register };

fn declare(builder: ArtifactDeclarationBuilder<DeclarationReady>) -> ArtifactDeclarationBuilder<DeclarationReady> {
    static COMPOSERS: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    builder.composers(semio_s_artifact_stdio_semio_base::semio_written(entries(), &COMPOSERS))
}

fn register() {
    register_composer_entries(entries()).expect("static Stdio registration must be available and conflict-free");
}
