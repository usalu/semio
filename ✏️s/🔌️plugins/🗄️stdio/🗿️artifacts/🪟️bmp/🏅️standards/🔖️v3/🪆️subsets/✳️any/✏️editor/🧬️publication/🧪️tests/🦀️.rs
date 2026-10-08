//! 🧪️ Exact owned BMP publication uses neutral native sample cases and independent image-rs decoding.
use super::*;
use semio_framework_value::{FromValue,retained_clone::RetainedCloneSource};
use semio_framework_value::retirement::SharedValueRetirementFactory;
use crate::schema::mutations::{PaintIndexedRegion,PaintDirectRegion};
fn close(cursor:&mut BmpPublicationCursor) {cursor.begin_close();for _ in 0..10000 {if cursor.close_step(1,4096).unwrap()==SnapshotRetirementStep::Complete {assert!(cursor.terminal_is_empty());return;}}panic!("BMP retained publication did not close");}
#[test]
fn retained_bmp_publication_keeps_exact_components_indices_and_reserved_samples() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let base=BmpSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse_bytes(row["snapshot"].to_string().as_bytes(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())).unwrap();
        let paint=&row["paint"];let region=BmpRegion{x:paint["x"].as_u64().unwrap()as u32,y:paint["y"].as_u64().unwrap()as u32,width:paint["width"].as_u64().unwrap()as u32,height:paint["height"].as_u64().unwrap()as u32};let revision=crate::schema::operations::bmp_revision(&base);
        let mutation=if let Some(index)=paint["paletteIndex"].as_u64(){BmpMutation::PaintIndexedRegion(PaintIndexedRegion{revision,x:region.x,y:region.y,width:region.width,height:region.height,palette_index:index as u8})}else{BmpMutation::PaintDirectRegion(PaintDirectRegion{revision,x:region.x,y:region.y,width:region.width,height:region.height,red:paint["red"].as_u64().unwrap()as u8,green:paint["green"].as_u64().unwrap()as u8,blue:paint["blue"].as_u64().unwrap()as u8,alpha:paint["alpha"].as_u64().unwrap()as u8})};
        let source=RetainedCloneSource::from_authority(Arc::new(base.clone()),());let mutation=RetainedCloneSource::from_authority(Arc::new(mutation),());let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_depth:64, maximum_release_bytes: 4096 };let mut cursor=BmpPublicationCursor::new();let mut post=base.clone();let mut turns=0;
        loop {turns+=1;assert!(turns<10000);let step=cursor.advance(source.borrow(),&mut post,mutation.borrow(),grant).unwrap();assert!(step.progress().fits(grant));if matches!(step,RetainedCloneEditStep::Complete(_)){break;}}
        match &post.image.pixels {BmpPixels::Indexed{indices}=>assert_eq!(serde_json::json!(indices),row["expectedIndices"]),BmpPixels::Direct{samples}=>assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(samples)).unwrap(),row["expectedSamples"])}
        let inverse=cursor.take_inverse().unwrap();assert_eq!(inverse,vec![BmpMutation::SetSnapshot(SetSnapshot{snapshot:base.clone()})]);close(&mut cursor);
        if post.image.profile==crate::schema::snapshot::BmpProfile::DirectRgb32 {let bytes=crate::standards::v_v3::subsets::any::io::encode_bmp(&post).unwrap();let (width,height,visual)=semio_s_artifact_stdio_bmp_test_oracle::standards::v_v3::subsets::any::oracle_visual_rgba8(&bytes).unwrap();assert_eq!((width,height),(post.image.width,post.image.height));assert_eq!(visual,crate::schema::operations::bmp_rgba8_preview(&post).unwrap());}
        let mut retirement=semio_framework_value::retirement::owned_retirement(inverse);while retirement.close_step(1,4096).unwrap()!=SnapshotRetirementStep::Complete {}assert!(retirement.terminal_is_empty());
        eprintln!("[DEBUG] retained BMP publication case={} turns={turns} preciseSamples={}",row["name"],post.image.width*post.image.height);
    }
}
#[test]
fn retained_bmp_publication_cancellation_closes_partial_result_copy() {
    let source=RetainedCloneSource::from_authority(Arc::new(BmpSnapshot::default()),());let owner=Arc::new(BmpMutation::SetSnapshot(SetSnapshot{snapshot:BmpSnapshot::default()}));let mutation=RetainedCloneSource::from_authority(Arc::clone(&owner),());let mut post=BmpSnapshot::default();let mut cursor=BmpPublicationCursor::new();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_depth:64, maximum_release_bytes: 4096 };
    for _ in 0..300 {cursor.advance(source.borrow(),&mut post,mutation.borrow(),grant).unwrap();if cursor.phase==2&&cursor.copy.is_some(){break;}}
    cursor.cancel();close(&mut cursor);drop(mutation.into_owner());assert_eq!(Arc::strong_count(&owner),1);
}

#[test]
fn retained_bmp_publication_canonical_reader_preserves_neutral_owned_fields() {
    use semio_framework_value::FromValue;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json")).unwrap();
    let grant=store::ArtifactStoreOneItemGrant{maximum_items:1,maximum_bytes:17};
    for case in fixture["cases"].as_array().unwrap() {
        let snapshot=BmpSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse_bytes(case["snapshot"].to_string().as_bytes(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())).unwrap();
        let mutation=Arc::new(BmpMutation::SetSnapshot(SetSnapshot{snapshot}));
        let expected:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(mutation.as_ref())).unwrap();
        let mut reader=store::ArtifactCanonicalJsonReader::new(Arc::clone(&mutation),Arc::new(SharedValueRetirementFactory::<BmpMutation>::default()));
        let mut encoded=Vec::new();let mut chunk=[0;17];let mut turns=0;
        while !reader.is_complete() {turns+=1;assert!(turns<100000);let count=reader.encode_chunk(grant,&mut chunk).unwrap();assert!(count<=grant.maximum_bytes);encoded.extend_from_slice(&chunk[..count]);}
        let actual:serde_json::Value=serde_json::from_slice(&encoded).unwrap();assert_eq!(actual,expected);assert_eq!(actual["payload"]["snapshot"],case["snapshot"]);
        reader.begin_close();for _ in 0..100000 {if reader.close_step(grant).unwrap()==SnapshotRetirementStep::Complete {break;}}
        assert!(reader.terminal_is_empty());assert_eq!(Arc::strong_count(&mutation),1);
        eprintln!("[DEBUG] retained bmp canonical neutral={} turns={turns} bytes={}",case["name"],encoded.len());
    }
}
