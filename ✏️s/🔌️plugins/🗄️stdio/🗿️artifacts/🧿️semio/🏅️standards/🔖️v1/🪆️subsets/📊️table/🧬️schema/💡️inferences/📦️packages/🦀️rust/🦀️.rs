//! 💡️ The semio table inferences: shape, moments and entropy fields derived from a table snapshot, with their text and binary codecs.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    #[path = "."]
    pub mod inferences {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod shape {
            #[path = "../../📐shape/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod moments {
            #[path = "../../📊moments/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod entropy {
            #[path = "../../🎲entropy/🦀️.rs"]
            mod component;
            pub use component::*;
        }
    }
}
#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
        pub mod inferences;
    }
    #[path = "."]
    pub mod text {
        #[path = "../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
        pub mod inferences;
    }
}
