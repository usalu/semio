//! 💡️ inferences of `semio-s-artifact-stdio-jpg` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::JpgSnapshot;
pub use schema::snapshot::JpgImage;
pub mod schema {
    pub use super::standards::v_jfif_1_01::subsets::document::schema::*;
}
pub use crate::standards::v_jfif_1_01::subsets::document::io::artifact_items::{STDIO_JPG_DOCUMENT_SCHEMA};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_jfif_1_01 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod document {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../../📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod dimensions {
                            #[path = "../../📐dimensions/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_jpg_operations::standards::v_jfif_1_01::subsets::document::io::transforms_and_tables;
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod snapshot {
                            #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🧮️decoded-components/🦀️.rs"]
                            pub mod decoded_components;
                            #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🎛️encode-options/🦀️.rs"]
                            pub mod encode_options;
                        }
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod snapshot {
                            #[path = "../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "."]
                        pub mod snapshot {
                            #[path = "../../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod baseline {
                #[path = "."]
                pub mod schema {
                    #[path="../../../../../🧱️baseline/🧬️schema/🏅️conformance/🦀️.rs"]
                    pub mod conformance;
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    pub use semio_s_artifact_stdio_jpg_operations::examples::*;
    #[path = "."]
    pub mod demo {
        pub use semio_s_artifact_stdio_jpg_operations::examples::demo::*;
        #[cfg(test)]
        #[path = "../../../../📚️examples/🎬️demo/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
}
