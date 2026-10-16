//! 🔬️ tests of `semio-s-artifact-stdio-epw` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;

pub use semio_s_artifact_stdio_epw_io_registry::{declaration, definition};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod energyplus {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_epw_io_registry::standards::energyplus::subsets::any::io::*;
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_stdio_epw_snapshot::standards::energyplus::subsets::any::io::sqlite::snapshot::*;
                            #[path = "."]
                            pub mod receiving {
                                pub use semio_s_artifact_stdio_epw_snapshot::standards::energyplus::subsets::any::io::sqlite::snapshot::receiving::*;
                                #[cfg(test)]
                                #[path="../../../../🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🦀️.rs"]
                                mod tests;
                            }
                        }
                    }
                    #[cfg(test)]
                    #[path = "../../🦀️.rs"]
                    mod tests;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_epw_schema::standards::energyplus::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_epw_snapshot::standards::energyplus::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_epw_schema::standards::energyplus::subsets::any::schema::inferences;
                }
            }
        }
    }
}
