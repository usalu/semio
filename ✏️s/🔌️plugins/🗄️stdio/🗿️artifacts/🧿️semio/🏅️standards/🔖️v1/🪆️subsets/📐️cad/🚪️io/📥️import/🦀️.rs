//! 📥️ The `cad` subset's import conversions: the deserializer leaves that carry `s.stdio.semio` `cad` documents into dwg, dxf, step, and the
//! `CONVERSION` that registers and declares their composer rows.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;

    #[path = "."]
    pub mod deserializers {
        #[path = "."]
        pub mod artifacts {
            #[path = "."]
            pub mod dxf {
                #[path = "."]
                pub mod v_r12 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧩️deserializers/🗿️artifacts/🔄️dxf/🔖️r12/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod dwg {
                #[path = "."]
                pub mod v_ac1024 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod step {
                #[path = "."]
                pub mod v_ap214 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧩️deserializers/🗿️artifacts/📐️step/🔖️ap214/✳️any/🦀️.rs"]
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
    ENTRIES.get_or_init(|| vec![deserializer_entry_of::<deserializers::artifacts::dxf::v_r12::any::SemioCadFromDxf>(), deserializer_entry_of::<deserializers::artifacts::dwg::v_ac1024::any::SemioCadFromDwg>(), deserializer_entry_of::<deserializers::artifacts::step::v_ap214::any::SemioCadFromStep>()]).as_slice()
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
