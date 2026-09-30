//! 🔒️ Layer protection persists across text, binary and subtree duplication.
use super::*;
use crate::{diff::diff_patch_layer,RasterLayerPatch};
use crate::standards::v1::subsets::any::schema::snapshot;
use store::{ArtifactDsl,ArtifactPack};
#[test]
fn protection_vectors_and_document_round_trips() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧫️fixtures/🔒️protection/🔣️.json")).unwrap();
    let mut document=empty_raster_snapshot();document.layers=dsl::json::from_json_str(&fixture["layers"].to_string()).unwrap();
    for row in fixture["cases"].as_array().unwrap() {let expected:LayerProtection=dsl::json::from_json_str(&row["expected"].to_string()).unwrap();assert_eq!(layer_protection(&document.layers,row["id"].as_str().unwrap()),Some(expected));}
    assert_eq!(layer_protection(&document.layers,"missing"),None);
    let text=RasterSnapshot::parse_dsl(&document.print_dsl()).unwrap();assert_eq!(text,document);
    let binary=RasterSnapshot::decode_pack(&document.encode_pack()).unwrap();assert_eq!(binary,document);
    let copy=clone_layer(&document.layers[0]);let RasterLayerNode::Group {children,..}=&copy else {panic!("group")};assert!(layer_locked(&children[0]));assert!(!layer_locked(&children[1]));crate::retire_raster_layer(copy);
    snapshot::retire_raster_snapshot(text);snapshot::retire_raster_snapshot(binary);snapshot::retire_raster_snapshot(document);
}

#[test]
fn protection_mutations_preserve_inverse_and_refuse_stale_guards() {
    use protocol::Mutation;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧫️fixtures/🔒️protection/🔣️.json")).unwrap();
    for row in fixture["changes"].as_array().unwrap() {
        let mut base=empty_raster_snapshot();base.layers=dsl::json::from_json_str(&fixture["layers"].to_string()).unwrap();
        let operation=crate::RasterMutation::ChangeLayerLocked(crate::mutations::change_layer_locked::ChangeLayerLocked {layer_id:row["id"].as_str().unwrap().into(),expected:row["expected"].as_bool().unwrap(),locked:row["locked"].as_bool().unwrap()});
        let (next,messages)=semio_framework_os_kernel::apply_mutation(&base,&operation).unwrap();
        if row["accepted"].as_bool().unwrap() {
            assert!(messages.is_empty());assert_eq!(layer_locked(find_layer(&next.layers,row["id"].as_str().unwrap()).unwrap()),row["locked"].as_bool().unwrap());
            let inverse=operation.inverse(&base).pop().unwrap();let (restored,messages)=semio_framework_os_kernel::apply_mutation(&next,&inverse).unwrap();assert!(messages.is_empty());assert_eq!(restored,base);snapshot::retire_raster_snapshot(restored);inverse.retire_cold();
        } else {assert!(!messages.is_empty());assert_eq!(next,base);}
        operation.retire_cold();snapshot::retire_raster_snapshot(next);snapshot::retire_raster_snapshot(base);
    }
}

#[test]
fn combined_patches_keep_the_last_protection_change() {
    use protocol::MutationDiff;
    let mut base=empty_raster_snapshot();base.layers.push(create_layer_of_kind("pixel"));let id=layer_node_id(&base.layers[0]).to_owned();
    let mut patch=diff_patch_layer(&id,RasterLayerPatch {name:Some("Renamed".into()),..Default::default()});
    patch.absorb(diff_patch_layer(&id,RasterLayerPatch {locked:Some(true),..Default::default()}));
    let next=patch.apply(&base).unwrap();assert!(layer_locked(&next.layers[0]));assert_eq!(layer_name(&next.layers[0]),"Renamed");
    patch.absorb(diff_patch_layer(&id,RasterLayerPatch {locked:Some(false),..Default::default()}));
    let unlocked=patch.apply(&base).unwrap();assert!(!layer_locked(&unlocked.layers[0]));assert_eq!(layer_name(&unlocked.layers[0]),"Renamed");
    patch.retire_cold();snapshot::retire_raster_snapshot(unlocked);snapshot::retire_raster_snapshot(next);snapshot::retire_raster_snapshot(base);
}

#[test]
fn lock_mutation_matches_the_committed_snapshot_and_canonical_diff() {
    use protocol::{Mutation,MutationDiff};
    let before:RasterSnapshot=dsl::json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/🔒️change-layer-locked/🔒️protects-layer-content/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let expected:RasterSnapshot=dsl::json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/🔒️change-layer-locked/🔒️protects-layer-content/📸️snapshot/➡️after/🔣️.json")).unwrap();
    let operation:crate::RasterMutation=dsl::json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/🔒️change-layer-locked/🔒️protects-layer-content/🦠️mutation/🔣️.json")).unwrap();
    let diff_json=include_str!("../../../🧫️fixtures/🧬️mutations/🔒️change-layer-locked/🔒️protects-layer-content/🔺️diff/🔣️.json");
    let (diff,messages)=operation.diff(&before).into_parts();assert!(messages.is_empty());
    let encoded=dsl::json::from_dsl_value(&dsl::ToValue::to_value(&diff));let committed=dsl::json::parse(diff_json).unwrap();assert!(dsl::json::value_eq_ignoring_object_order(&encoded,&committed));
    let after=diff.apply(&before).unwrap();assert_eq!(after,expected);
    diff.retire_cold();operation.retire_cold();for document in [before,after,expected] {snapshot::retire_raster_snapshot(document);}
}
