//! 🧬️ mutations of `semio-s-artifact-stdio-semio-document` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../../🦀️.rs"]
        pub mod mutations;
    }
    #[path = "."]
    pub mod text {
        pub use semio_s_artifact_stdio_semio_document_snapshot::io::text::snapshot;
        #[path = "../../../../📝️text/🧬️mutations/🦀️.rs"]
        pub mod mutations;
    }
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_document_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_document_examples::schema::diff;
    #[path = "."]
    pub mod mutations {
        #[path = "../../../../../🧬️schema/🧬️mutations/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "../../../../../✏️editor/🧭️edit-rules/🦀️.rs"]
    pub mod edit_rules;
}
semio_s_artifact_stdio_semio_base::semio_member_owner!(schema::snapshot::SemioDocumentSnapshot, schema::mutations::SemioDocumentMutation);
