//! 💡️ inferences of `semio-s-artifact-stdio-zip` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::ZipSnapshot;
pub mod schema {
    pub use super::standards::v2_0::subsets::base::schema::*;
}
pub use crate::viewer::zip::base::modes::view::artifact_items::{STDIO_ZIP_DOCUMENT_SCHEMA};
pub use crate::viewer::zip::base::modes::view::artifact_items_2::{ZIP_ARTIFACT_SCHEMA_ID};
pub use crate::viewer::zip::base::modes::view::artifact_items_3::{ARTIFACT_DEFINITION_SCHEMA};
pub use crate::viewer::zip::base::modes::view::artifact_items_4::{artifact_kind};
pub use crate::viewer::zip::base::modes::view::artifact_items_8::{formats};

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
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../../📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod entries {
                            #[path = "../../🗃️entries/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "../../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                }
            }
            #[path = "."]
            pub mod iso21320 {
                #[path = "."]
                pub mod schema {
                    #[path = "../../../../../🌐️iso21320/🧬️schema/🛡️conformance/🦀️.rs"]
                    pub mod conformance;
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
                    #[path = "../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod windows {
                        #[path = "."]
                        pub mod main {
                            #[path = "../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }
        #[path = "."]
        pub mod iso21320 {
            #[path = "."]
            pub mod modes {
                #[path = "."]
                pub mod view {
                    #[path = "../../../../../🌐️iso21320/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod windows {
                        #[path = "."]
                        pub mod main {
                            #[path = "../../../../../🌐️iso21320/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }
    }
}
