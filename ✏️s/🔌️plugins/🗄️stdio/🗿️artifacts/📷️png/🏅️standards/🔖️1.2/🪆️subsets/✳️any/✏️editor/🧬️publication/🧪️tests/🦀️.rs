//! 🧪️ Neutral publication vectors and independent PNG sample witnesses.
use super::*;
use semio_framework_value::retained_clone::RetainedCloneSource;
use semio_framework_value::retirement::SharedValueRetirementFactory;
use crate::schema::snapshot::{PngTextChunk,PngTextKind,PngNativePaint};

fn close(cursor:&mut PngPublicationCursor){
 cursor.begin_close();
 for _ in 0..100000{
  let copy=cursor.next_close_copy_byte_demand().unwrap().max(3);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:cursor.next_close_release_byte_demand().unwrap(),maximum_depth:cursor.next_close_depth_demand().unwrap()};
  let step=cursor.close_step(grant).unwrap();assert!(step.progress().fits(grant));if matches!(step,RetainedCloneStep::Complete(_)){assert!(cursor.terminal_is_empty());return;}
 }panic!("original image publication close did not terminate");
}

#[test]
fn retained_png_publication_prepares_authored_intent_and_validates_with_fuel() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut snapshot=PngSnapshot::default();snapshot.image.width=case["width"].as_u64().unwrap()as u32;snapshot.image.height=case["height"].as_u64().unwrap()as u32;
        let original:Vec<u16>=case["base"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u16).collect();snapshot.image.samples=original.repeat((snapshot.image.width*snapshot.image.height)as usize);
        snapshot.image.text_chunks.push(PngTextChunk{keyword:"Owner".into(),value:"é".repeat(case["textLength"].as_u64().unwrap()as usize),kind:PngTextKind::Text,compressed:false,language_tag:String::new(),translated_keyword:String::new()});
        let region=case["region"].clone();let region=PngRegion{x:region["x"].as_u64().unwrap()as u32,y:region["y"].as_u64().unwrap()as u32,width:region["width"].as_u64().unwrap()as u32,height:region["height"].as_u64().unwrap()as u32};
        let color:Vec<u16>=case["paint"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u16).collect();let paint=PngNativePaint::rgba(color[0],color[1],color[2],color[3]);
        let revision=crate::schema::operations::png_revision(&snapshot);
        let mut expected=snapshot.clone();for y in region.y..region.y+region.height {for x in region.x..region.x+region.width {let index=((y*snapshot.image.width+x)*4)as usize;expected.image.samples[index..index+4].copy_from_slice(&color);}}
        let source=RetainedCloneSource::from_authority(Arc::new(snapshot.clone()),());let mutation=RetainedCloneSource::from_authority(Arc::new(PngMutation::PaintNativeSamples(crate::schema::mutations::PaintNativeSamplesMutation{revision,region,paint})),());
        let grant=RetainedCloneGrant {maximum_items:case["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:case["maximumBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:case["maximumBytes"].as_u64().unwrap()as usize,maximum_depth:64, maximum_release_bytes: case["maximumBytes"].as_u64().unwrap()as usize };
        let mut cursor=PngPublicationCursor::new();let mut post=snapshot.clone();let mut turns=0;
        loop {turns+=1;assert!(turns<10000);let step=cursor.advance(source.borrow(),&mut post,mutation.borrow(),grant).unwrap();assert!(step.progress().fits(grant));if matches!(step,RetainedCloneEditStep::Complete(_)){break;}}
        assert!(turns>case["minimumTurns"].as_u64().unwrap()as usize);assert_eq!(post,expected);
        let inverse=cursor.take_inverse().unwrap();assert_eq!(inverse,vec![PngMutation::ReplaceSamples(crate::schema::mutations::ReplaceSamples{region,samples:snapshot.image.region_samples(region).unwrap()})]);close(&mut cursor);
        let mut bytes=Vec::new();{let mut encoder=png::Encoder::new(&mut bytes,post.image.width,post.image.height);encoder.set_color(png::ColorType::Rgba);encoder.set_depth(png::BitDepth::Eight);let mut writer=encoder.write_header().unwrap();writer.write_image_data(&expected.image.samples.iter().map(|v|*v as u8).collect::<Vec<_>>()).unwrap();}
        let mut reader=png::Decoder::new(std::io::Cursor::new(bytes)).read_info().unwrap();let mut decoded=vec![0;reader.output_buffer_size().unwrap()];let info=reader.next_frame(&mut decoded).unwrap();assert_eq!(decoded[..info.buffer_size()],post.image.samples.iter().map(|v|*v as u8).collect::<Vec<_>>());
        let mut retirement=semio_framework_value::retirement::owned_retirement(inverse);while retirement.close_step(1,4096).unwrap()!=SnapshotRetirementStep::Complete {} assert!(retirement.terminal_is_empty());
        eprintln!("[DEBUG] retained PNG publication case={} turns={turns} exactSamples={}",case["id"],post.image.samples.len());
    }
}
#[test]
fn retained_png_publication_cancellation_releases_mutation_projection_lease() {
    let snapshot=Arc::new(PngSnapshot::default());let source=RetainedCloneSource::from_authority(Arc::clone(&snapshot),());let owner=Arc::new(PngMutation::ReplaceImage(ReplaceImage{image:PngImage::default()}));let mutation=RetainedCloneSource::from_authority(Arc::clone(&owner),());let mut post=PngSnapshot::default();
    let mut cursor=PngPublicationCursor::new();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_depth:64, maximum_release_bytes: 4096 };
    for _ in 0..100 {cursor.advance(source.borrow(),&mut post,mutation.borrow(),grant).unwrap();if cursor.phase==2&&cursor.copy.is_some(){break;}}
    cursor.cancel();close(&mut cursor);drop(mutation.into_owner());assert_eq!(Arc::strong_count(&owner),1);drop(source.into_owner());assert_eq!(Arc::strong_count(&snapshot),1);
}

#[test]
fn retained_png_publication_canonical_reader_preserves_neutral_owned_fields() {
    use semio_framework_value::FromValue;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json")).unwrap();
    let grant=store::ArtifactStoreOneItemGrant{maximum_items:1,maximum_bytes:17};
    for case in fixture["cases"].as_array().unwrap() {
        let snapshot=PngSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse_bytes(case["snapshot"].to_string().as_bytes(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())).unwrap();
        let snapshot=Arc::new(snapshot);
        let expected:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(snapshot.as_ref())).unwrap();
        let lifetime=Arc::downgrade(&snapshot);
        let mut reader=store::ArtifactCanonicalJsonReader::new(snapshot,Arc::new(SharedValueRetirementFactory::<PngSnapshot>::default()));
        let mut encoded=Vec::new();let mut chunk=[0;17];let mut turns=0;
        while !reader.is_complete() {turns+=1;assert!(turns<100000);let count=reader.encode_chunk(grant,&mut chunk).unwrap();assert!(count<=grant.maximum_bytes);encoded.extend_from_slice(&chunk[..count]);}
        let actual:serde_json::Value=serde_json::from_slice(&encoded).unwrap();assert_eq!(actual,expected);assert_eq!(actual,case["snapshot"]);
        reader.begin_close();for _ in 0..100000 {if reader.close_step(store::ArtifactStoreOneItemGrant{maximum_items:1,maximum_bytes:4096}).unwrap()==SnapshotRetirementStep::Complete {break;}}
        assert!(reader.terminal_is_empty());assert_eq!(lifetime.strong_count(),0);assert!(lifetime.upgrade().is_none());
        eprintln!("[DEBUG] retained png canonical neutral={} turns={turns} bytes={}",case["name"],encoded.len());
    }
}

#[test]
fn retained_png_publication_canonical_diff_preserves_neutral_owned_fields() {
    use semio_framework_value::FromValue;
    use crate::schema::diff::PngDiff;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json")).unwrap();
    let grant=store::ArtifactStoreOneItemGrant{maximum_items:1,maximum_bytes:17};
    for case in fixture["cases"].as_array().unwrap() {
        let snapshot=PngSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse_bytes(case["snapshot"].to_string().as_bytes(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())).unwrap();
        for image in [None,Some(snapshot.image)] {
            let diff=Arc::new(PngDiff{image,..PngDiff::default()});let lifetime=Arc::downgrade(&diff);
            let expected:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(diff.as_ref())).unwrap();
            let mut reader=store::ArtifactCanonicalJsonReader::new(diff,Arc::new(SharedValueRetirementFactory::<PngDiff>::default()));
            let mut encoded=Vec::new();let mut chunk=[0;17];let mut turns=0;
            while !reader.is_complete(){turns+=1;assert!(turns<100000);let count=reader.encode_chunk(grant,&mut chunk).unwrap();assert!(count<=grant.maximum_bytes);encoded.extend_from_slice(&chunk[..count]);}
            let actual:serde_json::Value=serde_json::from_slice(&encoded).unwrap();assert_eq!(actual,expected);if actual.get("image").is_some(){assert_eq!(actual["image"],case["snapshot"]["image"]);}
            reader.begin_close();for _ in 0..100000 {if reader.close_step(store::ArtifactStoreOneItemGrant{maximum_items:1,maximum_bytes:4096}).unwrap()==SnapshotRetirementStep::Complete{break;}}
            assert!(reader.terminal_is_empty());assert!(lifetime.upgrade().is_none());
            eprintln!("[DEBUG] retained PNG diff neutral={} turns={turns} bytes={}",case["name"],encoded.len());
        }
    }
}


#[test]
fn retained_png_publication_replays_neutral_intent_profiles() {
    use semio_framework_value::FromValue;
    use crate::schema::snapshot::PngNativeProfile;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let snapshot=PngSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse_bytes(row["snapshot"].to_string().as_bytes(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())).unwrap();
        let p=&row["paint"];let region=PngRegion{x:p["x"].as_u64().unwrap()as u32,y:p["y"].as_u64().unwrap()as u32,width:p["width"].as_u64().unwrap()as u32,height:p["height"].as_u64().unwrap()as u32};
        let profile=match p["profile"].as_str().unwrap(){"indexed"=>PngNativeProfile::Indexed,"grayscale"=>PngNativeProfile::Grayscale,"grayscale-alpha"=>PngNativeProfile::GrayscaleAlpha,"rgb"=>PngNativeProfile::Rgb,"rgba"=>PngNativeProfile::Rgba,_=>panic!("unknown neutral native profile")};
        let sample=|key:&str|p[key].as_u64().unwrap()as u16;let paint=PngNativePaint{profile,first:sample("first"),second:sample("second"),third:sample("third"),fourth:sample("fourth")};
        let source=RetainedCloneSource::from_authority(Arc::new(snapshot.clone()),());
        let mutation=RetainedCloneSource::from_authority(Arc::new(PngMutation::PaintNativeSamples(crate::schema::mutations::PaintNativeSamplesMutation{revision:crate::schema::operations::png_revision(&snapshot),region,paint})),());
        let mut cursor=PngPublicationCursor::new();let mut post=snapshot.clone();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_depth:64, maximum_release_bytes: 4096 };let mut turns=0;
        loop {turns+=1;assert!(turns<100000);let step=cursor.advance(source.borrow(),&mut post,mutation.borrow(),grant).unwrap();assert!(step.progress().fits(grant));if matches!(step,RetainedCloneEditStep::Complete(_)){break;}}
        let expected:Vec<u16>=row["expectedSamples"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u16).collect();assert_eq!(post.image.samples,expected);assert!(post.image.same_metadata(&snapshot.image));
        let restores=if post.image.samples==snapshot.image.samples{Vec::new()}else{vec![PngMutation::ReplaceSamples(crate::schema::mutations::ReplaceSamples{region,samples:snapshot.image.region_samples(region).unwrap()})]};assert_eq!(cursor.take_inverse().unwrap(),restores);close(&mut cursor);
        eprintln!("[DEBUG] retained PNG authored intent neutral={} turns={turns} exactSamples={}",row["name"],post.image.samples.len());
    }
}


#[test]
fn retained_png_publication_metadata_intents_match_neutral_targets() {
    use semio_framework_value::FromValue;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json")).unwrap();
    let value=|row:&serde_json::Value|semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse_bytes(row.to_string().as_bytes(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    for row in fixture["metadataEdits"].as_array().unwrap() {
        let snapshot=PngSnapshot::from_value(value(&row["snapshot"])).unwrap();
        let target=PngSnapshot::from_value(value(&row["expected"])).unwrap();
        let mutation=PngMutation::ReplaceImage(ReplaceImage{image:target.image});
        assert!(PngPublication.preflight(&mutation,store::HistoryLane::Document).is_ok());
        let source=RetainedCloneSource::from_authority(Arc::new(snapshot.clone()),());let mutation=RetainedCloneSource::from_authority(Arc::new(mutation),());let mut post=snapshot;let mut cursor=PngPublicationCursor::new();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_depth:64, maximum_release_bytes: 4096 };let mut turns=0;let mut paused=false;
        loop {
            turns+=1;assert!(turns<100000);
            if cursor.phase==8&&!paused {
                for zero in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:0,..grant}] {
                    let step=cursor.advance(source.borrow(),&mut post,mutation.borrow(),zero).unwrap();assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(&post,source.borrow().get());
                }
                paused=true;
            }
            let step=cursor.advance(source.borrow(),&mut post,mutation.borrow(),grant).unwrap();assert!(step.progress().fits(grant));if matches!(step,RetainedCloneEditStep::Complete(_)){break;}
        }
        assert!(paused);
        let expected=PngSnapshot::from_value(value(&row["expected"])).unwrap();assert_eq!(post,expected);let inverse=cursor.take_inverse().unwrap();assert_eq!(inverse,vec![PngMutation::ReplaceImage(ReplaceImage{image:source.borrow().get().image.clone()})]);close(&mut cursor);
        eprintln!("[DEBUG] retained PNG metadata intent neutral={} turns={turns} exactMetadata=true",row["name"]);
    }
}

#[test]
fn retained_png_publication_replacement_and_inverse_use_bounded_typed_image_owners(){
    use protocol::Mutation;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json")).unwrap();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_depth:64,maximum_release_bytes:4096};
    for row in fixture["cases"].as_array().unwrap().iter(){
        let target=PngSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse_bytes(row["snapshot"].to_string().as_bytes(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())).unwrap();
        let base=PngSnapshot::default();let mut post=base.clone();
        let source=RetainedCloneSource::from_authority(Arc::new(base.clone()),());let mutation=RetainedCloneSource::from_authority(Arc::new(PngMutation::ReplaceImage(ReplaceImage{image:target.image.clone()})),());
        let mut cursor=PngPublicationCursor::new();let mut turns=0;
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
        let mut retirement=semio_framework_value::retirement::owned_retirement(inverse);while retirement.close_step(1,4096).unwrap()!=SnapshotRetirementStep::Complete{}assert!(retirement.terminal_is_empty());
        eprintln!("[DEBUG] retained Png typed image replacement neutral={} turns={turns} inverseCapacityGranted=true nativeWordsExact=true",row["name"]);
    }
}
