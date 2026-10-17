//! 🔖️ v2-0 of `semio-s-artifact-stdio-gltf-model` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v2_0 {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "../../🪆️subsets/♾️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🦀️.rs"]
                    pub mod serializers_artifacts_json;
                    #[path = "../../🪆️subsets/♾️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🦀️.rs"]
                    pub mod export_serializers_artifacts;
                    #[path = "../../🪆️subsets/♾️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🦀️.rs"]
                    pub mod deserializers_artifacts_json;
                    #[path = "../../🪆️subsets/♾️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🦀️.rs"]
                    pub mod import_deserializers_artifacts;
                    #[path = "../../🪆️subsets/♾️any/🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                }
            }
        }
    }
}
