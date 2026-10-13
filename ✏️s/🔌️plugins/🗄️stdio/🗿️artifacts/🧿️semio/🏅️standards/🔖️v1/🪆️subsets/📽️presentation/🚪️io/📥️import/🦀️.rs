//! 📥️ The `presentation` subset's import conversions: the deserializer leaves that carry `s.stdio.semio` `presentation` documents into pptx, and the
//! `CONVERSION` that registers and declares their composer rows.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;

    #[path = "."]
    pub mod deserializers {
        #[path = "."]
        pub mod artifacts {
            #[path = "."]
            pub mod pptx {
                #[path = "."]
                pub mod v_ecma_376 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧩️deserializers/🗿️artifacts/🎞️pptx/🔖️ecma-376/✳️any/🦀️.rs"]
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
    ENTRIES.get_or_init(|| vec![deserializer_entry_of::<deserializers::artifacts::pptx::v_ecma_376::any::SemioPresentationFromPptx>()]).as_slice()
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
