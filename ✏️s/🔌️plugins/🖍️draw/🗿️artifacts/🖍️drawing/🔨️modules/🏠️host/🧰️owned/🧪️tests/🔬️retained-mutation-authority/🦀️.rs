use crate::mutations::SetLayerFillRule;
use super::*;
use crate::mutations::{
    CreateLayer, DeleteLayer, DuplicateLayer, RenameLayer, ReorderLayer, ReplaceLayerFill, ReplaceLayerStroke, SetLayerBlendMode, SetLayerBooleanOperation, SetLayerLocked, SetLayerOpacity, SetLayerVisible, UpdateLayerTraceParams,
    UpdateLayerTransform,
};

#[test]
fn paged_native_drawing_snapshot_initialization_catalog_admits_one_original_page_and_empty_drop() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🏗️initialization/📚️catalog/🧫️fixtures/🔣️.json")).unwrap();
    let lanes = law["lanes"].as_array().unwrap();
    let mut catalog = Some(store::ArtifactStoreInitializationOwnerCatalog::try_new().unwrap());
    let original = catalog.as_ref().unwrap().admitted_items();
    assert_eq!(original % lanes.len(), 0);
    let slots = original / lanes.len();
    let mut released = 0;
    for (index, lane) in lanes.iter().enumerate() {
        let (demand, heap) = observe(|| next_initialization_catalog_close_byte_demand(&catalog).unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(demand > 0);
        for (items, bytes) in [(0, demand), (1, 0), (1, demand - 1)] {
            let (step, heap) = observe(|| close_initialization_catalog(&mut catalog, items, bytes).unwrap());
            assert_eq!(step, store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(catalog.as_ref().unwrap().admitted_items(), (lanes.len() - index) * slots);
            assert_eq!(next_initialization_catalog_close_byte_demand(&catalog).unwrap(), demand);
        }
        let (step, heap) = observe(|| close_initialization_catalog(&mut catalog, 1, demand).unwrap());
        assert_eq!(step, store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: demand });
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, demand));
        assert_eq!(catalog.as_ref().unwrap().admitted_items(), (lanes.len() - index - 1) * slots);
        released += demand;
        eprintln!("[DEBUG] Drawing initializer original catalog lane={lane} exact admitted page release={demand}, zero/one-below0heap; catalog birth cold/uncredited");
    }
    assert!(catalog.as_ref().unwrap().terminal_is_empty());
    assert_eq!(next_initialization_catalog_close_byte_demand(&catalog).unwrap(), 0);
    let (step, heap) = observe(|| close_initialization_catalog(&mut catalog, 1, 0).unwrap());
    assert_eq!(step, store::SnapshotRetirementStep::Complete);
    assert!(catalog.is_none());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    let (_, heap) = observe(|| drop(catalog));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    eprintln!("[DEBUG] Drawing initializer catalog all six original pages exact physical total={released}, inline terminal handoff/drop0heap; remaining initializer frontiers separately required");
}

#[test]
fn paged_native_drawing_snapshot_retirement_keeps_original_variants_and_exact_grants() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    use semio_framework_value::{list::PagedList, paged::PagedUtf8};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../♻️retirement/🧫️fixtures/🔣️.json")).unwrap();
    let snapshot: DrawingSnapshot = serde_json::from_str(include_str!("../../../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let layer = snapshot.layers[0].clone();
    let owners = vec![
        DrawingRetirementOwner::Snapshot(snapshot),
        DrawingRetirementOwner::Asset(DrawingImageAsset{width:1,height:1,samples:vec![[13,27,89,255]].into()}),
        DrawingRetirementOwner::Mutation(DrawingMutation::RenameLayer(RenameLayer { layer_id: "native-owner".into(), new_name: "Grüße\0🧬".into() })),
        DrawingRetirementOwner::Layer(layer),
        DrawingRetirementOwner::Fill(FillStyle::LinearGradient { x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0, stops: vec![GradientStop { offset: 0.5, color: [0.1, 0.2, 0.3, 1.0] }].into() }),
        DrawingRetirementOwner::Stroke(StrokeStyle { color: [0.1, 0.2, 0.3, 1.0], width: 2.0, cap: crate::StrokeCap::Round, join: crate::StrokeJoin::Bevel, dash: Some(vec![1.0, 2.0].into()) }),
        DrawingRetirementOwner::String(PagedUtf8::try_from_str(&"Grüße\0🧬".repeat(1000)).unwrap()),
        DrawingRetirementOwner::Segments(PagedList::default()),
        DrawingRetirementOwner::SegmentCollections(PagedList::try_from_iter([PagedList::<PathSegment, {usize::MAX}>::default()]).unwrap()),
        DrawingRetirementOwner::HistoryId(String::from("native-history-owner")),
    ];
    assert_eq!(owners.len(), law["variants"].as_array().unwrap().len());
    let maximum = law["maximumBytes"].as_u64().unwrap() as usize;
    assert!(size_of::<DrawingOwnedRetirement>() <= maximum);
    for (value, kind) in owners.into_iter().zip(law["variants"].as_array().unwrap()) {
        let (mut owner, heap) = observe(|| DrawingOwnedRetirement::new(value));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let mut copied = 0;
        let mut released = 0;
        let mut terminal = false;
        for _ in 0..100000 {
            let (grant, heap) = observe(|| owner.next_grant().unwrap());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            let bytes = grant.maximum_copy_bytes + grant.maximum_capacity_bytes + grant.maximum_release_bytes;
            assert!(bytes <= maximum);
            let (demand, heap) = observe(|| store::ErasedSnapshotRetirement::next_close_byte_demand(&owner));
            assert_eq!(demand, bytes);
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            for (items, allowance) in [(0, maximum), (1, 0)] {
                let (step, heap) = observe(|| store::ErasedSnapshotRetirement::close_step(&mut owner, items, allowance).unwrap());
                assert!(matches!(step, store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            }
            if bytes != 0 {
                let (step, heap) = observe(|| store::ErasedSnapshotRetirement::close_step(&mut owner, 1, bytes - 1).unwrap());
                assert!(matches!(step, store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(owner.next_grant().unwrap(), grant);
            }
            let (step, heap) = observe(|| owner.close_granted(grant).unwrap());
            let progress = step.progress();
            assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
            assert!(progress.copied_items <= 1);
            assert!(progress.copied_bytes + progress.retained_capacity_bytes + progress.released_bytes <= bytes);
            if progress.copied_bytes != 0 { assert_eq!(heap.released_bytes, 0); }
            copied += progress.copied_bytes;
            released += progress.released_bytes;
            if matches!(step, RetainedCloneStep::Complete(_)) { terminal = true; break; }
        }
        assert!(terminal && store::ErasedSnapshotRetirement::terminal_is_empty(&owner));
        let (_, heap) = observe(|| drop(owner));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        eprintln!("[DEBUG] Drawing native retirement variant={kind} constructor/query/zero/below/terminal0heap, exactCopy={copied} exactPhysicalRelease={released}; every actual allocation/release separately admitted within{maximum}");
    }
}

#[test]
fn paged_native_drawing_snapshot_decoded_field_close_admits_payload_and_physical_work() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    fn run<T: semio_framework_value::retirement::RetireOwned>(value: T, row: &serde_json::Value) {
        let (mut owner, heap) = observe(|| DrawingDecodedFieldRetirement::try_new(value).unwrap_or_else(|_| panic!("native field has typed retirement")));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let mut copied = 0;
        let mut released = 0;
        let mut terminal = false;
        for _ in 0..100000 {
            let (demand, heap) = observe(|| owner.next_close_byte_demand().unwrap());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert!(demand <= 4096);
            for (items, bytes) in [(0, 4096), (1, 0)] {
                let (step, heap) = observe(|| owner.step(items, bytes).unwrap());
                assert_eq!(step.progress(), Default::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            }
            if demand != 0 {
                let (step, heap) = observe(|| owner.step(1, demand - 1).unwrap());
                assert_eq!(step.progress(), Default::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(owner.next_close_byte_demand().unwrap(), demand);
            }
            let (step, heap) = observe(|| owner.step(1, demand.max(1)).unwrap());
            let progress = step.progress();
            assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
            assert!(progress.copied_items <= 1);
            assert!(progress.copied_bytes + progress.retained_capacity_bytes + progress.released_bytes <= demand.max(1));
            if progress.copied_bytes != 0 { assert_eq!(heap.released_bytes, 0); }
            copied += progress.copied_bytes;
            released += progress.released_bytes;
            if matches!(step, RetainedCloneStep::Complete(_)) { terminal = true; break; }
        }
        assert!(terminal && owner.terminal_is_empty());
        assert_eq!(copied, row["totalCopyBytes"].as_u64().unwrap() as usize);
        assert_eq!(owner.next_close_byte_demand().unwrap(), 0);
        let (_, heap) = observe(|| drop(owner));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        eprintln!("[DEBUG] Drawing decoded field kind={} exactCopy={copied} physicalRelease={released}; constructor/demand/zero/below0heap, copy-free0 and terminal0heap", row["kind"]);
    }
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/♻️retirement/🧫️fixtures/📏️copy-demand/🔣️.json")).unwrap();
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
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
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
    eprintln!("[DEBUG] Drawing native asset ordinals preserve actual insertion order and empty/long UTF8/NUL keys; cursor birth/read/advance/drop and foreign-owner refusal allocate/release0");
}

#[test]
fn paged_native_drawing_snapshot_clone_preserves_the_neutral_schema() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    use semio_framework_value::{retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneSource, RetainedCloneStep}, retirement::controlled::ControlledRetirement};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    let mut input: serde_json::Value = serde_json::from_str(include_str!("../../../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let text = law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize);
    input["id"] = serde_json::Value::String(text.clone());
    input["title"] = serde_json::Value::String(text.clone());
    input["layers"][1]["children"][0]["id"] = serde_json::Value::String(format!("{text}\0"));
    input["layers"][1]["children"][0]["name"] = serde_json::Value::String(format!("{text}\0"));
    input["layers"][1]["children"][0]["content"] = serde_json::Value::String(format!("{text}\0"));
    input["assets"] = serde_json::json!({text.clone(): {"mime":"image/png","data":text}});
    let gradients: serde_json::Value = serde_json::from_str(include_str!("../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎨️fill/🧫️fixtures/🔣️.json")).unwrap();
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
    eprintln!("[DEBUG] Drawing paged snapshot Serde conservation, constructor/zero grants, partial cancellation and every actual clone/retirement birth+release admitted within4096");
}

#[global_allocator]
static DRAWING_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

fn drawing_snapshot_heap_grant(turn: usize, bytes: usize) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    assert_eq!(bytes, DRAWING_OWNED_FIELD_BYTES);
    initial_snapshot_clone_grant(turn)
}

fn drawing_snapshot_heap_admission(progress: semio_framework_value::retained_clone::RetainedCloneProgress, grant: semio_framework_value::retained_clone::RetainedCloneGrant, allocation: semio_framework_trace::HeapAllocationObservation) {
    assert!(progress.fits(grant));
    assert!(!allocation.overflowed);
    assert!(allocation.requested_bytes <= progress.retained_capacity_bytes && allocation.released_bytes <= progress.released_bytes, "Drawing snapshot actual ownership exceeded admission: {allocation:?} {progress:?}");
    assert!(progress.copied_bytes.saturating_add(progress.retained_capacity_bytes).saturating_add(progress.released_bytes) <= 4096);
}

#[test]
fn paged_native_drawing_snapshot_borrowed_lookup_measures_frames_and_alias_closure() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    use semio_framework_value::{retained_clone::{RetainedCloneSource, RetainedCloneStep}, paged::PagedUtf8};
    use crate::standards::v1::subsets::any::schema::snapshot::lookup::{DrawingLayerLookupCursor, DrawingLayerLookupStep};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    let mut input: serde_json::Value = serde_json::from_str(include_str!("../../../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let budget = law["bodyBytes"].as_u64().unwrap() as usize;
    let duplicate = { let mut value = input["layers"][1]["children"][0].clone(); value["name"] = "Later Duplicate".into(); value };
    input["layers"].as_array_mut().unwrap().push(duplicate);
    let long = format!("{}\0", law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize));
    let mut deep = input["layers"][0].clone();
    deep["id"] = long.clone().into();
    deep["name"] = "Long UTF8".into();
    for depth in 0..law["lookup"]["deep"].as_u64().unwrap() {
        let mut group = input["layers"][1].clone();
        group["id"] = format!("nested-{depth}").into();
        group["children"] = serde_json::json!([deep]);
        deep = group;
    }
    input["layers"].as_array_mut().unwrap().push(deep);
    let source = RetainedCloneSource::from_authority(std::sync::Arc::new(serde_json::from_value::<DrawingSnapshot>(input).unwrap()), ());
    let before = serde_json::to_value(source.borrow().get()).unwrap();
    let mut cases = law["lookup"]["cases"].as_array().unwrap().clone();
    let deep_path = std::iter::once(source.borrow().get().layers.len() - 1).chain(std::iter::repeat_n(0usize, law["lookup"]["deep"].as_u64().unwrap() as usize)).collect::<Vec<_>>();
    cases.push(serde_json::json!({"target":long,"name":"Long UTF8","path":deep_path}));
    cases.push(serde_json::json!({"target":"missing-after-deep-tree","name":null,"path":null}));
    for case in cases {
        let target = RetainedCloneSource::from_authority(std::sync::Arc::new(PagedUtf8::<{usize::MAX}>::try_from_str(case["target"].as_str().unwrap()).unwrap()), ());
        for pause in law["cancelAt"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize).chain([usize::MAX]) {
            let (mut cursor, allocation) = observe(|| DrawingLayerLookupCursor::new(source.borrow(), target.borrow()));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (zero, allocation) = observe(|| cursor.advance(Default::default()).unwrap());
            assert!(matches!(zero, DrawingLayerLookupStep::Pending(progress) if progress == Default::default()));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let mut complete = false;
            for turn in 0..100000 {
                if turn == pause { break; }
                let grant = drawing_snapshot_heap_grant(turn, budget);
                let (result, allocation) = observe(|| cursor.advance(grant));
                let step = result.unwrap();
                let (done, progress) = match step { DrawingLayerLookupStep::Pending(progress) => (false, progress), DrawingLayerLookupStep::Complete { progress, .. } => (true, progress) };
                drawing_snapshot_heap_admission(progress, grant, allocation);
                if done { complete = true; break; }
            }
            if pause == usize::MAX {
                assert!(complete);
                let (length, allocation) = observe(|| cursor.path_len());
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                let expected_path = case["path"].as_array();
                assert_eq!(length, expected_path.map(Vec::len));
                if let Some(path) = expected_path {
                    for (index, ordinal) in path.iter().enumerate() {
                        let (actual, allocation) = observe(|| cursor.path_index(index));
                        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                        assert_eq!(actual, Some(ordinal.as_u64().unwrap() as usize));
                    }
                    assert!(cursor.path_index(path.len()).is_none());
                }
                let output = cursor.take().unwrap();
                assert!(cursor.take().is_none());
                assert!(cursor.path_len().is_none());
                assert!(cursor.path_index(0).is_none());
                assert_eq!(output.map(|node| crate::schema::layer_base(node.get()).name.to_string_owner()), case["name"].as_str().map(str::to_owned));
            }
            let (_, allocation) = observe(|| cursor.begin_close());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (zero, allocation) = observe(|| cursor.close_granted(Default::default()).unwrap());
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
        }
    }
    assert_eq!(serde_json::to_value(source.borrow().get()).unwrap(), before);
    eprintln!("[DEBUG] Drawing borrowed depth90 tree lookup preserves first duplicate/long UTF8/NUL, zero-allocation ordinal paths, and admits every real frame birth, body and cancellation release within4096");
}

#[test]
fn paged_native_drawing_snapshot_rename_preparation_preserves_borrowed_inverse_before_disposition() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    use semio_framework_value::retained_clone::{RetainedCloneSource, RetainedCloneStep};
    use crate::standards::v1::subsets::metadata::schema::mutations::rename_layer::prepare::{DrawingRenamePreparationCursor, DrawingRenamePreparationStep};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    let mut input: serde_json::Value = serde_json::from_str(include_str!("../../../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let long = format!("{}\0", law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize));
    input["layers"][1]["children"][0]["name"] = long.clone().into();
    let mut long_identity = input["layers"][0].clone();
    long_identity["id"] = long.clone().into();
    long_identity["name"] = "Long Identity".into();
    input["layers"].as_array_mut().unwrap().push(long_identity);
    let mut duplicate = input["layers"][0].clone();
    duplicate["name"] = "Later Duplicate".into();
    input["layers"].as_array_mut().unwrap().push(duplicate);
    let mut deep = input["layers"][0].clone();
    deep["id"] = "deep-rename".into();
    deep["name"] = "Deep Original".into();
    for depth in 0..law["lookup"]["deep"].as_u64().unwrap() {
        let mut parent = input["layers"][1].clone();
        parent["id"] = format!("rename-nested-{depth}").into();
        parent["children"] = serde_json::Value::Array(vec![deep]);
        deep = parent;
    }
    input["layers"].as_array_mut().unwrap().push(deep);
    let source = RetainedCloneSource::from_authority(std::sync::Arc::new(serde_json::from_value::<DrawingSnapshot>(input).unwrap()), ());
    let before = serde_json::to_value(source.borrow().get()).unwrap();
    let budget = law["bodyBytes"].as_u64().unwrap() as usize;
    assert!(std::mem::size_of::<DrawingRenamePreparationCursor>() <= budget);
    for case in law["rename"]["cases"].as_array().unwrap() {
        let name = if case["repeatNewName"].as_bool().unwrap_or(false) { format!("{}{}{}", case["newName"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize), if case["disposition"] == "no-op" || case["suffix"].is_string() { "\0" } else { "" }, case["suffix"].as_str().unwrap_or("")) } else { case["newName"].as_str().unwrap().to_owned() };
        let target = if case["repeatTarget"].as_bool().unwrap_or(false) { long.as_str() } else { case["target"].as_str().unwrap() };
        let mutation = RetainedCloneSource::from_authority(std::sync::Arc::new(crate::mutations::RenameLayer { layer_id: target.into(), new_name: name.into() }), ());
        let mutation_before = serde_json::to_value(mutation.borrow().get()).unwrap();
        for pause in [0usize, 1, 3, 7, 17, 64, 256, 1024, usize::MAX] {
            let (mut cursor, allocation) = observe(|| DrawingRenamePreparationCursor::new(source.borrow(), mutation.borrow()));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (zero, allocation) = observe(|| cursor.advance(Default::default()).unwrap());
            assert!(matches!(zero, DrawingRenamePreparationStep::Pending(progress) if progress == Default::default()));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let mut complete = false;
            for turn in 0..100000 {
                if turn == pause { break; }
                let grant = drawing_snapshot_heap_grant(turn, budget);
                let (result, allocation) = observe(|| cursor.advance(grant));
                let step = result.unwrap();
                let (done, progress) = match step { DrawingRenamePreparationStep::Pending(progress) => (false, progress), DrawingRenamePreparationStep::Complete { progress, .. } => (true, progress) };
                drawing_snapshot_heap_admission(progress, grant, allocation);
                if done { complete = true; break; }
            }
            if pause == usize::MAX {
                assert!(complete);
                let plan = cursor.take().unwrap();
                assert!(cursor.take().is_none());
                assert_eq!(plan.disposition.as_str(), case["disposition"].as_str().unwrap());
                let expected = case["inverseName"].as_str().map(|name| if name == "long" { long.as_str() } else { name });
                assert_eq!(plan.inverse_name().map(|name| name.get().to_string_owner()), expected.map(str::to_owned));
                assert_eq!(plan.payload.get(), mutation.borrow().get());
                let (_, allocation) = observe(|| drop(plan));
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            }
            let (_, allocation) = observe(|| cursor.begin_close());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (zero, allocation) = observe(|| cursor.close_granted(Default::default()).unwrap());
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
            assert_eq!(serde_json::to_value(mutation.borrow().get()).unwrap(), mutation_before);
        }
    }
    eprintln!("[DEBUG] Drawing borrowed rename preserves first identity/old inverse, empty and long UTF8/NUL names; every real lookup/comparison/cancellation birth+release <=4096 and constructor0");
}

#[test]
fn paged_native_drawing_snapshot_owned_rename_preparation_preserves_original_binding_and_root_projection() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    use semio_framework_value::retained_clone::{RetainedCloneSource, RetainedCloneStep};
    use crate::standards::v1::subsets::metadata::schema::mutations::rename_layer::prepare::owned::{DrawingOwnedRenamePreparationCursor, DrawingOwnedRenamePreparationStep};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    let mut input: serde_json::Value = serde_json::from_str(include_str!("../../../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let long = format!("{}\0", law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize));
    input["layers"][1]["children"][0]["name"] = long.clone().into();
    let mut long_identity = input["layers"][0].clone();
    long_identity["id"] = long.clone().into();
    long_identity["name"] = "Long Identity".into();
    input["layers"].as_array_mut().unwrap().push(long_identity);
    let mut duplicate = input["layers"][0].clone();
    duplicate["name"] = "Later Duplicate".into();
    input["layers"].as_array_mut().unwrap().push(duplicate);
    let mut deep = input["layers"][0].clone();
    deep["id"] = "deep-rename".into();
    deep["name"] = "Deep Original".into();
    for depth in 0..law["lookup"]["deep"].as_u64().unwrap() {
        let mut parent = input["layers"][1].clone();
        parent["id"] = format!("rename-nested-{depth}").into();
        parent["children"] = serde_json::Value::Array(vec![deep]);
        deep = parent;
    }
    input["layers"].as_array_mut().unwrap().push(deep);
    let source = RetainedCloneSource::from_authority(std::sync::Arc::new(serde_json::from_value::<DrawingSnapshot>(input).unwrap()), ());
    let before = serde_json::to_value(source.borrow().get()).unwrap();
    let budget = law["bodyBytes"].as_u64().unwrap() as usize;
    fn assert_static<T: Send + Sync + 'static>() {}
    assert_static::<DrawingOwnedRenamePreparationCursor>();
    assert!(std::mem::size_of::<DrawingOwnedRenamePreparationCursor>() <= budget);
    for case in law["rename"]["cases"].as_array().unwrap() {
        let name = if case["repeatNewName"].as_bool().unwrap_or(false) { format!("{}{}{}", case["newName"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize), if case["disposition"] == "no-op" || case["suffix"].is_string() { "\0" } else { "" }, case["suffix"].as_str().unwrap_or("")) } else { case["newName"].as_str().unwrap().to_owned() };
        let target = if case["repeatTarget"].as_bool().unwrap_or(false) { long.as_str() } else { case["target"].as_str().unwrap() };
        let mutation = RetainedCloneSource::from_authority(std::sync::Arc::new(crate::mutations::RenameLayer { layer_id: target.into(), new_name: name.into() }), ());
        let mutation_before = serde_json::to_value(mutation.borrow().get()).unwrap();
        let foreign = RetainedCloneSource::from_authority(std::sync::Arc::new(mutation.borrow().get().clone()), ());
        for pause in [0usize, 1, 3, 7, 17, 64, 256, 1024, usize::MAX] {
            let (mut cursor, allocation) = observe(|| DrawingOwnedRenamePreparationCursor::new(source.project_owned(0, |snapshot| snapshot)).unwrap());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (zero, allocation) = observe(|| cursor.advance(mutation.borrow(), Default::default()).unwrap());
            assert!(matches!(zero, DrawingOwnedRenamePreparationStep::Pending(progress) if progress == Default::default()));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let mut complete = false;
            for turn in 0..100000 {
                if turn == pause { break; }
                let grant = drawing_snapshot_heap_grant(turn, budget);
                let (result, allocation) = observe(|| cursor.advance(mutation.borrow(), grant));
                let step = result.unwrap();
                let (done, progress) = match step { DrawingOwnedRenamePreparationStep::Pending(progress) => (false, progress), DrawingOwnedRenamePreparationStep::Complete { progress, .. } => (true, progress) };
                drawing_snapshot_heap_admission(progress, grant, allocation);
                if turn == 0 {
                    let (zero, allocation) = observe(|| cursor.advance(foreign.borrow(), Default::default()).unwrap());
                    assert!(matches!(zero, DrawingOwnedRenamePreparationStep::Pending(progress) if progress == Default::default()));
                    assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                    assert!(cursor.advance(foreign.borrow(), grant).is_err());
                }
                if done { complete = true; break; }
            }
            if pause == usize::MAX {
                assert!(complete);
                let mut plan = cursor.take().unwrap();
                assert!(cursor.take().is_none());
                let (checked, allocation) = observe(|| plan.check_original(mutation.borrow()));
                checked.unwrap();
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                assert!(plan.check_original(foreign.borrow()).is_err());
                assert_eq!(plan.disposition.as_str(), case["disposition"].as_str().unwrap());
                let expected = case["inverseName"].as_str().map(|name| if name == "long" { long.as_str() } else { name });
                assert_eq!(plan.inverse_name().unwrap().map(|name| name.get().to_string_owner()), expected.map(str::to_owned));
                assert_eq!(plan.path_len(), case["path"].as_array().map(Vec::len));
                if let Some(expected) = case["path"].as_array() {
                    for (index, ordinal) in expected.iter().enumerate() {
                        let (actual, allocation) = observe(|| plan.path_index(index));
                        assert_eq!(actual, Some(ordinal.as_u64().unwrap() as usize));
                        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                    }
                }
                let (_, allocation) = observe(|| plan.begin_close());
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                let (zero, allocation) = observe(|| plan.close_granted(Default::default()).unwrap());
                assert_eq!(zero.progress(), Default::default());
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                assert!(plan.path_len().is_none());
                for turn in 0..100000 {
                    let grant = drawing_snapshot_heap_grant(turn, budget);
                    let (result, allocation) = observe(|| plan.close_granted(grant));
                    let step = result.unwrap();
                    drawing_snapshot_heap_admission(step.progress(), grant, allocation);
                    if matches!(step, RetainedCloneStep::Complete(_)) { break; }
                }
                assert!(plan.terminal_is_empty());
                let (_, allocation) = observe(|| drop(plan));
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            }
            let (_, allocation) = observe(|| cursor.begin_close());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (zero, allocation) = observe(|| cursor.close_granted(Default::default()).unwrap());
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
            assert_eq!(serde_json::to_value(mutation.borrow().get()).unwrap(), mutation_before);
        }
    }
    eprintln!("[DEBUG] Drawing static owned rename preserves first identity/old inverse, empty and long UTF8/NUL names; every real lookup/comparison/cancellation birth+release <=4096 and constructor0");
}

#[test]
fn paged_native_drawing_snapshot_rename_inverse_measures_every_owned_page_and_retirement() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    use semio_framework_value::{retained_clone::{RetainedCloneSource, RetainedCloneStep}, retirement::controlled::ControlledRetirement};
    use crate::standards::v1::subsets::metadata::schema::mutations::rename_layer::prepare::inverse::DrawingRenameInverseCursor;
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    let mut input: serde_json::Value = serde_json::from_str(include_str!("../../../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let long = format!("{}\0", law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize));
    input["layers"][1]["children"][0]["name"] = long.clone().into();
    let mut long_identity = input["layers"][0].clone();
    long_identity["id"] = long.clone().into();
    long_identity["name"] = "Long Identity".into();
    input["layers"].as_array_mut().unwrap().push(long_identity);
    let mut duplicate = input["layers"][0].clone();
    duplicate["name"] = "Later Duplicate".into();
    input["layers"].as_array_mut().unwrap().push(duplicate);
    let mut deep = input["layers"][0].clone();
    deep["id"] = "deep-rename".into();
    deep["name"] = "Deep Original".into();
    for depth in 0..law["lookup"]["deep"].as_u64().unwrap() {
        let mut parent = input["layers"][1].clone();
        parent["id"] = format!("rename-nested-{depth}").into();
        parent["children"] = serde_json::Value::Array(vec![deep]);
        deep = parent;
    }
    input["layers"].as_array_mut().unwrap().push(deep);
    let source = RetainedCloneSource::from_authority(std::sync::Arc::new(serde_json::from_value::<DrawingSnapshot>(input).unwrap()), ());
    let before = serde_json::to_value(source.borrow().get()).unwrap();
    let budget = law["bodyBytes"].as_u64().unwrap() as usize;
    fn assert_static<T: Send + 'static>() {}
    assert_static::<DrawingRenameInverseCursor>();
    assert!(std::mem::size_of::<DrawingRenameInverseCursor>() <= budget);
    for case in law["rename"]["cases"].as_array().unwrap() {
        let name = if case["repeatNewName"].as_bool().unwrap_or(false) { format!("{}{}{}", case["newName"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize), if case["disposition"] == "no-op" || case["suffix"].is_string() { "\0" } else { "" }, case["suffix"].as_str().unwrap_or("")) } else { case["newName"].as_str().unwrap().to_owned() };
        let target = if case["repeatTarget"].as_bool().unwrap_or(false) { long.as_str() } else { case["target"].as_str().unwrap() };
        let mutation = RetainedCloneSource::from_authority(std::sync::Arc::new(crate::mutations::RenameLayer { layer_id: target.into(), new_name: name.into() }), ());
        let mutation_before = serde_json::to_value(mutation.borrow().get()).unwrap();
        for pause in [0usize, 1, 3, 7, 17, 64, 256, 1024, 4096, usize::MAX] {
            let (mut cursor, allocation) = observe(|| DrawingRenameInverseCursor::new(source.project_owned(0, |snapshot| snapshot)).unwrap());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (zero, allocation) = observe(|| cursor.advance(mutation.borrow(), Default::default()).unwrap());
            assert_eq!(zero.progress(), Default::default());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let mut complete = false;
            for turn in 0..100000 {
                if turn == pause { break; }
                let grant = drawing_snapshot_heap_grant(turn, budget);
                let (result, allocation) = observe(|| cursor.advance(mutation.borrow(), grant));
                let step = result.unwrap();
                drawing_snapshot_heap_admission(step.progress(), grant, allocation);
                if matches!(step, RetainedCloneStep::Complete(_)) { complete = true; break; }
            }
            if pause == usize::MAX {
                assert!(complete);
                let output = cursor.take().unwrap();
                assert!(cursor.take().is_none());
                let expected = case["inverseName"].as_str().map(|name| if name == "long" { long.as_str() } else { name });
                assert_eq!(output.len(), usize::from(expected.is_some()));
                if let Some(name) = expected {
                    let DrawingMutation::RenameLayer(payload) = output.get(0).unwrap() else { panic!("wrong native inverse variant") };
                    assert!(payload.layer_id.eq_str(target));
                    assert!(payload.new_name.eq_str(name));
                }
                let (result, allocation) = observe(|| ControlledRetirement::new(output));
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                let mut owner = result.unwrap();
                let (zero, allocation) = observe(|| owner.step(Default::default()).unwrap());
                assert_eq!(zero.progress(), Default::default());
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
            let (zero, allocation) = observe(|| cursor.close_granted(Default::default()).unwrap());
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
            assert_eq!(serde_json::to_value(mutation.borrow().get()).unwrap(), mutation_before);
        }
    }
    eprintln!("[DEBUG] Drawing native rename inverse preserves missing/no-op/first-target old name; each real ID/name clone, literal/page birth, partial cancellation and returned owner release admitted within4096, constructor0");
}

#[test]
fn paged_native_drawing_snapshot_owned_lookup_measures_root_alias_and_native_pages() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    use semio_framework_value::{paged::PagedUtf8, retained_clone::{RetainedCloneSource, RetainedCloneStep}};
    use crate::standards::v1::subsets::any::schema::snapshot::lookup::owned::{DrawingOwnedLayerLookupCursor, DrawingOwnedLayerLookupStep};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    let mut input: serde_json::Value = serde_json::from_str(include_str!("../../../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let mut duplicate = input["layers"][1]["children"][0].clone();
    duplicate["name"] = "Later Duplicate".into();
    input["layers"].as_array_mut().unwrap().push(duplicate);
    let long = format!("{}\0", law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize));
    let mut deep = input["layers"][0].clone();
    deep["id"] = long.clone().into();
    deep["name"] = "Long UTF8".into();
    for depth in 0..law["lookup"]["deep"].as_u64().unwrap() {
        let mut group = input["layers"][1].clone();
        group["id"] = format!("nested-{depth}").into();
        group["children"] = serde_json::json!([deep]);
        deep = group;
    }
    input["layers"].as_array_mut().unwrap().push(deep);
    let source = RetainedCloneSource::from_authority(std::sync::Arc::new(serde_json::from_value::<DrawingSnapshot>(input).unwrap()), ());
    let before = serde_json::to_value(source.borrow().get()).unwrap();
    let budget = law["bodyBytes"].as_u64().unwrap() as usize;
    let deep_path = std::iter::once(source.borrow().get().layers.len() - 1).chain(std::iter::repeat_n(0usize, law["lookup"]["deep"].as_u64().unwrap() as usize)).collect::<Vec<_>>();
    let mut cases = law["lookup"]["cases"].as_array().unwrap().clone();
    cases.push(serde_json::json!({"target":long,"name":"Long UTF8","path":deep_path}));
    cases.push(serde_json::json!({"target":"missing-after-deep-tree","name":null,"path":null}));
    fn static_owner<T: Send + Sync + 'static>() {}
    static_owner::<DrawingOwnedLayerLookupCursor>();
    assert!(std::mem::size_of::<DrawingOwnedLayerLookupCursor>() <= budget);
    for case in cases {
        let target = RetainedCloneSource::from_authority(std::sync::Arc::new(PagedUtf8::<{usize::MAX}>::try_from_str(case["target"].as_str().unwrap()).unwrap()), ());
        for pause in law["cancelAt"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize).chain([usize::MAX]) {
            let (created, allocation) = observe(|| DrawingOwnedLayerLookupCursor::new(source.project_owned(0, |snapshot| snapshot)));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let mut cursor = created.unwrap();
            let (zero, allocation) = observe(|| cursor.advance(target.borrow(), Default::default()).unwrap());
            assert!(matches!(zero, DrawingOwnedLayerLookupStep::Pending(progress) if progress == Default::default()));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let mut complete = false;
            for turn in 0..100000 {
                if turn == pause { break; }
                let grant = drawing_snapshot_heap_grant(turn, budget);
                let (result, allocation) = observe(|| cursor.advance(target.borrow(), grant));
                let (done, progress) = match result.unwrap() { DrawingOwnedLayerLookupStep::Pending(progress) => (false, progress), DrawingOwnedLayerLookupStep::Complete { progress, .. } => (true, progress) };
                drawing_snapshot_heap_admission(progress, grant, allocation);
                if done { complete = true; break; }
            }
            if pause == usize::MAX {
                assert!(complete);
                assert_eq!(cursor.path_len(), case["path"].as_array().map(Vec::len));
                if let Some(path) = case["path"].as_array() { for (index, ordinal) in path.iter().enumerate() { assert_eq!(cursor.path_index(index), Some(ordinal.as_u64().unwrap() as usize)); } }
                let (output, allocation) = observe(|| cursor.take().unwrap());
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                assert!(cursor.take().is_none() && cursor.path_len().is_none());
                assert_eq!(output.as_ref().map(|node| crate::schema::layer_base(node.borrow().unwrap().get()).name.to_string_owner()), case["name"].as_str().map(str::to_owned));
                if let Some(mut node) = output {
                    let (step, allocation) = observe(|| node.close_step(1).unwrap());
                    assert_eq!(step, store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
                    assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                    assert!(node.terminal_is_empty());
                    let (_, allocation) = observe(|| drop(node));
                    assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                }
            }
            let (_, allocation) = observe(|| cursor.begin_close());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (zero, allocation) = observe(|| cursor.close_granted(Default::default()).unwrap());
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
    eprintln!("[DEBUG] owned Drawing lookup holds actual immutable root projections, reborrows target each turn, preserves first duplicate/depth90/UTF8/NUL and admits every native page birth/release within4096; constructor/alias close0heap");
}

fn admit_string_destination(value: &mut DrawingNativeText) {
    let length = value.len();
    let mut chunks = std::mem::take(value).into_retained_chunks();
    if chunks.is_empty() { chunks.push(String::with_capacity(DRAWING_OWNED_FIELD_BYTES)); }
    let first = chunks.get_mut(0).unwrap();
    if first.capacity() < DRAWING_OWNED_FIELD_BYTES { first.try_reserve_exact(DRAWING_OWNED_FIELD_BYTES.saturating_sub(first.len())).expect("native Drawing fixture chunk backing admits its exact destination extent"); }
    assert!(first.capacity() >= DRAWING_OWNED_FIELD_BYTES);
    *value = DrawingNativeText::from_retained_chunks(chunks, length).unwrap();
}

fn reserve_native_layer_slots(value: &mut DrawingNativeLayers, minimum: usize) {
    while value.capacity() < minimum { value.reserve_one(usize::MAX).unwrap_or_else(|_| panic!("cold native Drawing fixture page allocation admits")); }
}

fn native_layer_backing(value: &DrawingNativeLayers) -> usize { value.first().map_or(0, |layer| layer as *const _ as usize) }

fn native_text_backing(value: &semio_framework_value::paged::PagedUtf8<{usize::MAX}>) -> usize { value.retained_chunks().first().map_or(0, |chunk| chunk.as_ptr() as usize) }

fn admit_layer_string_destinations(layer: &mut DrawingLayerNode) {
    let base = crate::schema::layer_base_mut(layer);
    admit_string_destination(&mut base.id);
    admit_string_destination(&mut base.name);
    admit_string_destination(&mut base.blend_mode);
    match layer {
        DrawingLayerNode::Group(group) => {
            for child in &mut group.children {
                admit_layer_string_destinations(child);
            }
        }
        DrawingLayerNode::Boolean(boolean) => admit_string_destination(&mut boolean.operation),
        _ => {}
    }
}

fn initialize_drawing_mutation_arena_pool_for_test() {
    let operation = semio_framework_job::OperationId(7_900);
    let generation = semio_framework_job::Generation(79);
    let mut job = DrawingMutationArenaBootstrapJob::new(operation, generation).expect("fixed Drawing arena bootstrap job admission");
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    for _ in 0..1_000 {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        match job.step(&mut context) {
            DrawingMutationArenaBootstrapStep::Ready => return,
            DrawingMutationArenaBootstrapStep::Pending { advanced_items } => assert_eq!(advanced_items, 1),
            DrawingMutationArenaBootstrapStep::Blocked => {}
            DrawingMutationArenaBootstrapStep::Cancelled => panic!("Drawing mutation arena bootstrap fixture was unexpectedly cancelled"),
            DrawingMutationArenaBootstrapStep::Fault(error) => panic!("Drawing mutation arena pool initialization faulted: {error}"),
        }
    }
    panic!("Drawing mutation arena pool initialization did not terminate")
}

fn nested_snapshot() -> DrawingSnapshot {
    initialize_drawing_mutation_arena_pool_for_test();
    let mut snapshot = crate::standards::v1::subsets::any::schema::default_drawing_document("drawing-retained-mutation", None);
    let shape = crate::schema::create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Shape")).to_string().into()).expect("nonempty authored identity"), "Shape");
    let boolean = crate::schema::create_drawing_boolean_layer(crate::schema::identity::DrawingIdentity::admit((("Boolean")).to_string().into()).expect("nonempty authored identity"), "Boolean", "union", vec![crate::schema::layer_id(&shape).into()]);
    let trace = crate::schema::create_drawing_trace_layer(crate::schema::identity::DrawingIdentity::admit((("Trace")).to_string().into()).expect("nonempty authored identity"), "Trace", "asset-a");
    let mut group = crate::schema::create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("Group")).to_string().into()).expect("nonempty authored identity"), "Group");
    if let DrawingLayerNode::Group(value) = &mut group {
        value.children.push(shape);
        value.children.push(boolean);
        value.children.push(trace);
    }
    snapshot.layers.push(group);
    snapshot.layers.iter_mut().for_each(admit_layer_string_destinations);
    snapshot.assets.insert("asset-a".into(), DrawingImageAsset { width: 1, height: 1, samples: vec![[0,0,0,0];1].into() });
    snapshot
}

fn document_over_byte_bound_snapshot() -> DrawingSnapshot {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    use semio_framework_value::{list::PagedList, paged::PagedUtf8};
    let (mut snapshot, allocation) = observe(|| crate::standards::v1::subsets::any::schema::default_drawing_document("drawing-document-plus-one", None));
    assert!(!allocation.overflowed);
    let base = allocation.requested_bytes.checked_sub(allocation.released_bytes).unwrap();
    let (empty_title, allocation) = observe(|| PagedUtf8::<{usize::MAX}>::from_retained_chunks(PagedList::try_from_iter((0..64).map(|_| String::new())).unwrap(), 0).unwrap());
    assert!(!allocation.overflowed);
    let scaffold = allocation.requested_bytes.checked_sub(allocation.released_bytes).unwrap();
    drop(empty_title);
    let mut remaining = DRAWING_MAXIMUM_NESTED_BYTES.checked_add(1).unwrap().checked_sub(size_of::<DrawingSnapshot>() + base + scaffold).unwrap();
    let spare = remaining;
    let capacities: [usize; 64] = std::array::from_fn(|_| {
        let capacity = remaining.min(DRAWING_OWNED_FIELD_BYTES);
        remaining -= capacity;
        capacity
    });
    assert_eq!(remaining, 0);
    let (title, allocation) = observe(|| PagedUtf8::<{usize::MAX}>::from_retained_chunks(PagedList::try_from_iter(capacities.into_iter().map(String::with_capacity)).unwrap(), 0).unwrap());
    assert!(!allocation.overflowed);
    let retained = allocation.requested_bytes.checked_sub(allocation.released_bytes).unwrap();
    assert_eq!(retained, scaffold + spare);
    assert_eq!(size_of::<DrawingSnapshot>() + base + retained, DRAWING_MAXIMUM_NESTED_BYTES + 1, "independent allocator witness materializes the exact D + 1 retained-source boundary");
    snapshot.title = Some(title);
    snapshot
}

fn drain_snapshot(value: DrawingSnapshot) {
    let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&DrawingSnapshotRetirementFactory, value);
    for _ in 0..100_000 {
        match retirement.close_step(1, DRAWING_OWNED_FIELD_BYTES).expect("Drawing snapshot retirement") {
            store::SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                drop(retirement);
                return;
            }
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= DRAWING_OWNED_FIELD_BYTES);
            }
            store::SnapshotRetirementStep::Blocked => panic!("owned Drawing snapshot retirement cannot block"),
        }
    }
    panic!("Drawing snapshot retirement did not terminate")
}

fn drain_mutation(value: DrawingMutation) {
    let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&DrawingMutationRetirementFactory, value);
    for _ in 0..100_000 {
        match retirement.close_step(1, DRAWING_OWNED_FIELD_BYTES).expect("Drawing mutation retirement") {
            store::SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                drop(retirement);
                return;
            }
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= DRAWING_OWNED_FIELD_BYTES);
            }
            store::SnapshotRetirementStep::Blocked => panic!("owned Drawing mutation retirement cannot block"),
        }
    }
    panic!("Drawing mutation retirement did not terminate")
}

/// 🚪️ Drives one candidate close to terminal. The arena pool is PROCESS-global and returned under
/// `try_lock`, so a concurrent law holding it makes one slice answer `Blocked` — contention, not a
/// stuck ladder; the slice is retried inside the same bound, which still fails a ladder that never ends.
fn close_candidate(authority: &mut DrawingMutationCandidateAuthority, mut source: Option<&mut DrawingSnapshot>) {
    for _ in 0..100_000 {
        match authority.close_step(source.as_deref_mut(), DRAWING_OWNED_FIELD_BYTES).expect("Drawing candidate close") {
            store::SnapshotRetirementStep::Complete => {
                assert!(authority.terminal_is_empty());
                return;
            }
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= DRAWING_OWNED_FIELD_BYTES);
            }
            store::SnapshotRetirementStep::Blocked => std::thread::yield_now(),
        }
    }
    panic!("Drawing candidate close did not terminate")
}

/// 🧮️ Borrows a candidate from the shared process pool the way the production initializer does: a
/// transiently contended pool (another law holds it for one bounded borrow) is borrowed again.
fn borrowed_candidate(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<DrawingMutationCandidateAuthority, &'static str> {
    for _ in 0..1_000_000 {
        match DrawingMutationCandidateAuthority::try_new(operation, generation) {
            Err(DrawingMutationArenaBorrowError::Contended) => std::thread::yield_now(),
            other => return other.map_err(DrawingMutationArenaBorrowError::as_str),
        }
    }
    Err("drawing-store.mutation-arena-pool-contended")
}

fn apply(mut source: DrawingSnapshot, mutation: &DrawingMutation) -> Result<DrawingSnapshot, (DrawingSnapshot, &'static str)> {
    initialize_drawing_mutation_arena_pool_for_test();
    let operation = semio_framework_job::OperationId(8_001);
    let generation = semio_framework_job::Generation(81);
    let mut authority = borrowed_candidate(operation, generation).expect("Drawing candidate fixed owner arenas admit");
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    for _ in 0..200_000 {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        match authority.step(&mut source, mutation, &mut context) {
            Ok(true) => {
                authority.take().expect("Drawing mutation overlay exact terminal commit witness");
                assert!(authority.terminal_is_empty());
                drop(authority);
                return Ok(source);
            }
            Ok(false) => {}
            Err(error) => {
                close_candidate(&mut authority, Some(&mut source));
                drop(authority);
                return Err((source, error));
            }
        }
    }
    close_candidate(&mut authority, Some(&mut source));
    drop(authority);
    drain_snapshot(source);
    panic!("Drawing mutation candidate did not terminate")
}

fn live_workset(source: &mut DrawingSnapshot, mutation: &DrawingMutation) -> Result<DrawingMutationWorksetPlan, &'static str> {
    initialize_drawing_mutation_arena_pool_for_test();
    let operation = semio_framework_job::OperationId(8_004);
    let generation = semio_framework_job::Generation(84);
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let mut authority = borrowed_candidate(operation, generation)?;
    for _ in 0..100_000 {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        authority.step(source, mutation, &mut context)?;
        if let Some(workset) = authority.workset {
            close_candidate(&mut authority, Some(source));
            drop(authority);
            return Ok(workset);
        }
    }
    close_candidate(&mut authority, Some(source));
    drop(authority);
    Err("drawing-store.test-mutation-preflight-incomplete")
}

fn planned_clone_workset(source: &mut DrawingSnapshot, mutation: &DrawingMutation) -> Result<(DrawingMutationWorksetPlan, DrawingCloneWorkTotals), &'static str> {
    initialize_drawing_mutation_arena_pool_for_test();
    let operation = semio_framework_job::OperationId(8_006);
    let generation = semio_framework_job::Generation(86);
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let mut authority = borrowed_candidate(operation, generation)?;
    for _ in 0..100_000 {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        authority.step(source, mutation, &mut context)?;
        if authority.phase == DrawingMutationCandidatePhase::BindOverlay {
            let workset = authority.workset.ok_or("drawing-store.test-clone-workset-missing")?;
            let totals = authority.clone_work.as_ref().and_then(DrawingLayerCloneWorkAuthority::totals).ok_or("drawing-store.test-clone-work-false-terminal")?;
            if authority.overlay.is_some() {
                return Err("drawing-store.test-clone-bound-overlay-too-early");
            }
            close_candidate(&mut authority, Some(source));
            drop(authority);
            return Ok((workset, totals));
        }
    }
    close_candidate(&mut authority, Some(source));
    drop(authority);
    Err("drawing-store.test-clone-work-preflight-incomplete")
}

fn digest(mutation: &DrawingMutation) -> Result<[u8; 32], &'static str> {
    let mut authority = DrawingMutationDigestAuthority::new();
    let mut output = store::ArtifactStoreInitializationDigest::new(b"drawing.test.mutation");
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    for _ in 0..100_000 {
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(8_002),
            semio_framework_job::Generation(82),
            semio_framework_job::StepBudget::new(1, u64::MAX),
            cancel.clone(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
        );
        match authority.step(mutation, &mut output, &mut context) {
            Ok(true) => {
                assert!(authority.terminal_is_empty());
                drop(authority);
                return Ok(output.finish());
            }
            Ok(false) => {}
            Err(error) => {
                while !authority.terminal_is_empty() {
                    let _ = authority.close_step(DRAWING_OWNED_FIELD_BYTES);
                }
                drop(authority);
                return Err(error);
            }
        }
    }
    panic!("Drawing mutation digest did not terminate")
}

fn rich_layer() -> DrawingLayerNode {
    let mut group = crate::schema::create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("Digest Group")).to_string().into()).expect("nonempty authored identity"), "Digest Group");
    let base = crate::schema::layer_base_mut(&mut group);
    base.visible = false;
    base.locked = true;
    base.opacity = 0.75;
    base.blend_mode = "multiply".into();
    base.transform = crate::DrawingTransform { x: 1.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 0.5, shear: 0.75 };
    base.attributes.fill = Some(FillStyle::RadialGradient { cx: 1.0, cy: 2.0, r: 3.0, stops: vec![GradientStop { offset: 0.25, color: [0.1, 0.2, 0.3, 0.4] }].into() });
    base.attributes.stroke = Some(StrokeStyle { color: [0.5, 0.6, 0.7, 0.8], width: 2.0, cap: crate::StrokeCap::Round, join: crate::StrokeJoin::Bevel, dash: Some(vec![1.0, 2.0].into()) });
    if let DrawingLayerNode::Group(value) = &mut group {
        value.children.push(crate::schema::create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Shape")).to_string().into()).expect("nonempty authored identity"), "Shape"));
        value.children.push(crate::standards::v1::subsets::any::schema::create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("Path")).to_string().into()).expect("nonempty authored identity"), 
            "Path",
            vec![
                PathSegment::Move { to: [1.0, 2.0] },
                PathSegment::Line { to: [3.0, 4.0] },
                PathSegment::Quad { ctrl: [5.0, 6.0], to: [7.0, 8.0] },
                PathSegment::Cubic { ctrl1: [9.0, 10.0], ctrl2: [11.0, 12.0], to: [13.0, 14.0] },
                PathSegment::Arc { rx: 15.0, ry: 16.0, rotation: 17.0, large_arc: true, sweep: false, to: [18.0, 19.0] },
                PathSegment::Close,
            ].into(),
        ));
        value.children.push(crate::schema::create_drawing_text_layer(crate::schema::identity::DrawingIdentity::admit((("Text")).to_string().into()).expect("nonempty authored identity"), "Text"));
        value.children.push(crate::schema::create_drawing_image_layer(crate::schema::identity::DrawingIdentity::admit((("Image")).to_string().into()).expect("nonempty authored identity"), "Image", "asset-reference"));
        value.children.push(crate::schema::create_drawing_boolean_layer(crate::schema::identity::DrawingIdentity::admit((("Boolean")).to_string().into()).expect("nonempty authored identity"), "Boolean", "union", vec!["a".into(), "b".into()].into()));
        value.children.push(crate::schema::create_drawing_trace_layer(crate::schema::identity::DrawingIdentity::admit((("Trace")).to_string().into()).expect("nonempty authored identity"), "Trace", "trace-source"));
    }
    group
}

fn create_digest(layer: DrawingLayerNode) -> [u8; 32] {
    let mutation = DrawingMutation::CreateLayer(CreateLayer { parent_id: Some("parent".into()), index: Some(3), layer: Box::new(layer) });
    let output = digest(&mutation).expect("rich Drawing create mutation hashes");
    drain_mutation(mutation);
    output
}

fn assert_mutation_digest_distinct(left: DrawingMutation, right: DrawingMutation) {
    let left_digest = digest(&left).expect("left Drawing mutation hashes");
    let right_digest = digest(&right).expect("right Drawing mutation hashes");
    assert_ne!(left_digest, right_digest, "changing one Drawing mutation semantic field changes the retained SHA-256 authority");
    drain_mutation(left);
    drain_mutation(right);
}

fn rich_child(layer: &mut DrawingLayerNode, index: usize) -> &mut DrawingLayerNode {
    let DrawingLayerNode::Group(group) = layer else { panic!("rich Drawing fixture root remains a Group") };
    group.children.get_mut(index).expect("rich Drawing fixture child")
}

#[test]
fn retained_drawing_mutation_candidate_covers_all_variants_and_returns_exact_owners() {
    let source = nested_snapshot();
    let group = crate::schema::layer_id(source.layers.last().expect("group")).clone();
    let (shape, boolean, trace) = match source.layers.last().expect("group") {
        DrawingLayerNode::Group(value) => (crate::schema::layer_id(&value.children[0]).clone(), crate::schema::layer_id(&value.children[1]).clone(), crate::schema::layer_id(&value.children[2]).clone()),
        _ => unreachable!("Drawing fixture group remains exact"),
    };
    let mutations = vec![
        crate::mutations::set_group_isolation(group.clone(),true),
        DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: shape.clone(), visible: false }),
        DrawingMutation::SetLayerLocked(SetLayerLocked { layer_id: shape.clone(), locked: true }),
        DrawingMutation::SetLayerOpacity(SetLayerOpacity { layer_id: shape.clone(), opacity: 0.5 }),
        DrawingMutation::SetLayerFillRule(SetLayerFillRule {layer_id:shape.clone(),fill_rule:crate::FillRule::Nonzero}),
        DrawingMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id: shape.clone(), blend_mode: "multiply".into() }),
        DrawingMutation::RenameLayer(RenameLayer { layer_id: shape.clone(), new_name: "Renamed".into() }),
        DrawingMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: shape.clone(), transform: crate::DrawingTransform { x: 1.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 0.5, shear: 0.75 } }),
        DrawingMutation::ReplaceLayerFill(ReplaceLayerFill {
            layer_id: shape.clone(),
            fill: Some(FillStyle::LinearGradient { x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0, stops: vec![GradientStop { offset: 0.0, color: [1.0, 0.0, 0.0, 1.0] }, GradientStop { offset: 1.0, color: [0.0, 0.0, 1.0, 1.0] }].into() }),
        }),
        DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: shape.clone(), stroke: Some(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 2.0, cap: crate::StrokeCap::Round, join: crate::StrokeJoin::Bevel, dash: Some(vec![1.0, 2.0].into()) }) }),
        DrawingMutation::SetLayerBooleanOperation(SetLayerBooleanOperation { layer_id: boolean, boolean_operation: "difference".into() }),
        DrawingMutation::UpdateLayerTraceParams(UpdateLayerTraceParams { layer_id: trace, params: crate::DrawingTraceParams { threshold: 0.4, simplify_epsilon: 1.2 } }),
        DrawingMutation::CreateLayer(CreateLayer { parent_id: Some(group.clone()), index: Some(1), layer: Box::new(crate::standards::v1::subsets::any::schema::create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("Created")).to_string().into()).expect("nonempty authored identity"), "Created", vec![PathSegment::Move { to: [0.0, 0.0] }].into())) }),
        DrawingMutation::DuplicateLayer(DuplicateLayer { identities: duplicate_assignments(&source,&shape), layer_id: shape.clone() }),
        DrawingMutation::DeleteLayer(DeleteLayer { layer_id: shape.clone() }),
        DrawingMutation::ReorderLayer(ReorderLayer { layer_id: shape, parent_id: None, index: 0 }),
    ];
    for mutation in mutations {
        let value = apply(nested_snapshot(), &mutation).expect("retained Drawing mutation applies");
        drain_snapshot(value);
    }
    drain_snapshot(source);
}
#[test]
fn retained_image_asset_mutations_replay_sparse_delta_and_reference_refusal(){
    let before:DrawingSnapshot=serde_json::from_str(include_str!("../../../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/📥️import-image-asset/➕️adds/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let after:DrawingSnapshot=serde_json::from_str(include_str!("../../../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/📥️import-image-asset/➕️adds/📸️snapshot/➡️after/🔣️.json")).unwrap();
    let mutation=crate::mutations::import_image_asset("bitmap".into(),after.assets.get("bitmap").unwrap().clone());let imported=apply(before.clone(),&mutation).unwrap();assert_eq!(imported,after);drain_mutation(mutation);
    let removal=crate::mutations::remove_image_asset("bitmap".into());let restored=apply(imported,&removal).unwrap();assert_eq!(restored,before);drain_snapshot(restored);
    let mut referenced=after.clone();let mut image=crate::schema::create_drawing_image_layer(crate::schema::identity::DrawingIdentity::admit((("Image")).to_string().into()).expect("nonempty authored identity"), "Image","bitmap");crate::schema::layer_base_mut(&mut image).id="image".into();referenced.layers.push(image);let expected=referenced.clone();let (rejected,error)=apply(referenced,&removal).unwrap_err();assert_eq!(error,"drawing-store.asset-still-referenced");assert_eq!(rejected,expected);drain_snapshot(rejected);
    let mut referenced=after.clone();let mut trace=crate::schema::create_drawing_trace_layer(crate::schema::identity::DrawingIdentity::admit((("Trace")).to_string().into()).expect("nonempty authored identity"), "Trace","bitmap");crate::schema::layer_base_mut(&mut trace).id="trace".into();referenced.layers.push(trace);let expected=referenced.clone();let (rejected,error)=apply(referenced,&removal).unwrap_err();assert_eq!(error,"drawing-store.asset-still-referenced");assert_eq!(rejected,expected);drain_snapshot(rejected);drain_mutation(removal);drain_snapshot(before);drain_snapshot(after);
    eprintln!("[DEBUG] Retained native image asset import/remove exact neutral delta and unchanged image/trace reference refusals");
}

#[test]
fn retained_drawing_process_arena_pool_cap_plus_one_returns_exact_slots_and_rejects_stale_aba() {
    let pool = DrawingMutationArenaPool::try_new().expect("isolated Drawing process arena pool claims exact bytes and items before operation admission");
    let mut first = Vec::new();
    for index in 0..DRAWING_MUTATION_ARENA_POOL_CAPACITY {
        let candidate = DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(8_100 + index as u64), semio_framework_job::Generation(100 + index as u64), pool.clone())
            .expect("each fixed Drawing process arena slot admits exactly once");
        first.push((
            candidate.arena_slot,
            candidate.arena_generation,
            candidate.container_reverse.as_ref().expect("reverse arena owner").as_ptr(),
            candidate.container_output.as_ref().expect("output arena owner").as_ptr(),
            candidate.overlay_pages.as_ref().expect("page arena owner")[0].as_ptr(),
            candidate.duplicate_id_owner.as_ref().expect("duplicate id owner").as_ptr(),
            candidate,
        ));
    }
    match DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(8_999), semio_framework_job::Generation(999), pool.clone()) {
        Err(error) => assert_eq!(error, "drawing-store.mutation-arena-pool-saturated"),
        Ok(_) => panic!("fixed Drawing arena pool must reject capacity +1"),
    }
    for entry in &mut first {
        for phase in 0..4 {
            assert_eq!(entry.6.return_arena_owner().expect("one fixed Drawing root returns per opportunity"), Some(phase == 3));
            let state = pool.state.try_lock().expect("isolated Drawing pool is uncontended");
            let slot = &state.slots[entry.0];
            let returned = usize::from(slot.reverse.is_some()) + usize::from(slot.output.is_some()) + usize::from(slot.pages.is_some()) + usize::from(slot.duplicate_id.is_some());
            assert_eq!(returned, phase + 1, "exactly one fixed arena root returns per grant");
            assert_eq!(slot.leased, phase < 3, "slot becomes available only after the fourth exact owner returns");
        }
        close_candidate(&mut entry.6, None);
    }
    let first_witnesses: Vec<_> = first.iter().map(|entry| (entry.0, entry.1, entry.2, entry.3, entry.4, entry.5)).collect();
    for entry in first {
        drop(entry.6);
    }

    let mut second = Vec::new();
    for index in 0..DRAWING_MUTATION_ARENA_POOL_CAPACITY {
        let candidate = DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(8_200 + index as u64), semio_framework_job::Generation(200 + index as u64), pool.clone()).expect("returned Drawing arena slot re-admits");
        second.push(candidate);
    }
    for candidate in &second {
        let witness = first_witnesses.iter().find(|entry| entry.0 == candidate.arena_slot).expect("same fixed Drawing arena slot returns");
        assert!(candidate.arena_generation > witness.1, "slot generation advances to reject stale ABA returns");
        assert_eq!(candidate.container_reverse.as_ref().expect("reverse arena returned").as_ptr(), witness.2);
        assert_eq!(candidate.container_output.as_ref().expect("output arena returned").as_ptr(), witness.3);
        assert_eq!(candidate.overlay_pages.as_ref().expect("page arena returned")[0].as_ptr(), witness.4);
        assert_eq!(candidate.duplicate_id_owner.as_ref().expect("duplicate owner returned").as_ptr(), witness.5);
    }
    for candidate in &mut second {
        close_candidate(candidate, None);
    }
    for candidate in second {
        drop(candidate);
    }
}

fn step_arena_bootstrap(bootstrap: &mut DrawingMutationArenaPoolBootstrap) -> Result<bool, &'static str> {
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let mut context =
        semio_framework_job::StepContext::new(semio_framework_job::OperationId(7_902), semio_framework_job::Generation(79), semio_framework_job::StepBudget::new(1, u64::MAX), cancel, semio_framework_job::default_now_us, &mut preview_sequence);
    bootstrap.step(&mut context)
}

thread_local! {
    static BOOTSTRAP_YIELD_CLOCK: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

fn bootstrap_yield_clock() -> Option<u64> {
    Some(BOOTSTRAP_YIELD_CLOCK.with(|clock| {
        let now = clock.get();
        clock.set(now + 1);
        now
    }))
}

#[test]
fn retained_drawing_arena_bootstrap_deadline_between_guards_preserves_and_resumes_its_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧮️mutation-admission/🔣️.json")).unwrap();
    let law = &fixture["bootstrapYield"];
    let mut bootstrap = DrawingMutationArenaPoolBootstrap::production(DrawingMutationArenaBootstrapAdmission::fixed().unwrap());
    assert_eq!(step_arena_bootstrap(&mut bootstrap), Ok(false));
    assert_eq!(step_arena_bootstrap(&mut bootstrap), Ok(false));
    let allocation = bootstrap.allocation;
    let operation = semio_framework_job::OperationId(7_906);
    let generation = semio_framework_job::Generation(79);
    let mut job = DrawingMutationArenaBootstrapJob::new(operation, generation).unwrap();
    let mut state = DrawingMutationArenaProcessState::Building(bootstrap);
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    BOOTSTRAP_YIELD_CLOCK.with(|clock| clock.set(law["clockReadsUs"][0].as_u64().unwrap()));
    let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, law["deadlineUs"].as_u64().unwrap()), cancel.clone(), bootstrap_yield_clock, &mut preview_sequence);
    assert!(!context.should_yield(), "the outer admission guard still has time");
    let yielded = job.step_locked(&mut state, &mut context);
    let retained = match &state {
        DrawingMutationArenaProcessState::Building(owner) => Some((owner.allocation - allocation, owner.fault)),
        _ => None,
    };
    let mut terminal = None;
    for _ in 0..1_000 {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        match job.step_locked(&mut state, &mut context) {
            DrawingMutationArenaBootstrapStep::Pending { .. } | DrawingMutationArenaBootstrapStep::Blocked => {}
            outcome => { terminal = Some(outcome); break; }
        }
    }
    let actual = [if retained.is_some() { "yielded" } else { "retired" }, if terminal == Some(DrawingMutationArenaBootstrapStep::Ready) { "resumed" } else { "faulted" }];
    assert!(job.terminal, "the resumed or faulted owner completes its bounded lifecycle");
    drop(state);
    assert_eq!(yielded, DrawingMutationArenaBootstrapStep::Pending { advanced_items: 1 });
    assert_eq!(retained, Some((law["expectedAllocationDelta"].as_u64().unwrap() as usize, None)), "budget exhaustion preserves the exact building owner without allocating or faulting");
    assert_eq!(serde_json::to_value(actual).unwrap(), law["expectedStages"]);
}

fn close_arena_bootstrap_step(bootstrap: &mut DrawingMutationArenaPoolBootstrap) -> store::SnapshotRetirementStep {
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let mut context =
        semio_framework_job::StepContext::new(semio_framework_job::OperationId(7_903), semio_framework_job::Generation(79), semio_framework_job::StepBudget::new(1, u64::MAX), cancel, semio_framework_job::default_now_us, &mut preview_sequence);
    bootstrap.close_step(&mut context)
}

fn close_arena_bootstrap(bootstrap: &mut DrawingMutationArenaPoolBootstrap) -> usize {
    let mut released_roots = 0;
    for _ in 0..10_000 {
        match close_arena_bootstrap_step(bootstrap) {
            store::SnapshotRetirementStep::Pending { released_items, .. } => {
                assert!(released_items <= 1, "Drawing arena bootstrap releases at most one exact root per grant");
                released_roots += released_items;
            }
            store::SnapshotRetirementStep::Complete => {
                assert!(bootstrap.terminal_is_empty());
                return released_roots;
            }
            store::SnapshotRetirementStep::Blocked => panic!("isolated Drawing arena bootstrap never blocks"),
        }
    }
    panic!("Drawing arena bootstrap retirement did not terminate")
}

fn build_arena_bootstrap_owners(bootstrap: &mut DrawingMutationArenaPoolBootstrap) {
    for _ in 0..10_000 {
        if bootstrap.owner == DRAWING_MUTATION_ARENA_POOL_CAPACITY {
            return;
        }
        assert_eq!(step_arena_bootstrap(bootstrap), Ok(false));
    }
    panic!("Drawing arena bootstrap did not construct its fixed owner catalog")
}

#[test]
fn retained_drawing_arena_bootstrap_failure_at_each_allocation_retires_one_exact_root_per_grant() {
    let allocations = DRAWING_MUTATION_ARENA_POOL_CAPACITY * 20;
    for failure_at in 0..allocations {
        let mut bootstrap = DrawingMutationArenaPoolBootstrap::new(Some(failure_at), None, usize::MAX, usize::MAX);
        let fault = loop {
            match step_arena_bootstrap(&mut bootstrap) {
                Ok(false) => {}
                Ok(true) => panic!("injected Drawing arena allocation failure was not observed"),
                Err(error) => break error,
            }
        };
        assert_eq!(fault, "drawing-store.mutation-arena-bootstrap-injected-allocation");
        assert_eq!(bootstrap.allocation, failure_at + 1);
        assert_eq!(close_arena_bootstrap(&mut bootstrap), failure_at, "every successfully constructed Vec/String/page root is handed to the retained fault cursor");
        drop(bootstrap);
    }
}

#[test]
fn retained_drawing_arena_bootstrap_failure_after_each_bundle_keeps_every_root_until_terminal_close() {
    for owner in 0..DRAWING_MUTATION_ARENA_POOL_CAPACITY {
        let mut bootstrap = DrawingMutationArenaPoolBootstrap::new(None, Some(owner), usize::MAX, usize::MAX);
        let fault = loop {
            match step_arena_bootstrap(&mut bootstrap) {
                Ok(false) => {}
                Ok(true) => panic!("injected Drawing arena bundle failure was not observed"),
                Err(error) => break error,
            }
        };
        assert_eq!(fault, "drawing-store.mutation-arena-bootstrap-injected-owner");
        assert_eq!(bootstrap.owner, owner + 1);
        assert_eq!(close_arena_bootstrap(&mut bootstrap), (owner + 1) * 20, "every completed bundle remains in the retained construction-fault owner");
        drop(bootstrap);
    }
}

#[test]
fn retained_drawing_arena_bootstrap_advances_one_allocation_per_turn_and_withholds_incomplete_pool() {
    let mut bootstrap = DrawingMutationArenaPoolBootstrap::production(DrawingMutationArenaBootstrapAdmission::fixed().expect("fixed Drawing arena bootstrap claim"));
    let mut turns = 0;
    while !bootstrap.ready {
        let allocation = bootstrap.allocation;
        assert!(bootstrap.take_pool().is_none(), "an incomplete Drawing arena bootstrap cannot publish an operation-admission pool");
        assert!(matches!(step_arena_bootstrap(&mut bootstrap), Ok(false) | Ok(true)));
        assert!(bootstrap.allocation.saturating_sub(allocation) <= 1, "one governed bootstrap turn allocates at most one retained root");
        turns += 1;
        assert!(turns < 1_000);
    }
    let pool = bootstrap.take_pool().expect("terminal Drawing arena bootstrap publishes the fixed process pool");
    assert_eq!(pool.state.try_lock().expect("isolated Drawing arena pool is uncontended").slots.len(), DRAWING_MUTATION_ARENA_POOL_CAPACITY);
    drop(bootstrap);
    drop(pool);
}

#[test]
fn retained_drawing_arena_bootstrap_exact_cap_and_plus_one_rejection_preserve_every_owner_until_close() {
    let mut exact = DrawingMutationArenaPoolBootstrap::new(None, None, usize::MAX, usize::MAX);
    build_arena_bootstrap_owners(&mut exact);
    let admitted_items = exact.admitted_items;
    let admitted_bytes = exact.admitted_bytes;
    let fixed = DrawingMutationArenaBootstrapAdmission::fixed().expect("four-slot bootstrap derives its claim from one configured arena owner");
    assert_eq!((fixed.maximum_items, fixed.maximum_bytes), (admitted_items, admitted_bytes), "boot4 is exactly pool4 multiplied by the actual one-owner A");
    exact.maximum_items = admitted_items;
    exact.maximum_bytes = admitted_bytes;
    assert_eq!(step_arena_bootstrap(&mut exact), Ok(true), "allocator-returned Drawing arena capacities admit at the exact boundary");
    assert_eq!(close_arena_bootstrap(&mut exact), DRAWING_MUTATION_ARENA_POOL_CAPACITY * 20);
    drop(exact);

    for (maximum_items, maximum_bytes) in [(admitted_items - 1, admitted_bytes), (admitted_items, admitted_bytes - 1)] {
        let mut rejected = DrawingMutationArenaPoolBootstrap::new(None, None, usize::MAX, usize::MAX);
        build_arena_bootstrap_owners(&mut rejected);
        rejected.maximum_items = maximum_items;
        rejected.maximum_bytes = maximum_bytes;
        assert_eq!(step_arena_bootstrap(&mut rejected), Err("drawing-store.mutation-arena-pool-capacity"));
        assert_eq!(close_arena_bootstrap(&mut rejected), DRAWING_MUTATION_ARENA_POOL_CAPACITY * 20, "aggregate +1 rejection retains all eighty exact roots until cursorized close");
        drop(rejected);
    }
}

#[test]
fn retained_drawing_arena_default_second_app_and_borrow_only_request_without_allocation() {
    let state = DRAWING_MUTATION_ARENA_POOL.get_or_init(|| std::sync::Mutex::new(DrawingMutationArenaProcessState::Inert));
    let guard = state.try_lock().expect("isolated Drawing request fixture owns the inert process metadata");
    let witness = match &*guard {
        DrawingMutationArenaProcessState::Inert => (0, 0),
        DrawingMutationArenaProcessState::Building(bootstrap) => (1, bootstrap.allocation),
        DrawingMutationArenaProcessState::Ready(_) => (2, 0),
        DrawingMutationArenaProcessState::Retiring(bootstrap) => (3, bootstrap.allocation),
        DrawingMutationArenaProcessState::Fault(_) => (4, 0),
    };
    assert_eq!(request_drawing_mutation_arena_pool(), DrawingMutationArenaPoolAvailability::Contended);
    assert_eq!(request_drawing_mutation_arena_pool(), DrawingMutationArenaPoolAvailability::Contended, "a second app request coalesces fixed metadata without allocation");
    match borrow_drawing_mutation_arena() {
        Err(error) => assert_eq!(error, DrawingMutationArenaBorrowError::Contended),
        Ok(_) => panic!("borrow under process contention cannot expose an arena owner"),
    }
    let after = match &*guard {
        DrawingMutationArenaProcessState::Inert => (0, 0),
        DrawingMutationArenaProcessState::Building(bootstrap) => (1, bootstrap.allocation),
        DrawingMutationArenaProcessState::Ready(_) => (2, 0),
        DrawingMutationArenaProcessState::Retiring(bootstrap) => (3, bootstrap.allocation),
        DrawingMutationArenaProcessState::Fault(_) => (4, 0),
    };
    assert_eq!(after, witness, "default/request/borrow cannot advance a bootstrap allocation while no governed job owns the process turn");
    drop(guard);
}

#[test]
fn retained_drawing_arena_bootstrap_job_cancel_budget_contention_and_saturation_are_governed() {
    let operation = semio_framework_job::OperationId(7_904);
    let generation = semio_framework_job::Generation(79);
    let mut job = DrawingMutationArenaBootstrapJob::new(operation, generation).expect("fixed Drawing bootstrap admission claim");
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let mut exhausted = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(0, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
    assert_eq!(job.step(&mut exhausted), DrawingMutationArenaBootstrapStep::Blocked, "zero-budget bootstrap cannot allocate or retire");

    let state = DRAWING_MUTATION_ARENA_POOL.get_or_init(|| std::sync::Mutex::new(DrawingMutationArenaProcessState::Inert));
    let guard = state.try_lock().expect("isolated Drawing bootstrap fixture owns process contention");
    let mut contended = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
    assert_eq!(job.step(&mut contended), DrawingMutationArenaBootstrapStep::Blocked, "process contention leaves the exact bootstrap owner untouched");
    drop(guard);

    let local_operation = semio_framework_job::OperationId(7_905);
    let local_generation = semio_framework_job::Generation(79);
    let mut local_job = DrawingMutationArenaBootstrapJob::new(local_operation, local_generation).expect("local Drawing bootstrap job claims its fixed admission");
    let local_cancel = semio_framework_job::root_cancel_token();
    let mut local_preview_sequence = 0;
    let mut local_state = DrawingMutationArenaProcessState::Inert;
    for _ in 0..3 {
        let mut admitted = semio_framework_job::StepContext::new(local_operation, local_generation, semio_framework_job::StepBudget::new(1, u64::MAX), local_cancel.clone(), semio_framework_job::default_now_us, &mut local_preview_sequence);
        assert_eq!(local_job.step_locked(&mut local_state, &mut admitted), DrawingMutationArenaBootstrapStep::Pending { advanced_items: 1 });
    }
    let allocated = match &local_state {
        DrawingMutationArenaProcessState::Building(bootstrap) => bootstrap.allocation,
        _ => panic!("three governed Drawing bootstrap turns retain one partially allocated bundle"),
    };
    assert_eq!(allocated, 1, "only admitted worker turns may advance allocation boundaries");
    local_cancel.cancel_now();
    for _ in 0..100 {
        let mut cancelled = semio_framework_job::StepContext::new(local_operation, local_generation, semio_framework_job::StepBudget::new(1, u64::MAX), local_cancel.clone(), semio_framework_job::default_now_us, &mut local_preview_sequence);
        match local_job.step_locked(&mut local_state, &mut cancelled) {
            DrawingMutationArenaBootstrapStep::Pending { advanced_items } => assert!(advanced_items <= 1),
            DrawingMutationArenaBootstrapStep::Cancelled => break,
            DrawingMutationArenaBootstrapStep::Blocked => {}
            DrawingMutationArenaBootstrapStep::Ready | DrawingMutationArenaBootstrapStep::Fault(_) => panic!("cancelled partial Drawing bootstrap must retire to exact Cancelled"),
        }
    }
    assert!(local_job.terminal);

    let pool = DrawingMutationArenaPool::try_new().expect("isolated fixed Drawing pool admits exact saturation fixture");
    let mut candidates = Vec::new();
    for index in 0..DRAWING_MUTATION_ARENA_POOL_CAPACITY {
        candidates
            .push(DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(7_910 + index as u64), semio_framework_job::Generation(80 + index as u64), pool.clone()).expect("each fixed Drawing pool slot admits once"));
    }
    assert!(matches!(DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(7_999), semio_framework_job::Generation(99), pool), Err("drawing-store.mutation-arena-pool-saturated")));
    for candidate in &mut candidates {
        close_candidate(candidate, None);
    }
    for candidate in candidates {
        drop(candidate);
    }
}

#[test]
fn retained_drawing_depth_plus_one_and_hostile_fields_fault_then_close_terminal_empty() {
    let mut layer = crate::standards::v1::subsets::any::schema::create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("leaf")).to_string().into()).expect("nonempty authored identity"), "leaf", Default::default());
    for depth in 0..=DRAWING_MAXIMUM_LAYER_DEPTH {
        let mut parent = crate::schema::create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit(((&format!("depth-{depth}"))).to_string().into()).expect("nonempty authored identity"), &format!("depth-{depth}"));
        if let DrawingLayerNode::Group(value) = &mut parent {
            value.children.push(layer);
        }
        layer = parent;
    }
    let mut source = crate::standards::v1::subsets::any::schema::default_drawing_document("drawing-depth-plus-one", None);
    source.layers = vec![layer].into();
    let mutation = DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: "missing".into(), visible: false });
    let (source, error) = apply(source, &mutation).expect_err("retained Drawing depth +1 authority rejects");
    assert_eq!(error, "drawing-store.preflight-depth-capacity");
    drain_snapshot(source);

    let source = nested_snapshot();
    let mutation = DrawingMutation::RenameLayer(RenameLayer { layer_id: "x".repeat(DRAWING_OWNED_FIELD_BYTES + 1), new_name: "hostile".into() });
    let (source, _) = apply(source, &mutation).expect_err("hostile Drawing field rejects");
    drain_snapshot(source);
}

#[test]
fn retained_drawing_container_false_terminal_saturation_and_interrupted_close_preserve_exact_owner() {
    let mut snapshot = crate::standards::v1::subsets::any::schema::default_drawing_document("rebuild-reservation", None);
    snapshot.layers = vec![crate::standards::v1::subsets::any::schema::create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("first")).to_string().into()).expect("nonempty authored identity"), "first", Default::default()), crate::standards::v1::subsets::any::schema::create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("second")).to_string().into()).expect("nonempty authored identity"), "second", Default::default())].into();
    let mutation = DrawingMutation::CreateLayer(CreateLayer { parent_id: None, index: Some(1), layer: Box::new(crate::standards::v1::subsets::any::schema::create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("pending")).to_string().into()).expect("nonempty authored identity"), "pending", Default::default())) });
    let workset = live_workset(&mut snapshot, &mutation).expect("live Drawing rebuild workset admitted");
    let source = std::mem::take(&mut snapshot.layers);
    let DrawingMutation::CreateLayer(mut create) = mutation else { unreachable!() };
    let pending = *std::mem::replace(&mut create.layer, Box::new(crate::standards::v1::subsets::any::schema::create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("retired-placeholder")).to_string().into()).expect("nonempty authored identity"), "retired-placeholder", Default::default())));
    drain_mutation(DrawingMutation::CreateLayer(create));
    drain_snapshot(snapshot);
    let mut reverse = Vec::new();
    let mut output = Vec::new();
    reverse.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY).expect("fixed Drawing reverse arena");
    output.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY).expect("fixed Drawing output arena");
    let source_owner = native_layer_backing(&source);
    let reverse_owner = reverse.as_ptr();
    let output_owner = output.as_ptr();
    let mut authority = DrawingContainerRebuildAuthority::new(source, Some(0), Some(1), Some(pending), reverse, output, workset).unwrap_or_else(|_| panic!("fixed Drawing rebuild admitted"));
    assert!(authority.take().is_none(), "false terminal cannot expose a partially rebuilt owner");
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    for _ in 0..3 {
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(8_005),
            semio_framework_job::Generation(85),
            semio_framework_job::StepBudget::new(1, u64::MAX),
            cancel.clone(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
        );
        assert!(!authority.step(&mut context).expect("Drawing rebuild advances before interruption"));
    }
    let mut rollback_turns = 0;
    while !authority.rollback_step().expect("Drawing rebuild rollback advances one exact owner") {
        rollback_turns += 1;
    }
    assert_eq!(rollback_turns, authority.move_count, "one recorded owner move rolls back per close grant");
    let restored = authority.source.take().expect("original Drawing source owner returns");
    let pending = authority.pending.take().expect("pending Drawing owner returns");
    let reverse = authority.reverse.take().expect("reverse Drawing scratch owner returns");
    let output = authority.output.take().expect("output Drawing scratch owner returns");
    assert_eq!(native_layer_backing(&restored), source_owner);
    assert_eq!(reverse.as_ptr(), reverse_owner);
    assert_eq!(output.as_ptr(), output_owner);
    assert!(authority.removed.is_none());
    authority.finish_handoff().expect("Drawing rebuild rollback reaches exact terminal handoff");
    assert!(authority.terminal_is_empty());
    drop(authority);
    drop(reverse);
    drop(output);
    drain_mutation(DrawingMutation::CreateLayer(CreateLayer { parent_id: None, index: None, layer: Box::new(pending) }));
    let mut restored_snapshot = crate::standards::v1::subsets::any::schema::default_drawing_document("restored-rebuild", None);
    restored_snapshot.layers = restored;
    drain_snapshot(restored_snapshot);
}

#[test]
fn retained_drawing_rebuild_fault_after_every_phase_rolls_back_exact_container_and_reuses_pool_slot() {
    for phase in 0..=3 {
        for stale in [false, true] {
            let pool = DrawingMutationArenaPool::try_new().expect("isolated Drawing rollback pool admits exact owners");
            let mut source = crate::standards::v1::subsets::any::schema::default_drawing_document("rebuild-rollback", None);
            reserve_native_layer_slots(&mut source.layers, DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY);
            for index in 0..3 {
                source.layers.push(crate::standards::v1::subsets::any::schema::create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit(((&format!("source-{index}"))).to_string().into()).expect("nonempty authored identity"), &format!("source-{index}"), Default::default()));
            }
            let source_owner = native_layer_backing(&source.layers);
            let source_ids: Vec<_> = source.layers.iter().map(|layer| crate::schema::layer_id(layer).to_string()).collect();
            let mutation = DrawingMutation::CreateLayer(CreateLayer { parent_id: None, index: Some(1), layer: Box::new(crate::standards::v1::subsets::any::schema::create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("pending")).to_string().into()).expect("nonempty authored identity"), "pending", Default::default())) });
            let operation = semio_framework_job::OperationId(8_500 + phase as u64);
            let generation = semio_framework_job::Generation(850 + phase as u64);
            let mut authority = DrawingMutationCandidateAuthority::try_new_from_pool(operation, generation, pool.clone()).expect("Drawing rollback candidate borrows one exact pool slot");
            let slot = authority.arena_slot;
            let arena_generation = authority.arena_generation;
            let reverse_owner = authority.container_reverse.as_ref().expect("Drawing rollback reverse owner").as_ptr();
            let output_owner = authority.container_output.as_ref().expect("Drawing rollback output owner").as_ptr();
            let page_catalog_owner = authority.overlay_pages.as_ref().expect("Drawing rollback page catalog owner").as_ptr();
            let page_owners: [usize; DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY] = std::array::from_fn(|index| authority.overlay_pages.as_ref().expect("Drawing rollback page owner")[index].as_ptr() as usize);
            let duplicate_owner = authority.duplicate_id_owner.as_ref().expect("Drawing rollback duplicate owner").as_ptr();
            let cancel = semio_framework_job::root_cancel_token();
            let mut preview_sequence = 0;
            for _ in 0..100_000 {
                if authority.rebuild.as_ref().is_some_and(|rebuild| rebuild.recorded_move_in_phase(phase)) {
                    break;
                }
                let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
                assert!(!authority.step(&mut source, &mutation, &mut context).expect("Drawing rollback fixture reaches every internal rebuild phase"));
            }
            assert!(authority.rebuild.as_ref().is_some_and(|rebuild| rebuild.recorded_move_in_phase(phase)));
            if !stale {
                cancel.cancel_now();
            }
            let mut rejected = semio_framework_job::StepContext::new(
                operation,
                if stale { semio_framework_job::Generation(generation.0 + 1) } else { generation },
                semio_framework_job::StepBudget::new(1, u64::MAX),
                cancel,
                semio_framework_job::default_now_us,
                &mut preview_sequence,
            );
            assert_eq!(authority.step(&mut source, &mutation, &mut rejected), Err(if stale { "drawing-store.mutation-candidate-stale-authority" } else { "drawing-store.mutation-candidate-cancelled" }));
            close_candidate(&mut authority, Some(&mut source));
            assert_eq!(native_layer_backing(&source.layers), source_owner, "rollback restores the exact original live native page backing");
            assert_eq!(source.layers.iter().map(|layer| crate::schema::layer_id(layer)).collect::<Vec<_>>(), source_ids.iter().map(String::as_str).collect::<Vec<_>>(), "rollback restores exact FIFO layer order");
            drop(authority);

            let mut reused = DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(operation.0 + 100), semio_framework_job::Generation(generation.0 + 100), pool.clone())
                .expect("rolled-back Drawing pool slot re-admits immediately");
            assert_eq!(reused.arena_slot, slot);
            assert!(reused.arena_generation > arena_generation);
            assert_eq!(reused.container_reverse.as_ref().expect("returned reverse owner").as_ptr(), reverse_owner);
            assert_eq!(reused.container_output.as_ref().expect("returned output owner").as_ptr(), output_owner);
            assert_eq!(reused.overlay_pages.as_ref().expect("returned page catalog owner").as_ptr(), page_catalog_owner);
            assert_eq!(std::array::from_fn::<_, DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY, _>(|index| reused.overlay_pages.as_ref().expect("returned page owner")[index].as_ptr() as usize), page_owners);
            assert_eq!(reused.duplicate_id_owner.as_ref().expect("returned duplicate owner").as_ptr(), duplicate_owner);
            close_candidate(&mut reused, None);
            drop(reused);
            drain_mutation(mutation);
            drain_snapshot(source);
        }
    }
}

#[test]
fn retained_drawing_reorder_fault_after_source_handoff_restores_exact_nested_fifo_and_pool_roots() {
    for stale in [false, true] {
        let pool = DrawingMutationArenaPool::try_new().expect("isolated Drawing reorder rollback pool admits exact owners");
        let mut source = nested_snapshot();
        let (group_id, target, source_owner, source_ids) = match source.layers.last_mut().expect("Drawing reorder rollback group") {
            DrawingLayerNode::Group(group) => {
                reserve_native_layer_slots(&mut group.children, DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY);
                (group.base.id.clone(), crate::schema::layer_id(&group.children[0]).clone(), native_layer_backing(&group.children), group.children.iter().map(|layer| crate::schema::layer_id(layer).to_string()).collect::<Vec<_>>())
            }
            _ => unreachable!("Drawing reorder rollback fixture remains a group"),
        };
        let mutation = DrawingMutation::ReorderLayer(ReorderLayer { layer_id: target, parent_id: Some(group_id), index: 2 });
        let operation = semio_framework_job::OperationId(8_700);
        let generation = semio_framework_job::Generation(870);
        let mut authority = DrawingMutationCandidateAuthority::try_new_from_pool(operation, generation, pool.clone()).expect("Drawing reorder rollback candidate borrows one exact pool slot");
        let slot = authority.arena_slot;
        let reverse_owner = authority.container_reverse.as_ref().expect("Drawing reorder reverse owner").as_ptr();
        let output_owner = authority.container_output.as_ref().expect("Drawing reorder output owner").as_ptr();
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        for _ in 0..100_000 {
            if authority.rebuild_role == Some(DrawingContainerRebuildRole::Destination) && authority.rebuild.as_ref().is_some_and(|rebuild| rebuild.recorded_move_in_phase(2)) {
                break;
            }
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
            assert!(!authority.step(&mut source, &mutation, &mut context).expect("Drawing reorder rollback fixture reaches destination rebuild after source handoff"));
        }
        assert_eq!(authority.rebuild_role, Some(DrawingContainerRebuildRole::Destination));
        assert!(authority.source_undo.is_some(), "source handoff keeps the exact insertion undo authority until destination publication");
        if !stale {
            cancel.cancel_now();
        }
        let mut rejected = semio_framework_job::StepContext::new(
            operation,
            if stale { semio_framework_job::Generation(generation.0 + 1) } else { generation },
            semio_framework_job::StepBudget::new(1, u64::MAX),
            cancel,
            semio_framework_job::default_now_us,
            &mut preview_sequence,
        );
        assert_eq!(authority.step(&mut source, &mutation, &mut rejected), Err(if stale { "drawing-store.mutation-candidate-stale-authority" } else { "drawing-store.mutation-candidate-cancelled" }));
        close_candidate(&mut authority, Some(&mut source));
        drop(authority);
        let DrawingLayerNode::Group(group) = source.layers.last().expect("Drawing reorder rollback group remains retained") else { unreachable!("Drawing reorder rollback group remains a group") };
        assert_eq!(native_layer_backing(&group.children), source_owner, "source undo restores the exact nested live native page backing");
        assert_eq!(group.children.iter().map(|layer| crate::schema::layer_id(layer)).collect::<Vec<_>>(), source_ids.iter().map(String::as_str).collect::<Vec<_>>(), "source undo restores the exact nested FIFO order");

        let mut reused = DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(operation.0 + 1), semio_framework_job::Generation(generation.0 + 1), pool).expect("reorder rollback returns the exact pool slot");
        assert_eq!(reused.arena_slot, slot);
        assert_eq!(reused.container_reverse.as_ref().expect("returned reorder reverse owner").as_ptr(), reverse_owner);
        assert_eq!(reused.container_output.as_ref().expect("returned reorder output owner").as_ptr(), output_owner);
        close_candidate(&mut reused, None);
        drop(reused);
        drain_mutation(mutation);
        drain_snapshot(source);
    }
}

#[test]
fn retained_drawing_schema_digest_distinguishes_every_nested_semantic_field() {
    let baseline = rich_layer();
    let baseline_digest = create_digest(baseline.clone());
    let mut variants = Vec::new();

    let modifiers: &[fn(&mut DrawingLayerNode)] = &[
        |value| if let DrawingLayerNode::Group(group)=value {group.isolation=true;},
        |value| crate::schema::layer_base_mut(value).attributes.fill_rule = crate::FillRule::Nonzero,
        |value| crate::schema::layer_base_mut(value).id = "different-id".into(),
        |value| crate::schema::layer_base_mut(value).name = "different-name".into(),
        |value| crate::schema::layer_base_mut(value).transform.x = 9.0,
        |value| crate::schema::layer_base_mut(value).transform.y = 9.0,
        |value| crate::schema::layer_base_mut(value).transform.scale_x = 9.0,
        |value| crate::schema::layer_base_mut(value).transform.scale_y = 9.0,
        |value| crate::schema::layer_base_mut(value).attributes.fill = None,
        |value| {
            if let Some(FillStyle::RadialGradient { cx, .. }) = &mut crate::schema::layer_base_mut(value).attributes.fill {
                *cx = 9.0;
            }
        },
        |value| {
            if let Some(FillStyle::RadialGradient { cy, .. }) = &mut crate::schema::layer_base_mut(value).attributes.fill {
                *cy = 9.0;
            }
        },
        |value| {
            if let Some(FillStyle::RadialGradient { r, .. }) = &mut crate::schema::layer_base_mut(value).attributes.fill {
                *r = 9.0;
            }
        },
        |value| {
            if let Some(FillStyle::RadialGradient { stops, .. }) = &mut crate::schema::layer_base_mut(value).attributes.fill {
                stops[0].offset = 0.75;
            }
        },
        |value| {
            if let Some(FillStyle::RadialGradient { stops, .. }) = &mut crate::schema::layer_base_mut(value).attributes.fill {
                stops[0].color[2] = 0.9;
            }
        },
        |value| crate::schema::layer_base_mut(value).attributes.stroke = None,
        |value| crate::schema::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").color[0] = 0.9,
        |value| crate::schema::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").width = 9.0,
        |value| crate::schema::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").cap = crate::StrokeCap::Square,
        |value| crate::schema::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").join = crate::StrokeJoin::Round,
        |value| crate::schema::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").dash.as_mut().expect("dash")[0] = 9.0,
        |value| {
            if let DrawingLayerNode::Shape(shape) = rich_child(value, 0) {
                shape.rect.as_mut().expect("rect").width = 9.0;
            }
        },
        |value| {
            if let DrawingLayerNode::Shape(shape) = rich_child(value, 0) {
                shape.ellipse = Some(crate::DrawingEllipse { cx: 1.0, cy: 2.0, rx: 3.0, ry: 4.0 });
            }
        },
        |value| {
            if let DrawingLayerNode::Shape(shape) = rich_child(value, 0) {
                shape.circle = Some(crate::DrawingCircle { cx: 1.0, cy: 2.0, r: 3.0 });
            }
        },
        |value| {
            if let DrawingLayerNode::Shape(shape) = rich_child(value, 0) {
                shape.line = Some(crate::DrawingLine { x1: 1.0, y1: 2.0, x2: 3.0, y2: 4.0 });
            }
        },
        |value| {
            if let DrawingLayerNode::Shape(shape) = rich_child(value, 0) {
                shape.polygon = Some(crate::DrawingPolygon { points: vec![[1.0, 2.0], [3.0, 4.0]].into() });
            }
        },
        |value| {
            if let DrawingLayerNode::Path(path) = rich_child(value, 1) {
                path.segments[0] = PathSegment::Line { to: [1.0, 2.0] };
            }
        },
        |value| {
            if let DrawingLayerNode::Path(path) = rich_child(value, 1) {
                path.segments[1] = PathSegment::Line { to: [9.0, 4.0] };
            }
        },
        |value| {
            if let DrawingLayerNode::Path(path) = rich_child(value, 1) {
                path.segments[2] = PathSegment::Quad { ctrl: [9.0, 6.0], to: [7.0, 8.0] };
            }
        },
        |value| {
            if let DrawingLayerNode::Path(path) = rich_child(value, 1) {
                path.segments[3] = PathSegment::Cubic { ctrl1: [9.0, 10.0], ctrl2: [11.0, 12.0], to: [13.0, 20.0] };
            }
        },
        |value| {
            if let DrawingLayerNode::Text(text) = rich_child(value, 2) {
                text.x = 9.0;
            }
        },
        |value| {
            if let DrawingLayerNode::Text(text) = rich_child(value, 2) {
                text.y = 9.0;
            }
        },
        |value| {
            if let DrawingLayerNode::Text(text) = rich_child(value, 2) {
                text.content = "different text".into();
            }
        },
        |value| {
            if let DrawingLayerNode::Text(text) = rich_child(value, 2) {
                text.size = 9.0;
            }
        },
        |value| {
            if let DrawingLayerNode::Image(image) = rich_child(value, 3) {
                image.height = 9.0;
            }
        },
        |value| {
            if let DrawingLayerNode::Trace(trace) = rich_child(value, 5) {
                trace.params.simplify_epsilon = 9.0;
            }
        },
        |value| {
            let DrawingLayerNode::Group(group) = value else { unreachable!() };
            group.children.swap(0, 1);
        },
    ];
    for modifier in modifiers {
        let mut value = baseline.clone();
        modifier(&mut value);
        variants.push(value);
    }

    let mut value = baseline.clone();
    crate::schema::layer_base_mut(&mut value).visible = true;
    variants.push(value);
    let mut value = baseline.clone();
    crate::schema::layer_base_mut(&mut value).locked = false;
    variants.push(value);
    let mut value = baseline.clone();
    crate::schema::layer_base_mut(&mut value).opacity = 0.5;
    variants.push(value);
    let mut value = baseline.clone();
    crate::schema::layer_base_mut(&mut value).blend_mode = "screen".into();
    variants.push(value);
    let mut value = baseline.clone();
    crate::schema::layer_base_mut(&mut value).transform.rotation = 0.75;
    variants.push(value);
    let mut value = baseline.clone();
    crate::schema::layer_base_mut(&mut value).transform.shear = 1.25;
    variants.push(value);
    let mut value = baseline.clone();
    crate::schema::layer_base_mut(&mut value).attributes.fill = Some(FillStyle::Solid { color: [0.9, 0.2, 0.3, 0.4] });
    variants.push(value);
    let mut value = baseline.clone();
    crate::schema::layer_base_mut(&mut value).attributes.fill = Some(FillStyle::LinearGradient { x1: 1.0, y1: 2.0, x2: 3.0, y2: 4.0, stops: vec![GradientStop { offset: 0.5, color: [0.1, 0.2, 0.8, 0.4] }].into() });
    variants.push(value);
    let mut value = baseline.clone();
    crate::schema::layer_base_mut(&mut value).attributes.stroke = Some(StrokeStyle { color: [0.9, 0.6, 0.7, 0.8], width: 3.0, cap: crate::StrokeCap::Square, join: crate::StrokeJoin::Round, dash: Some(vec![2.0, 3.0].into()) });
    variants.push(value);
    let mut value = baseline.clone();
    if let DrawingLayerNode::Group(group) = &mut value {
        if let DrawingLayerNode::Path(path) = &mut group.children[1] {
            path.segments[4] = PathSegment::Arc { rx: 15.0, ry: 16.0, rotation: 18.0, large_arc: false, sweep: true, to: [20.0, 19.0] };
        }
    }
    variants.push(value);
    let mut value = baseline.clone();
    if let DrawingLayerNode::Group(group) = &mut value {
        if let DrawingLayerNode::Image(image) = &mut group.children[3] {
            image.image_key = "other-asset-reference".into();
            image.width = 2.0;
        }
    }
    variants.push(value);
    let mut value = baseline.clone();
    if let DrawingLayerNode::Group(group) = &mut value {
        if let DrawingLayerNode::Boolean(boolean) = &mut group.children[4] {
            boolean.operation = "difference".into();
            boolean.children.swap(0, 1);
        }
    }
    variants.push(value);
    let mut value = baseline.clone();
    if let DrawingLayerNode::Group(group) = &mut value {
        if let DrawingLayerNode::Trace(trace) = &mut group.children[5] {
            trace.source_key = "other-trace-source".into();
            trace.params.threshold = 0.25;
        }
    }
    variants.push(value);

    for variant in variants {
        assert_ne!(create_digest(variant), baseline_digest, "every Drawing layer scalar, style, geometry, order, and asset reference changes the SHA-256 semantic authority");
    }

    let id = semio_framework_value::paged::PagedUtf8::<{usize::MAX}>::from("layer");
    let all_payloads = [
        DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: id.clone(), visible: false }),
        DrawingMutation::SetLayerLocked(SetLayerLocked { layer_id: id.clone(), locked: true }),
        DrawingMutation::SetLayerOpacity(SetLayerOpacity { layer_id: id.clone(), opacity: 0.25 }),
        DrawingMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id: id.clone(), blend_mode: "screen".into() }),
        DrawingMutation::RenameLayer(RenameLayer { layer_id: id.clone(), new_name: "renamed".into() }),
        DrawingMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: id.clone(), transform: crate::DrawingTransform { x: 1.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 5.0, shear: 0.0 } }),
        DrawingMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: id.clone(), fill: Some(FillStyle::Solid { color: [0.1, 0.2, 0.3, 0.4] }) }),
        DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: id.clone(), stroke: Some(StrokeStyle { color: [0.1, 0.2, 0.3, 0.4], width: 2.0, cap: crate::StrokeCap::Round, join: crate::StrokeJoin::Bevel, dash: Some(vec![1.0].into()) }) }),
        DrawingMutation::SetLayerBooleanOperation(SetLayerBooleanOperation { layer_id: id.clone(), boolean_operation: "intersection".into() }),
        DrawingMutation::UpdateLayerTraceParams(UpdateLayerTraceParams { layer_id: id.clone(), params: crate::DrawingTraceParams { threshold: 0.25, simplify_epsilon: 0.5 } }),
        DrawingMutation::CreateLayer(CreateLayer { parent_id: Some("parent".into()), index: Some(2), layer: Box::new(baseline.clone()) }),
        DrawingMutation::DuplicateLayer(DuplicateLayer { identities: vec![crate::schema::identity::DrawingIdentityAssignment{source:(id.clone()),target:"copied-layer".into()}].into(), layer_id: id.clone() }),
        DrawingMutation::DeleteLayer(DeleteLayer { layer_id: id.clone() }),
        DrawingMutation::ReorderLayer(ReorderLayer { layer_id: id, parent_id: Some("parent".into()), index: 3 }),
    ];
    let mut digests = std::collections::HashSet::new();
    for payload in all_payloads {
        assert!(digests.insert(digest(&payload).expect("all fourteen Drawing mutation payloads hash distinctly")));
        drain_mutation(payload);
    }
    assert_mutation_digest_distinct(DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: "layer".into(), visible: false }), DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: "layer".into(), visible: true }));
    assert_mutation_digest_distinct(DrawingMutation::SetLayerLocked(SetLayerLocked { layer_id: "layer".into(), locked: false }), DrawingMutation::SetLayerLocked(SetLayerLocked { layer_id: "layer".into(), locked: true }));
    assert_mutation_digest_distinct(DrawingMutation::SetLayerFillRule(SetLayerFillRule {layer_id:"layer".into(),fill_rule:crate::FillRule::Evenodd}),DrawingMutation::SetLayerFillRule(SetLayerFillRule {layer_id:"layer".into(),fill_rule:crate::FillRule::Nonzero}));
    assert_mutation_digest_distinct(DrawingMutation::SetLayerOpacity(SetLayerOpacity { layer_id: "layer".into(), opacity: 0.25 }), DrawingMutation::SetLayerOpacity(SetLayerOpacity { layer_id: "layer".into(), opacity: 0.5 }));
    assert_mutation_digest_distinct(
        DrawingMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id: "layer".into(), blend_mode: "multiply".into() }),
        DrawingMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id: "layer".into(), blend_mode: "screen".into() }),
    );
    assert_mutation_digest_distinct(DrawingMutation::RenameLayer(RenameLayer { layer_id: "layer".into(), new_name: "left".into() }), DrawingMutation::RenameLayer(RenameLayer { layer_id: "layer".into(), new_name: "right".into() }));
    assert_mutation_digest_distinct(
        DrawingMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: "layer".into(), transform: crate::DrawingTransform { x: 1.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 5.0, shear: 0.0 } }),
        DrawingMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: "layer".into(), transform: crate::DrawingTransform { x: 6.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 5.0, shear: 0.0 } }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: "layer".into(), transform: crate::DrawingTransform { x:1.0,y:2.0,scale_x:3.0,scale_y:4.0,rotation:5.0,shear:0.0 } }),
        DrawingMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: "layer".into(), transform: crate::DrawingTransform { x:1.0,y:2.0,scale_x:3.0,scale_y:4.0,rotation:5.0,shear:0.5 } }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: "layer".into(), fill: None }),
        DrawingMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: "layer".into(), fill: Some(FillStyle::Solid { color: [0.1, 0.2, 0.3, 0.4] }) }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: "layer".into(), fill: Some(FillStyle::LinearGradient { x1: 0.0, y1: 1.0, x2: 2.0, y2: 3.0, stops: vec![GradientStop { offset: 0.5, color: [0.1, 0.2, 0.3, 0.4] }].into() }) }),
        DrawingMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: "layer".into(), fill: Some(FillStyle::LinearGradient { x1: 9.0, y1: 1.0, x2: 2.0, y2: 3.0, stops: vec![GradientStop { offset: 0.75, color: [0.1, 0.2, 0.8, 0.4] }].into() }) }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: "layer".into(), stroke: None }),
        DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: "layer".into(), stroke: Some(StrokeStyle { color: [0.1, 0.2, 0.3, 0.4], width: 1.0, cap: crate::StrokeCap::Round, join: crate::StrokeJoin::Bevel, dash: Some(vec![1.0].into()) }) }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: "layer".into(), stroke: Some(StrokeStyle { color: [0.1, 0.2, 0.3, 0.4], width: 1.0, cap: crate::StrokeCap::Round, join: crate::StrokeJoin::Bevel, dash: Some(vec![1.0].into()) }) }),
        DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: "layer".into(), stroke: Some(StrokeStyle { color: [0.9, 0.2, 0.3, 0.4], width: 2.0, cap: crate::StrokeCap::Square, join: crate::StrokeJoin::Round, dash: Some(vec![2.0].into()) }) }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::SetLayerBooleanOperation(SetLayerBooleanOperation { layer_id: "layer".into(), boolean_operation: "union".into() }),
        DrawingMutation::SetLayerBooleanOperation(SetLayerBooleanOperation { layer_id: "layer".into(), boolean_operation: "difference".into() }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::UpdateLayerTraceParams(UpdateLayerTraceParams { layer_id: "layer".into(), params: crate::DrawingTraceParams { threshold: 0.25, simplify_epsilon: 0.5 } }),
        DrawingMutation::UpdateLayerTraceParams(UpdateLayerTraceParams { layer_id: "layer".into(), params: crate::DrawingTraceParams { threshold: 0.75, simplify_epsilon: 1.5 } }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::CreateLayer(CreateLayer { parent_id: None, index: None, layer: Box::new(baseline.clone()) }),
        DrawingMutation::CreateLayer(CreateLayer { parent_id: Some("parent".into()), index: Some(1), layer: Box::new(baseline.clone()) }),
    );
    assert_mutation_digest_distinct(DrawingMutation::DuplicateLayer(DuplicateLayer { identities: vec![crate::schema::identity::DrawingIdentityAssignment{source:("left".into()),target:"copied-layer".into()}].into(), layer_id: "left".into() }), DrawingMutation::DuplicateLayer(DuplicateLayer { identities: vec![crate::schema::identity::DrawingIdentityAssignment{source:("right".into()),target:"copied-layer".into()}].into(), layer_id: "right".into() }));
    assert_mutation_digest_distinct(DrawingMutation::DeleteLayer(DeleteLayer { layer_id: "left".into() }), DrawingMutation::DeleteLayer(DeleteLayer { layer_id: "right".into() }));
    assert_mutation_digest_distinct(
        DrawingMutation::ReorderLayer(ReorderLayer { layer_id: "layer".into(), parent_id: None, index: 0 }),
        DrawingMutation::ReorderLayer(ReorderLayer { layer_id: "layer".into(), parent_id: Some("parent".into()), index: 1 }),
    );
    drain_snapshot(DrawingSnapshot { layers: vec![baseline].into(), ..crate::standards::v1::subsets::any::schema::default_drawing_document("digest-owner", None) });
}

#[test]
fn retained_drawing_workset_admits_exact_field_page_and_keeps_document_mutation_arena_and_clone_bounds_independent() {
    let mut exact_source = nested_snapshot();
    let exact_owner = exact_native_layer_backing(&source.layers);
    let exact_target = match exact_source.layers.last().expect("Drawing exact-boundary group") {
        DrawingLayerNode::Group(group) => crate::schema::layer_id(&group.children[0]).clone(),
        _ => unreachable!("Drawing exact-boundary group remains exact"),
    };
    let exact = DrawingMutation::RenameLayer(RenameLayer { layer_id: exact_target, new_name: "x".repeat(DRAWING_OWNED_FIELD_BYTES).into() });
    let workset = live_workset(&mut exact_source, &exact).expect("an exact field page obtains its independent fixed workset");
    let (configured_arena_items, configured_arena_bytes) = DrawingMutationArenaOwner::configured_totals().expect("the fixed arena has one derived owner claim");
    assert_eq!((workset.arena_items, workset.arena_bytes), (configured_arena_items, configured_arena_bytes), "one arena owner is charged exactly once");
    assert_eq!((workset.clone_items, workset.clone_bytes), (0, 0), "an overlay-only rename does not invent clone credit");
    assert_eq!(workset.workset_items().expect("workset item total"), workset.arena_items + workset.authority_items);
    assert_eq!(workset.workset_bytes().expect("workset byte total"), workset.arena_bytes + workset.authority_bytes);
    assert_eq!(
        workset.architectural_maximum_bytes().expect("derived architectural maximum"),
        DRAWING_MAXIMUM_NESTED_BYTES * 2 + DRAWING_OWNED_FIELD_BYTES + workset.arena_bytes + workset.authority_bytes,
        "the simultaneous maximum is derived from 2D + P + A + Q",
    );
    assert!(workset.simultaneous_bytes().expect("actual simultaneous ownership") <= workset.architectural_maximum_bytes().expect("derived architectural maximum"));
    let exact_source = apply(exact_source, &exact).expect("an exact 4096-byte retained overlay page is admitted");
    assert_eq!(exact_native_layer_backing(&source.layers), exact_owner, "exact boundary publication retains the source native container owner");
    drain_mutation(exact);
    drain_snapshot(exact_source);

    let plus_source = nested_snapshot();
    let plus_owner = native_layer_backing(&plus_source.layers);
    let plus_target = match plus_source.layers.last().expect("Drawing +1 group") {
        DrawingLayerNode::Group(group) => crate::schema::layer_id(&group.children[0]).clone(),
        _ => unreachable!("Drawing +1 group remains exact"),
    };
    let plus_one = DrawingMutation::RenameLayer(RenameLayer { layer_id: plus_target, new_name: "x".repeat(DRAWING_OWNED_FIELD_BYTES + 1).into() });
    let (plus_source, error) = apply(plus_source, &plus_one).expect_err("4096 +1 retained overlay page rejects");
    assert_eq!(error, "drawing-store.mutation-field-capacity");
    assert_eq!(native_layer_backing(&plus_source.layers), plus_owner, "+1 rejection returns the exact source owner without partial publication");
    drain_mutation(plus_one);
    drain_snapshot(plus_source);

    let source = document_over_byte_bound_snapshot();
    let target = crate::schema::layer_id(&source.layers[0]).clone();
    let source_owner = native_layer_backing(&source.layers);
    let mutation = DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: target, visible: false });
    let (source, error) = apply(source, &mutation).expect_err("a D + 1 source rejects independently of a small mutation");
    assert_eq!(error, "drawing-store.preflight-byte-capacity");
    assert_eq!(native_layer_backing(&source.layers), source_owner, "document rejection returns the exact source authority before overlay handoff");
    drain_mutation(mutation);
    drain_snapshot(source);
}

#[test]
fn retained_drawing_duplicate_plans_exact_clone_work_before_overlay_and_source_handoff() {
    let mut source = nested_snapshot();
    let source_owner = native_layer_backing(&source.layers);
    let target = match source.layers.last().expect("Drawing clone-plan group") {
        DrawingLayerNode::Group(group) => group.base.id.clone(),
        _ => unreachable!("Drawing clone-plan group remains exact"),
    };
    let mutation = DrawingMutation::DuplicateLayer(DuplicateLayer { identities: duplicate_assignments(&source,&target), layer_id: target });
    let (workset, actual_clone) = planned_clone_workset(&mut source, &mutation).expect("duplicate clone construction and census finish before binding the overlay");
    assert_eq!(native_layer_backing(&source.layers), source_owner, "clone planning retains the exact source owner");
    assert_eq!((workset.clone_items, workset.clone_bytes), (actual_clone.items, actual_clone.bytes), "workset clone credit equals the retained clone traversal's actual capacities");
    assert!(workset.clone_items > 0 && workset.clone_bytes > size_of::<DrawingLayerCloneAuthority>(), "a nested duplicate carries real subtree backing in addition to its clone cursor");
    assert_eq!(workset.workset_items().expect("duplicate workset items"), workset.arena_items + workset.authority_items + workset.clone_items);
    assert_eq!(workset.workset_bytes().expect("duplicate workset bytes"), workset.arena_bytes + workset.authority_bytes + workset.clone_bytes);

    let initial_layers = source.layers.len();
    let original_id = crate::schema::layer_id(source.layers.last().expect("original duplicate source")).to_string();
    let source = apply(source, &mutation).expect("a planned nested duplicate applies");
    assert_eq!(source.layers.len(), initial_layers + 1);
    assert_ne!(crate::schema::layer_id(source.layers.last().expect("published duplicate")), original_id.as_str());
    drain_mutation(mutation);
    drain_snapshot(source);
}

#[test]
fn drawing_mutation_admission_fixture_matches_native_dispositions_and_serde_json_carriers() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧮️mutation-admission/🔣️.json")).expect("neutral Drawing mutation-admission fixture parses through serde_json 1");
    assert_eq!(fixture["schema"], "drawing.mutation-admission/v1");
    assert_eq!(fixture["limits"]["documentBytes"].as_u64(), Some(DRAWING_MAXIMUM_NESTED_BYTES as u64));
    assert_eq!(fixture["limits"]["fieldBytes"].as_u64(), Some(DRAWING_OWNED_FIELD_BYTES as u64));
    let cases = fixture["cases"].as_array().expect("neutral admission cases");
    let case = |id: &str| cases.iter().find(|value| value["id"] == id).expect("declared neutral admission case");
    assert_eq!(case("exact-field-page")["expected"]["workClasses"], serde_json::json!(["fixed-arena", "overlay-page"]));
    assert_eq!(case("field-page-plus-one")["expected"]["fault"], "drawing-store.mutation-field-capacity");
    assert_eq!(case("source-document-plus-one")["expected"]["fault"], "drawing-store.preflight-byte-capacity");
    assert_eq!(case("duplicate-subtree")["expected"]["workClasses"], serde_json::json!(["fixed-arena", "clone-subtree"]));

    let exact_source = nested_snapshot();
    let exact_target = match exact_source.layers.last().expect("neutral exact-field group") {
        DrawingLayerNode::Group(group) => crate::schema::layer_id(&group.children[0]).clone(),
        _ => unreachable!("neutral exact-field group remains exact"),
    };
    let exact_mutation = DrawingMutation::RenameLayer(RenameLayer { layer_id: exact_target, new_name: "x".repeat(DRAWING_OWNED_FIELD_BYTES).into() });
    let exact_source = apply(exact_source, &exact_mutation).expect("neutral exact field-page disposition is applied");
    let exact_carrier: serde_json::Value = serde_json::from_str(&serde_json::to_string(&exact_source).expect("serde_json carrier writes exact-field result")).expect("serde_json carrier reads exact-field result");
    assert!(exact_carrier.to_string().contains(&format!("\"{}\"", "x".repeat(DRAWING_OWNED_FIELD_BYTES))), "third-party carrier exposes the changed layer name");
    drain_mutation(exact_mutation);
    drain_snapshot(exact_source);

    let plus_source = nested_snapshot();
    let plus_before = serde_json::to_value(&plus_source).expect("serde_json carrier records +1 source");
    let plus_target = match plus_source.layers.last().expect("neutral +1 group") {
        DrawingLayerNode::Group(group) => crate::schema::layer_id(&group.children[0]).clone(),
        _ => unreachable!("neutral +1 group remains exact"),
    };
    let plus_mutation = DrawingMutation::RenameLayer(RenameLayer { layer_id: plus_target, new_name: "x".repeat(DRAWING_OWNED_FIELD_BYTES + 1).into() });
    let (plus_source, plus_fault) = apply(plus_source, &plus_mutation).expect_err("neutral field-page +1 disposition is rejected");
    assert_eq!(plus_fault, case("field-page-plus-one")["expected"]["fault"].as_str().expect("neutral +1 fault"));
    assert_eq!(serde_json::to_value(&plus_source).expect("serde_json carrier records returned +1 source"), plus_before);
    drain_mutation(plus_mutation);
    drain_snapshot(plus_source);

    let bounded_source = document_over_byte_bound_snapshot();
    let bounded_target = crate::schema::layer_id(&bounded_source.layers[0]).clone();
    let bounded_before = serde_json::to_value(&bounded_source).expect("serde_json carrier records D + 1 source");
    let bounded_mutation = DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: bounded_target, visible: false });
    let (bounded_source, bounded_fault) = apply(bounded_source, &bounded_mutation).expect_err("neutral D + 1 disposition is rejected");
    assert_eq!(bounded_fault, case("source-document-plus-one")["expected"]["fault"].as_str().expect("neutral source fault"));
    assert_eq!(serde_json::to_value(&bounded_source).expect("serde_json carrier records returned D + 1 source"), bounded_before);
    drain_mutation(bounded_mutation);
    drain_snapshot(bounded_source);

    let duplicate_source = nested_snapshot();
    let initial_layers = duplicate_source.layers.len();
    let duplicate_target = crate::schema::layer_id(duplicate_source.layers.last().expect("neutral duplicate source")).clone();
    let duplicate_mutation = DrawingMutation::DuplicateLayer(DuplicateLayer { identities: duplicate_assignments(&duplicate_source,&duplicate_target), layer_id: duplicate_target.clone() });
    let duplicate_source = apply(duplicate_source, &duplicate_mutation).expect("neutral duplicate disposition is applied");
    let duplicate_carrier: serde_json::Value = serde_json::from_str(&serde_json::to_string(&duplicate_source).expect("serde_json carrier writes duplicate result")).expect("serde_json carrier reads duplicate result");
    assert_eq!(duplicate_carrier["layers"].as_array().expect("carrier layers").len(), initial_layers + 1);
    assert_ne!(crate::schema::layer_id(duplicate_source.layers.last().expect("neutral published duplicate")), &duplicate_target);
    drain_mutation(duplicate_mutation);
    drain_snapshot(duplicate_source);
}

#[test]
fn retained_drawing_duplicate_hash_frames_domain_id_and_name_lengths_without_concatenation_collision() {
    fn duplicate_id(id: &str, name: &str) -> String {
        let mut source = crate::standards::v1::subsets::any::schema::default_drawing_document("duplicate-framing", None);
        let mut layer = crate::standards::v1::subsets::any::schema::create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit(((name)).to_string().into()).expect("nonempty authored identity"), name, Default::default());
        let base = crate::schema::layer_base_mut(&mut layer);
        base.id.clear();
        base.id.push_str(id);
        admit_layer_string_destinations(&mut layer);
        source.layers.clear();
        reserve_native_layer_slots(&mut source.layers, 2);
        source.layers.push(layer);
        let mutation = DrawingMutation::DuplicateLayer(DuplicateLayer { identities: duplicate_assignments(&source,&semio_framework_value::paged::PagedUtf8::<{usize::MAX}>::from(id)), layer_id: id.into() });
        let source = apply(source, &mutation).expect("framed duplicate mutation applies");
        let duplicate = source.layers.get(1).map(crate::schema::layer_id).expect("duplicated layer remains retained").to_string();
        drain_mutation(mutation);
        drain_snapshot(source);
        duplicate
    }

    assert_ne!(duplicate_id("ab", "c"), duplicate_id("a", "bc"), "separate id/name length frames prevent concatenation collisions");
}

#[test]
fn retained_drawing_duplicate_name_uses_preadmitted_page_and_returns_exact_rejection_owner() {
    let mut source = crate::standards::v1::subsets::any::schema::default_drawing_document("duplicate-name-owner", None);
    let mut layer = crate::standards::v1::subsets::any::schema::create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("Layer")).to_string().into()).expect("nonempty authored identity"), "Layer", Default::default());
    admit_layer_string_destinations(&mut layer);
    let target = crate::schema::layer_id(&layer).clone();
    let original_name_owner = native_text_backing(&crate::schema::layer_base(&layer).name);
    source.layers.clear();
    reserve_native_layer_slots(&mut source.layers, 2);
    source.layers.push(layer);
    let mutation = DrawingMutation::DuplicateLayer(DuplicateLayer { identities: duplicate_assignments(&source,&target), layer_id: target });
    let source = apply(source, &mutation).expect("duplicate name suffix uses only pre-admitted destination and fixed scratch page");
    assert_eq!(native_text_backing(&crate::schema::layer_base(&source.layers[0]).name), original_name_owner, "last-valid name backing remains exact");
    assert_eq!(crate::schema::layer_base(&source.layers[1]).name, "Layer copy");
    drain_mutation(mutation);
    drain_snapshot(source);

    let mut rejected = crate::standards::v1::subsets::any::schema::default_drawing_document("duplicate-name-rejected", None);
    let layer = crate::standards::v1::subsets::any::schema::create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("Layer")).to_string().into()).expect("nonempty authored identity"), "Layer", Default::default());
    let target = crate::schema::layer_id(&layer).clone();
    rejected.layers = vec![layer].into();
    let exact_owner = native_layer_backing(&rejected.layers);
    let mutation = DrawingMutation::DuplicateLayer(DuplicateLayer { identities: duplicate_assignments(&rejected,&target), layer_id: target });
    let (rejected, error) = apply(rejected, &mutation).expect_err("unadmitted duplicate destination rejects without allocating after operation admission");
    assert_eq!(error, "drawing-store.duplicate-destination-capacity");
    assert_eq!(native_layer_backing(&rejected.layers), exact_owner, "duplicate rejection returns the exact source native container owner");
    drain_mutation(mutation);
    drain_snapshot(rejected);
}

#[test]
fn retained_drawing_cancel_stale_each_precommit_replay_candidate_container_stage_preserves_last_valid() {
    let stages = [
        DrawingMutationCandidatePhase::PreflightSource,
        DrawingMutationCandidatePhase::PreflightMutation,
        DrawingMutationCandidatePhase::LocateCloneSource,
        DrawingMutationCandidatePhase::PrepareOwnedValue,
        DrawingMutationCandidatePhase::PlanOwnedValue,
        DrawingMutationCandidatePhase::BindOverlay,
        DrawingMutationCandidatePhase::LocatePrimary,
        DrawingMutationCandidatePhase::LocateSecondary,
        DrawingMutationCandidatePhase::Apply,
        DrawingMutationCandidatePhase::RebuildSource,
        DrawingMutationCandidatePhase::LocateDestination,
        DrawingMutationCandidatePhase::RebuildDestination,
    ];
    for stage in stages {
        for stale in [false, true] {
            let mut source = nested_snapshot();
            let (group_id, target) = match source.layers.last().expect("Drawing group") {
                DrawingLayerNode::Group(group) => (group.base.id.clone(), crate::schema::layer_id(&group.children[0]).clone()),
                _ => unreachable!("Drawing fixture group remains exact"),
            };
            let last_valid_id = source.id.clone();
            let mutation = match stage {
                DrawingMutationCandidatePhase::LocateSecondary => DrawingMutation::CreateLayer(CreateLayer { parent_id: Some(group_id), index: Some(0), layer: Box::new(crate::standards::v1::subsets::any::schema::create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("cancel-create")).to_string().into()).expect("nonempty authored identity"), "cancel-create", Default::default())) }),
                DrawingMutationCandidatePhase::LocatePrimary => DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: target, visible: false }),
                DrawingMutationCandidatePhase::RebuildSource | DrawingMutationCandidatePhase::LocateDestination => DrawingMutation::ReorderLayer(ReorderLayer { layer_id: target, parent_id: Some(group_id), index: 2 }),
                _ => DrawingMutation::DuplicateLayer(DuplicateLayer { identities: duplicate_assignments(&source,&target), layer_id: target }),
            };
            let operation = semio_framework_job::OperationId(8_003);
            let generation = semio_framework_job::Generation(83);
            let mut authority = borrowed_candidate(operation, generation).expect("Drawing candidate fixed owner arenas admit");
            let cancel = semio_framework_job::root_cancel_token();
            let mut preview_sequence = 0;
            for _ in 0..100_000 {
                if authority.phase == stage {
                    break;
                }
                let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
                assert!(!authority.step(&mut source, &mutation, &mut context).expect("Drawing stage fixture advances"));
            }
            assert_eq!(authority.phase, stage, "fixture reaches each replay/candidate/container stage exactly");
            if !stale {
                cancel.cancel_now();
            }
            let mut rejected = semio_framework_job::StepContext::new(
                operation,
                if stale { semio_framework_job::Generation(84) } else { generation },
                semio_framework_job::StepBudget::new(1, u64::MAX),
                cancel,
                semio_framework_job::default_now_us,
                &mut preview_sequence,
            );
            assert_eq!(authority.step(&mut source, &mutation, &mut rejected), Err(if stale { "drawing-store.mutation-candidate-stale-authority" } else { "drawing-store.mutation-candidate-cancelled" }),);
            close_candidate(&mut authority, Some(&mut source));
            drop(authority);
            assert_eq!(source.id, last_valid_id, "cancel/stale close never publishes a partial candidate");
            drain_mutation(mutation);
            drain_snapshot(source);
        }
    }
}

#[test]
fn retained_drawing_committed_candidate_finishes_exact_owner_return_after_late_cancel_or_stale_context() {
    for stale in [false, true] {
        let mut source = nested_snapshot();
        let initial_layers = source.layers.len();
        let target = crate::schema::layer_id(source.layers.last().expect("Drawing duplicate source")).clone();
        let mutation = DrawingMutation::DuplicateLayer(DuplicateLayer { identities: duplicate_assignments(&source,&target), layer_id: target });
        let operation = semio_framework_job::OperationId(8_004);
        let generation = semio_framework_job::Generation(84);
        let mut authority = borrowed_candidate(operation, generation).expect("Drawing candidate fixed owner arenas admit");
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        for _ in 0..100_000 {
            if authority.phase == DrawingMutationCandidatePhase::Complete {
                break;
            }
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
            assert!(!authority.step(&mut source, &mutation, &mut context).expect("Drawing candidate reaches its committed phase"));
        }
        assert_eq!(authority.phase, DrawingMutationCandidatePhase::Complete);
        if !stale {
            cancel.cancel_now();
        }
        let terminal_generation = if stale { semio_framework_job::Generation(generation.0 + 1) } else { generation };
        let mut terminal = false;
        for _ in 0..100_000 {
            let mut context = semio_framework_job::StepContext::new(operation, terminal_generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
            if authority.step(&mut source, &mutation, &mut context).expect("a committed candidate finishes exact arena return despite a late context change") {
                terminal = true;
                break;
            }
        }
        assert!(terminal, "committed Drawing candidate reaches its terminal witness");
        authority.take().expect("committed Drawing candidate exact terminal owner handoff");
        assert!(authority.terminal_is_empty());
        assert_eq!(source.layers.len(), initial_layers + 1, "late cancellation or staleness cannot roll back an already-published duplicate");
        drop(authority);
        drain_mutation(mutation);
        drain_snapshot(source);
    }
}

#[test]
fn retained_path_geometry_mutation_preserves_appearance_and_retires_segments() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧬️schema/🧬️mutations/✏️update-path-geometry/🧫️fixtures/🔣️.json"))).unwrap();
    let before: semio_framework_value::list::PagedList<PathSegment, {usize::MAX}> = serde_json::from_value(fixture["before"].clone()).unwrap();
    let after: semio_framework_value::list::PagedList<PathSegment, {usize::MAX}> = serde_json::from_value(fixture["after"].clone()).unwrap();
    let mut layer = crate::standards::v1::subsets::any::schema::create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("Curve")).to_string().into()).expect("nonempty authored identity"), "Curve", before);
    crate::schema::layer_base_mut(&mut layer).opacity = 0.4;
    let id = crate::schema::layer_id(&layer).clone();
    let source = DrawingSnapshot { layers: vec![layer].into(), ..Default::default() };
    let mutation = crate::mutations::update_path_geometry(id, after.clone());
    let result = apply(source, &mutation).expect("retained geometry mutation applies");
    let DrawingLayerNode::Path(path) = &result.layers[0] else { panic!("Expected path") };
    assert_eq!(path.segments, after);
    assert_eq!(path.base.opacity, 0.4);
    drain_snapshot(result);
    drain_mutation(mutation);
}

#[test]
fn retained_path_edit_cancellation_preserves_the_entire_document() {
    initialize_drawing_mutation_arena_pool_for_test();
    let layer = crate::standards::v1::subsets::any::schema::create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("Curve")).to_string().into()).expect("nonempty authored identity"), "Curve", vec![PathSegment::Move { to: [0.0,0.0] }, PathSegment::Line { to: [10.0,0.0] }].into());
    let id = crate::schema::layer_id(&layer).clone();
    let mut source = DrawingSnapshot { layers: vec![layer].into(), ..Default::default() };
    let expected = source.clone();
    let mutation = crate::mutations::update_path_geometry(id, vec![PathSegment::Move { to: [5.0,5.0] }, PathSegment::Line { to: [20.0,20.0] }].into());
    let operation = semio_framework_job::OperationId(8_090);
    let generation = semio_framework_job::Generation(90);
    let mut authority = borrowed_candidate(operation,generation).unwrap();
    let cancel = semio_framework_job::root_cancel_token();
    let mut sequence = 0;
    for _ in 0..100_000 {
        if authority.segments_clone.as_ref().is_some_and(|clone| clone.index == 1) { break; }
        let mut context = semio_framework_job::StepContext::new(operation,generation,semio_framework_job::StepBudget::new(1,u64::MAX),cancel.clone(),semio_framework_job::default_now_us,&mut sequence);
        assert!(!authority.step(&mut source,&mutation,&mut context).unwrap());
    }
    assert_eq!(authority.segments_clone.as_ref().unwrap().index,1);
    cancel.cancel_now();
    let mut context = semio_framework_job::StepContext::new(operation,generation,semio_framework_job::StepBudget::new(1,u64::MAX),cancel,semio_framework_job::default_now_us,&mut sequence);
    assert_eq!(authority.step(&mut source,&mutation,&mut context),Err("drawing-store.mutation-candidate-cancelled"));
    close_candidate(&mut authority,Some(&mut source));
    assert_eq!(source,expected);
    drain_snapshot(source);
    drain_snapshot(expected);
    drain_mutation(mutation);
}

#[test]
fn retained_path_geometry_digest_distinguishes_control_points() {
    assert_mutation_digest_distinct(
        crate::mutations::update_path_geometry("path".into(),vec![PathSegment::Cubic { ctrl1: [1.0,2.0], ctrl2: [3.0,4.0], to: [5.0,6.0] }].into()),
        crate::mutations::update_path_geometry("path".into(),vec![PathSegment::Cubic { ctrl1: [2.0,2.0], ctrl2: [3.0,4.0], to: [5.0,6.0] }].into()),
    );
}

#[test]
fn retained_text_edit_preserves_identity_and_appearance() {
    initialize_drawing_mutation_arena_pool_for_test();
    let mut layer = crate::schema::create_drawing_text_layer(crate::schema::identity::DrawingIdentity::admit((("Caption")).to_string().into()).expect("nonempty authored identity"), "Caption");
    crate::schema::layer_base_mut(&mut layer).opacity = 0.4;
    let id = crate::schema::layer_id(&layer).clone();
    let source = DrawingSnapshot { layers: vec![layer].into(), ..Default::default() };
    let mutation = crate::mutations::update_text(id.clone(), "Grüße 🌍\n123".into(), 36.0,crate::DrawingFontFamily::Anta);
    let result = apply(source, &mutation).expect("retained text mutation applies");
    let DrawingLayerNode::Text(text) = &result.layers[0] else { panic!("Expected text") };
    assert_eq!(text.content, "Grüße 🌍\n123");
    assert_eq!(text.size, 36.0);
    assert_eq!(text.base.id, id);
    assert_eq!(text.base.opacity, 0.4);
    drain_snapshot(result);
    drain_mutation(mutation);
}

#[test]
fn retained_text_digest_distinguishes_content_and_size() {
    assert_mutation_digest_distinct(crate::mutations::update_text("text".into(), "A".into(), 24.0,crate::DrawingFontFamily::Anta), crate::mutations::update_text("text".into(), "B".into(), 24.0,crate::DrawingFontFamily::Anta));
    assert_mutation_digest_distinct(crate::mutations::update_text("text".into(), "A".into(), 24.0,crate::DrawingFontFamily::Anta), crate::mutations::update_text("text".into(), "A".into(), 36.0,crate::DrawingFontFamily::Anta));
}

#[test]
fn retained_text_edit_cancellation_keeps_the_complete_document() {
    initialize_drawing_mutation_arena_pool_for_test();
    let layer = crate::schema::create_drawing_text_layer(crate::schema::identity::DrawingIdentity::admit((("Caption")).to_string().into()).expect("nonempty authored identity"), "Caption");
    let id = crate::schema::layer_id(&layer).clone();
    let mut source = DrawingSnapshot { layers: vec![layer].into(), ..Default::default() };
    let expected = source.clone();
    let mutation = crate::mutations::update_text(id, "😀🙂Grüße".into(), 36.0,crate::DrawingFontFamily::Anta);
    let operation = semio_framework_job::OperationId(8_091);
    let generation = semio_framework_job::Generation(91);
    let mut authority = borrowed_candidate(operation, generation).unwrap();
    let cancel = semio_framework_job::root_cancel_token();
    let mut sequence = 0;
    for _ in 0..100_000 {
        if authority.text_clone.as_ref().is_some_and(|clone| clone.index == 4) { break; }
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut sequence);
        assert!(!authority.step(&mut source, &mutation, &mut context).unwrap());
    }
    assert_eq!(authority.text_clone.as_ref().unwrap().index, 4);
    cancel.cancel_now();
    let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel, semio_framework_job::default_now_us, &mut sequence);
    assert_eq!(authority.step(&mut source, &mutation, &mut context), Err("drawing-store.mutation-candidate-cancelled"));
    close_candidate(&mut authority, Some(&mut source));
    assert_eq!(source, expected);
    drain_snapshot(source);
    drain_snapshot(expected);
    drain_mutation(mutation);
}

#[test]
fn isolation_mutation_digest_observes_the_boolean() {
    assert_mutation_digest_distinct(crate::mutations::set_group_isolation("group".into(),false),crate::mutations::set_group_isolation("group".into(),true));
}

#[test]
fn retained_blend_mutations_validate_vocabulary_and_return_unchanged_rejections() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧫️fixtures/🎛️field-patch/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap().iter().filter(|case| case["patch"]["field"] == "blendMode") {
        let source = nested_snapshot();
        let id = crate::schema::layer_id(&source.layers[0]).to_owned();
        let mode = case["patch"]["value"].as_str().unwrap();
        let operation = crate::mutations::set_layer_blend_mode(id,mode.into());
        let result = apply(nested_snapshot(),&operation);
        if case["accepted"] == true {
            let actual = result.expect("canonical mode applies");
            let mut expected = source.clone();
            crate::schema::layer_base_mut(&mut expected.layers[0]).blend_mode = mode.into();
            assert_eq!(actual,expected);
            drain_snapshot(actual);drain_snapshot(expected);
        } else {
            let (actual,error) = result.expect_err("invalid mode is refused");
            assert_eq!(error,"drawing-store.mutation-blend-mode-invalid");
            assert_eq!(actual,source);
            drain_snapshot(actual);
        }
        drain_snapshot(source);drain_mutation(operation);
    }
}

#[test]
fn retained_authored_shape_and_image_facets_match_sparse_diffs_and_digest_all_arguments() {
    use crate::schema::shape_geometry::ShapeCoordinateField;
    let shape=crate::schema::create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Shape")).to_string().into()).expect("nonempty authored identity"), "Shape");let id=crate::schema::layer_id(&shape).clone();
    let before=DrawingSnapshot{layers:vec![shape].into(),..Default::default()};
    let value=apply(before,&crate::mutations::set_shape_coordinate(id,ShapeCoordinateField::RectWidth,None,64.0)).unwrap();
    let DrawingLayerNode::Shape(shape)=&value.layers[0] else{panic!()};assert_eq!(shape.rect.as_ref().unwrap().width,64.0);drain_snapshot(value);
    let image=crate::schema::create_drawing_image_layer(crate::schema::identity::DrawingIdentity::admit((("Image")).to_string().into()).expect("nonempty authored identity"), "Image","before");let id=crate::schema::layer_id(&image).clone();
    let before=DrawingSnapshot{layers:vec![image].into(),..Default::default()};
    let value=apply(before,&crate::mutations::update_image(id,"after".into(),64.0,32.0)).unwrap();
    let DrawingLayerNode::Image(image)=&value.layers[0] else{panic!()};assert_eq!(image.image_key,"after");assert_eq!((image.width,image.height),(64.0,32.0));drain_snapshot(value);
    assert_mutation_digest_distinct(crate::mutations::update_image("image".into(),"before".into(),64.0,32.0),crate::mutations::update_image("image".into(),"after".into(),64.0,32.0));
    assert_mutation_digest_distinct(crate::mutations::update_image("image".into(),"key".into(),64.0,32.0),crate::mutations::update_image("image".into(),"key".into(),65.0,32.0));
    assert_mutation_digest_distinct(crate::mutations::update_image("image".into(),"key".into(),64.0,32.0),crate::mutations::update_image("image".into(),"key".into(),64.0,33.0));
    assert_mutation_digest_distinct(crate::mutations::set_shape_coordinate("shape".into(),ShapeCoordinateField::RectX,None,1.0),crate::mutations::set_shape_coordinate("shape".into(),ShapeCoordinateField::RectY,None,1.0));
    assert_mutation_digest_distinct(crate::mutations::set_shape_coordinate("shape".into(),ShapeCoordinateField::PolygonX,Some(1),1.0),crate::mutations::set_shape_coordinate("shape".into(),ShapeCoordinateField::PolygonX,Some(2),1.0));
    assert_mutation_digest_distinct(crate::mutations::set_shape_coordinate("shape".into(),ShapeCoordinateField::RectX,None,1.0),crate::mutations::set_shape_coordinate("shape".into(),ShapeCoordinateField::RectX,None,2.0));
}

fn duplicate_assignments(document:&DrawingSnapshot,target:&DrawingNativeText)->semio_framework_value::list::PagedList<crate::schema::identity::DrawingIdentityAssignment,{usize::MAX}>{
 let node=crate::schema::find_drawing_layer(document,target).expect("authored duplicate source exists");let mut callback=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(1024*1024,&mut callback);
 crate::standards::v1::subsets::any::io::text::identity::clone::admit_clone_identities(node," copy",&mut control).expect("fixture duplicate identity admission")
}
