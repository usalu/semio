//! 🫧️ PDF view transient mutation aggregate — one verb, because the transient is a view cache with a single writer, not an edit history.

use crate::editor::pdf_view::transient::{PdfViewTransient, PdfViewTransientDiff};

#[path = "👁️set-view/🦀️.rs"]
mod set_view;
pub use set_view::SetView;
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = PdfViewTransient, diff = PdfViewTransientDiff, schema = "stdiopdfviewtransient", reference = PdfViewTransient, reference_diff = PdfViewTransientDiff, retained)]
pub enum PdfViewTransientMutation {
    #[dsl(key = "set-view")]
    SetView(SetView),
}
dsl::original_pack_value_operation_receiving!(PdfViewTransientMutation);
dsl::artifact_persisted_binary_operation!(PdfViewTransientMutation);
