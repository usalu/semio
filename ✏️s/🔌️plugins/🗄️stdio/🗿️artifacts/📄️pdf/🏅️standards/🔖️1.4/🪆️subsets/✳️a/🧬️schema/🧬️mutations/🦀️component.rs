//! 🧬️ `PdfA1Mutation` — the ISO 19005-1 (PDF/A-1) CONFORMANCE vocabulary of `stdio.pdf` 1.4. Every
//! variant's `diff()` is handcrafted directly against `base`, and every variant's `inverse()` is
//! handcrafted, reading whatever pre-state it needs out of the base.
//!
//! **Why this vocabulary has four variants and not fifteen.** PDF 1.4's retained snapshot is a bare
//! `PageDoc { width, height, text }` — no object graph — and this subset's own
//! `check_pdf_a_conformance` (`../../🦀️component.rs`) says so in as many words: it raises exactly two
//! diagnostics, `stdio.pdf.a.text-empty` when `page.text` is blank, and
//! `stdio.pdf.a.schema-gap-unverifiable`, which fires unconditionally on every document to record
//! that full ISO 19005-1 conformance cannot be checked from this schema at all. A vocabulary derived
//! honestly from that checker therefore has exactly ONE movable axis — the extractable text — and the
//! schema-gap axis is not movable by anything, because no mutation can give this snapshot an object
//! graph it does not have. Declaring the kinds the 1.7 `✳️a` subset legitimately owns (encryption,
//! JavaScript, launch actions, output intents, `/AFRelationship`, font embedding) would be
//! fabricating a vocabulary for a schema that cannot observe a single one of them.
//!
//! **And why it shares no variant with `1.4/✳️x`.** That sibling's `check_pdf_x_conformance` reads
//! `page.width`/`page.height` and never looks at the text; this one reads the text and never looks at
//! the geometry. Two subsets of one standard over one snapshot type, disjoint because their checkers
//! read different fields of it.
//!
//! `Diff` is `PdfDiff`, the SAME diff type `✳️any` uses — one snapshot type, one diff. What differs is
//! the vocabulary that produces it, which is what a subset is.
//!
//! @see ../../🧪️oracle/🔣️component.json — the mutation catalog `KINDS` is measured against.
//! @see ../🦀️component.rs — `check_pdf_a_conformance`, the one axis this vocabulary derives from.

use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::diff::PdfDiff;
use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;
use protocol::command::DiffAlgebra;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Class
/// 📝️ The text a class stamp writes. Real content, not a placeholder: this checker's only movable
/// axis is "is there extractable text at all", and this is a real sentence a reader would extract.
pub const CONFORMANT_TEXT: &str = "Reuse of load-bearing timber components in Swiss building stock";
//#endregion 🔖️Class

//#region 🔖️Mutations
/// 📐️ Typed conformance mutation for `stdio.pdf` 1.4 under ISO 19005-1. Both non-baseline variants
/// address the one axis `check_pdf_a_conformance` can honestly check.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum PdfA1Mutation {
    /// 🚫️ The identity element of the vocabulary.
    #[default]
    NoMutation,
    /// 🔄️ Replaces the whole document. Build the target with [`stamp_conformance`].
    SetSnapshot {
        snapshot: PdfSnapshot,
    },
    /// 📝️ Sets the page's extractable text to a non-empty value — the state
    /// `stdio.pdf.a.text-empty` stops firing in.
    SetPageText {
        text: String,
    },
    /// 📝️ Empties the page's extractable text — the state `stdio.pdf.a.text-empty` reports.
    ClearPageText,
}

/// 🧾️ Kebab-case spelling of every `PdfA1Mutation` variant, in declaration order — the catalog
/// `pdf-1-4-a` (`../../🧪️oracle/🔣️component.json`) is measured against this exact list.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-page-text", "clear-page-text"];
//#endregion 🔖️Mutations

//#region 🔖️Stamp
/// 🏅️ Stamps the one axis this subset owns into (or out of) its conformant state — the
/// whole-document target `SetSnapshot` carries.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn stamp_conformance(base: PdfSnapshot, stamped: bool) -> PdfSnapshot {
    let mut next = base;
    next.page.text = if stamped { CONFORMANT_TEXT.to_string() } else { String::new() };
    next
}
//#endregion 🔖️Stamp

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the diff is the single semantics
/// source, never a separate imperative apply path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_a_conformance_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfA1Mutation) -> protocol::MutationOutcome<PdfDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<PdfSnapshot> for PdfA1Mutation {
    type Diff = PdfDiff;

    fn diff(&self, base: &PdfSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        let mut next = base.clone();
        match self {
            Self::NoMutation => {}
            Self::SetSnapshot { snapshot } => next = snapshot.clone(),
            Self::SetPageText { text } => next.page.text = text.clone(),
            Self::ClearPageText => next.page.text = String::new(),
        }
        protocol::MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    /// ↩️ Every undo reads the base's OWN text rather than assuming a stamp is bijective. The
    /// documents this vocabulary runs on already carry real text, so `SetSnapshot`'s undo is
    /// `SetSnapshot` with the base itself — not the opposite stamp, which would clear text the base
    /// genuinely had.
    fn inverse(&self, base: &PdfSnapshot) -> Vec<Self> {
        vec![match self {
            Self::NoMutation => Self::NoMutation,
            Self::SetSnapshot { .. } => Self::SetSnapshot { snapshot: base.clone() },
            Self::SetPageText { .. } | Self::ClearPageText => {
                if base.page.text.is_empty() {
                    Self::ClearPageText
                } else {
                    Self::SetPageText { text: base.page.text.clone() }
                }
            }
        }]
    }
}
//#endregion 🔖️MutationTrait

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PageDoc;
    use protocol::MutationDiff;

    fn base() -> PdfSnapshot {
        PdfSnapshot { schema: crate::artifacts::pdf::STDIO_PDF_DOCUMENT_SCHEMA.into(), page: PageDoc { width: 595.276, height: 841.89, text: "a real abstract".into() } }
    }

    /// 🧭️ `kind_of` is an EXHAUSTIVE match — the compiler refuses this file if a variant is added
    /// without a kebab-case spelling here. The second half reads the sibling oracle manifest's
    /// `kinds` array as text and asserts the same list, in the same order.
    #[test]
    fn kinds_match_enum_and_catalog() {
        fn kind_of(mutation: &PdfA1Mutation) -> &'static str {
            match mutation {
                PdfA1Mutation::NoMutation => "no-mutation",
                PdfA1Mutation::SetSnapshot { .. } => "set-snapshot",
                PdfA1Mutation::SetPageText { .. } => "set-page-text",
                PdfA1Mutation::ClearPageText => "clear-page-text",
            }
        }
        let samples = [PdfA1Mutation::NoMutation, PdfA1Mutation::SetSnapshot { snapshot: PdfSnapshot::default() }, PdfA1Mutation::SetPageText { text: String::new() }, PdfA1Mutation::ClearPageText];
        assert_eq!(samples.iter().map(kind_of).collect::<Vec<_>>(), KINDS);

        let manifest = include_str!("../../🧪️oracle/🔣️component.json");
        let needle = "\"kinds\": [";
        let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
        let end = start + manifest[start..].find(']').expect("kinds array is closed");
        let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
        assert_eq!(declared, KINDS, "the oracle manifest's kinds must match PdfA1Mutation exactly");
    }

    /// ⚖️ `apply(inverse(m), apply(m, base))` must recover `base`, for every variant.
    #[test]
    fn mutation_apply_inverse_round_trips_every_variant() {
        let base = base();
        for mutation in [
            PdfA1Mutation::NoMutation,
            PdfA1Mutation::SetSnapshot { snapshot: stamp_conformance(base.clone(), true) },
            PdfA1Mutation::SetPageText { text: "another abstract".into() },
            PdfA1Mutation::ClearPageText,
        ] {
            let mut state = base.clone();
            apply_a_conformance_mutation(&mut state, &mutation);
            for undo in mutation.inverse(&base) {
                state = undo.diff(&state).diff().apply(&state).expect("the inverse diff applies");
            }
            assert_eq!(state, base, "apply(inverse(m), apply(m, base)) must recover base for {mutation:?}");
        }
    }

    /// 👁️ Every non-baseline variant genuinely moves the axis this subset's checker reads.
    #[test]
    fn every_variant_moves_the_axis_the_checker_reads() {
        let base = base();
        for mutation in [PdfA1Mutation::SetSnapshot { snapshot: stamp_conformance(base.clone(), true) }, PdfA1Mutation::SetPageText { text: "another abstract".into() }, PdfA1Mutation::ClearPageText] {
            let mut state = base.clone();
            apply_a_conformance_mutation(&mut state, &mutation);
            assert_ne!(state.page.text, base.page.text, "{mutation:?} must move page.text");
        }
    }
}
//#endregion 🧪️Tests
