pub(crate) mod retirement {
    use crate::standards::v1::subsets::any::schema::snapshot::RasterSnapshot;

    pub(crate) fn retire_raster_snapshot(snapshot: RasterSnapshot) {
        crate::host::owned::retire_raster_value_cold(snapshot);
    }
}

use crate::standards::v1::subsets::any::io::binary::mutations::*;
use crate::mutations::{add_layer_asset, change_layer_adjustment_kind, change_layer_blend_mode, change_layer_opacity, change_layer_visible, create_layer, delete_layer, move_layer, remove_layer_asset, rename_layer, reorder_layers, resize_layer};
use crate::standards::v1::subsets::any::io::text::snapshot::empty_raster_document;
use crate::{RasterAssetChild, RasterLayerNode, RasterOwnedMap, RasterOwnedMapInsert, RasterSnapshot, RasterTransform, RASTER_DOCUMENT_SCHEMA};
use crate::host::owned::{raster_document_store_owners, retire_raster_value_cold, RasterOneItemApply};

/// ⛽️ The step grant every retained candidate law drives under: ample items and credits for a bounded apply.
const RASTER_TEST_FUEL: u64 = 64;

fn raster_test_wallet() -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 4_096, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 64 }
}

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let document = empty_raster_document();
    let operation = RasterMutation::CreateLayer(create_layer::mutation::CreateLayer {
        parent_id: None,
        index: document.layers.len(),
        layer: Box::new(RasterLayerNode::Pixel {
            id: "op-binary-test".into(),
            name: "Op Binary Test".into(),
            visible: true, locked: false,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: RasterTransform::default(),
            mask: None,
            width: Some(64),
            height: Some(64),
            image_key: None,
        }),
    });
    store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).await.expect("encode");
    assert_eq!(decode_op(&bytes).await.expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn raster_document_text_round_trips_store_with_applied_operation() {
    use crate::RasterSnapshot;

    let envelope = store::create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "doc-text-test", empty_raster_document(), None);
    let mut store = store::ArtifactStore::new(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    // 🔐️ The history ledger refuses an insertion from a store without its domain owner catalog
    // ("edit history insertion requires its exact mutation retirement factory"): a raster store is
    // built with the artifact's own `raster_document_store_owners`, never bare.
    store.install_document_store_owners_exact(raster_document_store_owners()).map_err(|(error, _)| error).expect("the Raster owner catalog installs on a fresh store");
    store
        .dispatch(store::ArtifactCommand::Apply {
            mutations: vec![RasterMutation::CreateLayer(create_layer::mutation::CreateLayer {
                parent_id: None,
                index: 1,
                layer: Box::new(RasterLayerNode::Adjustment {
                    id: "adjust-text".into(),
                    name: "Levels".into(),
                    visible: true, locked: false,
                    opacity: 1.0,
                    blend_mode: "normal".into(),
                    transform: RasterTransform::default(),
                    adjustment_kind: "levels".into(),
                    params: RasterOwnedMap::new(),
                }),
            })],
            transaction: None,
        })
        .await
        .expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
    store::os_store::test_support::close_plain_test_store(&mut store);
}

/// ▶️ Drives one mutation the way the interactive document lane does (`RasterOneItemApply` over its
/// retained candidate) and hands back the published post snapshot.
fn drive_raster_candidate(base: &RasterSnapshot, operation: &RasterMutation, operation_id: u64) -> RasterSnapshot {
    let operation_id = semio_framework_job::OperationId(operation_id);
    let generation = semio_framework_job::Generation(1);
    let mut apply = RasterOneItemApply::new();
    for _ in 0..200_000 {
        let (post, _receipt) = apply.advance(base, operation, operation_id, generation, RASTER_TEST_FUEL, raster_test_wallet()).expect("retained Raster apply");
        if let Some(post) = post {
            assert!(apply.terminal_is_empty(), "a published candidate has retired every displaced owner");
            return post;
        }
    }
    panic!("retained Raster apply did not reach a bounded terminal")
}

#[test]
fn raster_owned_map_removal_returns_exact_pair_and_populated_drop_refuses() {
    let mut map = RasterOwnedMap::new();
    let key = String::from("exact-key");
    let key_pointer = key.as_ptr();
    map.insert(key, 7_u8).expect("one exact map entry");
    let mut removed = map.remove_entry("exact-key").expect("pair-returning removal");
    let (removed_key, removed_value) = removed.take();
    assert_eq!(removed_key.as_ptr(), key_pointer);
    assert_eq!(removed_value, 7);
    drop(removed_key);
    while let Some(page) = map.take_empty_page_backing() {
        page.release();
    }
    drop(map);

    let result = std::panic::catch_unwind(|| {
        let mut populated = RasterOwnedMap::new();
        populated.insert(String::from("must-retire"), 9_u8).expect("one populated page");
        drop(populated);
    });
    assert!(result.is_err(), "populated Raster map ordinary Drop must fail closed");
}

#[test]
fn retained_mask_keys_and_transforms_survive_clone_and_bounded_retirement() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧬️schema/🧫️fixtures/🎭️mask/🔣️.json")).unwrap();
    for (index,case) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let mask:crate::RasterLayerMask=semio_framework_pack_json::from_json_str(&case["mask"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mut base=empty_raster_document();
        let id=crate::standards::v1::subsets::any::schema::layer_node_id(&base.layers[0]).to_owned();
        if let RasterLayerNode::Pixel {mask:target,..}=&mut base.layers[0] {*target=Some(mask.clone());}
        let children=std::mem::take(&mut base.layers);
        base.layers.push(RasterLayerNode::Group {id:"masked-group".into(),name:"Masked Group".into(),visible:true, locked: false,opacity:1.0,blend_mode:"normal".into(),transform:RasterTransform::default(),mask:Some(mask.clone()),children});
        let operation=RasterMutation::RenameLayer(rename_layer::RenameLayer {layer_id:id,new_name:"Renamed".into()});
        let candidate=drive_raster_candidate(&base,&operation,700+index as u64);
        let RasterLayerNode::Group {mask:group_mask,children,..}=&candidate.layers[0] else {panic!("group missing")};
        assert_eq!(group_mask.as_ref(),Some(&mask));
        let RasterLayerNode::Pixel {mask:pixel_mask,..}=&children[0] else {panic!("pixel missing")};
        assert_eq!(pixel_mask.as_ref(),Some(&mask));
        retirement::retire_raster_snapshot(candidate);retirement::retire_raster_snapshot(base);
    }
}

/// 🖼️ Play-grid boot regression (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP, pane measured
/// blank on :6033 2026-09-21): the interactive document lane applies `add-layer-asset` through this
/// candidate authority, and the composite surface reads its pixels back out of the asset pool
/// through `crate::raster_asset`. The authority minted the handle from a RAW `(mime, data)` digest —
/// disagreeing with the canonical content-addressed id every other route mints — and inserted it
/// WITHOUT the decoded content, so the demo booted with `assetsJson == "{}"`; the snapshot clone
/// every subsequent mutation runs then rebuilt each handle field by field and dropped whatever
/// materialization was left.
#[test]
fn retained_asset_apply_and_snapshot_clone_keep_the_composite_pixels() {
    let asset = crate::standards::v1::subsets::any::io::semio_image_snapshot_from_raster_asset(&crate::examples::art_raster_demo::emblem_image_asset()).expect("fixture image decodes");
    let minted = crate::mint_raster_image_child("semio-emblem", &asset);
    let base = empty_raster_document();
    let layer_id = crate::standards::v1::subsets::any::schema::layer_node_id(&base.layers[0]).to_string();
    let add = RasterMutation::AddLayerAsset(add_layer_asset::mutation::AddLayerAsset { asset_id: "semio-emblem".into(), asset });
    let added = drive_raster_candidate(&base, &add, 880);
    assert_eq!(added.assets.get("semio-emblem").expect("the retained apply inserts the asset child").child_id, minted.child_id, "the retained apply mints this artifact's own canonical content-addressed child id");
    assert!(crate::raster_asset(&added.assets, "semio-emblem").is_some(), "the composite reads its pixels back out of the applied asset pool");

    let rename = RasterMutation::RenameLayer(rename_layer::mutation::RenameLayer { layer_id, new_name: "Backdrop".into() });
    let renamed = drive_raster_candidate(&added, &rename, 881);
    assert!(crate::raster_asset(&renamed.assets, "semio-emblem").is_some(), "every later mutation clones the snapshot and must carry its materialized assets across");
    retirement::retire_raster_snapshot(renamed);
    retirement::retire_raster_snapshot(added);
    retirement::retire_raster_snapshot(base);
}

/// 🧾️ LAW: an `add-layer-asset` whose image is larger than one retirement grant applies AND retires.
/// The byte owner used to answer `Pending { 0, 0 }` for any buffer over the grant, so the store refused
/// every asset over 4 KiB up front (`raster-store.mutation-asset-capacity`) — the curated demo's real
/// 512×512 emblem (25 KiB) was refused on every `setActiveExample` in the live pane (ticket
/// 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP, :6033 2026-09-23). The owner now releases from the tail
/// within each grant, the same contract draw's owned strings follow.
#[test]
fn an_asset_over_one_retirement_grant_applies_and_its_operation_retires_within_every_grant() {
    let asset = crate::standards::v1::subsets::any::io::semio_image_snapshot_from_raster_asset(&crate::examples::art_raster_demo::emblem_image_asset()).expect("fixture image decodes");
    let bytes = asset.frames.iter().map(|frame| frame.rgba8.len()).sum::<usize>();
    assert!(bytes > store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES * 4, "the demo emblem is real media, several grants long: {bytes} B");
    let base = empty_raster_document();
    let add = RasterMutation::AddLayerAsset(add_layer_asset::mutation::AddLayerAsset { asset_id: "semio-emblem".into(), asset });
    let added = drive_raster_candidate(&base, &add, 882);
    assert!(crate::raster_asset(&added.assets, "semio-emblem").is_some(), "the applied pool resolves the emblem's pixels");
    retire_raster_value_cold(add);
    retirement::retire_raster_snapshot(added);
    retirement::retire_raster_snapshot(base);
}

#[test]
fn raster_empty_snapshot_retires_to_terminal_without_hidden_allocation() {
    let snapshot = RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: Vec::new(), assets: RasterOwnedMap::new() };
    retire_raster_value_cold(snapshot);
}

#[test]
fn raster_owned_map_cap_plus_one_returns_exact_owner_and_populated_pages_retire_explicitly() {
    let mut assets = RasterOwnedMap::new();
    for index in 0..crate::RASTER_OWNED_MAP_CAPACITY {
        assets
            .insert(
                format!("asset-{index:02}"),
                store::ArtifactChild::new(
                    format!("child-{index:02}"),
                    semio_framework_artifact_reference::ArtifactRef { artifact_id: format!("artifact-{index:02}"), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } },
                ),
            )
            .expect("fixed Raster map admits its exact item capacity");
    }
    let rejected_key = String::from("asset-overflow");
    let rejected_child_id = String::from("child-overflow");
    let key_pointer = rejected_key.as_ptr();
    let child_pointer = rejected_child_id.as_ptr();
    let rejected = assets
        .insert(
            rejected_key,
            store::ArtifactChild::new(rejected_child_id, semio_framework_artifact_reference::ArtifactRef { artifact_id: "overflow".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } }),
        )
        .expect_err("fixed Raster map rejects capacity plus one");
    assert_eq!(rejected.key.as_ptr(), key_pointer);
    assert_eq!(rejected.value.child_id.as_ptr(), child_pointer);

    let (old_key_pointer, old_child_pointer) = {
        let (key, child) = assets.entry_at(0).expect("first admitted Raster map entry");
        (key.as_ptr(), child.child_id.as_ptr())
    };
    let mut replacement_key = String::with_capacity(64);
    replacement_key.push_str("asset-00");
    let replacement_key_pointer = replacement_key.as_ptr();
    let replacement_child_id = String::from("replacement-child");
    let replacement_child_pointer = replacement_child_id.as_ptr();
    let mut replaced = match assets
        .insert_pre_admitted(
            replacement_key,
            store::ArtifactChild::new(
                replacement_child_id,
                semio_framework_artifact_reference::ArtifactRef { artifact_id: "replacement-artifact".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } },
            ),
        )
        .expect("replacement preserves fixed capacity")
    {
        RasterOwnedMapInsert::Replaced(previous) => previous,
        RasterOwnedMapInsert::Inserted => panic!("replacement returns the exact displaced pair"),
    };
    let (previous_key, previous_child) = replaced.take();
    assert_eq!(previous_key.as_ptr(), old_key_pointer);
    assert_eq!(previous_child.child_id.as_ptr(), old_child_pointer);
    let (installed_key, installed_child) = assets.entry_at(0).expect("replacement remains in stable order");
    assert_eq!(installed_key.as_ptr(), replacement_key_pointer);
    assert_eq!(installed_child.child_id.as_ptr(), replacement_child_pointer);
    retire_raster_value_cold((previous_key, previous_child));

    let snapshot = RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: Vec::new(), assets };
    retire_raster_value_cold(snapshot);
}

#[test]
fn raster_populated_dsl_materialization_max_plus_one_nested_cancel_fault_panic_and_close_are_exact() {
    let mut params = RasterOwnedMap::new();
    let mut first_key_pointer = std::ptr::null();
    for index in 0..crate::RASTER_OWNED_MAP_CAPACITY {
        let key = format!("key-{index:02}");
        if index == 0 {
            first_key_pointer = key.as_ptr();
        }
        let value = semio_framework_value::DslValue::Object(vec![("nested".into(), semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::String(format!("value-{index}")), semio_framework_value::DslValue::Object(vec![("leaf".into(), semio_framework_value::DslValue::uint(index as u64))])]))]);
        params.insert(key, value).expect("maximum populated DSL map remains exactly page admitted");
    }
    assert_eq!(params.len(), crate::RASTER_OWNED_MAP_CAPACITY);
    assert_eq!(params.entry_at(0).expect("first exact map owner remains installed").0.as_ptr(), first_key_pointer);

    let plus_one_key = String::from("key-plus-one");
    let plus_one_key_pointer = plus_one_key.as_ptr();
    let plus_one_value = semio_framework_value::DslValue::String("plus-one-value".into());
    let rejected = params.insert(plus_one_key, plus_one_value).expect_err("capacity plus one returns both exact owners");
    assert_eq!(rejected.key.as_ptr(), plus_one_key_pointer);
    assert_eq!(rejected.reason, "raster-map.item-capacity");

    // 🗂️ The whole-map DSL projection is REAL (the map is FIXED-capacity — 64 entries over 8 pages —
    // so a whole-map projection is bounded by construction and owes no paging authority; see
    // `RasterOwnedMap`'s own doc comment). It used to refuse a populated map outright, which trapped
    // every whole-document route. What this law still pins is the OWNERSHIP half: projecting the map
    // leaves every key, value and page owner exactly where it was.
    let output = semio_framework_dsl_record::DslField::to_value(&params);
    let semio_framework_dsl_record::FieldValue::Map(projected) = &output else { panic!("a populated Raster owned map projects as a map") };
    assert_eq!(projected.len(), crate::RASTER_OWNED_MAP_CAPACITY);
    assert_eq!(params.len(), crate::RASTER_OWNED_MAP_CAPACITY);
    assert_eq!(params.entry_at(0).expect("the projection keeps the first exact key/value/page owner installed").0.as_ptr(), first_key_pointer);
    retire_raster_value_cold((rejected.key, rejected.value));

    // 🗂️ …and the input direction recovers that same real content rather than faulting on it.
    let mut parsed = <RasterOwnedMap<semio_framework_value::DslValue> as semio_framework_dsl_record::DslField>::from_value(&output).expect("a populated Raster owned map parses its own projection");
    assert_eq!(parsed.len(), crate::RASTER_OWNED_MAP_CAPACITY);
    parsed.retire();
    let layer = RasterLayerNode::Adjustment { id: "dsl-output".into(), name: "DSL Output".into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "nested".into(), params };
    let snapshot = RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: vec![layer], assets: RasterOwnedMap::new() };
    retire_raster_value_cold(snapshot);
}

#[test]
fn raster_populated_serde_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact() {
    let mut params = RasterOwnedMap::new();
    let mut first_key_pointer = std::ptr::null();
    for index in 0..crate::RASTER_OWNED_MAP_CAPACITY {
        let key = format!("serde-key-{index:02}");
        if index == 0 {
            first_key_pointer = key.as_ptr();
        }
        let value = semio_framework_value::DslValue::Object(vec![("nested".into(), semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::String(format!("serde-value-{index}")), semio_framework_value::DslValue::Object(vec![("leaf".into(), semio_framework_value::DslValue::uint(index as u64))])]))]);
        params.insert(key, value).expect("maximum populated serde map remains exactly page admitted");
    }
    assert_eq!(params.len(), crate::RASTER_OWNED_MAP_CAPACITY);
    assert_eq!(params.entry_at(0).expect("first serde owner remains installed").0.as_ptr(), first_key_pointer);

    let plus_one_key = String::from("serde-key-plus-one");
    let plus_one_key_pointer = plus_one_key.as_ptr();
    let rejected = params.insert(plus_one_key, semio_framework_value::DslValue::String("serde-plus-one-value".into())).expect_err("serde capacity plus one returns both exact owners");
    assert_eq!(rejected.key.as_ptr(), plus_one_key_pointer);
    assert_eq!(rejected.reason, "raster-map.item-capacity");
    retire_raster_value_cold((rejected.key, rejected.value));

    let layer = RasterLayerNode::Adjustment { id: "serde-output".into(), name: "Serde Output".into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "nested".into(), params };
    // 🗂️ The public `ToValue` projection of a layer READS its populated parameter map whole (it used
    // to refuse it, which trapped every whole-document route) and must not consume, move or
    // reallocate a single owner while doing so — the pointer identity below is what proves it.
    let output = semio_framework_value::ToValue::to_value(&layer);
    let semio_framework_value::DslValue::Object(fields) = &output else { panic!("an adjustment layer projects as an object") };
    let projected = fields.iter().find(|(key, _)| key == "params").map(|(_, value)| value).expect("the projection carries the parameter map");
    let semio_framework_value::DslValue::Object(entries) = projected else { panic!("a populated parameter map projects as an object") };
    assert_eq!(entries.len(), crate::RASTER_OWNED_MAP_CAPACITY);
    let params = match &layer {
        RasterLayerNode::Adjustment { params, .. } => params,
        _ => unreachable!("serde fixture remains an adjustment"),
    };
    assert_eq!(params.len(), crate::RASTER_OWNED_MAP_CAPACITY);
    assert_eq!(params.entry_at(0).expect("the projection keeps the first exact owner installed").0.as_ptr(), first_key_pointer);

    let snapshot = RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: vec![layer], assets: RasterOwnedMap::new() };
    retire_raster_value_cold(snapshot);
}

#[test]
fn raster_populated_snapshot_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact() {
    let mut deepest = semio_framework_value::DslValue::String("deep-output-owner".into());
    for _ in 1..RASTER_MAXIMUM_NESTED_DEPTH {
        deepest = semio_framework_value::DslValue::Array(vec![deepest]);
    }
    let mut params = RasterOwnedMap::new();
    let mut first_param_pointer = std::ptr::null();
    for index in 0..crate::RASTER_OWNED_MAP_CAPACITY {
        let key = format!("output-param-{index:02}");
        if index == 0 {
            first_param_pointer = key.as_ptr();
        }
        let value = if index == 0 { std::mem::replace(&mut deepest, semio_framework_value::DslValue::Null) } else { semio_framework_value::DslValue::String(format!("output-value-{index}")) };
        params.insert(key, value).expect("maximum populated output parameter map remains exactly admitted");
    }
    let plus_one_param_key = String::from("output-param-plus-one");
    let plus_one_param_pointer = plus_one_param_key.as_ptr();
    let plus_one_param_value = String::from("rejected-output-value");
    let plus_one_param_value_pointer = plus_one_param_value.as_ptr();
    let rejected_param = params.insert(plus_one_param_key, semio_framework_value::DslValue::String(plus_one_param_value)).expect_err("output parameter capacity plus one returns both exact owners");
    assert_eq!(rejected_param.key.as_ptr(), plus_one_param_pointer);
    let rejected_param_value = match &rejected_param.value {
        semio_framework_value::DslValue::String(value) => value,
        _ => unreachable!("rejected output parameter remains the exact string variant"),
    };
    assert_eq!(rejected_param_value.as_ptr(), plus_one_param_value_pointer, "rejected output parameter returns the exact value allocation");
    assert_eq!(rejected_param.reason, "raster-map.item-capacity");
    retire_raster_value_cold((rejected_param.key, rejected_param.value));

    let mut assets = RasterOwnedMap::new();
    let mut first_asset_pointer = std::ptr::null();
    for index in 0..crate::RASTER_OWNED_MAP_CAPACITY {
        let key = format!("output-asset-{index:02}");
        if index == 0 {
            first_asset_pointer = key.as_ptr();
        }
        assets
            .insert(
                key,
                store::ArtifactChild::new(
                    format!("output-child-{index:02}"),
                    semio_framework_artifact_reference::ArtifactRef { artifact_id: format!("output-artifact-{index:02}"), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } },
                ),
            )
            .expect("maximum populated output asset map remains exactly admitted");
    }
    let plus_one_asset_key = String::from("output-asset-plus-one");
    let plus_one_asset_pointer = plus_one_asset_key.as_ptr();
    let plus_one_asset_child = store::ArtifactChild::new(
        "output-child-plus-one".into(),
        semio_framework_artifact_reference::ArtifactRef { artifact_id: "output-artifact-plus-one".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } },
    );
    let plus_one_asset_child_pointer = plus_one_asset_child.child_id.as_ptr();
    let rejected_asset = assets.insert(plus_one_asset_key, plus_one_asset_child).expect_err("output asset capacity plus one returns both exact owners");
    assert_eq!(rejected_asset.key.as_ptr(), plus_one_asset_pointer);
    assert_eq!(rejected_asset.value.child_id.as_ptr(), plus_one_asset_child_pointer, "rejected output asset returns the exact child allocation");
    assert_eq!(rejected_asset.reason, "raster-map.item-capacity");
    retire_raster_value_cold((rejected_asset.key, rejected_asset.value));

    let layer = RasterLayerNode::Adjustment { id: "retained-output".into(), name: "Retained Output".into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "deep".into(), params };
    let snapshot = RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: vec![layer], assets };
    // 🗜️ The whole-document pack codec READS the populated forest and asset pool (it used to refuse
    // them outright) and must not consume, move or reallocate a single owner while doing so — the
    // pointer identities asserted below are what prove it.
    let packed = <RasterSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
    let decoded = <RasterSnapshot as store::ArtifactPack>::decode_pack(&packed).expect("a populated Raster snapshot packs and decodes whole");
    assert_eq!(decoded.assets.len(), snapshot.assets.len(), "the whole-document pack carries every asset handle");
    assert_eq!(decoded.layers.len(), snapshot.layers.len(), "the whole-document pack carries every layer");
    retirement::retire_raster_snapshot(decoded);
    let params = match &snapshot.layers[0] {
        RasterLayerNode::Adjustment { params, .. } => params,
        _ => unreachable!("output fixture remains an adjustment"),
    };
    assert_eq!(params.entry_at(0).expect("fault and panic retain the first parameter owner").0.as_ptr(), first_param_pointer);
    assert_eq!(snapshot.assets.entry_at(0).expect("all mounted exporters retain the first asset owner").0.as_ptr(), first_asset_pointer);

    retire_raster_value_cold(snapshot);
}

#[test]
fn raster_maximum_combined_layer_and_value_depth_retires_to_terminal() {
    let mut value = semio_framework_value::DslValue::String("terminal".into());
    for _ in 1..32 {
        value = semio_framework_value::DslValue::Array(vec![value]);
    }
    let mut params = RasterOwnedMap::new();
    params.insert("deep".into(), value).expect("one fixed parameter page");
    let mut layer = RasterLayerNode::Adjustment { id: "adjustment".into(), name: "Adjustment".into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "levels".into(), params };
    for index in 1..32 {
        layer = RasterLayerNode::Group { id: format!("group-{index}"), name: "Group".into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: vec![layer] };
    }
    retire_raster_value_cold(RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: vec![layer], assets: RasterOwnedMap::new() });
}

#[test]
fn raster_nested_snapshot_and_child_handles_retire_one_owner_per_grant() {
    let mut params = RasterOwnedMap::new();
    params.insert("nested".repeat(16), semio_framework_value::DslValue::Object(vec![("array".repeat(16), semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::String("payload".repeat(64)), semio_framework_value::DslValue::String("tail".into())]))])).expect("bounded fixture operation succeeds");
    let adjustment = RasterLayerNode::Adjustment { id: "adjustment".into(), name: "Adjustment".into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "levels".into(), params };
    let mut snapshot = empty_raster_document();
    snapshot.title = Some("Nested raster".into());
    snapshot.layers.push(RasterLayerNode::Group { id: "group".into(), name: "Group".into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: vec![adjustment] });
    snapshot
        .assets
        .insert(
            "asset".into(),
            store::ArtifactChild::new("child".into(), semio_framework_artifact_reference::ArtifactRef { artifact_id: "artifact".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } }),
        )
        .expect("bounded fixture operation succeeds");
    retire_raster_value_cold(snapshot);
}

#[test]
fn raster_owner_caps_and_all_mutation_variants_retire_one_owner_per_grant() {
    let pixel =
        || Box::new(RasterLayerNode::Pixel { id: "pixel".into(), name: "Pixel".into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, width: Some(1), height: Some(1), image_key: None });
    let mutations = vec![
        RasterMutation::CreateLayer(create_layer::mutation::CreateLayer { parent_id: Some("root".into()), index: 0, layer: pixel() }),
        RasterMutation::DeleteLayer(delete_layer::mutation::DeleteLayer { layer_id: "pixel".into() }),
        RasterMutation::ReorderLayers(reorder_layers::mutation::ReorderLayers { layer_id: "pixel".into(), parent_id: Some("root".into()), index: 1 }),
        RasterMutation::RenameLayer(rename_layer::mutation::RenameLayer { layer_id: "pixel".into(), new_name: "Renamed".into() }),
        RasterMutation::ChangeLayerLocked(crate::mutations::change_layer_locked::ChangeLayerLocked {layer_id:"pixel".into(),expected:false,locked:true}),
        RasterMutation::ChangeLayerVisible(change_layer_visible::mutation::ChangeLayerVisible { layer_id: "pixel".into(), new_visible: false }),
        RasterMutation::ChangeLayerOpacity(change_layer_opacity::mutation::ChangeLayerOpacity { layer_id: "pixel".into(), new_opacity: 0.5 }),
        RasterMutation::ChangeLayerBlendMode(change_layer_blend_mode::mutation::ChangeLayerBlendMode { layer_id: "pixel".into(), new_blend_mode: "multiply".into() }),
        RasterMutation::MoveLayer(move_layer::mutation::MoveLayer { layer_id: "pixel".into(), new_x: 1.0, new_y: 2.0 }),
        RasterMutation::ResizeLayer(resize_layer::mutation::ResizeLayer { layer_id: "pixel".into(), new_width: 2, new_height: 3 }),
        RasterMutation::ChangeLayerAdjustmentKind(change_layer_adjustment_kind::mutation::ChangeLayerAdjustmentKind { layer_id: "pixel".into(), new_adjustment_kind: "levels".into() }),
        RasterMutation::AddLayerAsset(add_layer_asset::mutation::AddLayerAsset { asset_id: "asset".into(), asset: crate::standards::v1::subsets::any::schema::semio_image_from_rgba8(1,1,vec![1,2,3,255]) }),
        RasterMutation::RemoveLayerAsset(remove_layer_asset::mutation::RemoveLayerAsset { asset_id: "asset".into() }),
        RasterMutation::ChangeLayerPixels(crate::mutations::change_layer_pixels::ChangeLayerPixels { layer_id: "pixel".into(), expected_image_key: Some("previous".into()), content: crate::RasterPixelContent { image_key: Some("next".into()), width: None, height: None }, transform: None }),
        RasterMutation::ChangeLayerMask(crate::mutations::change_layer_mask::ChangeLayerMask {
            layer_id: "pixel".into(),
            expected: Some(crate::RasterLayerMask { enabled: true, linked: true, invert: false, width: None, height: None, image_key: Some("previous".into()), transform: RasterTransform::default() }),
            mask: Some(crate::RasterLayerMask { enabled: true, linked: false, invert: true, width: Some(2), height: Some(3), image_key: Some("next".into()), transform: RasterTransform::default() }),
        }),
        RasterMutation::ChangeLayerAdjustmentParameter(crate::mutations::change_layer_adjustment_parameter::ChangeLayerAdjustmentParameter {layer_id:"tone".into(),parameter:"brightness".into(),expected:None,value:Some(crate::RasterAdjustmentNumber::decimal(0.25))}),
        RasterMutation::ChangeLayerTransform(crate::mutations::change_layer_transform::ChangeLayerTransform {layer_id:"pixel".into(),expected:RasterTransform::default(),transform:RasterTransform {x:2.0,y:3.0,..Default::default()}}),
        RasterMutation::PaintStroke(crate::mutations::paint_stroke::PaintStroke {
            layer_id: "pixel".into(),
            target: "mask".into(),
            tool: "eraser".into(),
            brush: crate::mutations::paint_stroke::RasterBrush { size: 3.0, hardness: 0.5, opacity: 0.75, color: vec![0.2, 0.2, 0.2, 1.0] },
            points: vec![crate::mutations::paint_stroke::RasterStrokePoint { x: 0.5, y: 0.5 }, crate::mutations::paint_stroke::RasterStrokePoint { x: 1.5, y: 2.5 }],
            selection: Some(vec![crate::mutations::paint_stroke::RasterSelectionSpan { start: 0, length: 2, coverage: 255 }]),
        }),
        RasterMutation::FillRegion(crate::mutations::fill_region::FillRegion {
            layer_id: "pixel".into(),
            target: "mask".into(),
            seed: crate::mutations::fill_region::RasterSeed { x: 1, y: 2 },
            tolerance: 32,
            color: vec![0.2, 0.2, 0.2, 1.0],
            selection: Some(vec![crate::mutations::paint_stroke::RasterSelectionSpan { start: 0, length: 2, coverage: 255 }]),
        }),
        RasterMutation::ApplyFilter(crate::mutations::apply_filter::ApplyFilter {
            layer_id: "pixel".into(),
            filter: "brightness".into(),
            amount: -0.25,
            selection: Some(vec![crate::mutations::paint_stroke::RasterSelectionSpan { start: 0, length: 2, coverage: 128 }]),
        }),
        crate::mutations::transform_image::crop_image("pixel", 1, 2, 3, 4),
        crate::mutations::fill_selection::fill_selection("pixel", "mask", [0.5, 0.5, 0.5, 1.0]),
    ];
    assert_eq!(mutations.len(), <RasterMutation as protocol::SemanticMutation<RasterSnapshot>>::kinds().len());
    for mutation in mutations {
        retire_raster_value_cold(mutation);
    }
}

#[test]
fn raster_envelope_caps_and_plus_one_page_return_the_exact_fixed_owner() {
    assert_eq!(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, 4_096);
    assert_eq!(store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_PAGES, 64);
    assert_eq!(store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES, 262_144);
    let mut exact = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
    exact[0] = 0x52;
    exact[store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES - 1] = 0x7f;
    let rejected = store::ArtifactEnvelopeDecodePage::try_from_array(exact, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES + 1).expect_err("page cap plus one returns the exact caller owner");
    assert_eq!(rejected[0], 0x52);
    assert_eq!(rejected[store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES - 1], 0x7f);
}

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
/// `RasterMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
/// existing pack round-trip law (same pattern as `mathematical_protocol`'s own
/// `command_envelope_round_trip_holds_for_an_applied_operation`).
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::RasterSnapshot;
    use protocol::{ArtifactId, Edit, SchemaId};

    let envelope = store::create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "command-envelope-demo", empty_raster_document(), None);
    let mut store = store::ArtifactStore::new(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    // 🔐️ The history ledger refuses an insertion from a store without its domain owner catalog
    // ("edit history insertion requires its exact mutation retirement factory"): a raster store is
    // built with the artifact's own `raster_document_store_owners`, never bare.
    store.install_document_store_owners_exact(raster_document_store_owners()).map_err(|(error, _)| error).expect("the Raster owner catalog installs on a fresh store");
    store
        .dispatch(store::ArtifactCommand::Apply {
            mutations: vec![RasterMutation::CreateLayer(create_layer::mutation::CreateLayer {
                parent_id: None,
                index: 0,
                layer: Box::new(RasterLayerNode::Pixel {
                    id: "command-envelope-pixel".into(),
                    name: "Command Envelope Pixel".into(),
                    visible: true, locked: false,
                    opacity: 1.0,
                    blend_mode: "normal".into(),
                    transform: RasterTransform::default(),
                    mask: None,
                    width: Some(32),
                    height: Some(32),
                    image_key: None,
                }),
            })],
            transaction: None,
        })
        .await
        .expect("apply");
    let edit: &Edit<RasterMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    store::os_store::test_support::assert_command_envelope_round_trip::<RasterSnapshot, RasterMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
    store::os_store::test_support::close_plain_test_store(&mut store);
}
//#endregion 🔖️CommandEnvelopeTests

#[test]
fn retained_mask_mutations_match_cold_apply_and_undo() {
    use protocol::Mutation;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧬️schema/🧬️mutations/🎭️change-layer-mask/🧪️tests/🔣️.json")).unwrap();
    for (index, case) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let mut source = fixture["before"].clone();
        let resolve = |key: &serde_json::Value| key.as_str().map(|key| fixture[key].clone()).unwrap_or(serde_json::Value::Null);
        source["layers"][0]["mask"] = resolve(&case["before"]);
        let base: RasterSnapshot = semio_framework_pack_json::from_json_str(&source.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let operation: RasterMutation = semio_framework_pack_json::from_json_str(&serde_json::json!({"mutation":"changeLayerMask","layerId":"paint","expected":resolve(&case["before"]),"mask":resolve(&case["after"])}).to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let inverse = operation.inverse(&base).expect("valid retained mutation inverse fixture").remove(0);
        let (diff, _) = operation.diff(&base).into_parts();
        let cold = protocol::apply_diff(&diff, &base).unwrap();
        let candidate = drive_raster_candidate(&base, &operation, 930 + index as u64);
        assert_eq!(candidate, cold);
        let restored = drive_raster_candidate(&candidate, &inverse, 940 + index as u64);
        assert_eq!(restored, base);
        protocol::MutationDiff::retire_cold(diff);
        for document in [base, cold, candidate, restored] { retirement::retire_raster_snapshot(document); }
        protocol::Mutation::retire_cold(operation);
        protocol::Mutation::retire_cold(inverse);
    }
}

#[test]
fn retained_adjustment_parameters_match_cold_apply_and_undo() {
    use protocol::Mutation;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧬️schema/🧬️mutations/🎛️change-layer/🧪️tests/🔣️.json")).unwrap();
    for (index,row) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let parameter=row["parameter"].as_str().unwrap();
        let mut source=serde_json::json!({"schema":"raster.document","id":"retained-tone","layers":[{"kind":"adjustment","id":"tone","name":"Tone","adjustmentKind":"brightnessContrast","params":{"metadata":"preserve"}}]});
        if !row["before"].is_null() {source["layers"][0]["params"][parameter]=row["before"].clone();}
        let before:RasterSnapshot=semio_framework_pack_json::from_json_str(&source.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let operation:RasterMutation=semio_framework_pack_json::from_json_str(&serde_json::json!({"mutation":"changeLayerAdjustmentParameter","layerId":"tone","parameter":parameter,"expected":row["before"],"value":row["after"]}).to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let inverse=operation.inverse(&before).expect("valid retained mutation inverse fixture").remove(0);let (diff,_)=operation.diff(&before).into_parts();let cold=protocol::apply_diff(&diff, &before).unwrap();
        let candidate=drive_raster_candidate(&before,&operation,960+index as u64);assert_eq!(candidate,cold);
        let restored=drive_raster_candidate(&candidate,&inverse,970+index as u64);assert_eq!(restored,before);
        diff.retire_cold();for document in [before,cold,candidate,restored] {retirement::retire_raster_snapshot(document);}
        operation.retire_cold();inverse.retire_cold();
    }
}

#[test]
fn retained_asset_insertion_at_full_capacity_refuses_without_changing_source() {
    let mut base=crate::standards::v1::subsets::any::io::text::snapshot::empty_raster_snapshot();
    let asset=crate::RasterImageAsset {mime:"image/png".into(),data:semio_framework_pixels::encode_png(&semio_framework_pixels::RasterImage {width:1,height:1,pixels:vec![1,2,3,255]}).unwrap()};
    for index in 0..crate::RASTER_OWNED_MAP_CAPACITY {let key=format!("capacity-{index}");base.assets.insert(key.clone(),crate::mint_raster_asset_child(&key,&asset)).unwrap();}
    let before=base.clone();
    let operation=RasterMutation::AddLayerAsset(add_layer_asset::AddLayerAsset {asset_id:"overflow".into(),asset:crate::standards::v1::subsets::any::io::semio_image_snapshot_from_raster_asset(&asset).unwrap()});
    let mut apply=RasterOneItemApply::new();
    let mut refusal=None;
    for _ in 0..200_000 {
        match apply.advance(&base,&operation,semio_framework_job::OperationId(980),semio_framework_job::Generation(1),RASTER_TEST_FUEL,raster_test_wallet()) {
            Err(error)=>{refusal=Some(error.into_message());break;},
            Ok((Some(_),_))=>panic!("capacity overflow published"),
            Ok((None,_))=>{}
        }
    }
    assert!(refusal.is_some_and(|message|message.contains("raster-map.item-capacity")));
    assert_eq!(base,before);
    let wallet=raster_test_wallet();
    for _ in 0..200_000 {
        if apply.terminal_is_empty(){break;}
        let demand=apply.close_demands(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).unwrap();
        let grant=semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:wallet.maximum_copy_bytes.max(demand.copy_bytes),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1)};
        apply.close_step(grant).unwrap();
    }
    assert!(apply.terminal_is_empty());
    drop(apply);
    protocol::Mutation::retire_cold(operation);
    retirement::retire_raster_snapshot(base);
    retirement::retire_raster_snapshot(before);
}

#[test]
fn retained_layer_clone_preserves_protection_during_history_replay() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧫️fixtures/🔒️protection/🔣️.json")).unwrap();
    let mut base=crate::standards::v1::subsets::any::io::text::snapshot::empty_raster_snapshot();base.layers=semio_framework_pack_json::from_json_str(&fixture["layers"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let operation=RasterMutation::RenameLayer(rename_layer::RenameLayer {layer_id:"locked-pixel".into(),new_name:"Accepted history".into()});
    let candidate=drive_raster_candidate(&base,&operation,981);
    for row in fixture["cases"].as_array().unwrap() {let id=row["id"].as_str().unwrap();assert_eq!(crate::standards::v1::subsets::any::schema::layer_protection(&candidate.layers,id),crate::standards::v1::subsets::any::schema::layer_protection(&base.layers,id));}
    let operation=RasterMutation::RenameLayer(rename_layer::RenameLayer {layer_id:"locked-pixel".into(),new_name:"locked-pixel".into()});
    let restored=drive_raster_candidate(&candidate,&operation,982);assert_eq!(restored,base);
    retirement::retire_raster_snapshot(restored);retirement::retire_raster_snapshot(candidate);retirement::retire_raster_snapshot(base);
}

#[semio_framework_async_macros::async_test]
async fn raster_close_diagnosis_direct_store_add_layer_releases_all_owners() {
    let document=crate::standards::v1::subsets::any::io::text::snapshot::empty_raster_snapshot();
    let envelope=store::create_document_envelope::<RasterSnapshot,RasterMutation>(RASTER_DOCUMENT_SCHEMA,"close-owner-law",document,None);
    let mut document=store::ArtifactStore::new(envelope,protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.unwrap();
    document.install_document_store_owners_exact(raster_document_store_owners()).map_err(|(error, _)| error).expect("the Raster owner catalog installs on a fresh store");
    document.dispatch(store::ArtifactCommand::Apply{mutations:vec![RasterMutation::CreateLayer(create_layer::mutation::CreateLayer{parent_id:None,index:0,layer:Box::new(crate::standards::v1::subsets::any::schema::create_pixel_layer("Close",512,512))})],transaction:None}).await.unwrap();
    store::os_store::test_support::close_plain_test_store(&mut document);
    assert!(document.close_owned_store_terminal_is_empty(),"direct store AddLayer closure must release all owners");
}
