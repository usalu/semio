//! 📤️ The `animation` subset's export conversions: the serializer leaves that carry `s.stdio.semio` `animation` documents out of gif, gltf, mp4, and the
//! `CONVERSION` that registers and declares their composer rows.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;

#[path = "."]
pub mod serializers {
    #[path = "."]
    pub mod artifacts {
        #[path = "."]
        pub mod gif {
            #[path = "."]
            pub mod v89a {
                #[path = "."]
                pub mod any {
                    #[path = "🧵️serializers/🗿️artifacts/🎞️gif/🔖️89a/✳️any/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
        #[path = "."]
        pub mod gltf {
            #[path = "."]
            pub mod v2_0 {
                #[path = "."]
                pub mod any {
                    #[path = "🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs"]
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
                    #[path = "🧵️serializers/🗿️artifacts/🎥️mp4/🔖️isobmff/✳️any/🦀️.rs"]
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
    ENTRIES.get_or_init(|| vec![serializer_entry_of::<serializers::artifacts::gltf::v2_0::any::SemioAnimationToGltf>(), serializer_entry_of::<serializers::artifacts::mp4::v_isobmff::any::SemioAnimationToMp4>(), serializer_entry_of::<serializers::artifacts::gif::v89a::any::SemioAnimationToGif>()]).as_slice()
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
