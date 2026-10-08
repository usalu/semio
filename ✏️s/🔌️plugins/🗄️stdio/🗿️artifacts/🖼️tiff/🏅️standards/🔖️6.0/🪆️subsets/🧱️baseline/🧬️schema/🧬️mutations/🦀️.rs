//! 🧬️ Baseline owned image interpretation mutations.
use crate::schema::diff::*;
use crate::schema::snapshot::*;
use protocol::Mutation;
#[path = "🔢️set-bits-per-sample/🦀️.rs"]
pub mod set_bits_per_sample;
#[path = "🌈️set-photometric-interpretation/🦀️.rs"]
pub mod set_photometric_interpretation;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_tiff_baseline_mutation(snapshot: &mut TiffSnapshot, mutation: &TiffBaselineMutation) -> protocol::MutationOutcome<TiffDiff> {
    let outcome = mutation.diff(snapshot);
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
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

/// 🧮️ The leaves that carry `base` to exactly `next`: the photometric interpretation and the bits per sample of the first page when they moved to SHORT words.
/// Anything else a candidate `next` changes is outside the vocabulary, and the exact replay refuses it.
pub fn net_mutations(base: &TiffSnapshot, next: &TiffSnapshot) -> Vec<TiffBaselineMutation> {
    let photometric = first_page_shorts(next, TAG_PHOTOMETRIC).filter(|words| words.len() == 1 && first_page_shorts(base, TAG_PHOTOMETRIC) != Some(*words)).map(|words| TiffBaselineMutation::SetPhotometricInterpretation(set_photometric_interpretation::SetPhotometricInterpretation { photometric: words[0] }));
    let bits = first_page_shorts(next, TAG_BITS_PER_SAMPLE).filter(|words| first_page_shorts(base, TAG_BITS_PER_SAMPLE) != Some(*words)).map(|words| TiffBaselineMutation::SetBitsPerSample(set_bits_per_sample::SetBitsPerSample { bits: words.to_vec() }));
    photometric.into_iter().chain(bits).collect()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
