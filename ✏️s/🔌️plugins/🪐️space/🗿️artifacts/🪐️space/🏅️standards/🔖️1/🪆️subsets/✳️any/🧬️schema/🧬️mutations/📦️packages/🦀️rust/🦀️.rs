//! 🧬️ mutations of `semio-s-artifact-space-space` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "../../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "../../../../🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_space_space_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "../../../⚙️operations/🦀️.rs"]
                    pub mod operations;
                    pub use semio_s_artifact_space_space_schema::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod create_artifact {
                            #[path = "../../🌱create-artifact/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🌱create-artifact/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🌱create-artifact/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "../../🌱create-artifact/🧪️tests/🧪️appends/🦀️.rs"]
                            mod tests_appends_artifact_3_to_the_index;
                        }
                        #[path = "."]
                        pub mod delete_artifact {
                            #[path = "../../🗑️delete-artifact/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🗑️delete-artifact/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🗑️delete-artifact/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "../../🗑️delete-artifact/🧪️tests/🧪️removes/🦀️.rs"]
                            mod tests_removes_artifact_2_from_the_index;
                        }
                        #[path = "."]
                        pub mod rename_artifact {
                            #[path = "../../🏷️rename-artifact/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🏷️rename-artifact/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🏷️rename-artifact/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "../../🏷️rename-artifact/🧪️tests/🧪️renames-artifact-1/🦀️.rs"]
                            mod tests_renames_artifact_1;
                        }
                        #[path = "."]
                        pub mod touch_artifact {
                            #[path = "../../🕒touch-artifact/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🕒touch-artifact/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🕒touch-artifact/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "../../🕒touch-artifact/🧪️tests/🧪️stamps/🦀️.rs"]
                            mod tests_stamps_artifact_1_with_a_new_editor;
                        }
                    }
                }
            }
        }
    }
}
