//! 🧬️ Drawing retained mutation laws over the native snapshot: initialization catalog admission, retirement variants, decoded field close, asset cursor and the neutral schema clone, with the heap witness allocator.

use super::*;
use semio_framework_test_fixtures::fixture_text;

#[test]
fn paged_native_drawing_snapshot_initialization_catalog_admits_one_original_page_and_empty_drop() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    let law: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🧰️owned/🏗️initialization/📚️catalog/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let lanes = law["lanes"].as_array().unwrap();
    let mut catalog = Some(store::ArtifactStoreInitializationOwnerCatalog::try_new().unwrap());
    let original = catalog.as_ref().unwrap().admitted_items();
    assert_eq!(original % lanes.len(), 0);
    let slots = original / lanes.len();
    let maximum_items = law["maximumItems"].as_u64().unwrap() as usize;
    for (index, _lane) in lanes.iter().enumerate() {
        let (demand, heap) = observe(|| initialization_catalog_close_demands(&catalog).unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(demand.release_bytes > 0);
        assert_eq!((demand.copy_bytes, demand.capacity_bytes), (0, 0));
        let exact = RetainedCloneGrant { maximum_items, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        let refused = [
            RetainedCloneGrant { maximum_items: 0, ..exact },
            RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..exact },
        ];
        for grant in refused {
            let (step, heap) = observe(|| close_initialization_catalog(&mut catalog, grant).unwrap());
            assert_eq!(step.progress(), RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(catalog.as_ref().unwrap().admitted_items(), (lanes.len() - index) * slots);
            assert_eq!(initialization_catalog_close_demands(&catalog).unwrap(), demand);
        }
        let (step, heap) = observe(|| close_initialization_catalog(&mut catalog, exact).unwrap());
        assert_eq!(step.progress().copied_items, 1);
        assert_eq!(step.progress().released_bytes, demand.release_bytes);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, demand.release_bytes));
        assert_eq!(catalog.as_ref().map_or(0, |catalog| catalog.admitted_items()), (lanes.len() - index - 1) * slots);
        if index + 1 == lanes.len() { assert!(matches!(step, RetainedCloneStep::Complete(_))); }
    }
    assert!(catalog.is_none());
    assert_eq!(law["coldBirthCredited"], false);
    let (step, heap) = observe(|| close_initialization_catalog(&mut catalog, RetainedCloneGrant { maximum_items: 1, maximum_depth: 1, ..Default::default() }).unwrap());
    assert!(matches!(step, RetainedCloneStep::Complete(_)));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    let (_, heap) = observe(|| drop(catalog));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, law["terminalDropBytes"].as_u64().unwrap() as usize));
}

#[test]
fn paged_native_drawing_snapshot_retirement_keeps_original_variants_and_exact_grants() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    use semio_framework_value::paged::PagedUtf8;
    use semio_framework_value_persistent::list::PagedList;
    let law: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🧰️owned/♻️retirement/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let snapshot: DrawingSnapshot = serde_json::from_str(fixture_text!("../../../../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let layer = snapshot.layers[0].clone();
    let owners = vec![
        DrawingRetirementOwner::Snapshot(snapshot),
        DrawingRetirementOwner::Asset(DrawingImageAsset{width:1,height:1,samples:vec![[13,27,89,255]].into()}),
        DrawingRetirementOwner::Mutation(DrawingMutation::RenameLayer(RenameLayer { layer_id: "native-owner".into(), new_name: "Grüße\0🧬".into() })),
        DrawingRetirementOwner::Layer(layer),
        DrawingRetirementOwner::Fill(FillStyle::LinearGradient { x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0, stops: vec![GradientStop { offset: 0.5, color: [0.1, 0.2, 0.3, 1.0] }].into() }),
        DrawingRetirementOwner::Stroke(StrokeStyle { color: [0.1, 0.2, 0.3, 1.0], width: 2.0, cap: semio_s_artifact_draw_drawing_model::stroke::StrokeCap::Round, join: semio_s_artifact_draw_drawing_model::stroke::StrokeJoin::Bevel, dash: Some(vec![1.0, 2.0].into()) }),
        DrawingRetirementOwner::String(PagedUtf8::try_from_str(&"Grüße\0🧬".repeat(1000)).unwrap()),
        DrawingRetirementOwner::Segments(PagedList::default()),
        DrawingRetirementOwner::SegmentCollections(PagedList::try_from_iter([PagedList::<PathSegment, {usize::MAX}>::default()]).unwrap()),
        DrawingRetirementOwner::HistoryId(String::from("native-history-owner")),
    ];
    assert_eq!(owners.len(), law["variants"].as_array().unwrap().len());
    let maximum = law["maximumBytes"].as_u64().unwrap() as usize;
    assert!(size_of::<DrawingDecodedFieldRetirement<DrawingRetirementOwner>>() <= maximum);
    for (value, kind) in owners.into_iter().zip(law["variants"].as_array().unwrap()) {
        let (mut owner, heap) = observe(|| DrawingDecodedFieldRetirement::try_new(value).unwrap_or_else(|_| panic!("Drawing native owner has typed retirement authority")));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (law["constructorAllocationBytes"].as_u64().unwrap() as usize, 0));
        let mut copied = 0;
        let mut released = 0;
        let mut terminal = false;
        for _ in 0..100000 {
            let (grant, heap) = observe(|| owner.next_grant().unwrap());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            let bytes = grant.maximum_copy_bytes + grant.maximum_capacity_bytes + grant.maximum_release_bytes;
            assert!(bytes <= maximum);
            let (demand, heap) = observe(|| owner.demands().unwrap());
            assert_eq!(demand.copy_bytes + demand.capacity_bytes + demand.release_bytes, bytes);
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            let mut refused = vec![RetainedCloneGrant { maximum_items: 0, maximum_copy_bytes: maximum, maximum_capacity_bytes: maximum, maximum_release_bytes: maximum, ..grant }];
            if bytes != 0 {
                refused.push(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: 0, ..grant });
                refused.push(RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes.saturating_sub(1), maximum_capacity_bytes: grant.maximum_capacity_bytes.saturating_sub(1), maximum_release_bytes: grant.maximum_release_bytes.saturating_sub(1), ..grant });
            }
            for denied in refused {
                let (step, heap) = observe(|| owner.step(denied).unwrap());
                assert_eq!(step.progress(), RetainedCloneProgress::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(owner.next_grant().unwrap(), grant);
            }
            let (step, heap) = observe(|| owner.step(grant).unwrap());
            let progress = step.progress();
            assert!(progress.fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
            assert!(progress.copied_items <= 1);
            if progress.copied_bytes != 0 { assert_eq!(heap.released_bytes, 0); }
            copied += progress.copied_bytes;
            released += progress.released_bytes;
            if matches!(step, RetainedCloneStep::Complete(_)) { terminal = true; break; }
        }
        assert!(terminal && owner.terminal_is_empty());
        let (_, heap) = observe(|| drop(owner));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    }
}

#[test]
fn paged_native_drawing_snapshot_decoded_field_close_admits_payload_and_physical_work() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    fn run<T: semio_framework_value::retirement::RetireOwned>(value: T, row: &serde_json::Value) {
        let (mut owner, heap) = observe(|| DrawingDecodedFieldRetirement::try_new(value).unwrap_or_else(|_| panic!("native field has typed retirement")));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let mut copied = 0;
        let mut observed = false;
        let mut terminal = false;
        for _ in 0..100000 {
            let (demand, heap) = observe(|| owner.demands().unwrap());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            let (copy, capacity, release) = (demand.copy_bytes, demand.capacity_bytes, demand.release_bytes);
            assert!(copy + capacity + release <= 4096);
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: demand.depth.max(1) };
            let (zero, heap) = observe(|| owner.step(RetainedCloneGrant::default()).unwrap());
            assert_eq!(zero.progress(), Default::default());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            if copy > 0 {
                observed = true;
                assert_eq!(copy, row["minimumCopyBytes"].as_u64().unwrap() as usize);
                assert_eq!((capacity, release), (0, 0));
                let (denied, heap) = observe(|| owner.step(RetainedCloneGrant { maximum_copy_bytes: copy - 1, maximum_capacity_bytes: 0, ..grant }).unwrap());
                assert_eq!(denied.progress(), Default::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(owner.demands().unwrap().copy_bytes, copy);
            }
            let (step, heap) = observe(|| owner.step(grant).unwrap());
            assert!(step.progress().fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
            if copy > 0 { assert_eq!(step.progress().released_bytes, 0); }
            copied += step.progress().copied_bytes;
            if matches!(step, RetainedCloneStep::Complete(_)) { terminal = true; break; }
        }
        assert_eq!(observed, row["totalCopyBytes"].as_u64().unwrap() > 0);
        assert!(terminal && owner.terminal_is_empty());
        assert_eq!(copied, row["totalCopyBytes"].as_u64().unwrap() as usize);
        assert_eq!(owner.demands().unwrap().copy_bytes, 0);
        let (_, heap) = observe(|| drop(owner));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    }
    let law: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🧫️fixtures/📏️copy-demand/🔣️.json")).unwrap();
    for row in law["cases"].as_array().unwrap() {
        match row["kind"].as_str().unwrap() {
            "u32" => run(row["value"].as_u64().unwrap() as u32, row),
            "text" => { let mut text = String::with_capacity(128); text.push_str(row["value"].as_str().unwrap()); run(text, row); }
            "u32-list" => run(row["value"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u32).collect::<Vec<_>>(), row),
            _ => panic!("unknown typed field law"),
        }
    }
}

#[test]
fn paged_native_drawing_snapshot_asset_cursor_preserves_actual_ordinal_and_zero_heap() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    use semio_framework_value::paged::{PagedMap, PagedUtf8};
    let law: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🧰️owned/🧪️tests/🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    let long = format!("{}\0",law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize));
    let keys = law["assetKeyOrder"].as_array().unwrap().iter().map(|key| if key == "long" { long.clone() } else { key.as_str().unwrap().to_owned() }).collect::<Vec<_>>();
    let assets = PagedMap::<_, {usize::MAX}>::try_from_entries(keys.iter().map(|key| (PagedUtf8::try_from_str(key).unwrap(), DrawingImageAsset { width: 1, height: 1, samples: vec![[0,0,0,0];1].into() }))).unwrap();
    let foreign = PagedMap::<DrawingImageAsset, {usize::MAX}>::default();
    assert!(size_of::<DrawingAssetBoundsCursor>() <= size_of::<usize>() * 3);
    let (mut cursor, allocation) = observe(DrawingAssetBoundsCursor::new);
    assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
    for key in keys {
        let (row, allocation) = observe(|| cursor.next(&assets).unwrap());
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        assert!(row.unwrap().0.eq_str(&key));
        let (refused, allocation) = observe(|| cursor.next(&foreign));
        assert_eq!(refused.unwrap_err(), "drawing-store.preflight-asset-owner-changed");
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        let (_, allocation) = observe(|| cursor.advance().unwrap());
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
    }
    let (row, allocation) = observe(|| cursor.next(&assets).unwrap());
    assert!(row.is_none());
    assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
    let (_, allocation) = observe(|| drop(cursor));
    assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
}

#[test]
fn paged_native_drawing_snapshot_clone_preserves_the_neutral_schema() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    use semio_framework_value::{retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneSource, RetainedCloneStep}, retirement::controlled::ControlledRetirement};
    let law: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🧰️owned/🧪️tests/🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    let mut input: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let text = law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize);
    input["id"] = serde_json::Value::String(text.clone());
    input["title"] = serde_json::Value::String(text.clone());
    input["layers"][1]["children"][0]["id"] = serde_json::Value::String(format!("{text}\0"));
    input["layers"][1]["children"][0]["name"] = serde_json::Value::String(format!("{text}\0"));
    input["layers"][1]["children"][0]["content"] = serde_json::Value::String(format!("{text}\0"));
    input["assets"] = serde_json::json!({text.clone(): {"mime":"image/png","data":text}});
    let gradients: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🎨️fill/🔣️.json")).unwrap();
    let mut gradient = gradients[0]["after"].clone();
    let stops = gradient["stops"].as_array().unwrap();
    let repeated: Vec<serde_json::Value> = (0..law["repeat"].as_u64().unwrap() as usize).map(|index| stops[index % stops.len()].clone()).collect();
    gradient["stops"] = serde_json::Value::Array(repeated);
    input["layers"][0]["attributes"]["fill"] = gradient;
    let source = RetainedCloneSource::from_authority(std::sync::Arc::new(serde_json::from_value::<DrawingSnapshot>(input.clone()).unwrap()), ());
    let before = serde_json::to_value(source.borrow().get()).unwrap();
    let budget = law["bodyBytes"].as_u64().unwrap() as usize;
    let zero_bytes = law["constructorAllocationBytes"].as_u64().unwrap() as usize;
    assert!(std::mem::size_of::<<DrawingSnapshot as RetainedClone>::Cursor>() <= budget);
    for pause in law["cancelAt"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize).chain([usize::MAX]) {
        let (mut cursor, allocation) = observe(DrawingSnapshot::retained_clone_cursor);
        assert!(!allocation.overflowed);
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (zero_bytes, zero_bytes));
        let (zero, allocation) = observe(|| cursor.advance(source.borrow(), RetainedCloneGrant::default()).unwrap());
        assert_eq!(zero.progress(), Default::default());
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        let mut complete = false;
        for turn in 0..100000 {
            if turn == pause { break; }
            let grant = drawing_snapshot_heap_grant(turn, budget);
            let (result, allocation) = observe(|| cursor.advance(source.borrow(), grant));
            let step = result.unwrap();
            drawing_snapshot_heap_admission(step.progress(), grant, allocation);
            if matches!(step, RetainedCloneStep::Complete(_)) { complete = true; break; }
        }
        if pause == usize::MAX {
            assert!(complete);
            let output = cursor.take().unwrap();
            assert!(cursor.take().is_none());
            assert_eq!(serde_json::to_value(&output).unwrap(), before);
            let (retirement, allocation) = observe(|| ControlledRetirement::new(output));
            let mut owner = retirement.unwrap();
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            for turn in 0..100000 {
                let grant = drawing_snapshot_heap_grant(turn, budget);
                let (result, allocation) = observe(|| owner.step(grant));
                let step = result.unwrap();
                drawing_snapshot_heap_admission(step.progress(), grant, allocation);
                if matches!(step, RetainedCloneStep::Complete(_)) { break; }
            }
            assert!(owner.terminal_is_empty());
            let (_, allocation) = observe(|| drop(owner));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        }
        let (_, allocation) = observe(|| cursor.begin_close());
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        let (zero, allocation) = observe(|| cursor.close_granted(RetainedCloneGrant::default()).unwrap());
        assert_eq!(zero.progress(), Default::default());
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        for turn in 0..100000 {
            let grant = drawing_snapshot_heap_grant(turn, budget);
            let (result, allocation) = observe(|| cursor.close_granted(grant));
            let step = result.unwrap();
            drawing_snapshot_heap_admission(step.progress(), grant, allocation);
            if matches!(step, RetainedCloneStep::Complete(_)) { break; }
        }
        assert!(cursor.terminal_is_empty());
        let (_, allocation) = observe(|| drop(cursor));
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        assert_eq!(serde_json::to_value(source.borrow().get()).unwrap(), before);
    }
}

#[global_allocator]
static DRAWING_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

pub(super) fn drawing_snapshot_heap_grant(turn: usize, bytes: usize) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    assert_eq!(bytes, DRAWING_OWNED_FIELD_BYTES);
    initial_snapshot_clone_grant(turn)
}

pub(super) fn drawing_snapshot_heap_admission(progress: semio_framework_value::retained_clone::RetainedCloneProgress, grant: semio_framework_value::retained_clone::RetainedCloneGrant, allocation: semio_framework_trace::HeapAllocationObservation) {
    assert!(progress.fits(grant));
    assert!(!allocation.overflowed);
    assert!(allocation.requested_bytes <= progress.retained_capacity_bytes && allocation.released_bytes <= progress.released_bytes, "Drawing snapshot actual ownership exceeded admission: {allocation:?} {progress:?}");
    assert!(progress.copied_bytes.saturating_add(progress.retained_capacity_bytes).saturating_add(progress.released_bytes) <= 4096);
}
