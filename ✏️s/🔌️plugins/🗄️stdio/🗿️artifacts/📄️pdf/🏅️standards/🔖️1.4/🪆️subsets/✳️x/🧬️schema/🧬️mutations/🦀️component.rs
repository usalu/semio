//! 🧬️ `PdfX1Mutation` — the ISO 15930 (PDF/X) CONFORMANCE vocabulary of `stdio.pdf` 1.4. Every
//! variant's `diff()` is handcrafted directly against `base`, and every variant's `inverse()` is
//! handcrafted, reading whatever pre-state it needs out of the base.
//!
//! **Why this vocabulary has four variants and not sixteen.** PDF 1.4's retained snapshot is a bare
//! `PageDoc { width, height, text }` — no object graph — and this subset's own
//! `check_pdf_x_conformance` (`../../🦀️component.rs`) raises exactly two diagnostics:
//! `stdio.pdf.x.degenerate-page-size` when the page's width or height is not strictly positive, and
//! `stdio.pdf.x.schema-gap-unverifiable`, which fires unconditionally to record that full ISO 15930
//! conformance cannot be checked from this schema at all. The one movable axis is therefore PAGE
//! GEOMETRY, and this vocabulary is that axis and nothing else. Declaring what the 1.7 `✳️x` subset
//! legitimately owns — per-page `/TrimBox`, `/GTS_PDFX` output intents with a `/DestOutputProfile`,
//! encryption dictionaries, font embedding — would be fabricating a vocabulary for a schema that
//! cannot observe a single one of them.
//!
//! **And why it shares no variant with `1.4/✳️a`.** That sibling's `check_pdf_a_conformance` reads
//! `page.text` and never looks at the geometry; this one reads the geometry and never looks at the
//! text. Two subsets of one standard over one snapshot type, disjoint because their checkers read
//! different fields of it.
//!
//! ⚠️ One honest limit, recorded rather than glossed: the `✳️any` subset's `decode_pdf` hardcodes
//! `612×792` for every input and never reads a real page's `/MediaBox` (that subset's own oracle
//! module documents it against this artifact's real fixture, whose true box is `595.276×841.89`).
//! This vocabulary can therefore SET geometry that this repository's own encoder will write, but a
//! decode of someone else's document will not deliver the geometry it actually had. That is a codec
//! gap, not a vocabulary gap, and closing it is out of this vocabulary's scope.
//!
//! @see ../../🧪️oracle/🔣️component.json — the mutation catalog `KINDS` is measured against.
//! @see ../🦀️component.rs — `check_pdf_x_conformance`, the one axis this vocabulary derives from.

use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::diff::PdfDiff;
use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;
use protocol::command::DiffAlgebra;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Class
/// 📐️ The geometry a class stamp writes: ISO 216 A4 in PDF user-space units, the real trim size this
/// artifact's committed fixture is typeset at, rather than a made-up number.
pub const CONFORMANT_WIDTH: f64 = 595.276;
pub const CONFORMANT_HEIGHT: f64 = 841.89;
//#endregion 🔖️Class

//#region 🔖️Mutations
/// 📐️ Typed conformance mutation for `stdio.pdf` 1.4 under ISO 15930. Both non-baseline variants
/// address the one axis `check_pdf_x_conformance` can honestly check.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum PdfX1Mutation {
    /// 🚫️ The identity element of the vocabulary.
    #[default]
    NoMutation,
    /// 🔄️ Replaces the whole document. Build the target with [`stamp_conformance`].
    SetSnapshot {
        snapshot: PdfSnapshot,
    },
    /// 📐️ Sets the page to a strictly positive size — the state
    /// `stdio.pdf.x.degenerate-page-size` stops firing in.
    SetPageSize {
        width: f64,
        height: f64,
    },
    /// 📐️ Collapses the page's width to zero — the degenerate state
    /// `stdio.pdf.x.degenerate-page-size` reports. Height is left alone, so the mutation moves
    /// exactly the one dimension the checker's `width > 0.0 && height > 0.0` test needs to fail.
    CollapsePageSize,
}

/// 🧾️ Kebab-case spelling of every `PdfX1Mutation` variant, in declaration order — the catalog
/// `pdf-1-4-x` (`../../🧪️oracle/🔣️component.json`) is measured against this exact list.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-page-size", "collapse-page-size"];
//#endregion 🔖️Mutations

//#region 🔖️Stamp
/// 🏅️ Stamps the one axis this subset owns into (or out of) its conformant state — the
/// whole-document target `SetSnapshot` carries.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn stamp_conformance(base: PdfSnapshot, stamped: bool) -> PdfSnapshot {
    let mut next = base;
    if stamped {
        next.page.width = CONFORMANT_WIDTH;
        next.page.height = CONFORMANT_HEIGHT;
    } else {
        next.page.width = 0.0;
    }
    next
}
//#endregion 🔖️Stamp

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the diff is the single semantics
/// source, never a separate imperative apply path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_x_conformance_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfX1Mutation) -> protocol::MutationOutcome<PdfDiff> {
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
impl Mutation<PdfSnapshot> for PdfX1Mutation {
    type Diff = PdfDiff;

    fn diff(&self, base: &PdfSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        let mut next = base.clone();
        match self {
            Self::NoMutation => {}
            Self::SetSnapshot { snapshot } => next = snapshot.clone(),
            Self::SetPageSize { width, height } => {
                next.page.width = *width;
                next.page.height = *height;
            }
            Self::CollapsePageSize => next.page.width = 0.0,
        }
        protocol::MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    /// ↩️ Every undo reads the base's OWN geometry rather than assuming a stamp is bijective: a
    /// document already typeset at some size would not be restored by the opposite stamp.
    fn inverse(&self, base: &PdfSnapshot) -> Vec<Self> {
        vec![match self {
            Self::NoMutation => Self::NoMutation,
            Self::SetSnapshot { .. } => Self::SetSnapshot { snapshot: base.clone() },
            Self::SetPageSize { .. } | Self::CollapsePageSize => Self::SetPageSize { width: base.page.width, height: base.page.height },
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

    #[test]
    fn kinds_match_enum_and_catalog() {
        fn kind_of(mutation: &PdfX1Mutation) -> &'static str {
            match mutation {
                PdfX1Mutation::NoMutation => "no-mutation",
                PdfX1Mutation::SetSnapshot { .. } => "set-snapshot",
                PdfX1Mutation::SetPageSize { .. } => "set-page-size",
                PdfX1Mutation::CollapsePageSize => "collapse-page-size",
            }
        }
        let samples = [PdfX1Mutation::NoMutation, PdfX1Mutation::SetSnapshot { snapshot: PdfSnapshot::default() }, PdfX1Mutation::SetPageSize { width: 0.0, height: 0.0 }, PdfX1Mutation::CollapsePageSize];
        assert_eq!(samples.iter().map(kind_of).collect::<Vec<_>>(), KINDS);

        let manifest = include_str!("../../🧪️oracle/🔣️component.json");
        let needle = "\"kinds\": [";
        let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
        let end = start + manifest[start..].find(']').expect("kinds array is closed");
        let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
        assert_eq!(declared, KINDS, "the oracle manifest's kinds must match PdfX1Mutation exactly");
    }

    #[test]
    fn mutation_apply_inverse_round_trips_every_variant() {
        let base = base();
        for mutation in [
            PdfX1Mutation::NoMutation,
            PdfX1Mutation::SetSnapshot { snapshot: stamp_conformance(base.clone(), false) },
            PdfX1Mutation::SetPageSize { width: 419.528, height: 595.276 },
            PdfX1Mutation::CollapsePageSize,
        ] {
            let mut state = base.clone();
            apply_x_conformance_mutation(&mut state, &mutation);
            for undo in mutation.inverse(&base) {
                state = undo.diff(&state).diff().apply(&state).expect("the inverse diff applies");
            }
            assert_eq!(state, base, "apply(inverse(m), apply(m, base)) must recover base for {mutation:?}");
        }
    }

    /// 👁️ Every non-baseline variant genuinely moves the axis this subset's checker reads, and
    /// `CollapsePageSize` really does flip the checker's own verdict.
    #[test]
    fn every_variant_moves_the_axis_the_checker_reads() {
        let base = base();
        for mutation in [PdfX1Mutation::SetSnapshot { snapshot: stamp_conformance(base.clone(), false) }, PdfX1Mutation::SetPageSize { width: 419.528, height: 595.276 }, PdfX1Mutation::CollapsePageSize] {
            let mut state = base.clone();
            apply_x_conformance_mutation(&mut state, &mutation);
            assert_ne!((state.page.width, state.page.height), (base.page.width, base.page.height), "{mutation:?} must move the page geometry");
        }
        let mut collapsed = base.clone();
        apply_x_conformance_mutation(&mut collapsed, &PdfX1Mutation::CollapsePageSize);
        assert!(!(collapsed.page.width > 0.0 && collapsed.page.height > 0.0), "CollapsePageSize must flip check_pdf_x_conformance's own degeneracy test");
    }
}
//#endregion 🧪️Tests
