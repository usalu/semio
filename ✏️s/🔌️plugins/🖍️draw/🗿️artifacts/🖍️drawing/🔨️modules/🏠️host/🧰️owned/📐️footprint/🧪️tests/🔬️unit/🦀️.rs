//! 🧪️ Native retained text footprint and exact zero-allocation closure.

use super::*;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use semio_framework_value::{list::PagedList, retained_clone::{RetainedCloneGrant, RetainedCloneStep}, retirement::controlled::ControlledRetirement};

#[test]
fn paged_native_drawing_snapshot_duplicate_close_preserves_physical_grant_axes() {
    use super::super::{DrawingDuplicateRewriteAuthority, initial_snapshot_clone_grant};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    let retained = law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize);
    for partial in law["cancelAt"].as_array().unwrap() {
        let partial = partial.as_u64().unwrap() as usize;
        for held in [false, true] {
            let original = if held { PagedUtf8::<{usize::MAX}>::try_from_str(&retained).unwrap() } else { Default::default() };
            let (mut rewrite, allocation) = observe(|| DrawingDuplicateRewriteAuthority::new(String::new(), String::new()));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            *rewrite.native_id = original;
            if !held {
                for turn in 0..partial {
                    rewrite.append_active = true;
                    let grant = initial_snapshot_clone_grant(turn);
                    let (step, allocation) = observe(|| rewrite.append.advance("layer-0123456789abcdef", &mut *rewrite.native_id, grant).unwrap());
                    assert!(step.progress().fits(grant));assert!(allocation.requested_bytes <= step.progress().retained_capacity_bytes);assert!(allocation.released_bytes <= step.progress().released_bytes);
                }
            }
            for grant in [RetainedCloneGrant::default(), RetainedCloneGrant { maximum_items: 0, maximum_copy_bytes: 4096, maximum_capacity_bytes: 4096, maximum_release_bytes: 4096, maximum_depth: usize::MAX }] {
                let (step, allocation) = observe(|| rewrite.close_granted(grant).unwrap());
                assert_eq!(step.progress(), Default::default());assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            }
            let mut released = 0;
            let mut turns = 0;
            for turn in 0..100000 {
                let grant = initial_snapshot_clone_grant(turn);
                let (step, allocation) = observe(|| rewrite.close_granted(grant).unwrap());
                let progress = step.progress();assert!(progress.fits(grant));assert!(!allocation.overflowed);
                assert!(allocation.requested_bytes <= progress.retained_capacity_bytes);assert!(allocation.released_bytes <= progress.released_bytes);
                assert!(progress.copied_bytes + progress.retained_capacity_bytes + progress.released_bytes <= 4096);
                released += allocation.released_bytes;turns += 1;
                if matches!(step, RetainedCloneStep::Complete(_)) { break; }
            }
            let (step, allocation) = observe(|| rewrite.close_granted(RetainedCloneGrant::one_release_turn(4096, usize::MAX)).unwrap());
            assert_eq!(step, RetainedCloneStep::Complete(Default::default()));assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (owners, allocation) = observe(|| rewrite.take_owners().expect("duplicate closure returns empty pool owners once"));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));assert!(rewrite.take_owners().is_none());assert!(rewrite.terminal_is_empty());
            let (_, allocation) = observe(|| { drop(owners);drop(rewrite); });assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            eprintln!("[DEBUG] Drawing duplicate cancellation partial={partial} held={held} turns={turns} actualReleased={released}; exact three-axis grant, zero constructor/pause/terminal drop");
        }
    }
}

#[test]
fn paged_native_drawing_snapshot_decoded_field_close_admits_actual_birth_and_release() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    for bytes in law["decodedFieldClose"]["physicalBackingCapacities"].as_array().unwrap() {
        let bytes = bytes.as_u64().unwrap() as usize;
        let mut field = String::with_capacity(bytes);
        if bytes > 0 { field.push('x'); }
        let retained = field.capacity();
        let (mut retirement, allocation) = observe(|| super::super::DrawingDecodedFieldRetirement::try_new(field).unwrap_or_else(|_| panic!("native decoded text supports retirement")));
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        for (items, bytes) in [(0, 4096), (1, 0)] {
            let (step, allocation) = observe(|| retirement.step(items, bytes).unwrap());
            assert_eq!(step.progress(), Default::default());assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        }
        let mut released = 0;
        let mut turns = 0;
        let mut scaffold_extent = 0;
        for _ in 0..10000 {
            let (demand, allocation) = observe(|| retirement.next_close_byte_demand().unwrap());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            if demand > 0 {
                let (step, allocation) = observe(|| retirement.step(1, demand - 1).unwrap());
                assert_eq!(step.progress(), Default::default());assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            }
            let grant = 4096.max(demand);
            scaffold_extent = scaffold_extent.max(demand);
            let (step, allocation) = observe(|| retirement.step(1, grant).unwrap());
            let progress = step.progress();
            assert!(!allocation.overflowed);assert!(allocation.requested_bytes <= progress.retained_capacity_bytes);assert!(allocation.released_bytes <= progress.released_bytes);
            assert!(progress.retained_capacity_bytes + progress.released_bytes <= grant);
            released += allocation.released_bytes;turns += 1;
            if matches!(step, RetainedCloneStep::Complete(_)) && retirement.terminal_is_empty() { break; }
        }
        assert!(retirement.terminal_is_empty());assert!(released >= retained);
        let (_, allocation) = observe(|| drop(retirement));assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        eprintln!("[DEBUG] Drawing decoded native field close retained={retained} turns={turns} exact admitted scaffold extent={scaffold_extent} body4096; zero heap constructor/pause/undergrant/demand/terminal drop");
    }
}

#[test]
fn paged_native_drawing_snapshot_duplicate_identity_matches_independent_siphash() {
    use std::hash::Hash;
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    let suffix = law["duplicateIdentity"]["suffix"].as_str().unwrap();
    let mut ids: Vec<String> = law["duplicateIdentity"]["ids"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().into()).collect();
    ids.push(law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize) + "\0");
    for id in ids {
        let material = format!("{id}{suffix}");
        let mut oracle = siphasher::sip::SipHasher13::new_with_keys(0, 0);
        material.as_bytes().hash(&mut oracle);
        let expected = format!("layer-{:016x}", oracle.finish());
        assert_eq!(expected, crate::schema::create_drawing_id("layer", material.as_bytes()));
        let chunks = std::iter::once(String::new()).chain(id.chars().flat_map(|value| [value.to_string(), String::new()]));
        let source = PagedUtf8::<{usize::MAX}>::from_retained_chunks(PagedList::try_from_iter(chunks).unwrap(), id.len()).unwrap();
        let foreign = PagedUtf8::<{usize::MAX}>::default();
        for bytes in [1usize, 7, 4096] {
            let (mut cursor, allocation) = observe(DrawingDuplicateIdentityCursor::default);
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));assert!(size_of_val(&cursor) <= 4096);
            for (items, bytes) in [(0, bytes), (1, 0)] {
                let (step, allocation) = observe(|| cursor.step(&source, suffix, items, bytes).unwrap());
                assert_eq!(step, DrawingDuplicateIdentityStep::default());assert!(cursor.terminal_is_empty());
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            }
            let mut total = 0;
            for _ in 0..100000 {
                let (step, allocation) = observe(|| cursor.step(&source, suffix, 1, bytes).unwrap());
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));assert!(step.observed_bytes <= bytes);total += step.observed_bytes;
                let (refused, allocation) = observe(|| cursor.step(&foreign, suffix, 1, bytes));
                assert_eq!(refused.unwrap_err(), "drawing-store.duplicate-identity-owner-changed");assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                if step.complete { break; }
            }
            assert_eq!(cursor.identity(), Some(expected.as_str()));assert_eq!(total, size_of::<usize>() + material.len());
            let (done, allocation) = observe(|| cursor.close_step(0));assert!(!done);assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (done, allocation) = observe(|| cursor.close_step(1));assert!(done);assert!(cursor.terminal_is_empty());assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (_, allocation) = observe(|| drop(cursor));assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            eprintln!("[DEBUG] Drawing duplicate original UTF8/NUL ID+suffix exact SipHash1-3 cold authority {expected} bytes={total} grant={bytes}; zero heap birth/read/refusal/close/drop");
        }
        for turn in law["cancelAt"].as_array().unwrap() {
            let mut cursor = DrawingDuplicateIdentityCursor::default();
            for _ in 0..turn.as_u64().unwrap() { cursor.step(&source, suffix, 1, 7).unwrap(); }
            let (done, allocation) = observe(|| cursor.close_step(1));assert!(done);assert!(cursor.terminal_is_empty());assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (_, allocation) = observe(|| drop(cursor));assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        }
    }
}

#[test]
fn paged_native_drawing_snapshot_sparse_path_patch_preserves_semantics_and_native_retirement() {
    const BEFORE: &str = include_str!("../../../../../../🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧫️fixtures/🧬️mutations/✏️update-path-geometry/✏️reshape/📸️snapshot/⬅️before/🔣️.json");
    const AFTER: &str = include_str!("../../../../../../🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧫️fixtures/🧬️mutations/✏️update-path-geometry/✏️reshape/📸️snapshot/➡️after/🔣️.json");
    const DELTA: &str = include_str!("../../../../../../🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧫️fixtures/🧬️mutations/✏️update-path-geometry/✏️reshape/🔺️diff/🔣️.json");
    let before: DrawingSnapshot = serde_json::from_str(BEFORE).unwrap();
    let mut delta: crate::diff::DrawingDiff = serde_json::from_str(DELTA).unwrap();
    let after = protocol::apply_diff(&delta, &before).unwrap();
    assert_eq!(serde_json::to_value(&after).unwrap(), serde_json::from_str::<serde_json::Value>(AFTER).unwrap());
    let segments = delta.layers.as_mut().unwrap().modified[0].patch.path_segments.take().unwrap();
    assert!(segments.allocated_bytes() > 0);
    let (mut retirement, allocation) = observe(|| ControlledRetirement::new(segments).unwrap_or_else(|_| panic!("native path owner supports controlled retirement")));
    assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
    let (step, allocation) = observe(|| retirement.step(RetainedCloneGrant::default()).unwrap());
    assert_eq!(step.progress(), Default::default());
    assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
    let mut turns = 0;
    for turn in 0..100000 {
        let grant = match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(4096, usize::MAX), 1 => RetainedCloneGrant::one_payload_turn(4096, usize::MAX), _ => RetainedCloneGrant::one_release_turn(4096, usize::MAX) };
        let (step, allocation) = observe(|| retirement.step(grant).unwrap());
        let progress = step.progress();
        assert!(!allocation.overflowed);assert!(allocation.requested_bytes <= progress.retained_capacity_bytes);assert!(allocation.released_bytes <= progress.released_bytes);
        assert!(progress.copied_bytes + progress.retained_capacity_bytes + progress.released_bytes <= 4096);
        turns += 1;
        if matches!(step, RetainedCloneStep::Complete(_)) && retirement.terminal_is_empty() { break; }
    }
    assert!(retirement.terminal_is_empty());
    let (_, allocation) = observe(|| drop(retirement));assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
    eprintln!("[DEBUG] Drawing native sparse path patch exact independent before/after semantic arrays and controlled path-owner three-axis retirement turns={turns}; cold whole diff/apply uncredited");
}

#[test]
fn paged_native_drawing_snapshot_identifier_equality_obeys_independent_unicode_law() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    let mut cases: Vec<(String, String, bool)> = law["textEquality"].as_array().unwrap().iter().map(|value| (value["left"].as_str().unwrap().into(), value["right"].as_str().unwrap().into(), value["equal"].as_bool().unwrap())).collect();
    let long = law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize);
    cases.push((long.clone() + "\0", long.clone() + "\0", true));
    cases.push((long.clone() + "x", long + "y", false));
    for (left, right, expected) in cases {
        let left = PagedUtf8::<{usize::MAX}>::try_from_str(&left).unwrap();
        let pieces = std::iter::once(String::new()).chain(right.chars().flat_map(|value| [value.to_string(), String::new()]));
        let right = PagedUtf8::<{usize::MAX}>::from_retained_chunks(PagedList::try_from_iter(pieces).unwrap(), right.len()).unwrap();
        let foreign = PagedUtf8::<{usize::MAX}>::default();
        for grant in [2usize, 14, 4096] {
            let (mut cursor, allocation) = observe(DrawingTextEqualityCursor::default);
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            assert!(std::mem::size_of_val(&cursor) <= 4096);
            for (items, bytes) in [(0, grant), (1, 0)] {
                let (step, allocation) = observe(|| cursor.step(&left, &right, items, bytes).unwrap());
                assert_eq!(step, DrawingTextEqualityStep::default());assert!(cursor.terminal_is_empty());
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            }
            let mut completed = false;
            let mut observed = 0;
            for _ in 0..100000 {
                let (step, allocation) = observe(|| cursor.step(&left, &right, 1, grant).unwrap());
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                assert!(step.compared_bytes <= grant);observed += step.compared_bytes;
                let (refused, allocation) = observe(|| cursor.step(&foreign, &right, 1, grant));
                assert_eq!(refused.unwrap_err(), "drawing-store.text-equality-owner-changed");
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                if step.complete { completed = true;break; }
            }
            assert!(completed);assert_eq!(cursor.result(), Some(expected));
            let (complete, allocation) = observe(|| cursor.close_step(0));assert!(!complete);
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (complete, allocation) = observe(|| cursor.close_step(1));assert!(complete);assert!(cursor.terminal_is_empty());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (_, allocation) = observe(|| drop(cursor));assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            eprintln!("[DEBUG] Drawing exact native ID equality grant={grant} expected={expected} observed={observed} zero heap birth/pause/compare/source-refusal/close/drop; Unicode/NUL/empty backing and independent chunk partitions");
        }
        for stage in law["cancelAt"].as_array().unwrap() {
            let mut cursor = DrawingTextEqualityCursor::default();
            for _ in 0..stage.as_u64().unwrap() { cursor.step(&left, &right, 1, 14).unwrap(); }
            let (complete, allocation) = observe(|| cursor.close_step(1));assert!(complete);assert!(cursor.terminal_is_empty());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (_, allocation) = observe(|| drop(cursor));assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        }
    }
}

#[test]
fn paged_native_drawing_snapshot_record_footprint_matches_actual_root_and_asset_heap() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    let long = law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize) + "\0";
    let (snapshot, allocation) = observe(|| crate::schema::default_drawing_document(&long, Some(&long)));
    assert!(!allocation.overflowed);
    let root_retained = allocation.requested_bytes.checked_sub(allocation.released_bytes).unwrap();
    let ((key, asset), allocation) = observe(|| (PagedUtf8::try_from_str(&long).unwrap(), DrawingImageAsset { width: long.len() as u32, height: 1, samples: vec![[1,2,3,4];long.len()].into() }));
    assert!(!allocation.overflowed);
    let asset_retained = allocation.requested_bytes.checked_sub(allocation.released_bytes).unwrap();
    for (source, retained) in [(DrawingRecordFootprintSource::Snapshot(&snapshot), root_retained), (DrawingRecordFootprintSource::Asset(&key, &asset), asset_retained)] {
        let (mut cursor, allocation) = observe(DrawingRecordFootprintCursor::default);
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        assert!(std::mem::size_of_val(&cursor) <= 4096);
        let (complete, allocation) = observe(|| cursor.step(source, 0).unwrap());
        assert!(!complete);assert!(cursor.terminal_is_empty());
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        let mut complete = false;
        for _ in 0..100000 {
            let (step, allocation) = observe(|| cursor.step(source, 1));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let foreign = match source { DrawingRecordFootprintSource::Snapshot(_) => DrawingRecordFootprintSource::Asset(&key, &asset), _ => DrawingRecordFootprintSource::Snapshot(&snapshot) };
            let (refused, allocation) = observe(|| cursor.step(foreign, 1));
            assert_eq!(refused.unwrap_err(), "drawing-store.record-footprint-owner-changed");
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            if step.unwrap() { complete = true; break; }
        }
        assert!(complete);assert_eq!(cursor.totals().unwrap().backing_bytes, retained);
        let (complete, allocation) = observe(|| cursor.close_step(0));assert!(!complete);
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        let (complete, allocation) = observe(|| cursor.close_step(1));assert!(complete);assert!(cursor.terminal_is_empty());
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        let (_, allocation) = observe(|| drop(cursor));assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        for stage in law["cancelAt"].as_array().unwrap() {
            let mut cursor = DrawingRecordFootprintCursor::default();
            for _ in 0..stage.as_u64().unwrap() { cursor.step(source, 1).unwrap(); }
            let (complete, allocation) = observe(|| cursor.close_step(1));assert!(complete);assert!(cursor.terminal_is_empty());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (_, allocation) = observe(|| drop(cursor));assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        }
        eprintln!("[DEBUG] Drawing direct native record footprint actualRetainedHeap={retained} exact zero heap birth/pause/read/source-refusal/partial-cancel/close/drop; inline and descendant owners counted separately");
    }
}

#[test]
fn paged_native_drawing_snapshot_direct_layer_footprint_matches_actual_retained_heap() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    let long = law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize) + "\0";
    for kind in 0..7 {
        let (source, allocation) = observe(|| {
            let mut source = match kind {
                0 => crate::schema::create_drawing_shape_layer_rect(&long),
                1 => crate::schema::create_drawing_path_layer(&long, PagedList::default()),
                2 => crate::schema::create_drawing_text_layer(&long),
                3 => crate::schema::create_drawing_image_layer(&long, &long),
                4 => crate::schema::create_drawing_group_layer(&long),
                5 => crate::schema::create_drawing_boolean_layer(&long, "union", PagedList::try_from_iter([PagedUtf8::try_from_str(&long).unwrap(), PagedUtf8::default()]).unwrap()),
                _ => crate::schema::create_drawing_trace_layer(&long, &long),
            };
            if let DrawingLayerNode::Text(value) = &mut source { value.content = PagedUtf8::try_from_str(&long).unwrap(); }
            source
        });
        assert!(!allocation.overflowed);
        let retained = allocation.requested_bytes.checked_sub(allocation.released_bytes).unwrap();
        let foreign = crate::schema::create_drawing_group_layer("foreign");
        let (mut cursor, allocation) = observe(DrawingLayerFootprintCursor::default);
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        assert!(std::mem::size_of_val(&cursor) <= 4096);
        let (complete, allocation) = observe(|| cursor.step(&source, 0).unwrap());
        assert!(!complete);assert!(cursor.terminal_is_empty());
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        let mut complete = false;
        for _ in 0..100000 {
            let (step, allocation) = observe(|| cursor.step(&source, 1));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (refused, allocation) = observe(|| cursor.step(&foreign, 1));
            assert_eq!(refused.unwrap_err(), "drawing-store.layer-footprint-owner-changed");
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            if step.unwrap() { complete = true; break; }
        }
        assert!(complete);assert_eq!(cursor.totals().unwrap().backing_bytes, retained);
        let (complete, allocation) = observe(|| cursor.close_step(0));assert!(!complete);
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        let (complete, allocation) = observe(|| cursor.close_step(1));assert!(complete);assert!(cursor.terminal_is_empty());
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        let (_, allocation) = observe(|| drop(cursor));assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        for stage in law["cancelAt"].as_array().unwrap() {
            let mut cursor = DrawingLayerFootprintCursor::default();
            for _ in 0..stage.as_u64().unwrap() { cursor.step(&source, 1).unwrap(); }
            let (complete, allocation) = observe(|| cursor.close_step(1));assert!(complete);assert!(cursor.terminal_is_empty());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (_, allocation) = observe(|| drop(cursor));assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        }
        eprintln!("[DEBUG] Drawing direct native layer footprint kind={kind} actualRetainedHeap={retained} exact zero heap birth/pause/read/source-refusal/partial-cancel/close/drop; descendants separately owned");
    }
}

#[test]
fn paged_native_drawing_snapshot_text_digest_matches_independent_chunked_sha256() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    let text = law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize) + "\0";
    let source = PagedUtf8::<{usize::MAX}>::try_from_str(&text).unwrap();
    let foreign = PagedUtf8::<{usize::MAX}>::default();
    let tag = law["textDigest"]["tag"].as_u64().unwrap() as u16;
    for maximum_bytes in [1usize, 7, 4096] {
        let (mut cursor, allocation) = observe(DrawingTextDigestCursor::default);
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        assert!(std::mem::size_of_val(&cursor) <= 4096);
        let mut digest = semio_framework_hash::Sha256::new();
        let (step, allocation) = observe(|| cursor.step(&source, tag, 0, maximum_bytes, |_| panic!("zero items observed a byte")));
        assert_eq!(step.unwrap(), DrawingTextDigestStep::default());assert!(cursor.terminal_is_empty());
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        let (step, allocation) = observe(|| cursor.step(&source, tag, 1, 0, |_| panic!("zero bytes observed a byte")));
        assert_eq!(step.unwrap(), DrawingTextDigestStep::default());assert!(cursor.terminal_is_empty());
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        let mut bytes = 0usize;
        let mut complete = false;
        for _ in 0..100000 {
            let (step, allocation) = observe(|| cursor.step(&source, tag, 1, maximum_bytes, |value| digest.update(value)));
            let step = step.unwrap();assert!(step.observed_bytes <= maximum_bytes);
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            bytes += step.observed_bytes;
            let (refused, allocation) = observe(|| cursor.step(&foreign, tag, 1, maximum_bytes, |_| panic!("foreign text observed")));
            assert_eq!(refused.unwrap_err(), "drawing-store.text-digest-owner-changed");
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            if step.complete { complete = true; break; }
        }
        assert!(complete);assert_eq!(bytes, 11 + source.len());
        assert_eq!(cursor.totals().unwrap().chunks, source.retained_chunks().len());
        let actual: String = digest.finalize().into_iter().map(|byte| format!("{byte:02x}")).collect();
        assert_eq!(actual, law["textDigest"]["sha256"].as_str().unwrap());
        let (complete, allocation) = observe(|| cursor.close_step(0));assert!(!complete);
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        let (complete, allocation) = observe(|| cursor.close_step(1));assert!(complete);assert!(cursor.terminal_is_empty());
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        let (_, allocation) = observe(|| drop(cursor));assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        eprintln!("[DEBUG] Drawing native text digest byteGrant={maximum_bytes} observed={bytes} independentSha256={actual} zero-heap constructor/pauses/source refusal/body/close/drop");
    }
    for stage in law["cancelAt"].as_array().unwrap() {
        let mut cursor = DrawingTextDigestCursor::default();
        for _ in 0..stage.as_u64().unwrap() { cursor.step(&source, tag, 1, 7, |_| {}).unwrap(); }
        let (complete, allocation) = observe(|| cursor.close_step(1));assert!(complete);assert!(cursor.terminal_is_empty());
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        let (_, allocation) = observe(|| drop(cursor));assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
    }
    assert_eq!(source, text.as_str());
}

#[test]
fn paged_native_drawing_snapshot_text_footprint_counts_empty_spare_backing_exactly() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    let capacities: [usize; 4] = std::array::from_fn(|index| law["ownedChunkCapacities"][index].as_u64().unwrap() as usize);
    let (source, allocation) = observe(|| {
        let chunks: [String; 4] = std::array::from_fn(|index| String::with_capacity(capacities[index]));
        let chunks = PagedList::<String, {usize::MAX}>::try_from_iter(chunks).unwrap();
        PagedUtf8::<{usize::MAX}>::from_retained_chunks(chunks, 0).unwrap()
    });
    assert!(!allocation.overflowed);
    let retained = allocation.requested_bytes.checked_sub(allocation.released_bytes).unwrap();
    assert!(retained > law["bodyBytes"].as_u64().unwrap() as usize);
    let foreign = PagedUtf8::<{usize::MAX}>::default();
    let (mut cursor, allocation) = observe(DrawingTextFootprintCursor::default);
    assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
    assert!(std::mem::size_of_val(&cursor) <= 6 * std::mem::size_of::<usize>());
    let (complete, allocation) = observe(|| cursor.step(&source, 0).unwrap());
    assert!(!complete);assert!(cursor.terminal_is_empty());
    assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
    for _ in 0..capacities.len() + 3 {
        let (result, allocation) = observe(|| cursor.step(&source, 1));
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        let complete = result.unwrap();
        let (refused, allocation) = observe(|| cursor.step(&foreign, 1));
        assert_eq!(refused.unwrap_err(), "drawing-store.text-footprint-owner-changed");
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        if complete { break; }
    }
    assert_eq!(cursor.totals(), Some(DrawingTextFootprint { chunks: capacities.len(), backing_bytes: retained }));
    assert!(source.is_empty());assert_eq!(source.retained_chunks().len(), capacities.len());
    let (complete, allocation) = observe(|| cursor.close_step(0));
    assert!(!complete);assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
    let (complete, allocation) = observe(|| cursor.close_step(1));
    assert!(complete);assert!(cursor.terminal_is_empty());
    assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
    let (_, allocation) = observe(|| drop(cursor));
    assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
    let (retirement, allocation) = observe(|| ControlledRetirement::new(source));
    let mut retirement = retirement.unwrap();
    assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
    for turn in 0..10000 {
        let grant = match turn % 3 {
            0 => RetainedCloneGrant::one_capacity_turn(4096, usize::MAX),
            1 => RetainedCloneGrant::one_payload_turn(4096, usize::MAX),
            _ => RetainedCloneGrant::one_release_turn(4096, usize::MAX),
        };
        let (result, allocation) = observe(|| retirement.step(grant));
        let step = result.unwrap();let progress = step.progress();
        assert!(!allocation.overflowed);
        assert!(allocation.requested_bytes <= progress.retained_capacity_bytes);
        assert!(allocation.released_bytes <= progress.released_bytes);
        assert!(progress.retained_capacity_bytes + progress.copied_bytes + progress.released_bytes <= 4096);
        if matches!(step, RetainedCloneStep::Complete(_)) { break; }
    }
    assert!(retirement.terminal_is_empty());
    let (_, allocation) = observe(|| drop(retirement));
    assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
    eprintln!("[DEBUG] Drawing native text footprint exactly counts actual empty/spare/full chunk backing with zero constructor/read/foreign refusal/close/drop heap; original source owns all storage until controlled retirement");
}
