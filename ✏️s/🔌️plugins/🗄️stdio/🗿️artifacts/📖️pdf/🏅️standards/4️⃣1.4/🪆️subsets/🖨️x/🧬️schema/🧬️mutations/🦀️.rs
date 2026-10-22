//! 🧬️ Transparent PDF 1.4/X mutation registry and delegation.

use crate::standards::v1_4::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};

//#region 🔖️Leaves
pub use semio_s_artifact_stdio_pdf_tests::standards::v1_4::subsets::x::schema::mutations::set_page_size;
pub use set_page_size::SetPageSize;
#[path = "📉️collapse-page-size/🦀️.rs"]
pub mod collapse_page_size;
pub use collapse_page_size::CollapsePageSize;
pub use set_page_size::{CONFORMANT_HEIGHT, CONFORMANT_WIDTH};
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", content = "payload", rename_all = "kebab-case", deny_unknown_fields)]
#[mutations(snapshot = PdfSnapshot, diff = PdfDiff, schema = "s.stdio.pdf.1.4.x", reference = PdfSnapshot, reference_diff = PdfDiff)]
pub enum PdfX1Mutation {
    SetPageSize(SetPageSize),
    CollapsePageSize(CollapsePageSize),
}

//#endregion 🔖️Aggregate

//#region 🔖️Delegation
//#endregion 🔖️Delegation

//#region 🧪️Structure

//#endregion 🧪️Structure
