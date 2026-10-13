//! 📤️ The `image` subset's export conversions: the serializer leaves that carry `s.stdio.semio` `image` documents out of bmp, gif, jpg, png, tiff, and the
//! `CONVERSION` that registers and declares their composer rows.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;

    #[path = "."]
    pub mod serializers {
        #[path = "."]
        pub mod artifacts {
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod v1_2 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod jpg {
                #[path = "."]
                pub mod v_jfif_1_01 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧵️serializers/🗿️artifacts/📸️jpg/🔖️jfif-1.01/♾️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
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
            pub mod bmp {
                #[path = "."]
                pub mod v_v3 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧵️serializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod tiff {
                #[path = "."]
                pub mod v6_0 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧵️serializers/🗿️artifacts/🖼️tiff/🔖️6.0/✳️any/🦀️.rs"]
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
    ENTRIES.get_or_init(|| vec![serializer_entry_of::<serializers::artifacts::png::v1_2::any::SemioImageToPng>(), serializer_entry_of::<serializers::artifacts::jpg::v_jfif_1_01::any::SemioImageToJpg>(), serializer_entry_of::<serializers::artifacts::gif::v89a::any::SemioImageToGif>(), serializer_entry_of::<serializers::artifacts::bmp::v_v3::any::SemioImageToBmp>(), serializer_entry_of::<serializers::artifacts::tiff::v6_0::any::SemioImageToTiff>()]).as_slice()
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
