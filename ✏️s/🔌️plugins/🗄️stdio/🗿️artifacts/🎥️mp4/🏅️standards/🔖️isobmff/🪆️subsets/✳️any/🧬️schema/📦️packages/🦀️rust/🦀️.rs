//! 🧬️ schema of `semio-s-artifact-stdio-mp4` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod isobmff {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_mp4_inferences::standards::isobmff::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod diff {
                        #[path = "../../🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod h264 {
                        pub use semio_s_artifact_stdio_mp4_inferences::standards::isobmff::subsets::any::io::h264::*;
                        #[cfg(test)]
                        #[path = "../../../🚪️io/🎥️h264/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    pub use semio_s_artifact_stdio_mp4_inferences::examples::*;
    #[path = "."]
    pub mod demo {
        pub use semio_s_artifact_stdio_mp4_inferences::examples::demo::*;
        #[cfg(test)]
        #[path = "../../../📚️examples/🎬️demo/🧪️tests/🔬️unit/🦀️.rs"]
        mod tests;
    }
}
