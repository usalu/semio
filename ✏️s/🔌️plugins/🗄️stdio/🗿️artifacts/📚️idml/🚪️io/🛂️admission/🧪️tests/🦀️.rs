use semio_s_artifact_stdio_idml::{IdmlSnapshot,io::admission::*};
use crate::schema::IdmlPart;

fn grant(work:usize)->IdmlAdmissionGrant {IdmlAdmissionGrant{maximum_work:work,maximum_copy_bytes:2048,maximum_capacity_bytes:1024,maximum_release_bytes:0}}
fn finish(cursor:&mut IdmlAdmissionCursor,source:IdmlAdmissionSource<'_>,work:usize)->IdmlAdmissionStep {
    for _ in 0..1000000 {let step=cursor.advance(source,grant(work));assert!(step.work<=work);assert!(step.copied_bytes<=2048);assert!(step.retained_capacity_bytes<=1024);if step.done||step.error.is_some(){return step;}}
    panic!("IDML admission stalled");
}
fn close(cursor:&mut IdmlAdmissionCursor){cursor.begin_close();while cursor.next_release_bytes()!=0{let bytes=cursor.next_release_bytes();assert_eq!(cursor.close_one(bytes-1).released_bytes,0);assert_eq!(cursor.close_one(bytes).released_bytes,bytes);}assert!(cursor.terminal_is_empty());}
fn snapshot(row:&serde_json::Value)->IdmlSnapshot {
    IdmlSnapshot{dom_version:row["version"].as_str().unwrap().into(),parts:row["parts"].as_array().unwrap().iter().map(|row|{let text=row[1].as_str().unwrap();IdmlPart{path:row[0].as_str().unwrap().into(),bytes:if text=="$mime"{crate::PACKAGE_MIME.as_bytes().into()}else{text.as_bytes().into()}}}).collect()}
}
#[test]
fn portable_package_admission_and_cold_codec_agree(){
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧪️fixtures/🔣️.json")).unwrap();
    for row in corpus["cases"].as_array().unwrap(){let snapshot=snapshot(row);let valid=row["valid"].as_bool().unwrap();
        for work in [1,7,64,4096]{let mut cursor=IdmlAdmissionCursor::new();let step=finish(&mut cursor,IdmlAdmissionSource::snapshot(&snapshot),work);assert_eq!(step.error.is_none(),valid,"{} {:?}",row["id"],step.error);if let Some(error)=row["error"].as_str(){assert_eq!(step.error,Some(error),"{}",row["id"]);}if valid{let view=step.version.unwrap();assert_eq!(view.decoded_length,snapshot.dom_version.len());let mut decoder=IdmlVersionDecoder::new(view);let mut text=String::new();loop{let step=decoder.advance(&snapshot.parts[view.part_index].bytes,work);assert!(step.work<=work);assert!(step.error.is_none());if let Some(byte)=step.byte{text.push(byte as char);}if step.done{break;}}assert_eq!(text,snapshot.dom_version);}close(&mut cursor);}
        assert_eq!(snapshot.package().is_ok(),valid,"{}",row["id"]);
    }
}
#[test]
fn numeric_pages_and_original_overlay_have_exact_lifecycles(){
    let mut snapshot=IdmlSnapshot{dom_version:"8.0".into(),parts:vec![IdmlPart{path:"mimetype".into(),bytes:crate::PACKAGE_MIME.as_bytes().into()},IdmlPart{path:"designmap.xml".into(),bytes:b"<Document DOMVersion=\"8.0\"/>".into()}]};
    for index in 0..2049{snapshot.parts.push(IdmlPart{path:format!("Stories/{index}.xml"),bytes:b"<Story/>".into()});}
    let source=IdmlAdmissionSource::snapshot(&snapshot);let mut cursor=IdmlAdmissionCursor::new();
    while cursor.next_capacity_bytes()==0{let step=cursor.advance(source,IdmlAdmissionGrant{maximum_capacity_bytes:1023,..grant(7)});assert!(step.error.is_none());}
    assert_eq!(cursor.next_copy_bytes(),2048);let step=cursor.advance(source,IdmlAdmissionGrant{maximum_copy_bytes:2047,..grant(7)});assert_eq!(step.retained_capacity_bytes,0);assert_eq!(step.work,0);
    assert!(finish(&mut cursor,source,7).done);close(&mut cursor);
    for cut in [0,1,20,200]{let mut cursor=IdmlAdmissionCursor::new();for _ in 0..cut{cursor.advance(source,grant(7));}close(&mut cursor);}
    let replacement=b"<Document DOMVersion=\"9&#46;0\"/>";let overlay=IdmlAdmissionSource{snapshot:&snapshot,replacement:Some(IdmlPartReplacement{index:1,bytes:replacement}),compare_version:false};let mut cursor=IdmlAdmissionCursor::new();let step=finish(&mut cursor,overlay,64);assert!(step.done);assert_eq!(step.version.unwrap().decoded_length,3);close(&mut cursor);
}

#[test]
fn admitted_index_looks_up_original_unicode_member_names_under_single_byte_grants(){
    let snapshot=IdmlSnapshot{dom_version:"8.0".into(),parts:vec![IdmlPart{path:"mimetype".into(),bytes:crate::PACKAGE_MIME.as_bytes().into()},IdmlPart{path:"designmap.xml".into(),bytes:b"<Document DOMVersion=\"8.0\"/>".into()},IdmlPart{path:"Stories/日本語.xml".into(),bytes:b"<Story/>".into()}]};
    let source=IdmlAdmissionSource::snapshot(&snapshot);let mut cursor=IdmlAdmissionCursor::new();assert!(finish(&mut cursor,source,7).done);
    for (name,expected) in [("mimetype",Some(0)),("designmap.xml",Some(1)),("Stories/日本語.xml",Some(2)),("missing.xml",None)]{
        for work in [1,7,64,4096]{let mut lookup=IdmlNameLookup::new();assert_eq!(cursor.lookup(source,name,&mut lookup,0).work,0);loop{let step=cursor.lookup(source,name,&mut lookup,work);assert!(step.work<=work);assert!(step.error.is_none());if step.done{assert_eq!(step.index,expected);break;}}}
    }close(&mut cursor);
}
