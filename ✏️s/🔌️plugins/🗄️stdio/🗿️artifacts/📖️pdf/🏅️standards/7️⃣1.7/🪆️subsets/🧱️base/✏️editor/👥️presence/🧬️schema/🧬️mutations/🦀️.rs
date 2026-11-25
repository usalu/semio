//! 👥️ PDF presence mutation aggregate — one verb, because presence is a live snapshot of one collaborator, not an edit history.

use crate::editor::pdf_presence::lane::{PdfPresence, PdfPresenceDiff};

#[path = "👁️set-presence/🦀️.rs"]
mod set_presence;
pub use set_presence::SetPresence;
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = PdfPresence, diff = PdfPresenceDiff, schema = "stdiopdfpresence", reference = PdfPresence, reference_diff = PdfPresenceDiff, retained)]
pub enum PdfPresenceMutation {
    #[dsl(key = "set-presence")]
    SetPresence(SetPresence),
}
dsl::original_pack_value_operation_receiving!(PdfPresenceMutation);
dsl::artifact_persisted_binary_operation!(PdfPresenceMutation);
