//! 🧪️ Bounded schema-owned BMP sealing and exact first-party owner retirement.
use super::*;
use std::sync::Arc;
use semio_framework_value::{FromValue,ToValue,retirement::{RetireOwned,SharedValueRetirementFactory},SnapshotRetirementStep};

fn verify<T:store::ArtifactCanonicalJson+ToValue+RetireOwned+Sync>(value:T,expected:serde_json::Value) {
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&value)).unwrap(),expected);
    let owner=Arc::new(value);let lifetime=Arc::downgrade(&owner);
    let mut reader=store::ArtifactCanonicalJsonReader::new(owner,Arc::new(SharedValueRetirementFactory::<T>::default()));
    let grant=store::ArtifactStoreOneItemGrant{maximum_items:1,maximum_bytes:17};
    let mut bytes=Vec::new();let mut chunk=[0;17];let mut turns=0;
    for zero in [store::ArtifactStoreOneItemGrant{maximum_items:0,..grant},store::ArtifactStoreOneItemGrant{maximum_bytes:0,..grant}]{assert_eq!(reader.encode_chunk(zero,&mut chunk).unwrap(),0);assert_eq!(reader.completed_bytes(),0);}
    while !reader.is_complete(){turns+=1;assert!(turns<100000);let count=reader.encode_chunk(grant,&mut chunk).unwrap();assert!(count<=grant.maximum_bytes);bytes.extend_from_slice(&chunk[..count]);}
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&bytes).unwrap(),expected);
    reader.begin_close();for _ in 0..100000{if reader.close_step(store::ArtifactStoreOneItemGrant{maximum_items:1,maximum_bytes:4096}).unwrap()==SnapshotRetirementStep::Complete{break;}}
    assert!(reader.terminal_is_empty());assert!(lifetime.upgrade().is_none());
    eprintln!("[DEBUG] BMP schema canonical owner turns={turns} bytes={} retired=true",bytes.len());
}

fn verify_cancel<T:store::ArtifactCanonicalJson+RetireOwned+Sync>(value:T) {
    let owner=Arc::new(value);let lifetime=Arc::downgrade(&owner);
    let mut reader=store::ArtifactCanonicalJsonReader::new(owner,Arc::new(SharedValueRetirementFactory::<T>::default()));
    let grant=store::ArtifactStoreOneItemGrant{maximum_items:1,maximum_bytes:17};let mut chunk=[0;17];
    assert!(reader.encode_chunk(grant,&mut chunk).unwrap()<=17);
    reader.cancel();reader.begin_close();for _ in 0..100000{if reader.close_step(store::ArtifactStoreOneItemGrant{maximum_items:1,maximum_bytes:4096}).unwrap()==SnapshotRetirementStep::Complete{break;}}
    assert!(reader.terminal_is_empty());assert!(lifetime.upgrade().is_none());
    eprintln!("[DEBUG] BMP schema canonical cancellation retired=true");
}

#[test]
fn schema_owned_bmp_canonical_sealing_preserves_neutral_snapshot_and_diff() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap(){
        let expected=case["snapshot"].clone();
        let snapshot=BmpSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse_bytes(expected.to_string().as_bytes(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())).unwrap();
        verify_cancel(BmpDiff{image:Some(snapshot.image.clone()),rects:Vec::new()});verify_cancel(snapshot.clone());
        verify(BmpDiff{image:Some(snapshot.image.clone()),rects:Vec::new()},serde_json::json!({"image":expected["image"]}));
        verify(snapshot,expected);
    }
    verify(BmpDiff::default(),serde_json::json!({}));
    let rect=BmpSampleRect{region:BmpRegion{x:1,y:2,width:1,height:1},indices:vec![3],samples:Vec::new()};
    verify(BmpDiff{image:None,rects:vec![rect]},serde_json::json!({"rects":[{"region":{"x":1,"y":2,"width":1,"height":1},"indices":[3]}]}));
}
#[test]
fn schema_owned_bmp_replacement_diff_and_inverse_preserve_neutral_native_words(){
    use protocol::Mutation;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json")).unwrap();
    let base=BmpSnapshot::default();
    for case in fixture["cases"].as_array().unwrap().iter().chain(fixture["replacementCases"].as_array().unwrap().iter()){
        let target=BmpSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse_bytes(case["snapshot"].to_string().as_bytes(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())).unwrap();
        let mutation=BmpMutation::ReplaceImage(crate::schema::mutations::ReplaceImage{image:target.image.clone()});
        let text=protocol::OpText::print_op(&mutation);assert_eq!(<BmpMutation as protocol::OpText>::parse_op(&text).unwrap(),mutation);
        let binary=protocol::OpBinary::encode_op(&mutation).unwrap();assert_eq!(<BmpMutation as protocol::OpBinary>::decode_op(&binary).unwrap(),mutation);
        verify(mutation.clone(),serde_json::json!({"mutation":"replace-image","payload":{"image":case["snapshot"]["image"]}}));
        let actual=protocol::apply_diff(mutation.diff(&base).diff(),&base).unwrap();assert_eq!(actual,target);
        let mut restored=actual.clone();for inverse in mutation.inverse(&base).unwrap().into_iter().rev(){restored=protocol::apply_diff(inverse.diff(&restored).diff(),&restored).unwrap();}assert_eq!(restored,base);
        let bytes=crate::standards::v_v3::subsets::any::io::encode_bmp(&actual).unwrap();assert_eq!(crate::standards::v_v3::subsets::any::io::decode_bmp(&bytes).unwrap(),actual);
        if case["name"]=="bitfields32-ten-bit-513-exact-image-replacement"{let(offset_width,offset_height,visual)=semio_s_artifact_stdio_bmp_test_oracle::standards::v_v3::subsets::any::oracle_visual_rgba8(&bytes).unwrap();assert_eq!((offset_width,offset_height),(actual.image.width,actual.image.height));assert_eq!(visual,crate::schema::operations::bmp_rgba8_preview(&actual).unwrap());let offset=u32::from_le_bytes(bytes[10..14].try_into().unwrap())as usize;let word=u32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap());assert_eq!((word&1072693248)>>20,513);}
        eprintln!("[DEBUG] Bmp typed replacement neutral={} nativeWordsExact=true inverseExact=true",case["name"]);
    }
}
