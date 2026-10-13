//! 🪆️ The `text` subset of the semio artifact: its schema, io codecs, editor and viewer, and the retirement and member-owner
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
        #[path = "."]
        pub mod profile {
            #[path = "🧬️schema/💡️inferences/📊profile/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "🧬️schema/💡️inferences/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod mutations {
        #[path = "🧬️schema/🧬️mutations/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod insert_run {
            #[path = "🧬️schema/🧬️mutations/📥insert-run/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/📥insert-run/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/📥insert-run/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod remove_run {
            #[path = "🧬️schema/🧬️mutations/🗑️remove-run/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🗑️remove-run/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🗑️remove-run/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod edit_run {
            #[path = "🧬️schema/🧬️mutations/✏️edit-run/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/✏️edit-run/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/✏️edit-run/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod change_run_language {
            #[path = "🧬️schema/🧬️mutations/🌐change-run-language/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🌐change-run-language/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🌐change-run-language/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod reorder_runs {
            #[path = "🧬️schema/🧬️mutations/🔀reorder-runs/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🔀reorder-runs/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🔀reorder-runs/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod add_mark {
            #[path = "🧬️schema/🧬️mutations/➕add-mark/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/➕add-mark/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/➕add-mark/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod remove_mark {
            #[path = "🧬️schema/🧬️mutations/➖remove-mark/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/➖remove-mark/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/➖remove-mark/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
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

semio_framework_value::artifact_retire_leaf!(schema::snapshot::SemioTextMarkKind);
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioTextMark { kind, href });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioTextRun { language, content, marks });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioTextSnapshot { schema, runs });

semio_s_artifact_stdio_semio_base::semio_member_owner!(schema::snapshot::SemioTextSnapshot, schema::mutations::SemioTextMutation);
