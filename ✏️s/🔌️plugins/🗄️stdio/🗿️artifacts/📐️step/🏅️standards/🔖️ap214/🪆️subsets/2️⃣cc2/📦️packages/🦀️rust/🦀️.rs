//! 🛡️ `stdio.step` conformance class 2: composition, analysis, conformance checks and mutations.

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
    pub mod v_ap214 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod cc2 {
                #[path = "../../🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "../../🧬️schema/🦀️.rs"]
                pub mod schema;
            }
        }
    }
}
