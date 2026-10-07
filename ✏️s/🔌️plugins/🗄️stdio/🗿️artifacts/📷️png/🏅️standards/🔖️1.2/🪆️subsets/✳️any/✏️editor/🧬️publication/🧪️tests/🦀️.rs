//! 🧪️ Neutral publication vectors and independent PNG sample witnesses.
use super::*;
use semio_framework_value::retained_clone::RetainedCloneSource;
use crate::schema::snapshot::{PngTextChunk,PngTextKind,PngNativePaint};

fn close(cursor:&mut PngPublicationCursor) {
    cursor.begin_close();
    for _ in 0..10000 {if cursor.close_step(1,4096).unwrap()==SnapshotRetirementStep::Complete {assert!(cursor.terminal_is_empty());return;}}
    panic!("retained PNG cursor close did not terminate");
}
#[test]
fn retained_png_publication_clones_authoritative_result_and_validates_with_fuel() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut snapshot=PngSnapshot::default();snapshot.image.width=case["width"].as_u64().unwrap()as u32;snapshot.image.height=case["height"].as_u64().unwrap()as u32;
        let original:Vec<u16>=case["base"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u16).collect();snapshot.image.samples=original.repeat((snapshot.image.width*snapshot.image.height)as usize);
        snapshot.image.text_chunks.push(PngTextChunk{keyword:"Owner".into(),value:"é".repeat(case["textLength"].as_u64().unwrap()as usize),kind:PngTextKind::Text,compressed:false,language_tag:String::new(),translated_keyword:String::new()});
        let region=case["region"].clone();let region=PngRegion{x:region["x"].as_u64().unwrap()as u32,y:region["y"].as_u64().unwrap()as u32,width:region["width"].as_u64().unwrap()as u32,height:region["height"].as_u64().unwrap()as u32};
        let color:Vec<u16>=case["paint"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u16).collect();let paint=PngNativePaint::rgba(color[0],color[1],color[2],color[3]);
        let revision=crate::schema::operations::png_revision(&snapshot);
        let mut expected=snapshot.clone();for y in region.y..region.y+region.height {for x in region.x..region.x+region.width {let index=((y*snapshot.image.width+x)*4)as usize;expected.image.samples[index..index+4].copy_from_slice(&color);}}
        let source=RetainedCloneSource::from_authority(Arc::new(snapshot.clone()),());let mutation=RetainedCloneSource::from_authority(Arc::new(PngMutation::PaintNativeSamples(crate::schema::mutations::PaintNativeSamplesMutation{revision,region,paint,result:expected.clone()})),());
        let grant=RetainedCloneGrant{maximum_items:case["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:case["maximumBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:case["maximumBytes"].as_u64().unwrap()as usize,maximum_depth:64};
        let mut cursor=PngPublicationCursor::new();let mut post=snapshot.clone();let mut turns=0;
        loop {turns+=1;assert!(turns<10000);let step=cursor.advance(source.borrow(),&mut post,mutation.borrow(),grant).unwrap();assert!(step.progress().fits(grant));if matches!(step,RetainedCloneEditStep::Complete(_)){break;}}
        assert!(turns>case["minimumTurns"].as_u64().unwrap()as usize);assert_eq!(post,expected);
        let inverse=cursor.take_inverse().unwrap();assert_eq!(inverse,vec![PngMutation::SetSnapshot(SetSnapshot{snapshot:snapshot.clone()})]);close(&mut cursor);
        let mut bytes=Vec::new();{let mut encoder=png::Encoder::new(&mut bytes,post.image.width,post.image.height);encoder.set_color(png::ColorType::Rgba);encoder.set_depth(png::BitDepth::Eight);let mut writer=encoder.write_header().unwrap();writer.write_image_data(&expected.image.samples.iter().map(|v|*v as u8).collect::<Vec<_>>()).unwrap();}
        let mut reader=png::Decoder::new(std::io::Cursor::new(bytes)).read_info().unwrap();let mut decoded=vec![0;reader.output_buffer_size().unwrap()];let info=reader.next_frame(&mut decoded).unwrap();assert_eq!(decoded[..info.buffer_size()],post.image.samples.iter().map(|v|*v as u8).collect::<Vec<_>>());
        let mut retirement=semio_framework_value::retirement::owned_retirement(inverse);while retirement.close_step(1,4096).unwrap()!=SnapshotRetirementStep::Complete {} assert!(retirement.terminal_is_empty());
        eprintln!("[DEBUG] retained PNG publication case={} turns={turns} exactSamples={}",case["id"],post.image.samples.len());
    }
}
#[test]
fn retained_png_publication_cancellation_releases_mutation_projection_lease() {
    let snapshot=Arc::new(PngSnapshot::default());let source=RetainedCloneSource::from_authority(Arc::clone(&snapshot),());let owner=Arc::new(PngMutation::SetSnapshot(SetSnapshot{snapshot:PngSnapshot::default()}));let mutation=RetainedCloneSource::from_authority(Arc::clone(&owner),());let mut post=PngSnapshot::default();
    let mut cursor=PngPublicationCursor::new();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_depth:64};
    for _ in 0..100 {cursor.advance(source.borrow(),&mut post,mutation.borrow(),grant).unwrap();if cursor.phase==2&&cursor.copy.is_some(){break;}}
    cursor.cancel();close(&mut cursor);drop(mutation.into_owner());assert_eq!(Arc::strong_count(&owner),1);drop(source.into_owner());assert_eq!(Arc::strong_count(&snapshot),1);
}
