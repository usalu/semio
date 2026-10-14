//! 📥️ block-2d-io read.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "."]
pub mod deserializers {
    #[path = "."]
    pub mod artifacts {
        #[path = "."]
        pub mod json {
            #[path = "."]
            pub mod v_rfc8259 {
                #[path = "🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                pub mod any;
            }
        }
        #[path = "."]
        pub mod txt {
            #[path = "."]
            pub mod v_utf_8 {
                #[path = "🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                pub mod any;
            }
        }
        #[path = "."]
        pub mod zip {
            #[path = "."]
            pub mod v2_0 {
                #[path = "🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                pub mod any;
            }
        }
    }
}
//#endregion 🪢️Mounts
