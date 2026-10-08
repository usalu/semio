//! 🧬️ Transparent PDF 1.4/ANY mutation registry and delegation.

use crate::standards::v1_4::subsets::base::schema::{diff::PdfDiff, snapshot::{PageDoc, PdfSnapshot}};
use semio_framework_plugin::{Fault, FaultCode, FaultOrigin};
use semio_framework_value::{DslValue, FromValue};
use semio_s_artifact_stdio_contract::editing::{Carried, EditRules, EntityRule, InsertRule, RemoveRule, Selector, SnapshotEditEvent};

//#region 🔖️Leaves
#[path = "📥️insert-page/🦀️.rs"]
pub mod insert_page;
pub use insert_page::InsertPage;
#[path = "🗑️remove-page/🦀️.rs"]
pub mod remove_page;
pub use remove_page::RemovePage;
#[path = "🔀️move-page/🦀️.rs"]
pub mod move_page;
pub use move_page::MovePage;
#[path = "📐️resize-page/🦀️.rs"]
pub mod resize_page;
pub use resize_page::ResizePage;
#[path = "♻️replace-page-text/🦀️.rs"]
pub mod replace_page_text;
pub use replace_page_text::ReplacePageText;
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", content = "payload", rename_all = "kebab-case", deny_unknown_fields)]
#[mutations(snapshot = PdfSnapshot, diff = PdfDiff, schema = "s.stdio.pdf.1.4")]
pub enum PdfMutation {
    InsertPage(InsertPage),
    RemovePage(RemovePage),
    MovePage(MovePage),
    ResizePage(ResizePage),
    ReplacePageText(ReplacePageText),
}

//#endregion 🔖️Aggregate

//#region 🔖️Edit
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn refused(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(code), message)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pointer_segments(path: &str) -> Result<Vec<String>, Fault> {
    if path.is_empty() {
        return Ok(Vec::new());
    }
    let Some(rest) = path.strip_prefix('/') else { return Err(refused("pdf-edit.invalid-pointer", format!("'{path}' is not an RFC 6901 pointer"))) };
    Ok(rest.split('/').map(|raw| raw.replace("~1", "/").replace("~0", "~")).collect())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn page_position(segment: &str, length: usize, insert: bool) -> Result<usize, Fault> {
    if insert && segment == "-" {
        return Ok(length);
    }
    match segment.parse::<usize>() {
        Ok(index) if index < length || (insert && index == length) => Ok(index),
        _ => Err(refused("pdf-edit.invalid-index", format!("'{segment}' addresses no page of a document of {length}"))),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode<T: FromValue>(value: &DslValue) -> Result<T, Fault> {
    T::from_value(value.clone()).map_err(|error| refused("pdf-edit.schema-invalid", error.to_string()))
}

/// 📄️ The leaves turning page `index` into `next`: a resize for changed geometry, a text replacement for changed text.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn page_leaves(previous: &PageDoc, index: usize, next: &PageDoc) -> Vec<PdfMutation> {
    let mut leaves = Vec::new();
    if previous.width.to_bits() != next.width.to_bits() || previous.height.to_bits() != next.height.to_bits() {
        leaves.push(PdfMutation::ResizePage(ResizePage { index, width: next.width, height: next.height }));
    }
    if previous.text != next.text {
        leaves.push(PdfMutation::ReplacePageText(ReplacePageText { index, text: next.text.clone() }));
    }
    leaves
}

/// 🧭️ The details-pane edit table: which JSON-pointer edit raises which ONE concrete kind. A page's text or one side of its size raises
/// the matching page kind carrying the current other side; an insert into `/pages` raises `insert-page` at the position, a remove
/// raises `remove-page` by position. A whole page set and a page move are answered by [`special_edit`].
pub static EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/pages/*/text", "replace-page-text", "text").selecting(&[Selector::Index("index")]),
        EntityRule::new("/pages/*/width", "resize-page", "width").selecting(&[Selector::Index("index")]).carrying(&[Carried { payload: "height", pointer: "/pages/*/height" }]),
        EntityRule::new("/pages/*/height", "resize-page", "height").selecting(&[Selector::Index("index")]).carrying(&[Carried { payload: "width", pointer: "/pages/*/width" }]),
    ],
    inserts: &[InsertRule::new("/pages", "insert-page", "page").at("index")],
    removes: &[RemoveRule::by_index("/pages", "remove-page", "index")],
};

/// 🎯️ The edits the table cannot express: a whole page set (a resize and/or a text replacement) and a page moved to another position.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn special_edit(event: &SnapshotEditEvent, snapshot: &PdfSnapshot) -> Result<Option<Vec<PdfMutation>>, Fault> {
    let pages = &snapshot.pages;
    match event {
        SnapshotEditEvent::SetValue { path, value } => match pointer_segments(path)?.as_slice() {
            [lane, page] if lane == "pages" => {
                let index = page_position(page, pages.len(), false)?;
                Ok(Some(page_leaves(&pages[index], index, &decode::<PageDoc>(value)?)))
            }
            _ => Ok(None),
        },
        SnapshotEditEvent::MoveValue { from, path } => match (pointer_segments(from)?.as_slice(), pointer_segments(path)?.as_slice()) {
            ([lane_from, source], [lane_to, destination]) if lane_from == "pages" && lane_to == "pages" => {
                let (from, to) = (page_position(source, pages.len(), false)?, page_position(destination, pages.len(), true)?.min(pages.len().saturating_sub(1)));
                Ok(Some((from != to).then(|| PdfMutation::MovePage(MovePage { from, to })).into_iter().collect()))
            }
            _ => Ok(None),
        },
        _ => Ok(None),
    }
}
//#endregion 🔖️Edit

//#region 🔖️Delegation
//#endregion 🔖️Delegation

//#region 🔖️Codecs
//#endregion 🔖️Codecs

//#region 🧪️Structure
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Structure
