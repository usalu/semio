//! 🧱️ `stdio.dwg` reader records: the R2004 and R2007 containers, object-record field readers, entity records and file-level sections.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;

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
                    pub mod import {
                        #[path = "../../🦀️.rs"]
                        pub mod records;
                    }
                }
            }
        }
    }
}
