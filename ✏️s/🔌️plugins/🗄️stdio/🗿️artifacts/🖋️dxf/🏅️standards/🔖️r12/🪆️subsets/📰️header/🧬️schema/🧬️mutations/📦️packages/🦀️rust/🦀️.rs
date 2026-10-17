//! 🧬️ mutations of `semio-s-artifact-stdio-dxf` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::DxfSnapshot;
pub use schema::snapshot::DxfArc;
pub use schema::snapshot::DxfCircle;
pub use schema::snapshot::DxfInsert;
pub use schema::snapshot::DxfLine;
pub use schema::snapshot::DxfOther;
pub use schema::snapshot::DxfPolyline;
pub use schema::snapshot::DxfSolid;
pub use schema::snapshot::DxfText;
pub mod schema {
    pub use super::standards::v_r12::subsets::any::schema::*;
}

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_r12 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_dxf_snapshot::standards::v_r12::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_dxf_diff::standards::v_r12::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_stdio_dxf_inferences::standards::v_r12::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_stdio_dxf_inferences::standards::v_r12::subsets::any::schema::inferences::*;
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../../💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
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
                        #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        pub use semio_s_artifact_stdio_dxf_diff::standards::v_r12::subsets::any::io::binary::diff;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_stdio_dxf_snapshot::standards::v_r12::subsets::any::io::text::snapshot::entity_codec;
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod dxf {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_dxf_inferences::editor::dxf::modes::edit::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_dxf_inferences::editor::dxf::modes::edit::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod dxf {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                pub use semio_s_artifact_stdio_dxf_inferences::viewer::dxf::modes::view::windows;
            }
        }
    }
}
