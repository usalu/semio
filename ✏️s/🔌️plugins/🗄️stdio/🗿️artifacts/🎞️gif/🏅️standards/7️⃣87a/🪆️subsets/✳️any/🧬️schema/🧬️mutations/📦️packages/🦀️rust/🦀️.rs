//! 🧬️ mutations of `semio-s-artifact-stdio-gif` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v87a {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_gif_schema::standards::v87a::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_gif_inferences::standards::v87a::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_gif_inferences::standards::v87a::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod v89a {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_gif_snapshot::standards::v89a::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_stdio_gif_schema::standards::v89a::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod dimensions {
                            pub use semio_s_artifact_stdio_gif_schema::standards::v89a::subsets::any::schema::inferences::dimensions::*;
                            #[cfg(test)]
                            //#region 🧪️Tests
                            #[path = "../../../../../../../9️⃣89a/🪆️subsets/🧱️base/🧬️schema/💡️inferences/📐dimensions/🧪️tests/🔬️unit/🦀️.rs"]
                            mod tests;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "../../../../../../../9️⃣89a/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../../../9️⃣89a/🪆️subsets/🧱️base/🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../../../9️⃣89a/🪆️subsets/🧱️base/🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                }
            }
        }
    }
}
