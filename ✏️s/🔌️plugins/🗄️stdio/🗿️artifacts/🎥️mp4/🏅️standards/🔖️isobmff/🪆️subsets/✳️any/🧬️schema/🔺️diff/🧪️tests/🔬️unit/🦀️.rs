use super::*;
use crate::standards::isobmff::subsets::any::schema::snapshot::STDIO_MP4_DOCUMENT_SCHEMA;

fn sample(n: u8) -> Mp4Sample {
    Mp4Sample { data: vec![n], duration: u32::from(n) * 10, cts_offset: 0, sync: n % 2 == 0 }
}

fn track(id: u32, samples: Vec<Mp4Sample>) -> Mp4Track {
    Mp4Track { track_id: id, timescale: 1000, codec: Mp4Codec::default(), width: 64, height: 64, metadata: Mp4TrackMetadata::default(), chunk_sample_counts: vec![samples.len() as u32], samples }
}

fn snap(tracks: Vec<Mp4Track>) -> Mp4Snapshot {
    Mp4Snapshot { schema: STDIO_MP4_DOCUMENT_SCHEMA.into(), ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 0, compatible_brands: vec![] }, movie: Mp4Movie::default(), tracks }
}

//#endregion

//#region absorb_law — canonical index-transport cases (schema-design.md)
#[test]
fn absorb_insert_then_remove_before_matches_sequential() {
    let base: Vec<Mp4Sample> = vec![sample(1), sample(2)];
    // d1: insert `f` at final index 2 -> mid = [s1, s2, f]
    let f = Mp4Sample { data: vec![0xAA], duration: 1, cts_offset: 0, sync: true };
    let mut d1: Mp4SamplesDiff = IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: 2, item: f.clone() }] };
    let mid = apply_indexed(&base, &d1, apply_sample_diff);
    // d2: remove base index 0 from mid -> after = [s2, f]
    let d2: Mp4SamplesDiff = IndexedDiff { removed: vec![0], modified: vec![], added: vec![] };
    let after = apply_indexed(&mid, &d2, apply_sample_diff);
    let sequential = after.clone();

    absorb_indexed(&mut d1, d2, absorb_sample_rows, apply_sample_diff_mut);
    let combined = apply_indexed(&base, &d1, apply_sample_diff);
    assert_eq!(combined, sequential, "absorb(d1,d2).apply(base) must equal d2.apply(d1.apply(base))");
    assert_eq!(d1.removed, vec![0], "the real base removal must transport through");
}

#[test]
fn absorb_insert_insert_same_index_both_survive() {
    let base: Vec<Mp4Sample> = vec![sample(1)];
    let f = Mp4Sample { data: vec![0xAA], duration: 1, cts_offset: 0, sync: true };
    let g = Mp4Sample { data: vec![0xBB], duration: 2, cts_offset: 0, sync: false };
    let mut d1: Mp4SamplesDiff = IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: 1, item: f.clone() }] };
    let mid = apply_indexed(&base, &d1, apply_sample_diff);
    assert_eq!(mid, vec![sample(1), f.clone()]);
    let d2: Mp4SamplesDiff = IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: 1, item: g.clone() }] };
    let after = apply_indexed(&mid, &d2, apply_sample_diff);
    let sequential = after.clone();

    absorb_indexed(&mut d1, d2, absorb_sample_rows, apply_sample_diff_mut);
    let combined = apply_indexed(&base, &d1, apply_sample_diff);
    assert_eq!(combined, sequential);
    assert_eq!(combined.len(), 3, "both inserts at the same nominal index must survive (fixes the gif-style LWW-slot bug)");
}

#[test]
fn absorb_modify_patches_into_added_payload() {
    let base: Vec<Mp4Sample> = vec![sample(1)];
    let f = Mp4Sample { data: vec![0xAA], duration: 1, cts_offset: 0, sync: false };
    let mut d1: Mp4SamplesDiff = IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: 1, item: f.clone() }] };
    let mid = apply_indexed(&base, &d1, apply_sample_diff);
    let patch = Mp4SampleDiff { data: None, duration: Some(42), cts_offset: None, sync: Some(true) };
    let d2: Mp4SamplesDiff = IndexedDiff { removed: vec![], modified: vec![IndexedModified { index: 1, diff: patch }], added: vec![] };
    let after = apply_indexed(&mid, &d2, apply_sample_diff);
    let sequential = after.clone();

    absorb_indexed(&mut d1, d2, absorb_sample_rows, apply_sample_diff_mut);
    let combined = apply_indexed(&base, &d1, apply_sample_diff);
    assert_eq!(combined, sequential);
    assert_eq!(combined[1].duration, 42);
    assert!(combined[1].sync);
    assert!(d1.modified.is_empty(), "the patch must land INTO the carried added payload, not become a separate modified entry");
}

#[test]
fn absorb_modify_then_remove_drops_the_modification() {
    let base: Vec<Mp4Sample> = vec![sample(1), sample(2)];
    let mut d1: Mp4SamplesDiff = IndexedDiff { removed: vec![], modified: vec![IndexedModified { index: 0, diff: Mp4SampleDiff { data: None, duration: Some(77), cts_offset: None, sync: None } }], added: vec![] };
    let mid = apply_indexed(&base, &d1, apply_sample_diff);
    let d2: Mp4SamplesDiff = IndexedDiff { removed: vec![0], modified: vec![], added: vec![] };
    let after = apply_indexed(&mid, &d2, apply_sample_diff);
    let sequential = after.clone();

    absorb_indexed(&mut d1, d2, absorb_sample_rows, apply_sample_diff_mut);
    let combined = apply_indexed(&base, &d1, apply_sample_diff);
    assert_eq!(combined, sequential);
    assert!(d1.modified.is_empty(), "a merged-removed key's modified entry must be dropped");
}

#[test]
fn exact_fixture_empty_inverse_absorb_and_source_removal_laws() {
    let bytes = include_bytes!("../../../../🧫️fixtures/🎬️.mp4").to_vec();
    let base = crate::standards::isobmff::subsets::any::io::decode_mp4(&bytes).expect("decode exact MP4 fixture");

    let empty = Mp4Diff::default();
    assert!(empty.is_empty());
    assert_eq!(crate::standards::isobmff::subsets::any::io::encode_mp4(&protocol::apply_diff(&empty, &base).unwrap()), bytes);

    let track = Mp4TrackDiff { width: Some(base.tracks[0].width + 1), ..Default::default() };
    let diff = Mp4Diff { tracks: Some(IndexedDiff { removed: Vec::new(), modified: vec![IndexedModified { index: 0, diff: track }], added: Vec::new() }), ..Default::default() };
    let after = protocol::apply_diff(&diff, &base).unwrap();
    let inverse = diff.inverse(&base);
    assert_eq!(crate::standards::isobmff::subsets::any::io::encode_mp4(&protocol::apply_diff(&inverse, &after).unwrap()), bytes);

    let mut absorbed = diff;
    absorbed.absorb(inverse);
    assert_eq!(crate::standards::isobmff::subsets::any::io::encode_mp4(&protocol::apply_diff(&absorbed, &base).unwrap()), bytes);
}
//#endregion
