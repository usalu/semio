//! 🪆️ The `presentation` subset of the semio artifact: its schema, io codecs, editor and viewer, and the retirement and member-owner
//! implementations of its own types.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "🚪️io/🦀️.rs"]
    mod component;
    pub use component::*;
}
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
        pub mod outline {
            #[path = "🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
            mod component;
            pub use component::*;
        }
    }
    #[path = "."]
    pub mod mutations {
        #[path = "🧬️schema/🧬️mutations/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod examples {
    #[path = "📚️examples/📽️deck/🦀️.rs"]
    pub mod deck;
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

semio_framework_value::artifact_retire_leaf!(schema::snapshot::SlideFrame);
semio_framework_value::artifact_retire_struct!(schema::snapshot::SlidePictureImage { asset_id, mime, bytes });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SlideTableCell { blocks });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SlideTableRow { cells });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SlideMaster { id, shapes });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SlideLayout { id, master_id, shapes });
semio_framework_value::artifact_retire_struct!(schema::snapshot::Slide { id, layout_id, shapes, notes });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioPresentationSnapshot { schema, masters, layouts, slides });

semio_s_artifact_stdio_semio_base::semio_member_owner!(schema::snapshot::SemioPresentationSnapshot, schema::mutations::SemioPresentationMutation);
