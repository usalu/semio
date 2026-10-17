//! 🧬️ mutations of `semio-s-artifact-stdio-zip` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::ZipSnapshot;
#[cfg(test)]
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub mod schema {
    pub use super::standards::v2_0::subsets::base::schema::*;
}

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v2_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_zip_schema_2::standards::v2_0::subsets::base::schema::*;
                    pub use semio_s_artifact_stdio_zip_inferences::standards::v2_0::subsets::base::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_stdio_zip_inferences::standards::v2_0::subsets::base::schema::inferences::*;
                        pub use semio_s_artifact_stdio_zip_inferences::standards::v2_0::subsets::base::schema::inferences::*;
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../../💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "../../../🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
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
                        pub use semio_s_artifact_stdio_zip_inferences::standards::v2_0::subsets::base::io::binary::snapshot;
                        #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        #[path = "../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_zip_inferences::standards::v2_0::subsets::base::io::text::snapshot;
                        #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        #[path = "../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                }
            }
            #[path = "."]
            pub mod iso21320 {
                #[path = "."]
                pub mod schema {
                    #[path = "../../../../../🌐️iso21320/🧬️schema/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod zip {
        #[path = "."]
        pub mod base {
            #[path = "."]
            pub mod modes {
                #[path = "."]
                pub mod view {
                    pub use semio_s_artifact_stdio_zip_inferences::viewer::zip::base::modes::view::*;
                    #[cfg(test)]
                    #[path = "../../../../👁️viewer/🎭️modes/👁️view/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                    pub use semio_s_artifact_stdio_zip_inferences::viewer::zip::base::modes::view::windows;
                }
            }
        }
    }
}
