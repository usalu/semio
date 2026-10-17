//! 🧬️ schema of `semio-s-artifact-space-home` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::snapshot::SHomeSnapshot;
pub use semio_s_artifact_space_home_inferences::{S_HOME_DOCUMENT_SCHEMA};

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
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_space_home_inferences::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_space_home_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod digest {
                            pub use semio_s_artifact_space_home_inferences::standards::v1::subsets::any::schema::inferences::digest::*;
                            #[cfg(test)]
                            //#region 🧪️Tests
                            #[path = "../../💡️inferences/🆔digest/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod operations {
                        pub use semio_s_artifact_space_home_operations::standards::v1::subsets::any::schema::operations::*;
                        #[cfg(test)]
                        #[path = "../../⚙️operations/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_space_home_operations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod change_catalog_generation {
                            pub use semio_s_artifact_space_home_operations::standards::v1::subsets::any::schema::mutations::change_catalog_generation::*;
                            pub use semio_s_artifact_space_home_operations::standards::v1::subsets::any::schema::mutations::change_catalog_generation::diff;
                            pub use semio_s_artifact_space_home_operations::standards::v1::subsets::any::schema::mutations::change_catalog_generation::inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
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
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_space_home_examples::viewer::home::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_space_home_examples::viewer::home::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🏠️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
