//! 📤️ The `document` subset's export conversions: the serializer leaves that carry `s.stdio.semio` `document` documents out of docx, md, pdf, txt, and the
//! `CONVERSION` that registers and declares their composer rows.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;

    #[path = "."]
    pub mod serializers {
        #[path = "."]
        pub mod artifacts {
            #[path = "."]
            pub mod docx {
                #[path = "."]
                pub mod v_ecma_376 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧵️serializers/🗿️artifacts/📜️docx/🔖️ecma-376/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod md {
                #[path = "."]
                pub mod v_commonmark {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
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
                        #[path = "🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod pdf {
                #[path = "."]
                pub mod v1_7 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.7/✳️any/🦀️.rs"]
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
    ENTRIES.get_or_init(|| vec![serializer_entry_of::<serializers::artifacts::docx::v_ecma_376::any::SemioDocumentToDocx>(), serializer_entry_of::<serializers::artifacts::md::v_commonmark::any::SemioDocumentToMd>(), serializer_entry_of::<serializers::artifacts::txt::v_utf_8::any::SemioDocumentToTxt>(), serializer_entry_of::<serializers::artifacts::pdf::v1_7::any::SemioDocumentToPdf>()]).as_slice()
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

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
