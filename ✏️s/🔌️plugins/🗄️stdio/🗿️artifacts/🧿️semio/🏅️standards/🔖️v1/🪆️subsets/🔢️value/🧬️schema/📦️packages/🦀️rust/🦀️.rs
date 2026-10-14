//! 🧬️ The semio value model: schema root, snapshot and diff with their text, binary and SQLite codecs, and the snapshot retirement implementations.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "../../../🚪️io/💾️binary/🦀️.rs"]
    pub mod binary;
    #[path = "../../../🚪️io/📝️text/🦀️.rs"]
    pub mod text;
    #[path = "../../../🚪️io/🪶️sqlite/🦀️.rs"]
    pub mod sqlite;
}
#[path = "."]
pub mod schema {
    #[path = "../../🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod snapshot {
        #[path = "../../📸️snapshot/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod diff {
        #[path = "../../🔺️diff/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
semio_framework_value::artifact_retire_struct!(schema::snapshot::ValueId { value });

semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioValueEntry { key, value });

semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioValueNode { id, value });

semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioValueSnapshot { schema, root, nodes });

semio_s_artifact_stdio_semio_base::semio_snapshot_retirement!(schema::snapshot::SemioValueSnapshot);

