//! 🧬️ The semio table mutation vocabulary: every mutation with its diff and inverse, their text and binary codecs, and the member store owner.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    #[path = "."]
    pub mod mutations {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod create_column {
            #[path = "../../🏗️create-column/🦀️.rs"]
            mod component;
            #[path = "../../🏗️create-column/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🏗️create-column/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_column {
            #[path = "../../🗑️delete-column/🦀️.rs"]
            mod component;
            #[path = "../../🗑️delete-column/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🗑️delete-column/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod rename_column {
            #[path = "../../🏷️rename-column/🦀️.rs"]
            mod component;
            #[path = "../../🏷️rename-column/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🏷️rename-column/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod reorder_columns {
            #[path = "../../🔀reorder-columns/🦀️.rs"]
            mod component;
            #[path = "../../🔀reorder-columns/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🔀reorder-columns/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod insert_row {
            #[path = "../../📥insert-row/🦀️.rs"]
            mod component;
            #[path = "../../📥insert-row/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../📥insert-row/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod remove_row {
            #[path = "../../➖remove-row/🦀️.rs"]
            mod component;
            #[path = "../../➖remove-row/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../➖remove-row/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod reorder_rows {
            #[path = "../../🔃reorder-rows/🦀️.rs"]
            mod component;
            #[path = "../../🔃reorder-rows/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🔃reorder-rows/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod edit_cell {
            #[path = "../../✏️edit-cell/🦀️.rs"]
            mod component;
            #[path = "../../✏️edit-cell/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../✏️edit-cell/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
    }
}
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
semio_s_artifact_stdio_semio_base::semio_member_owner!(schema::snapshot::SemioTableSnapshot, schema::mutations::SemioTableMutation);

