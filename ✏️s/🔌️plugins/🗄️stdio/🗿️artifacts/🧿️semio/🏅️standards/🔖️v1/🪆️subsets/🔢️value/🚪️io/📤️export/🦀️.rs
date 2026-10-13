//! 📤️ The `value` subset's export conversions: the serializer leaves that carry `s.stdio.semio` `value` documents out of csv, json, xml, and the
//! `CONVERSION` that registers and declares their composer rows.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;

    #[path = "."]
    pub mod serializers {
        #[path = "."]
        pub mod artifacts {
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod v_rfc8259 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod xml {
                #[path = "."]
                pub mod v1_0 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧵️serializers/🗿️artifacts/📰️xml/🔖️1.0/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod v_rfc4180 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
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
    ENTRIES.get_or_init(|| vec![serializer_entry_of::<serializers::artifacts::json::v_rfc8259::any::SemioValueToJson>(), serializer_entry_of::<serializers::artifacts::xml::v1_0::any::SemioValueToXml>(), serializer_entry_of::<serializers::artifacts::csv::v_rfc4180::any::SemioValueToCsv>()]).as_slice()
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
