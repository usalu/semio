//! ⚙️ operations of `semio-s-artifact-space-home` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::snapshot::SHomeSnapshot;
pub use crate::standards::v1::subsets::any::schema::diff::SHomeDiff;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_space_home_inferences::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_space_home_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_space_home_inferences::standards::v1::subsets::any::schema::diff;
                    #[path = "../../🦀️.rs"]
                    pub mod operations;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod change_catalog_generation {
                            #[path = "../../../🧬️mutations/🔢️change-catalog-generation/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🧬️mutations/🔢️change-catalog-generation/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧬️mutations/🔢️change-catalog-generation/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "../../../🧬️mutations/🔢️change-catalog-generation/🧪️tests/🧪️bumps/🦀️.rs"]
                            pub mod tests_bumps_the_catalog_generation_to_7;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        #[path = "../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        #[path = "../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[cfg(test)]
        #[path = "../../../../📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
        pub mod tests;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod home {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_space_home_examples::viewer::home::modes::view::*;
                #[cfg(test)]
                #[path = "../../../../👁️viewer/🎭️modes/👁️view/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
                pub use semio_s_artifact_space_home_examples::viewer::home::modes::view::windows;
            }
        }
    }
}
