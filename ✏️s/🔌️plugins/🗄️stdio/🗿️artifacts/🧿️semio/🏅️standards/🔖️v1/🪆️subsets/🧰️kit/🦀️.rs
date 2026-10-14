//! 🪆️ The `kit` subset of the semio artifact: its schema, io codecs, editor and viewer, and the retirement and member-owner
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
        pub mod entries {
            #[path = "🧬️schema/💡️inferences/🗃️entries/🦀️.rs"]
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
        pub mod create_object {
            #[path = "🧬️schema/🧬️mutations/🏗️create-object/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🏗️create-object/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🏗️create-object/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_object {
            #[path = "🧬️schema/🧬️mutations/🪓delete-object/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🪓delete-object/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🪓delete-object/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_model {
            #[path = "🧬️schema/🧬️mutations/🏛️create-model/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🏛️create-model/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🏛️create-model/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_model {
            #[path = "🧬️schema/🧬️mutations/💣delete-model/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/💣delete-model/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/💣delete-model/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_properties {
            #[path = "🧬️schema/🧬️mutations/🏷️create-properties/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🏷️create-properties/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🏷️create-properties/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_properties {
            #[path = "🧬️schema/🧬️mutations/🚫delete-properties/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🚫delete-properties/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🚫delete-properties/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod bind_representation {
            #[path = "🧬️schema/🧬️mutations/🪢️bind-representation/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🪢️bind-representation/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🪢️bind-representation/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod unbind_representation {
            #[path = "🧬️schema/🧬️mutations/✂️unbind-representation/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/✂️unbind-representation/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/✂️unbind-representation/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod change_representation_pin {
            #[path = "🧬️schema/🧬️mutations/📌change-representation-pin/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/📌change-representation-pin/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/📌change-representation-pin/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod add_type {
            #[path = "🧬️schema/🧬️mutations/➕add-type/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/➕add-type/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/➕add-type/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod remove_type {
            #[path = "🧬️schema/🧬️mutations/➖remove-type/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/➖remove-type/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/➖remove-type/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod rename_type {
            #[path = "🧬️schema/🧬️mutations/✏️rename-type/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/✏️rename-type/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/✏️rename-type/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod add_design {
            #[path = "🧬️schema/🧬️mutations/🆕add-design/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🆕add-design/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🆕add-design/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod remove_design {
            #[path = "🧬️schema/🧬️mutations/🗑️remove-design/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🗑️remove-design/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🗑️remove-design/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod edit_design {
            #[path = "🧬️schema/🧬️mutations/🖊️edit-design/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🖊️edit-design/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🖊️edit-design/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod furniture {
        #[path = "📚️examples/🪑️furniture/🦀️.rs"]
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

semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioKitType { id, name, category });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioKitPiece { id, type_id, transform });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioKitConnection { id, connecting_piece_id, connecting_port, connected_piece_id, connected_port });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioKitDesign { id, name, pieces, connections });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioKitSnapshot { schema, types, designs, objects, models, properties, representations });

semio_s_artifact_stdio_semio_base::semio_snapshot_retirement!(schema::snapshot::SemioKitSnapshot);
semio_s_artifact_stdio_semio_base::semio_member_owner!(schema::snapshot::SemioKitSnapshot, schema::mutations::SemioKitMutation);
