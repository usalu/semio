//! 🍃️ The semio table mutation leaves: every payload with its diff and inverse, generic over the aggregate that wraps them.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    #[path = "."]
    pub mod mutations {
        use semio_s_artifact_stdio_semio_table_model::schema::diff::SemioTableDiff;
        use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;

        #[path = "."]
        pub mod create_column {
            #[path = "../../../🏗️create-column/🦀️.rs"]
            mod component;
            #[path = "../../../🏗️create-column/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🏗️create-column/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_column {
            #[path = "../../../🗑️delete-column/🦀️.rs"]
            mod component;
            #[path = "../../../🗑️delete-column/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🗑️delete-column/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod rename_column {
            #[path = "../../../🏷️rename-column/🦀️.rs"]
            mod component;
            #[path = "../../../🏷️rename-column/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🏷️rename-column/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod reorder_columns {
            #[path = "../../../🔀reorder-columns/🦀️.rs"]
            mod component;
            #[path = "../../../🔀reorder-columns/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🔀reorder-columns/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod insert_row {
            #[path = "../../../📥insert-row/🦀️.rs"]
            mod component;
            #[path = "../../../📥insert-row/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../📥insert-row/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod remove_row {
            #[path = "../../../➖remove-row/🦀️.rs"]
            mod component;
            #[path = "../../../➖remove-row/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../➖remove-row/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod reorder_rows {
            #[path = "../../../🔃reorder-rows/🦀️.rs"]
            mod component;
            #[path = "../../../🔃reorder-rows/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🔃reorder-rows/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod edit_cell {
            #[path = "../../../✏️edit-cell/🦀️.rs"]
            mod component;
            #[path = "../../../✏️edit-cell/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../✏️edit-cell/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }

        /// 🔗️ The aggregate-facing contract of every table leaf: an operation enum that diffs a table snapshot into a table diff and wraps each leaf.
        pub trait SemioTableOp: protocol::Mutation<SemioTableSnapshot, Diff = SemioTableDiff> + From<create_column::CreateColumn> + From<delete_column::DeleteColumn> + From<rename_column::RenameColumn> + From<reorder_columns::ReorderColumns> + From<insert_row::InsertRow> + From<remove_row::RemoveRow> + From<reorder_rows::ReorderRows> + From<edit_cell::EditCell> {}

        impl<T> SemioTableOp for T where T: protocol::Mutation<SemioTableSnapshot, Diff = SemioTableDiff> + From<create_column::CreateColumn> + From<delete_column::DeleteColumn> + From<rename_column::RenameColumn> + From<reorder_columns::ReorderColumns> + From<insert_row::InsertRow> + From<remove_row::RemoveRow> + From<reorder_rows::ReorderRows> + From<edit_cell::EditCell> {}
    }
}
