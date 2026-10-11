//! 🧬️ Baseline owned image interpretation mutations.
use crate::schema::diff::*;
use crate::schema::snapshot::*;
use protocol::Mutation;
#[path = "🔢️set-bits-per-sample/🦀️.rs"]
pub mod set_bits_per_sample;
#[path = "🌈️set-photometric-interpretation/🦀️.rs"]
pub mod set_photometric_interpretation;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = TiffSnapshot, diff = TiffDiff, schema = "TiffBaselineMutation")]
#[value(tag = "mutation", content = "payload", rename_all = "kebab-case")]
pub enum TiffBaselineMutation {
    SetPhotometricInterpretation(set_photometric_interpretation::SetPhotometricInterpretation),
    SetBitsPerSample(set_bits_per_sample::SetBitsPerSample),
}

pub const KINDS: &[&str] = &["set-photometric-interpretation", "set-bits-per-sample"];
crate::impl_serde_op_codec!(TiffBaselineMutation, "tiff-baseline-mutation");

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn tiff_baseline_conformance_codes(snapshot: &TiffSnapshot) -> Vec<String> {
    crate::standards::v6_0::subsets::baseline::schema::check_tiff_baseline_conformance(snapshot).into_iter().map(|finding| finding.code.0).collect()
}



/// 🏷️ The words of the first image page's single-tag entry `tag` when it holds SHORT words, else `None`: the only shape both baseline leaves can write back.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn first_page_shorts(base: &TiffSnapshot, tag: u16) -> Option<&[u16]> {
    match base.ifds.first()?.tag(tag)? {
        TiffValues::Short(words) => Some(words),
        _ => None,
    }
}

/// 🔺️ The sparse diff that sets `tag` of the first image page to `words`: empty when the page already holds them, a refusal when the page does not carry the tag as SHORT words (nothing could restore it).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn set_first_page_shorts(base: &TiffSnapshot, tag: u16, words: Vec<u16>) -> protocol::MutationOutcome<TiffDiff> {
    let Some(page) = base.ifds.first() else { return protocol::MutationOutcome::new(TiffDiff::default()) };
    match (page.tag(tag), first_page_shorts(base, tag)) {
        (Some(_), Some(old)) if old == words.as_slice() => protocol::MutationOutcome::new(TiffDiff::default()),
        (Some(_), Some(_)) => {
            let entries = TiffTagsDiff { modified: vec![TiffTagModified { tag, values: TiffValues::Short(words) }], ..TiffTagsDiff::default() };
            protocol::MutationOutcome::new(TiffDiff { ifds: Some(TiffIfdsDiff { modified: vec![TiffIfdModified { index: 0, diff: TiffIfdDiff { entries, ..TiffIfdDiff::default() } }], ..TiffIfdsDiff::default() }) })
        }
        _ => protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, "tiff baseline: the first page does not carry this tag as SHORT words", [format!("tag:{tag}")]),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
