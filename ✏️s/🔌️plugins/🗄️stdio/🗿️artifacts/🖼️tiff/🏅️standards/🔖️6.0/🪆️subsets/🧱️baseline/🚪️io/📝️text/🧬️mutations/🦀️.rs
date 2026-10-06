//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v6_0::subsets::baseline::schema::mutations::*;
use crate::standards::v6_0::subsets::document::schema::diff::{TiffDiff, TiffIfdDiff, TiffIfdModified, TiffIfdsDiff, TiffTagAdded, TiffTagModified, TiffTagsDiff};
use crate::standards::v6_0::subsets::document::schema::snapshot::{TiffFieldType, TiffSnapshot, TiffTag, TiffValues, TAG_BITS_PER_SAMPLE, TAG_COMPRESSION, TAG_PHOTOMETRIC, TAG_STRIP_OFFSETS, TAG_TILE_LENGTH, TAG_TILE_WIDTH};
use protocol::{Mutation, MutationDiff};

/// 👁️ The comparison surface `🧱️mutate-tiff-6-0-baseline` measures this vocabulary through: the five
/// Baseline TIFF axes as they stand on IFD 0 of the DECODED snapshot, plus
/// [`check_tiff_baseline_conformance`]'s verdict over them. It carries no pixels and no other tag on
/// purpose — this is a conformance-class vocabulary, and a Baseline class is a property of five
/// specific fields of IFD 0, not of the raster or of the other 65 530 tag numbers `✳️any`'s
/// `set-tag` can reach.
///
/// Rendered by hand rather than through `serde` because it is a PROJECTION, not the snapshot: the
/// snapshot's own serialization carries a multi-megabyte raster no comparison here should have to
/// walk.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_tiff_baseline_projection_json(snapshot: &TiffSnapshot) -> String {
    let list = |tag: u16| match snapshot.ifds.first().and_then(|ifd| ifd.entries.iter().find(|entry| entry.tag == tag)) {
        Some(entry) => match &entry.values {
            TiffValues::Short(values) => values.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(" "),
            TiffValues::Long(values) => values.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(" "),
            other => format!("{other:?}"),
        },
        None => "absent".to_string(),
    };
    let verdict = crate::standards::v6_0::subsets::baseline::schema::check_tiff_baseline_conformance(snapshot).into_iter().map(|finding| format!("\"{}\"", finding.code.0)).collect::<Vec<_>>().join(",");
    format!(
        "{{\"format\":\"tiff-baseline\",\"ifdCount\":{},\"compression\":\"{}\",\"photometric\":\"{}\",\"bitsPerSample\":\"{}\",\"tileWidth\":\"{}\",\"tileLength\":\"{}\",\"stripOffsets\":\"{}\",\"conformance\":[{verdict}]}}",
        snapshot.ifds.len(),
        list(TAG_COMPRESSION),
        list(TAG_PHOTOMETRIC),
        list(TAG_BITS_PER_SAMPLE),
        list(TAG_TILE_WIDTH),
        list(TAG_TILE_LENGTH),
        list(TAG_STRIP_OFFSETS)
    )
}
}
pub use mutations_codec::*;
