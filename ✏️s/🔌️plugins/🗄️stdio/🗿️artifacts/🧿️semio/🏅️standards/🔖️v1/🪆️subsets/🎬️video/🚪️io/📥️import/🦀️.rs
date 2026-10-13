//! 📥️ The `video` subset's import conversions: the deserializer leaves that carry `s.stdio.semio` `video` documents into avi, mp4, and the
//! `CONVERSION` that registers and declares their composer rows.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;

#[path = "."]
pub mod deserializers {
    #[path = "."]
    pub mod artifacts {
        #[path = "."]
        pub mod avi {
            #[path = "."]
            pub mod v1_0 {
                #[path = "."]
                pub mod any {
                    #[path = "🧩️deserializers/🗿️artifacts/📼️avi/🔖️1.0/✳️any/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
        #[path = "."]
        pub mod mp4 {
            #[path = "."]
            pub mod v_isobmff {
                #[path = "."]
                pub mod any {
                    #[path = "🧩️deserializers/🗿️artifacts/🎥️mp4/🔖️isobmff/✳️any/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}

use semio_s_artifact_stdio_semio_base::SemioConversion;
use semio_framework_plugin::{deserializer_entry_of, io::{register_composer_entries, ComposerEntry}, app::{ArtifactDeclarationBuilder, DeclarationReady}};

/// 🧾️ This direction's composer rows, built once.
pub fn entries() -> &'static [ComposerEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![deserializer_entry_of::<deserializers::artifacts::mp4::v_isobmff::any::SemioVideoFromMp4>(), deserializer_entry_of::<deserializers::artifacts::avi::v1_0::any::SemioVideoFromAvi>()]).as_slice()
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
