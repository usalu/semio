//! 🔺️ PdfDiff (1.4) — handcrafted sparse diff over `PageDoc{width,height,text}`. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION D2 `📄️pdf` row:
//! 1.4 stays the intentionally-frozen pre-real-codec stub (W0 recon: "keep it minimally alive
//! under its own path -- do NOT give it the full 1.7 object-graph model, that would contradict
//! its own documented scope boundary"), but its diff/mutation-diff pair still gets a REAL
//! handcrafted per-field patch instead of the generic `{snapshot: Option<PdfSnapshot>}`
//! full-replace template: `PageDoc` has no collections, so the sparse patch is a flat 3-field
//! struct (`width`/`height`/`text`), no collection triples needed.
//!
//! 🧪️ F6 (real `DiffCodec`): `PdfDiff` has zero `Option<Option<_>>` tri-state fields and zero
//! data-carrying enums anywhere in its (or `PdfSnapshot`'s) tree, so per f6-recon-report.md §3
//! it is on the DERIVE path -- `#[derive(dsl::DslDiff)]` compiles clean (verified via
//! `cargo check -p semio-s-plugin-stdio --lib`, zero pdf-scoped errors), giving a fully
//! generated `protocol::DiffCodec` impl with no hand-written body. `PdfSnapshot`/`PageDoc` both
//! needed `#[derive(dsl::DslRecord)]` added (cascading requirement, §3's decision rule) so this
//! struct's own field types satisfy `DslField`.

use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;
use protocol::MutationDiff;
use protocol::command::DiffAlgebra;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.pdf` (1.4). `schema` is an identity field and is never diffed. No
/// `snapshot: Option<PdfSnapshot>` full-replace slot -- even `SetSnapshot`'s diff is the sparse
/// field-by-field `between(base, next)`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslDiff)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf.diff")]
pub struct PdfDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl MutationDiff<PdfSnapshot> for PdfDiff {
    fn apply(&self, base: &PdfSnapshot) -> PdfSnapshot {
        let mut next = base.clone();
        if let Some(v) = self.width { next.page.width = v; }
        if let Some(v) = self.height { next.page.height = v; }
        if let Some(v) = &self.text { next.page.text = v.clone(); }
        next
    }

    /// ➕️ Structural, total, base-free, sequential-coalesce (`## Absorb` contract) -- flat
    /// scalars only (no collections), so absorb is plain per-field LWW.
    fn absorb(&mut self, other: Self) {
        if other.width.is_some() { self.width = other.width; }
        if other.height.is_some() { self.height = other.height; }
        if other.text.is_some() { self.text = other.text; }
    }
}

impl DiffAlgebra<PdfSnapshot> for PdfDiff {
    /// 🔁️ Diff-level undo, derived generically from `between` (correct by construction).
    fn inverse(&self, base: &PdfSnapshot) -> Self {
        let mid = self.apply(base);
        Self::between(&mid, base)
    }

    fn between(base: &PdfSnapshot, other: &PdfSnapshot) -> Self {
        PdfDiff {
            width: (base.page.width != other.page.width).then_some(other.page.width),
            height: (base.page.height != other.page.height).then_some(other.page.height),
            text: (base.page.text != other.page.text).then(|| other.page.text.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.width.is_none() && self.height.is_none() && self.text.is_none()
    }
}

/// 🧩 `SetSnapshot`'s diff is the sparse field-by-field `between(base, next)` -- no full-replace
/// slot exists on `PdfDiff` to short-circuit into.
pub fn diff_set_snapshot(base: &PdfSnapshot, next: &PdfSnapshot) -> PdfDiff {
    PdfDiff::between(base, next)
}
//#endregion 🔖️Diff

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PageDoc;
    use crate::artifacts::pdf::STDIO_PDF_DOCUMENT_SCHEMA;

    fn snap(width: f64, height: f64, text: &str) -> PdfSnapshot {
        PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), page: PageDoc { width, height, text: text.into() } }
    }

    //#region between_roundtrip_law
    #[test]
    fn between_roundtrip_law() {
        let a = snap(612.0, 792.0, "hello");
        let b = snap(300.0, 400.0, "world");
        assert_eq!(PdfDiff::between(&a, &b).apply(&a), b);
        assert_eq!(PdfDiff::between(&b, &a).apply(&b), a);
        assert!(PdfDiff::between(&a, &a).is_empty());
    }
    //#endregion between_roundtrip_law

    //#region inverse_law
    #[test]
    fn inverse_law_diff_level() {
        let a = snap(612.0, 792.0, "hello");
        let b = snap(300.0, 400.0, "world");
        let d = PdfDiff::between(&a, &b);
        let mid = d.apply(&a);
        assert_eq!(mid, b);
        assert_eq!(d.inverse(&a).apply(&mid), a);
    }
    //#endregion inverse_law

    //#region absorb_law
    #[test]
    fn absorb_law_sequential_composition() {
        let s0 = snap(612.0, 792.0, "a");
        let s1 = snap(300.0, 792.0, "a"); // width changed
        let s2 = snap(300.0, 400.0, "b"); // height + text changed
        let d1 = PdfDiff::between(&s0, &s1);
        let d2 = PdfDiff::between(&s1, &s2);
        let sequential = d2.apply(&d1.apply(&s0));
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&s0), sequential);
        assert_eq!(sequential, s2);
    }

    #[test]
    fn absorb_law_associativity() {
        let s0 = snap(1.0, 1.0, "a");
        let s1 = snap(2.0, 1.0, "a");
        let s2 = snap(2.0, 2.0, "b");
        let s3 = snap(3.0, 2.0, "c");
        let d1 = PdfDiff::between(&s0, &s1);
        let d2 = PdfDiff::between(&s1, &s2);
        let d3 = PdfDiff::between(&s2, &s3);
        let mut left = d1.clone(); left.absorb(d2.clone()); left.absorb(d3.clone());
        let mut right_tail = d2.clone(); right_tail.absorb(d3.clone());
        let mut right = d1.clone(); right.absorb(right_tail);
        assert_eq!(left.apply(&s0), s3);
        assert_eq!(right.apply(&s0), s3);
        assert_eq!(left, right);
    }
    //#endregion absorb_law

    //#region field_sweep
    fn sweep_a() -> PdfSnapshot { snap(612.0, 792.0, "base text") }
    fn sweep_b() -> PdfSnapshot { snap(300.5, 400.25, "changed text") }

    #[test]
    fn field_sweep_between_roundtrips_both_directions() {
        let (a, b) = (sweep_a(), sweep_b());
        assert_eq!(PdfDiff::between(&a, &b).apply(&a), b);
        assert_eq!(PdfDiff::between(&b, &a).apply(&b), a);
        assert!(PdfDiff::between(&a, &a).is_empty());
    }

    #[test]
    fn field_sweep_every_field_present_in_diff() {
        let (a, b) = (sweep_a(), sweep_b());
        let d = PdfDiff::between(&a, &b);
        assert!(d.width.is_some(), "width must be present");
        assert!(d.height.is_some(), "height must be present");
        assert!(d.text.is_some(), "text must be present");
        assert_eq!(d.width, Some(300.5));
        assert_eq!(d.height, Some(400.25));
        assert_eq!(d.text, Some("changed text".to_string()));
    }
    //#endregion field_sweep

    //#region diff_codec_text_binary_roundtrip_law
    /// 🧪️ F6: `protocol::DiffCodec` LAW (`🧰️framework/…/📡️spr/🎮️command/🦀️component.rs:126-165`),
    /// fully derived via `#[derive(dsl::DslDiff)]` -- exercises every field present AND the
    /// fully-empty diff, both text (`print_diff`/`parse_diff`) and binary
    /// (`encode_diff`/`decode_diff`) sides.
    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        use protocol::DiffCodec;
        let (a, b) = (sweep_a(), sweep_b());
        let cases = vec![PdfDiff::between(&a, &b), PdfDiff::between(&a, &a)];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must not contain a newline: {printed:?}");
            let parsed = PdfDiff::parse_diff(&printed).expect("parse_diff must accept its own print_diff output");
            assert_eq!(parsed, d, "parse_diff(print_diff(d)) must equal d");

            let encoded = d.encode_diff().expect("encode_diff must succeed");
            let decoded = PdfDiff::decode_diff(&encoded).expect("decode_diff must accept its own encode_diff output");
            assert_eq!(decoded, d, "decode_diff(encode_diff(d)) must equal d");
        }
    }
    //#endregion diff_codec_text_binary_roundtrip_law
}
//#endregion 🧪️Tests
