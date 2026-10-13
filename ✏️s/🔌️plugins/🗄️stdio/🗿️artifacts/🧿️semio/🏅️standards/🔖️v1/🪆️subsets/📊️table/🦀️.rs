//! 🪆️ The `table` subset of the semio artifact: its schema, io codecs, editor and viewer, and the retirement and member-owner
//! implementations of its own types.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
#[path = "."]
pub mod schema {
    #[path = "🧬️schema/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧬️schema/🧭️causal-internals/🦀️.rs"]
    pub mod causal_internals;
    #[path = "🧬️schema/🌀️entropy-internals/🦀️.rs"]
    pub mod entropy_internals;
    #[path = "🧬️schema/🎲️probability-internals/🦀️.rs"]
    pub mod probability_internals;
    #[path = "🧬️schema/📊️statistics-internals/🦀️.rs"]
    pub mod statistics_internals;
    #[path = "🧬️schema/📋️tabular-internals/🦀️.rs"]
    pub mod tabular_internals;
    #[path = "."]
    pub mod snapshot {
        #[path = "🧬️schema/📸️snapshot/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod diff {
        #[path = "🧬️schema/🔺️diff/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod inferences {
        #[path = "🧬️schema/💡️inferences/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod shape {
            #[path = "🧬️schema/💡️inferences/📐shape/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod moments {
            #[path = "🧬️schema/💡️inferences/📊moments/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod entropy {
            #[path = "🧬️schema/💡️inferences/🎲entropy/🦀️.rs"]
            mod component;
            pub use component::*;
        }
    }
    #[path = "."]
    pub mod mutations {
        #[path = "🧬️schema/🧬️mutations/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod create_column {
            #[path = "🧬️schema/🧬️mutations/🏗️create-column/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🏗️create-column/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🏗️create-column/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_column {
            #[path = "🧬️schema/🧬️mutations/🗑️delete-column/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🗑️delete-column/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🗑️delete-column/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod rename_column {
            #[path = "🧬️schema/🧬️mutations/🏷️rename-column/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🏷️rename-column/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🏷️rename-column/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod reorder_columns {
            #[path = "🧬️schema/🧬️mutations/🔀reorder-columns/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🔀reorder-columns/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🔀reorder-columns/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod insert_row {
            #[path = "🧬️schema/🧬️mutations/📥insert-row/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/📥insert-row/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/📥insert-row/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod remove_row {
            #[path = "🧬️schema/🧬️mutations/➖remove-row/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/➖remove-row/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/➖remove-row/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod reorder_rows {
            #[path = "🧬️schema/🧬️mutations/🔃reorder-rows/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🔃reorder-rows/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🔃reorder-rows/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod edit_cell {
            #[path = "🧬️schema/🧬️mutations/✏️edit-cell/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/✏️edit-cell/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/✏️edit-cell/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod sheet {
        #[path = "📚️examples/📃️sheet/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "✏️editor/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod presence {
        #[path = "✏️editor/👥️presence/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod config {
        #[path = "✏️editor/🎚️config/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            #[path = "✏️editor/🎭️modes/✏️edit/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                #[path = "."]
                pub mod main {
                    #[path = "✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "👁️viewer/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod presence {
        #[path = "👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod config {
        #[path = "👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            #[path = "👁️viewer/🎭️modes/👁️view/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                #[path = "."]
                pub mod main {
                    #[path = "👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}

semio_framework_value::artifact_retire_leaf!(schema::snapshot::SemioTableCellKind);
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioTableColumn { name, kind });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioTableRow { cells });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioTableSnapshot { schema, columns, rows });

semio_s_artifact_stdio_semio_base::semio_member_owner!(schema::snapshot::SemioTableSnapshot, schema::mutations::SemioTableMutation);
