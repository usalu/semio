//! 🧬️ The semio text mutation vocabulary: every mutation with its diff and inverse, their text and binary codecs, and the member store owner.

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
        pub mod insert_run {
            #[path = "../../📥insert-run/🦀️.rs"]
            mod component;
            #[path = "../../📥insert-run/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../📥insert-run/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod remove_run {
            #[path = "../../🗑️remove-run/🦀️.rs"]
            mod component;
            #[path = "../../🗑️remove-run/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🗑️remove-run/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod edit_run {
            #[path = "../../✏️edit-run/🦀️.rs"]
            mod component;
            #[path = "../../✏️edit-run/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../✏️edit-run/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod change_run_language {
            #[path = "../../🌐change-run-language/🦀️.rs"]
            mod component;
            #[path = "../../🌐change-run-language/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🌐change-run-language/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod reorder_runs {
            #[path = "../../🔀reorder-runs/🦀️.rs"]
            mod component;
            #[path = "../../🔀reorder-runs/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🔀reorder-runs/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod add_mark {
            #[path = "../../➕add-mark/🦀️.rs"]
            mod component;
            #[path = "../../➕add-mark/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../➕add-mark/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod remove_mark {
            #[path = "../../➖remove-mark/🦀️.rs"]
            mod component;
            #[path = "../../➖remove-mark/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../➖remove-mark/↩️inverse/🦀️.rs"]
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
semio_s_artifact_stdio_semio_base::semio_member_owner!(semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot, schema::mutations::SemioTextMutation);

