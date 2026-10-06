//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v_jfif_1_01::subsets::baseline::schema::mutations::*;
use crate::standards::v_jfif_1_01::subsets::document::schema::diff::{JpgComponentDiff, JpgComponentModified, JpgComponentsDiff, JpgDiff, JpgFrameChange, JpgFrameFieldsDiff, JpgHuffmanTableAdded, JpgHuffmanTableKey, JpgHuffmanTablesDiff};
use crate::standards::v_jfif_1_01::subsets::document::schema::snapshot::{JpgFrameComponent, JpgHuffmanTable, JpgSnapshot};
use protocol::{Mutation, MutationDiff};

/// 👁️ The comparison surface `🟣️mutate-jpg-jfif-1-01-baseline` measures this vocabulary through: the
/// five T.81 Annex F axes as they stand on the DECODED snapshot, plus
/// [`check_baseline_conformance`]'s verdict over them. It carries no pixels and no quantization
/// tables on purpose — this is a conformance-class vocabulary, and a class is a property of the
/// frame header and the entropy-coding mode, not of the raster.
///
/// Rendered by hand rather than through `serde` because it is a PROJECTION, not the snapshot: the
/// snapshot's own serialization carries a multi-megabyte raster that no comparison here should ever
/// have to walk, and the axes are exactly the ten this subset's own checker reads.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_jpg_baseline_projection_json(snapshot: &JpgSnapshot) -> String {
    let quoted = |values: Vec<String>| format!("[{}]", values.into_iter().map(|value| format!("\"{value}\"")).collect::<Vec<_>>().join(","));
    let tables = quoted(snapshot.huffman_tables.iter().map(|table| format!("{:?}:{}", table.class, table.id).to_lowercase()).collect());
    let components = quoted(snapshot.frame.as_ref().map(|frame| frame.components.iter().map(|component| format!("{}:{}x{}", component.id, component.h_sampling, component.v_sampling)).collect()).unwrap_or_default());
    let verdict = quoted(crate::standards::v_jfif_1_01::subsets::baseline::schema::check_baseline_conformance(snapshot).into_iter().map(|finding| finding.code.0).collect());
    format!(
        "{{\"format\":\"jpg-baseline\",\"sofMarker\":\"{:02x}\",\"precision\":{},\"arithmetic\":{},\"componentCount\":{},\"huffmanTables\":{tables},\"components\":{components},\"conformance\":{verdict}}}",
        snapshot.sof_marker,
        snapshot.frame.as_ref().map_or(0, |frame| frame.precision),
        snapshot.arithmetic,
        snapshot.frame.as_ref().map_or(0, |frame| frame.components.len())
    )
}


}
pub use mutations_codec::*;
