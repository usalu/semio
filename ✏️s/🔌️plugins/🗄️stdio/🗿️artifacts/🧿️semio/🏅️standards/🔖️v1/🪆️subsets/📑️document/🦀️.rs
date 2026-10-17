//! 🪆️ The `document` subset of the semio artifact: its schema, io codecs, editor and viewer, and the retirement and member-owner
//! implementations of its own types.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub mod io {
    pub use semio_s_artifact_stdio_semio_document_io::io::*;
    pub mod binary {
        pub use semio_s_artifact_stdio_semio_document_io::io::binary::*;
        pub use semio_s_artifact_stdio_semio_document_examples::io::binary::inferences;
    }
    pub mod sqlite {
        pub use semio_s_artifact_stdio_semio_document_examples::io::sqlite::*;
        pub use semio_s_artifact_stdio_semio_document_snapshot::io::sqlite::snapshot;
    }
    pub mod text {
        pub use semio_s_artifact_stdio_semio_document_io::io::text::*;
        pub use semio_s_artifact_stdio_semio_document_examples::io::text::inferences;
    }
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_document_schema::schema::*;

    pub use semio_s_artifact_stdio_semio_document_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_document_examples::schema::diff;
    pub use semio_s_artifact_stdio_semio_document_schema::schema::inferences;
    pub use semio_s_artifact_stdio_semio_document_mutations::schema::mutations;
}
pub use semio_s_artifact_stdio_semio_document_examples::examples;
#[path = "."]
pub mod editor {
    pub use semio_s_artifact_stdio_semio_document_editor::editor::*;

    pub use semio_s_artifact_stdio_semio_document_examples::editor::presence;
    pub use semio_s_artifact_stdio_semio_document_examples::editor::config;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            pub use semio_s_artifact_stdio_semio_document_examples::editor::modes::edit::*;

            pub use semio_s_artifact_stdio_semio_document_examples::editor::modes::edit::windows;
        }
    }
}
#[path = "."]
pub mod viewer {
    pub use semio_s_artifact_stdio_semio_document_io::viewer::*;

    pub use semio_s_artifact_stdio_semio_document_examples::viewer::presence;
    pub use semio_s_artifact_stdio_semio_document_examples::viewer::config;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            pub use semio_s_artifact_stdio_semio_document_examples::viewer::modes::view::*;

            pub use semio_s_artifact_stdio_semio_document_examples::viewer::modes::view::windows;
        }
    }
}

semio_framework_value::artifact_retire_struct!(schema::snapshot::RunStyle { bold, italic, underline, size, font, color, link });
semio_framework_value::artifact_retire_struct!(schema::snapshot::DocRun { text, style });
semio_framework_value::artifact_retire_struct!(schema::snapshot::DocStyle { id, name, based_on });
semio_framework_value::artifact_retire_struct!(schema::snapshot::DocImage { id, mime, bytes });
semio_framework_value::artifact_retire_struct!(schema::snapshot::DocListItem { blocks });
semio_framework_value::artifact_retire_struct!(schema::snapshot::DocTableCell { blocks });
semio_framework_value::artifact_retire_struct!(schema::snapshot::DocTableRow { cells });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioDocumentSnapshot { schema, styles, images, blocks });
semio_framework_value::artifact_retire_struct!(schema::mutations::DocBlockPath { segments, index });

semio_s_artifact_stdio_semio_base::semio_snapshot_retirement!(schema::snapshot::SemioDocumentSnapshot);
semio_s_artifact_stdio_semio_base::semio_member_owner!(schema::snapshot::SemioDocumentSnapshot, schema::mutations::SemioDocumentMutation);
