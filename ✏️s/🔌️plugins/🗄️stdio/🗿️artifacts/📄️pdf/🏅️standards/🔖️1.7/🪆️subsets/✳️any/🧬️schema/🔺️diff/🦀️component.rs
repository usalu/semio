//! 🔺️ PdfDiff (1.7) — sparse diff extended with page-level ops. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: beyond the universal
//! `{NoMutation, SetSnapshot}` stub, 1.7's mutation vocabulary needs its own diff shape per op
//! (mirrors gif 89a's `GifDiff` — one op field populated per mutation, `snapshot` is the
//! full-replace fast path).

use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{PdfInfo, PdfPage, PdfSnapshot};
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️OpPayloads
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInsert {
    pub index: usize,
    pub page: PdfPage,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaBoxChange {
    pub index: usize,
    pub media_box: [f64; 4],
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentAppend {
    pub index: usize,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoChange {
    pub info: PdfInfo,
}
//#endregion 🔖️OpPayloads

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.pdf.1.7`. Exactly one field is populated per mutation; `apply` checks
/// `snapshot` first (full-replace fast path), then applies whichever single op field is present.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf.1.7.diff")]
pub struct PdfDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<PdfSnapshot>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insert_page: Option<PageInsert>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_page_at: Option<usize>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_media_box: Option<MediaBoxChange>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub append_content: Option<ContentAppend>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_info: Option<InfoChange>,
}

impl MutationDiff<PdfSnapshot> for PdfDiff {
    fn apply(&self, base: &PdfSnapshot) -> PdfSnapshot {
        if let Some(snapshot) = &self.snapshot {
            return snapshot.clone();
        }
        let mut next = base.clone();
        if let Some(PageInsert { index, page }) = &self.insert_page {
            let at = (*index).min(next.pages.len());
            next.pages.insert(at, page.clone());
        }
        if let Some(index) = self.remove_page_at {
            if index < next.pages.len() {
                next.pages.remove(index);
            }
        }
        if let Some(MediaBoxChange { index, media_box }) = &self.set_media_box {
            if let Some(page) = next.pages.get_mut(*index) {
                page.media_box = *media_box;
            }
        }
        if let Some(ContentAppend { index, text }) = &self.append_content {
            if let Some(page) = next.pages.get_mut(*index) {
                if !page.text.is_empty() { page.text.push('\n'); }
                page.text.push_str(text);
            }
        }
        if let Some(InfoChange { info }) = &self.set_info {
            next.info = info.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() { self.snapshot = other.snapshot; }
        if other.insert_page.is_some() { self.insert_page = other.insert_page; }
        if other.remove_page_at.is_some() { self.remove_page_at = other.remove_page_at; }
        if other.set_media_box.is_some() { self.set_media_box = other.set_media_box; }
        if other.append_content.is_some() { self.append_content = other.append_content; }
        if other.set_info.is_some() { self.set_info = other.set_info; }
    }
}

pub fn diff_set_snapshot(snapshot: &PdfSnapshot) -> PdfDiff {
    PdfDiff { snapshot: Some(snapshot.clone()), ..Default::default() }
}
pub fn diff_insert_page(index: usize, page: PdfPage) -> PdfDiff {
    PdfDiff { insert_page: Some(PageInsert { index, page }), ..Default::default() }
}
pub fn diff_remove_page(index: usize) -> PdfDiff {
    PdfDiff { remove_page_at: Some(index), ..Default::default() }
}
pub fn diff_set_page_media_box(index: usize, media_box: [f64; 4]) -> PdfDiff {
    PdfDiff { set_media_box: Some(MediaBoxChange { index, media_box }), ..Default::default() }
}
pub fn diff_append_page_content(index: usize, text: String) -> PdfDiff {
    PdfDiff { append_content: Some(ContentAppend { index, text }), ..Default::default() }
}
pub fn diff_set_info(info: PdfInfo) -> PdfDiff {
    PdfDiff { set_info: Some(InfoChange { info }), ..Default::default() }
}
//#endregion 🔖️Diff
