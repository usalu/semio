//! 🪆️ The `object` subset of the semio artifact: its schema, io codecs, editor and viewer, and the retirement and member-owner
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
        pub mod composition {
            #[path = "🧬️schema/💡️inferences/🧩composition/🦀️.rs"]
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
        pub mod move_object {
            #[path = "🧬️schema/🧬️mutations/🚚move-object/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🚚move-object/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🚚move-object/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod rotate_object {
            #[path = "🧬️schema/🧬️mutations/🔄rotate-object/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🔄rotate-object/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🔄rotate-object/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod scale_object {
            #[path = "🧬️schema/🧬️mutations/📏scale-object/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/📏scale-object/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/📏scale-object/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_brep {
            #[path = "🧬️schema/🧬️mutations/🧱create-brep/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🧱create-brep/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🧱create-brep/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_brep {
            #[path = "🧬️schema/🧬️mutations/💥delete-brep/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/💥delete-brep/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/💥delete-brep/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_mesh {
            #[path = "🧬️schema/🧬️mutations/🕸️create-mesh/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🕸️create-mesh/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🕸️create-mesh/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_mesh {
            #[path = "🧬️schema/🧬️mutations/🧨delete-mesh/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🧨delete-mesh/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🧨delete-mesh/↩️inverse/🦀️.rs"]
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
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod crate_ {
        #[path = "📚️examples/📦️crate/🦀️.rs"]
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

semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioObjectSnapshot { schema, transform, brep, mesh, properties });

semio_s_artifact_stdio_semio_base::semio_member_owner!(schema::snapshot::SemioObjectSnapshot, schema::mutations::SemioObjectMutation);
