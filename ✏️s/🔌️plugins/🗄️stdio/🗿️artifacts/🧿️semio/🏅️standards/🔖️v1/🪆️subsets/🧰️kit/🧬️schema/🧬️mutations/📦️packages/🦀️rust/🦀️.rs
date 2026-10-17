//! 🧬️ mut-family of `semio-s-artifact-stdio-semio-kit` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_kit_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_kit_examples::schema::diff;
    #[path = "."]
    pub mod mutations {
        #[path = "."]
        pub mod create_object {
            #[path = "../../🏗️create-object/🦀️.rs"]
            mod component;
            #[path = "../../🏗️create-object/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🏗️create-object/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_object {
            #[path = "../../🪓delete-object/🦀️.rs"]
            mod component;
            #[path = "../../🪓delete-object/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🪓delete-object/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_model {
            #[path = "../../🏛️create-model/🦀️.rs"]
            mod component;
            #[path = "../../🏛️create-model/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🏛️create-model/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_model {
            #[path = "../../💣delete-model/🦀️.rs"]
            mod component;
            #[path = "../../💣delete-model/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../💣delete-model/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_properties {
            #[path = "../../🏷️create-properties/🦀️.rs"]
            mod component;
            #[path = "../../🏷️create-properties/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🏷️create-properties/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_properties {
            #[path = "../../🚫delete-properties/🦀️.rs"]
            mod component;
            #[path = "../../🚫delete-properties/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🚫delete-properties/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod bind_representation {
            #[path = "../../🪢️bind-representation/🦀️.rs"]
            mod component;
            #[path = "../../🪢️bind-representation/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🪢️bind-representation/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod unbind_representation {
            #[path = "../../✂️unbind-representation/🦀️.rs"]
            mod component;
            #[path = "../../✂️unbind-representation/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../✂️unbind-representation/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod change_representation_pin {
            #[path = "../../📌change-representation-pin/🦀️.rs"]
            mod component;
            #[path = "../../📌change-representation-pin/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../📌change-representation-pin/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
    }
}
