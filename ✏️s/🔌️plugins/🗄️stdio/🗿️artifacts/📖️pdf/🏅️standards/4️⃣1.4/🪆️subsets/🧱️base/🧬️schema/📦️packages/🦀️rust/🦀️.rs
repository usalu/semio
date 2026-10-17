//! 🧬️ schema of `semio-s-artifact-stdio-pdf` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_4 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_pdf_snapshot::standards::v1_4::subsets::base::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod outline {
                            #[path = "../../💡️inferences/🧾outline/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod v1_7 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod modules {
                    #[path = "."]
                    pub mod fonts {
                        #[path = "."]
                        pub mod truetype {
                            pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::modules::fonts::truetype::*;
                            #[cfg(test)]
                            #[path = "../../../../../../7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔤️fonts/🔠️truetype/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
        }
    }
}
