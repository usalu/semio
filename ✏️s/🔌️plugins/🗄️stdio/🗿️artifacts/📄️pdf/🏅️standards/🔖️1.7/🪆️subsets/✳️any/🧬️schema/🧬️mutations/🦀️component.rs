//! 🧬️ PdfMutation (1.7) — document mutation dispatch. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: the mutation vocabulary
//! beyond the universal `{NoMutation, SetSnapshot}` stub (plan D2's flagship list for `📄️pdf`),
//! real `apply` + `inverse` for each.

use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::{self, PdfDiff};
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{PdfInfo, PdfPage, PdfSnapshot};
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.pdf.1.7`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum PdfMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: PdfSnapshot,
    },
    InsertPage {
        index: usize,
        page: PdfPage,
    },
    RemovePage {
        index: usize,
    },
    SetPageMediaBox {
        index: usize,
        media_box: [f64; 4],
    },
    /// ➕ Appends `text` to the page's authoring text (newline-separated from whatever was
    /// already there). No natural minimal inverse exists for "append" within this vocabulary
    /// (there's no `RemovePageContent` counterpart) -- its `inverse` below uses the
    /// full-snapshot-restore escape hatch instead, same as `SetSnapshot`'s own inverse does.
    AppendPageContent {
        index: usize,
        text: String,
    },
    SetInfo {
        info: PdfInfo,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. Out-of-range page indices are no-ops rather than panics
/// -- a stale index (e.g. from a concurrent edit) should degrade gracefully, not crash the engine.
pub fn apply_pdf_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfMutation) {
    match mutation {
        PdfMutation::NoMutation => {}
        PdfMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
        PdfMutation::InsertPage { index, page } => {
            let at = (*index).min(snapshot.pages.len());
            snapshot.pages.insert(at, page.clone());
        }
        PdfMutation::RemovePage { index } => {
            if *index < snapshot.pages.len() {
                snapshot.pages.remove(*index);
            }
        }
        PdfMutation::SetPageMediaBox { index, media_box } => {
            if let Some(page) = snapshot.pages.get_mut(*index) {
                page.media_box = *media_box;
            }
        }
        PdfMutation::AppendPageContent { index, text } => {
            if let Some(page) = snapshot.pages.get_mut(*index) {
                if !page.text.is_empty() { page.text.push('\n'); }
                page.text.push_str(text);
            }
        }
        PdfMutation::SetInfo { info } => snapshot.info = info.clone(),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<PdfSnapshot> for PdfMutation {
    type Diff = PdfDiff;

    fn diff(&self, _base: &PdfSnapshot) -> Self::Diff {
        match self {
            PdfMutation::NoMutation => PdfDiff::default(),
            PdfMutation::SetSnapshot { snapshot } => diff::diff_set_snapshot(snapshot),
            PdfMutation::InsertPage { index, page } => diff::diff_insert_page(*index, page.clone()),
            PdfMutation::RemovePage { index } => diff::diff_remove_page(*index),
            PdfMutation::SetPageMediaBox { index, media_box } => diff::diff_set_page_media_box(*index, *media_box),
            PdfMutation::AppendPageContent { index, text } => diff::diff_append_page_content(*index, text.clone()),
            PdfMutation::SetInfo { info } => diff::diff_set_info(info.clone()),
        }
    }

    /// ↩️ Real, round-trippable inverses: `apply(inverse(m, base), apply(m, base)) == base` for
    /// every variant, proven by `mutation_apply_inverse_round_trips_every_variant` below.
    fn inverse(&self, base: &PdfSnapshot) -> Vec<Self> {
        match self {
            PdfMutation::NoMutation => vec![PdfMutation::NoMutation],
            PdfMutation::SetSnapshot { .. } => vec![PdfMutation::SetSnapshot { snapshot: base.clone() }],
            PdfMutation::InsertPage { index, .. } => vec![PdfMutation::RemovePage { index: *index }],
            PdfMutation::RemovePage { index } => match base.pages.get(*index) {
                Some(page) => vec![PdfMutation::InsertPage { index: *index, page: page.clone() }],
                None => vec![PdfMutation::NoMutation],
            },
            PdfMutation::SetPageMediaBox { index, .. } => {
                let prior = base.pages.get(*index).map(|p| p.media_box).unwrap_or([0.0, 0.0, 612.0, 792.0]);
                vec![PdfMutation::SetPageMediaBox { index: *index, media_box: prior }]
            }
            PdfMutation::AppendPageContent { .. } => vec![PdfMutation::SetSnapshot { snapshot: base.clone() }],
            PdfMutation::SetInfo { .. } => vec![PdfMutation::SetInfo { info: base.info.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for PdfMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for PdfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op encode",
            offset: 0,
            detail: e.to_string(),
        })
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op decode",
            offset: 0,
            detail: e.to_string(),
        })
    }
}
//#endregion OpCodecs

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;

    fn sample_page(seed: u8) -> PdfPage {
        PdfPage { media_box: [0.0, 0.0, 612.0, 792.0], crop_box: None, rotate: 0, text: format!("page-{seed}") }
    }

    fn base_snapshot() -> PdfSnapshot {
        PdfSnapshot {
            schema: "stdio.pdf.1.7".into(),
            declared_version: "1.7".into(),
            pages: vec![sample_page(1), sample_page(2), sample_page(3)],
            info: PdfInfo { title: Some("Base".into()), ..Default::default() },
            objects: Vec::new(),
        }
    }

    fn round_trips(base: &PdfSnapshot, mutation: PdfMutation) {
        let diff = mutation.diff(base);
        let mutated = diff.apply(base);
        let inverses = mutation.inverse(base);
        let mut restored = mutated.clone();
        for inv in &inverses {
            let inv_diff = inv.diff(&restored);
            restored = inv_diff.apply(&restored);
        }
        assert_eq!(&restored, base, "apply(inverse(m), apply(m, base)) must recover base for {mutation:?}");
    }

    #[test]
    fn mutation_apply_inverse_round_trips_every_variant() {
        let base = base_snapshot();
        round_trips(&base, PdfMutation::NoMutation);
        round_trips(&base, PdfMutation::SetSnapshot { snapshot: PdfSnapshot { info: PdfInfo { title: Some("X".into()), ..Default::default() }, ..base.clone() } });
        round_trips(&base, PdfMutation::InsertPage { index: 1, page: sample_page(9) });
        round_trips(&base, PdfMutation::RemovePage { index: 1 });
        round_trips(&base, PdfMutation::SetPageMediaBox { index: 0, media_box: [0.0, 0.0, 200.0, 300.0] });
        round_trips(&base, PdfMutation::AppendPageContent { index: 0, text: "more text".into() });
        round_trips(&base, PdfMutation::SetInfo { info: PdfInfo { author: Some("Ueli".into()), ..Default::default() } });
    }

    #[test]
    fn remove_page_out_of_range_is_noop_not_panic() {
        let base = base_snapshot();
        let mut snap = base.clone();
        apply_pdf_mutation(&mut snap, &PdfMutation::RemovePage { index: 99 });
        assert_eq!(snap, base);
    }
}
//#endregion Tests
