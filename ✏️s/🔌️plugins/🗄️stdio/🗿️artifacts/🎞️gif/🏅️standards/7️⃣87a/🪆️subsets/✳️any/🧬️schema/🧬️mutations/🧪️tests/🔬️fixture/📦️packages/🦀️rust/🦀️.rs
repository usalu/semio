//! 🔬️ fixture-tests of `semio-s-artifact-stdio-gif` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;

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
                    pub use semio_s_artifact_stdio_gif_inferences::standards::v87a::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_gif_inferences::standards::v87a::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_gif_mutations::standards::v87a::subsets::any::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../🦀️.rs"]
                        mod fixture_tests;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "../../../../../../🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
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
                    pub use semio_s_artifact_stdio_gif_mutations::standards::v89a::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_gif_mutations_2::standards::v89a::subsets::any::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../../../../../../../../9️⃣89a/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧪️tests/🔬️kinds/🦀️.rs"]
                        mod kinds_tests;
                        #[cfg(test)]
                        #[path = "../../../../../../../../../9️⃣89a/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs"]
                        mod fixture_tests;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../../../../9️⃣89a/🪆️subsets/🧱️base/🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "../../../../../../../../../9️⃣89a/🪆️subsets/🧱️base/🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                }
            }
        }
    }
}
