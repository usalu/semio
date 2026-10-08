//! 🧪️ Bounded schema-owned PNG sealing and exact first-party owner retirement.
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
    eprintln!("[DEBUG] PNG schema canonical owner turns={turns} bytes={} retired=true",bytes.len());
}

fn verify_cancel<T:store::ArtifactCanonicalJson+RetireOwned+Sync>(value:T) {
    let owner=Arc::new(value);let lifetime=Arc::downgrade(&owner);
    let mut reader=store::ArtifactCanonicalJsonReader::new(owner,Arc::new(SharedValueRetirementFactory::<T>::default()));
    let grant=store::ArtifactStoreOneItemGrant{maximum_items:1,maximum_bytes:17};let mut chunk=[0;17];
    assert!(reader.encode_chunk(grant,&mut chunk).unwrap()<=17);
    reader.cancel();reader.begin_close();for _ in 0..100000{if reader.close_step(store::ArtifactStoreOneItemGrant{maximum_items:1,maximum_bytes:4096}).unwrap()==SnapshotRetirementStep::Complete{break;}}
    assert!(reader.terminal_is_empty());assert!(lifetime.upgrade().is_none());
    eprintln!("[DEBUG] PNG schema canonical cancellation retired=true");
}

#[test]
fn schema_owned_png_canonical_sealing_preserves_neutral_snapshot_and_diff() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap(){
        let expected=case["snapshot"].clone();
        let snapshot=PngSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse_bytes(expected.to_string().as_bytes(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())).unwrap();
        verify_cancel(PngDiff{image:Some(snapshot.image.clone()),..PngDiff::default()});verify_cancel(snapshot.clone());
        verify(PngDiff{image:Some(snapshot.image.clone()),..PngDiff::default()},serde_json::json!({"image":expected["image"]}));
        verify(snapshot,expected);
    }
    verify(PngDiff::default(),serde_json::json!({}));
}
#[test]
fn schema_owned_png_replacement_diff_and_inverse_preserve_neutral_native_words(){
    use protocol::Mutation;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json")).unwrap();
    let base=PngSnapshot::default();
    for case in fixture["cases"].as_array().unwrap().iter(){
        let target=PngSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse_bytes(case["snapshot"].to_string().as_bytes(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())).unwrap();
        let mutation=PngMutation::ReplaceImage(crate::schema::mutations::ReplaceImage{image:target.image.clone()});
        let text=protocol::OpText::print_op(&mutation);assert_eq!(<PngMutation as protocol::OpText>::parse_op(&text).unwrap(),mutation);
        let binary=protocol::OpBinary::encode_op(&mutation).unwrap();assert_eq!(<PngMutation as protocol::OpBinary>::decode_op(&binary).unwrap(),mutation);
        verify(mutation.clone(),serde_json::json!({"mutation":"replace-image","payload":{"image":case["snapshot"]["image"]}}));
        let actual=protocol::apply_diff(mutation.diff(&base).diff(),&base).unwrap();assert_eq!(actual,target);
        let mut restored=actual.clone();for inverse in mutation.inverse(&base).unwrap().into_iter().rev(){restored=protocol::apply_diff(inverse.diff(&restored).diff(),&restored).unwrap();}assert_eq!(restored,base);
        let bytes=crate::standards::v1_2::subsets::any::io::encode_png(&actual).unwrap();assert_eq!(crate::standards::v1_2::subsets::any::io::decode_png(&bytes).unwrap(),actual);
        let mut independent=png::Decoder::new(std::io::Cursor::new(&bytes)).read_info().unwrap();let mut samples=vec![0;independent.output_buffer_size().unwrap()];let frame=independent.next_frame(&mut samples).unwrap();assert_eq!((frame.width,frame.height),(actual.image.width,actual.image.height));
        if actual.image.bit_depth==16{let words:Vec<u16>=samples[..frame.buffer_size()].chunks_exact(2).map(|word|u16::from_be_bytes([word[0],word[1]])).collect();assert_eq!(words,actual.image.samples);}
        eprintln!("[DEBUG] Png typed replacement neutral={} nativeWordsExact=true inverseExact=true",case["name"]);
    }
}
