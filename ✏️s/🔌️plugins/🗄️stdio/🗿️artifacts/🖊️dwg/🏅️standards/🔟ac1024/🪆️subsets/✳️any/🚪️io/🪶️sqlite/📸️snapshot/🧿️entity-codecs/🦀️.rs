//! 🧿️ `stdio.dwg` SQLite projection of entities: the entity record codec with its dimension and viewport bodies.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_ac1024 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "."]
                        pub mod snapshot {
                            use semio_s_artifact_stdio_dwg_sqlite_objects::standards::v_ac1024::subsets::any::io::sqlite::snapshot::{color, number, reader};
                            #[path = "../📐️entities/🦀️.rs"]
                            pub mod entities;
                            #[path = "../📐️entities/📏️dimension/🦀️.rs"]
                            pub mod entity_dimension;
                            #[path = "../📐️entities/🖼️viewport/🦀️.rs"]
                            pub mod entity_viewport;
                        }
                    }
                }
            }
        }
    }
}
