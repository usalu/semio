//! 📦️ `stdio.dwg` reader objects: the object-record walk and every non-entity object decoder.

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
                        pub mod objects;
                    }
                }
            }
        }
    }
}
