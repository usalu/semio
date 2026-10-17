//! ✏️ `stdio.dwg` SQLite projection of the drawing graph: header, drawing, associativity, actions, styles, layouts and constraints.

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
                            #[path = "../../../🔧️header/🦀️.rs"]
                            pub mod header;
                            #[path = "../../../🎬️actions/🦀️.rs"]
                            pub mod actions;
                            #[path = "../../../🖌️styles/👁️visual/🦀️.rs"]
                            pub mod visual_style;
                            #[path = "../../../🖌️styles/🗃️table/🦀️.rs"]
                            pub mod table_style;
                            #[path = "../../../📃️layout/🦀️.rs"]
                            pub mod layout;
                            pub use semio_s_artifact_stdio_dwg_sqlite_drawing_2::standards::v_ac1024::subsets::any::io::sqlite::snapshot::{drawing, associativity, material, mleader_style, constraints};
                        }
                    }
                }
            }
        }
    }
}
