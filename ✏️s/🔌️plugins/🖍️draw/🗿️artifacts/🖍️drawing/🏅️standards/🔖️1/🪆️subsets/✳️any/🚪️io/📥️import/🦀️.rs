//! 📥️ Drawing import: the deserializers that read SVG and JSON documents into a drawing.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;


//#region 🪢️TaxonomyMounts
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
        pub mod svg {
            #[path = "."]
            pub mod v1_1 {
                #[path = "🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                pub mod any;
            }
        }
    }
}
//#endregion 🪢️TaxonomyMounts
