//! 🎛️ `stdio.dwg` reader objects: the dynamic-block and associative-constraint object decoders.

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
                        #[path = "."]
                        pub mod objects {
                            #[path = "../🧩️blocks/🦀️.rs"]
                            pub mod blocks;
                            #[path = "../🔗️associativity/🦀️.rs"]
                            pub mod associativity;
                        }
                    }
                }
            }
        }
    }
}
