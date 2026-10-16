//! 👁️ viewer of `semio-s-artifact-stdio-stl` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use schema::snapshot::StlSnapshot;
pub mod engine {
    pub use super::standards::v_ascii::engine::*;
}
pub mod schema {
    pub use super::standards::v_ascii::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_stl_inferences::{STDIO_STL_DOCUMENT_SCHEMA};
pub use crate::viewer::stl::artifact_items_3::{ARTIFACT_DEFINITION_SCHEMA};
pub use crate::viewer::stl::artifact_items_4::{artifact_kind};
pub use crate::viewer::stl::artifact_items_5::{native_codec};
pub use crate::viewer::stl::artifact_items_6::{native_codecs};
pub use crate::viewer::stl::artifact_items_7::{definition};
pub use crate::viewer::stl::artifact_items_8::{formats};
pub use crate::viewer::stl::artifact_items_9::{assembly, declaration, pilot_languages};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_ascii {
        pub use semio_s_artifact_stdio_stl_mutations::standards::v_ascii::engine;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_stl_schema::standards::v_ascii::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_stl_inferences::standards::v_ascii::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_stl_inferences::standards::v_ascii::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_stl_mutations::standards::v_ascii::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_stl_inferences::standards::v_ascii::subsets::any::io::text::snapshot;
                        pub use semio_s_artifact_stdio_stl_mutations::standards::v_ascii::subsets::any::io::text::mutations;
                        pub use semio_s_artifact_stdio_stl_mutations::standards::v_ascii::subsets::any::io::text::diff;
                    }
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_stdio_stl_schema::standards::v_ascii::subsets::any::io::binary::snapshot;
                        pub use semio_s_artifact_stdio_stl_mutations::standards::v_ascii::subsets::any::io::binary::mutations;
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod binary {
                                    #[path = "."]
                                    pub mod v_raw {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_stdio_stl_inferences::standards::v_ascii::subsets::any::io::sqlite::snapshot::*;
                            #[cfg(test)]
                            #[path="../../../🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🦀️.rs"]
                            mod original_receiving_tests;
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod binary {
                                    #[path = "."]
                                    pub mod v_raw {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
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
    pub mod stl {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_stl_inferences::viewer::stl::modes::view::*;
                pub use semio_s_artifact_stdio_stl_inferences::viewer::stl::modes::view::windows;
            }
        }
    }
}
