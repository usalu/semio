//! 🪜️ `stdio.dwg` SQLite projection of the drawing graph: header, drawing, associativity, actions, styles, layouts and constraints (part 2).

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
                            #[path = "../✏️drawing/🦀️.rs"]
                            pub mod drawing;
                            #[path = "../🔗️associativity/🦀️.rs"]
                            pub mod associativity;
                            #[path = "../🖌️styles/🧱️material/🦀️.rs"]
                            pub mod material;
                            #[path = "../🖌️styles/↗️mleader/🦀️.rs"]
                            pub mod mleader_style;
                            #[path = "../📏️constraints/🦀️.rs"]
                            pub mod constraints;
                        }
                    }
                }
            }
        }
    }
}
