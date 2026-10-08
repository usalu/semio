//! 🧪️ Actual production metadata model and IO leaf with independent ID3 witnesses.
extern crate semio_framework_value_derive as value_derive;
#[path="../../../🧬️schema/📸️snapshot/🏷️metadata/🦀️.rs"]
pub mod admitted;
pub mod standards{pub mod mpeg1_layer3{pub mod subsets{pub mod any{pub mod schema{pub mod snapshot{pub use crate::admitted::*;}}}}}}
#[path="../🦀️.rs"]
pub mod native;
#[cfg(test)]
mod tests{
 use super::*;use id3::TagLike;use std::io::Cursor;
 #[test]
 fn actual_production_metadata_matches_independent_id3(){
  let mut oracle=id3::Tag::new();oracle.set_title("Café");oracle.set_artist("Semio");
  for version in [id3::Version::Id3v23,id3::Version::Id3v24]{let mut bytes=Vec::new();oracle.write_to(&mut bytes,version).unwrap();let(owned,_)=native::decode_id3v2(&bytes).unwrap();assert_eq!(owned.frames[0].content,admitted::Id3Content::Text{values:vec!["Café".into()]});let encoded=native::encode_id3v2(&owned).unwrap();let reopened=id3::Tag::read_from(Cursor::new(&encoded)).unwrap();assert_eq!(reopened.title(),Some("Café"));assert_eq!(reopened.artist(),Some("Semio"));assert_eq!(native::decode_id3v2(&encoded).unwrap().0,owned);}
  println!("[DEBUG] Actual production ID3 semantic model/IO admits independent v2.3/v2.4 and canonical UTF8 native output");
 }
 #[test]
 fn named_id3v1_fields_have_independent_native_witness(){
  let tag=admitted::Id3v1Tag{title:"Café".into(),artist:"Semio".into(),album:"Album".into(),year:"2026".into(),comment:"Comment".into(),track:Some(7),genre:Some(13)};let bytes=native::encode_id3v1(&tag).unwrap();let oracle=id3::v1::Tag::read_from(Cursor::new(bytes)).unwrap();assert_eq!(oracle.title,"Café");assert_eq!(oracle.artist,"Semio");assert_eq!(oracle.track,Some(7));assert_eq!(native::decode_id3v1(&bytes).unwrap(),tag);let mut padded=bytes;for byte in &mut padded[7..33]{*byte=32;}assert_eq!(native::decode_id3v1(&padded).unwrap(),tag);println!("[DEBUG] Named ID3v1 title/artist/track admitted from independent id3 v1 native witness");
 }
 #[test]
 fn different_native_text_encodings_have_equal_admitted_values_and_independent_oracles(){
  let variants=[vec![0,67,97,102,233],vec![1,255,254,67,0,97,0,102,0,233,0],vec![2,0,67,0,97,0,102,0,233],vec![3,67,97,102,195,169]];
  let expected=admitted::Id3Frame{id:"TIT2".into(),content:admitted::Id3Content::Text{values:vec!["Café".into()]}};
  for body in variants{assert_eq!(native::decode_id3_frame("TIT2".into(),&body).unwrap(),expected);let mut bytes=b"ID3\x04\x00\x00".to_vec();bytes.extend_from_slice(&native::encode_syncsafe((10+body.len())as u32));bytes.extend_from_slice(b"TIT2");bytes.extend_from_slice(&native::encode_syncsafe(body.len()as u32));bytes.extend_from_slice(&[0,0]);bytes.extend_from_slice(&body);assert_eq!(id3::Tag::read_from(Cursor::new(bytes)).unwrap().title(),Some("Café"));}
  assert!(native::decode_id3_frame("TIT2".into(),&[3,255]).is_err());assert!(native::decode_id3_frame("TIT2".into(),&[1,65,0]).is_err());assert!(native::decode_id3_frame("GEOB".into(),&[0,65]).is_err());assert!(native::encode_id3_frame_body(&admitted::Id3Frame{id:"TIT2".into(),content:admitted::Id3Content::Opaque{bytes:vec![0,65]}}).is_err());
  println!("[DEBUG] Four authored native Latin1/UTF16LE/UTF16BE/UTF8 vectors equal one semantic value and independent id3 title");
 }
 #[test]
 fn native_granted_body_slices_equal_canonical_reconstruction(){
  let frame=admitted::Id3Frame{id:"TXXX".into(),content:admitted::Id3Content::UserText{description:"source 世界".into(),values:vec!["Café".into(),"Semio".into()]}};let bytes=native::encode_id3_frame_body(&frame).unwrap();assert_eq!(native::id3_frame_body_len(&frame).unwrap(),bytes.len());
  for grant in [1,4,16]{let mut offset=0;let mut output=Vec::new();while offset<bytes.len(){let part=native::id3_frame_body_slice(&frame,offset,grant).unwrap();assert!(!part.is_empty()&&part.len()<=grant);offset+=part.len();output.extend(part);}assert_eq!(output,bytes);}
  assert_eq!(native::decode_id3_frame(frame.id.clone(),&bytes).unwrap(),frame);println!("[DEBUG] Actual native semantic body reconstruction respects neutral1/4/16 byte output slices");
 }

 #[test]
 fn shipped_real_mpeg_metadata_and_actual_dsl_schema_are_admitted(){
  let(tag,_)=native::decode_id3v2(include_bytes!("../../../🧫️fixtures/🔊️.mp3")).unwrap();let title=tag.frames.iter().find(|f|f.id=="TIT2").unwrap();assert_eq!(title.content,admitted::Id3Content::Text{values:vec!["Bauen mit Bestand (Ausschnitt)".into()]});let text=semio_framework_dsl_record::print(&tag.__dsl_to_record(),&admitted::Id3v2Tag::__dsl_spec(),semio_framework_dsl_record::JoinMode::Inline);assert!(!text.contains("data="));assert!(!text.contains("major-version"));println!("[DEBUG] Actual production metadata DSL source {text}");
 }

 #[test]
 fn actual_derived_metadata_records_follow_authored_grammar_and_protocol(){
  let fixture=semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(include_str!("../../../🧫️fixtures/🏷️metadata.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let tag=semio_framework_value::FromValue::from_value(fixture.get("id3v2").unwrap().clone()).unwrap();let tag:admitted::Id3v2Tag=tag;
  let mut grammar=semio_framework_dsl::parse_grammar(include_str!("../../📝️text/📸️snapshot/📖️.grammar.semio")).unwrap();grammar.start="id3v2-tag".into();let recognizer=semio_framework_dsl::Recognizer::compile(&grammar,&semio_framework_dsl::FragmentRegistry::new(),vec![]).unwrap();
  let text=semio_framework_dsl_record::print(&tag.__dsl_to_record(),&admitted::Id3v2Tag::__dsl_spec(),semio_framework_dsl_record::JoinMode::Inline);assert!(recognizer.recognize(&text).unwrap(),"{text}");let parsed=semio_framework_dsl_record::parse_exact(&text,&admitted::Id3v2Tag::__dsl_spec(),&semio_framework_dsl_record::ParseOptions::default()).unwrap();assert_eq!(admitted::Id3v2Tag::__dsl_from_record(&parsed).unwrap(),tag);
  let encoded=native::encode_id3v2(&tag).unwrap();assert_eq!(native::decode_id3v2(&encoded).unwrap().0,tag);let oracle=id3::Tag::read_from(Cursor::new(&encoded)).unwrap();assert_eq!(oracle.title(),Some("Café"));assert_eq!(oracle.comments().next().unwrap().text,"Grüße");assert_eq!(oracle.lyrics().next().unwrap().text,"Line one\nLine two");assert_eq!(oracle.pictures().next().unwrap().data,vec![137,80,78,71]);
  grammar.start="id3v1-tag".into();let recognizer=semio_framework_dsl::Recognizer::compile(&grammar,&semio_framework_dsl::FragmentRegistry::new(),vec![]).unwrap();for(track,genre)in [(None,None),(Some(7),Some(13))]{let tag=admitted::Id3v1Tag{title:"Café".into(),track,genre,..Default::default()};let text=semio_framework_dsl_record::print(&tag.__dsl_to_record(),&admitted::Id3v1Tag::__dsl_spec(),semio_framework_dsl_record::JoinMode::Inline);assert!(recognizer.recognize(&text).unwrap(),"{text}");}
  let protocol=semio_framework_dsl::parse_protocol(include_str!("../../💾️binary/📸️snapshot/📡️.protocol.semio")).unwrap();assert_eq!(protocol.schema,"stdio.mp3");println!("[DEBUG] Actual eight-variant metadata DSL records and named optional v1 records match authored grammar; independent id3 reads semantic text/comment/lyrics/picture");
 }

}
