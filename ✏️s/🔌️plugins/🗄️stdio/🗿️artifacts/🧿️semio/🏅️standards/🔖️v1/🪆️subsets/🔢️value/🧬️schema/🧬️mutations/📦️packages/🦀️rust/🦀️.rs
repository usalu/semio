//! 🧬️ The semio value mutation vocabulary: every mutation with its diff and inverse, their text and binary codecs, and the member store owner.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
        pub mod mutations;
    }
    #[path = "."]
    pub mod text {
        #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
        pub mod mutations;
    }
}
#[path = "."]
pub mod schema {
    #[path = "."]
    pub mod mutations {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
semio_s_artifact_stdio_semio_base::semio_member_owner!(semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot, schema::mutations::SemioValueMutation);

