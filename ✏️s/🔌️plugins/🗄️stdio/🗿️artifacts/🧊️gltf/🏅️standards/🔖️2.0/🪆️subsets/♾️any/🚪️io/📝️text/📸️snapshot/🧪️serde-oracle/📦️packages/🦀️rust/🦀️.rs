//! 🧪️ schema of `semio-s-artifact-stdio-gltf-model` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v2_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "../../../../../../🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_gltf_model_snapshot::standards::v2_0::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_gltf_model_snapshot_root::standards::v2_0::subsets::any::schema::snapshot_root;
                    #[path = "."]
                    pub mod diff {
                        #[path = "../../../../../../🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_gltf_model_snapshot::standards::v2_0::subsets::any::io::*;
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod snapshot {
                            #[cfg(any(test, feature = "testing"))]
                            #[path = "../../🦀️.rs"]
                            pub mod serde_oracle;
                            #[cfg(any(test, feature = "testing"))]
                            #[path = "../../../🧪️semantic-cache-codec/🦀️.rs"]
                            pub mod semantic_cache_codec;
                        }
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "../../../../../📥️import/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "../../../../../📤️export/🦀️.rs"]
                        mod component;
                        pub use component::*;
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
                                            #[path = "../../../../../📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_stdio_gltf_model_snapshot_root::standards::v2_0::subsets::any::io::sqlite::snapshot::*;
                            #[cfg(test)]
                            #[path = "../../../../../🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
        }
    }
}
