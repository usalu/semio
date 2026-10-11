//! 🧪️ Exact owned BMP publication uses neutral native sample cases and independent image-rs decoding.
use super::*;
use semio_framework_value::{FromValue,retained_clone::RetainedCloneSource};
use semio_framework_value::retirement::SharedValueRetirementFactory;
use crate::schema::mutations::{PaintIndexedRegion,PaintDirectRegion};
fn encoding_grant(items:usize,copy:usize)->store::ArtifactStoreOneItemGrant{store::ArtifactStoreOneItemGrant{maximum_items:items,maximum_copy_bytes:copy,maximum_capacity_bytes:65_536,maximum_release_bytes:65_536,maximum_depth:64}}
fn close_policy()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4_096,maximum_capacity_bytes:65_536,maximum_release_bytes:65_536,maximum_depth:64}}
fn retire_reader<T:Send+Sync+'static>(reader:&mut store::ArtifactCanonicalJsonReader<T>){for _ in 0..100000{if matches!(reader.close_step(close_policy()).unwrap(),RetainedCloneStep::Complete(_)){return;}}panic!("canonical reader did not retire");}
fn retire_owned_value<T:semio_framework_value::retirement::RetireOwned>(value:T){
 let birth=close_policy();
 let(mut owner,receipt)=semio_framework_value::retirement::admit_owned_retirement(value,birth).unwrap_or_else(|(error,_)|panic!("owned retirement birth: {error:?}"));assert!(receipt.fits(birth));
 for _ in 0..100000{if owner.terminal_is_empty(){break;}let step=owner.close_step(birth).unwrap();assert!(step.progress().fits(birth));if matches!(step,RetainedCloneStep::Complete(_)){break;}}
 assert!(owner.terminal_is_empty());
}

fn close(cursor:&mut BmpPublicationCursor){
 cursor.begin_close();
 for _ in 0..100000{
  let copy=cursor.next_close_copy_byte_demand().unwrap().max(3);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:cursor.next_close_release_byte_demand().unwrap(),maximum_depth:cursor.next_close_depth_demand().unwrap()};
  let step=cursor.close_step(grant).unwrap();assert!(step.progress().fits(grant));if matches!(step,RetainedCloneStep::Complete(_)){assert!(cursor.terminal_is_empty());return;}
 }panic!("original image publication close did not terminate");
}

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
        let inverse=cursor.take_inverse().unwrap();let before=base.image.region_rect(region).unwrap();let after=post.image.region_rect(region).unwrap();assert_eq!(inverse,if before!=after{vec![BmpMutation::ReplaceSamples(ReplaceSamples{region,indices:before.indices.clone(),samples:before.samples.clone()})]}else{Vec::new()});close(&mut cursor);
        if let Some(BmpMutation::ReplaceSamples(restore))=inverse.first(){
            let undo=RetainedCloneSource::from_authority(Arc::new(BmpMutation::ReplaceSamples(restore.clone())),());let current=RetainedCloneSource::from_authority(Arc::new(post.clone()),());let mut undone=post.clone();let mut undo_cursor=BmpPublicationCursor::new();let mut undo_turns=0;
            loop{undo_turns+=1;assert!(undo_turns<10000);let step=undo_cursor.advance(current.borrow(),&mut undone,undo.borrow(),grant).unwrap();if matches!(step,RetainedCloneEditStep::Complete(_)){break;}}
            assert_eq!(undone.image,base.image);assert_eq!(undo_cursor.take_inverse().unwrap(),vec![BmpMutation::ReplaceSamples(ReplaceSamples{region,indices:after.indices.clone(),samples:after.samples.clone()})]);close(&mut undo_cursor);
        }
        if post.image.profile==crate::schema::snapshot::BmpProfile::DirectRgb32 {let bytes=crate::standards::v_v3::subsets::any::io::encode_bmp(&post).unwrap();let (width,height,visual)=semio_s_artifact_stdio_bmp_test_oracle::standards::v_v3::subsets::any::oracle_visual_rgba8(&bytes).unwrap();assert_eq!((width,height),(post.image.width,post.image.height));assert_eq!(visual,crate::schema::operations::bmp_rgba8_preview(&post).unwrap());}
        retire_owned_value(inverse);
        eprintln!("[DEBUG] retained BMP publication case={} turns={turns} preciseSamples={}",row["name"],post.image.width*post.image.height);
    }
}
#[test]
fn retained_bmp_publication_cancellation_closes_partial_result_copy() {
    let source=RetainedCloneSource::from_authority(Arc::new(BmpSnapshot::default()),());let owner=Arc::new(BmpMutation::ReplaceImage(ReplaceImage{image:BmpImage::default()}));let mutation=RetainedCloneSource::from_authority(Arc::clone(&owner),());let mut post=BmpSnapshot::default();let mut cursor=BmpPublicationCursor::new();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_depth:64, maximum_release_bytes: 4096 };
    for _ in 0..300 {cursor.advance(source.borrow(),&mut post,mutation.borrow(),grant).unwrap();if cursor.phase==2&&cursor.copy.is_some(){break;}}
    cursor.cancel();close(&mut cursor);drop(mutation.into_owner());assert_eq!(Arc::strong_count(&owner),1);
}

#[test]
fn retained_bmp_publication_canonical_reader_preserves_neutral_owned_fields() {
    use semio_framework_value::FromValue;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json")).unwrap();
    let grant=encoding_grant(1,17);
    for case in fixture["cases"].as_array().unwrap() {
        let snapshot=BmpSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse_bytes(case["snapshot"].to_string().as_bytes(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())).unwrap();
        let snapshot=Arc::new(snapshot);
        let expected:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(snapshot.as_ref())).unwrap();
        let lifetime=Arc::downgrade(&snapshot);
        let mut reader=store::ArtifactCanonicalJsonReader::new(snapshot,Arc::new(SharedValueRetirementFactory::<BmpSnapshot>::default()));
        let mut encoded=Vec::new();let mut chunk=[0;17];let mut turns=0;
        while !reader.is_complete() {turns+=1;assert!(turns<100000);let step=reader.encode_chunk(grant,&mut chunk).unwrap();assert!(step.ownership.progress().fits(grant.retained_grant()));let count=step.written_bytes;assert!(count<=grant.maximum_copy_bytes);encoded.extend_from_slice(&chunk[..count]);}
        let actual:serde_json::Value=serde_json::from_slice(&encoded).unwrap();assert_eq!(actual,expected);assert_eq!(actual,case["snapshot"]);
        reader.begin_close();retire_reader(&mut reader);
        assert!(reader.terminal_is_empty());assert_eq!(lifetime.strong_count(),0);assert!(lifetime.upgrade().is_none());
        eprintln!("[DEBUG] retained bmp canonical neutral={} turns={turns} bytes={}",case["name"],encoded.len());
    }
}

#[test]
fn retained_bmp_publication_canonical_diff_preserves_neutral_owned_fields() {
    use crate::schema::diff::BmpDiff;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json")).unwrap();
    let grant=encoding_grant(1,17);
    for case in fixture["cases"].as_array().unwrap() {
        let snapshot=BmpSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse_bytes(case["snapshot"].to_string().as_bytes(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())).unwrap();
        for image in [None,Some(snapshot.image)] {
            let diff=Arc::new(BmpDiff{image,rects:Vec::new()});let lifetime=Arc::downgrade(&diff);
            let expected:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(diff.as_ref())).unwrap();
            let mut reader=store::ArtifactCanonicalJsonReader::new(diff,Arc::new(SharedValueRetirementFactory::<BmpDiff>::default()));
            let mut encoded=Vec::new();let mut chunk=[0;17];let mut turns=0;
            while !reader.is_complete(){turns+=1;assert!(turns<100000);let step=reader.encode_chunk(grant,&mut chunk).unwrap();assert!(step.ownership.progress().fits(grant.retained_grant()));let count=step.written_bytes;assert!(count<=grant.maximum_copy_bytes);encoded.extend_from_slice(&chunk[..count]);}
            let actual:serde_json::Value=serde_json::from_slice(&encoded).unwrap();assert_eq!(actual,expected);if actual.get("image").is_some(){assert_eq!(actual["image"],case["snapshot"]["image"]);}
            reader.begin_close();retire_reader(&mut reader);
            assert!(reader.terminal_is_empty());assert!(lifetime.upgrade().is_none());
            eprintln!("[DEBUG] retained BMP diff neutral={} turns={turns} bytes={}",case["name"],encoded.len());
        }
    }
}

#[test]
fn retained_bmp_publication_replacement_and_inverse_use_bounded_typed_image_owners(){
    use protocol::Mutation;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json")).unwrap();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_depth:64,maximum_release_bytes:4096};
    for row in fixture["cases"].as_array().unwrap().iter().chain(fixture["replacementCases"].as_array().unwrap().iter()){
        let target=BmpSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse_bytes(row["snapshot"].to_string().as_bytes(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())).unwrap();
        let base=BmpSnapshot::default();let mut post=base.clone();
        let source=RetainedCloneSource::from_authority(Arc::new(base.clone()),());let mutation=RetainedCloneSource::from_authority(Arc::new(BmpMutation::ReplaceImage(ReplaceImage{image:target.image.clone()})),());
        let mut cursor=BmpPublicationCursor::new();let mut turns=0;
        loop{
            turns+=1;assert!(turns<10000);
            if cursor.phase==8{
                let blocked=RetainedCloneGrant{maximum_capacity_bytes:0,..grant};
                assert_eq!(cursor.advance(source.borrow(),&mut post,mutation.borrow(),blocked).unwrap().progress(),RetainedCloneProgress::default());
                assert!(cursor.inverse.is_none());assert!(cursor.inverse_image.is_some());
            }
            let step=cursor.advance(source.borrow(),&mut post,mutation.borrow(),grant).unwrap();assert!(step.progress().fits(grant));
            if matches!(step,RetainedCloneEditStep::Complete(_)){break;}
        }
        assert_eq!(post,target);let inverse=cursor.take_inverse().unwrap();assert_eq!(inverse.len(),1);
        let restored=protocol::apply_diff(inverse[0].diff(&post).diff(),&post).unwrap();assert_eq!(restored,base);close(&mut cursor);
        retire_owned_value(inverse);
        eprintln!("[DEBUG] retained Bmp typed image replacement neutral={} turns={turns} inverseCapacityGranted=true nativeWordsExact=true",row["name"]);
    }
}
