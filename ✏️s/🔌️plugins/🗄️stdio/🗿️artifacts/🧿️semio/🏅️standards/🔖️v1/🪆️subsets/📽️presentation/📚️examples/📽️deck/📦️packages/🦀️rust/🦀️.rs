//! 📽️ examples of `semio-s-artifact-stdio-semio-presentation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
        pub mod inferences;
    }
    #[path = "."]
    pub mod text {
        #[path = "../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
        pub mod inferences;
    }
    #[path = "../../../../🚪️io/🪶️sqlite/🦀️.rs"]
    pub mod sqlite;
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_presentation_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_presentation_snapshot::schema::diff;
    #[path = "."]
    pub mod inferences {
        #[path = "../../../../🧬️schema/💡️inferences/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod outline {
            #[path = "../../../../🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
            mod component;
            pub use component::*;
        }
    }
    #[path = "."]
    pub mod mutations {
        #[path = "../../../../🧬️schema/🧬️mutations/🎬insert-slide/🦀️.rs"]
        pub mod insert_slide;
        #[path = "../../../../🧬️schema/🧬️mutations/📤️remove-slide/🦀️.rs"]
        pub mod remove_slide;
        #[path = "../../../../🧬️schema/🧬️mutations/🧾set-slide-notes/🦀️.rs"]
        pub mod set_slide_notes;
    }
}
#[path = "."]
pub mod examples {
    #[path = "../../🦀️.rs"]
    pub mod deck;
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod presence {
        #[path = "../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod config {
        #[path = "../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "../../../../✏️editor/🧭️edit-rules/🦀️.rs"]
    pub mod edit_rules;
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod presence {
        #[path = "../../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod config {
        #[path = "../../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
}
