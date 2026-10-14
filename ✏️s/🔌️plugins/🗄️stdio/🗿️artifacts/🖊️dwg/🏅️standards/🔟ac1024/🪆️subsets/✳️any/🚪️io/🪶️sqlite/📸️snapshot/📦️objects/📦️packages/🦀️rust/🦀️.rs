//! 📦️ `stdio.dwg` SQLite projection of objects: numeric cells, readers, colors, xrecords, tables, entities, blocks and evaluation.

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
                            #[path = "../../../🔢️number/🦀️.rs"]
                            pub mod number;
                            #[path = "../../../🫳️reader/🦀️.rs"]
                            pub mod reader;
                            #[path = "../../../🏷️xrecord/🦀️.rs"]
                            pub mod xrecord;
                            #[path = "../../../🗃️tables/🦀️.rs"]
                            pub mod tables;
                            #[path = "../../../🗃️tables/📇️records/🦀️.rs"]
                            pub mod records;
                            #[path = "../../../🎨️color/🦀️.rs"]
                            pub mod color;
                            #[path = "../../../📐️entities/🦀️.rs"]
                            pub mod entities;
                            #[path = "../../../📐️entities/📏️dimension/🦀️.rs"]
                            pub mod entity_dimension;
                            #[path = "../../../📐️entities/🖼️viewport/🦀️.rs"]
                            pub mod entity_viewport;
                            #[path = "../../🦀️.rs"]
                            pub mod objects;
                            #[path = "../../../🧮️evaluation/🦀️.rs"]
                            pub mod evaluation;
                            #[path = "../../../🧩️blocks/🦀️.rs"]
                            pub mod blocks;
                        }
                    }
                }
            }
        }
    }
}
