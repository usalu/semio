use semio_framework_test_fixtures::fixture_text;
use super::*;
fn grant(copy:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1}}
fn lookup(key:&str,candidates:&[String],page:usize,expected:Option<usize>,cut:Option<usize>){
 let pointers:Vec<*const u8>=candidates.iter().map(|value|value.as_ptr()).collect();let key_pointer=key.as_ptr();let(mut cursor,heap)=crate::observe_retirement_allocations(ExactKeyCursor::new);assert_eq!(heap,(0,0));let mut turns=0;
 while !cursor.ready()&&cut.is_none_or(|cut|turns<cut){
  let candidate=candidates.get(cursor.next_index()).map(|candidate|candidate.as_bytes());let body=ExactKeyCursor::fixed_copy_bytes()+2*page;let demand=cursor.demand(key.as_bytes(),candidate,body).unwrap();let issued=grant(demand.copy_bytes);
  for denied in[RetainedCloneGrant{maximum_items:0,..issued},RetainedCloneGrant{maximum_copy_bytes:ExactKeyCursor::fixed_copy_bytes()-1,..issued},RetainedCloneGrant{maximum_depth:0,..issued}]{let before=cursor;let(step,heap)=crate::observe_retirement_allocations(||cursor.advance(key.as_bytes(),candidate,denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(cursor,before);}
  let(step,heap)=crate::observe_retirement_allocations(||cursor.advance(key.as_bytes(),candidate,issued).unwrap());assert_eq!(heap,(0,0));assert_eq!(step.progress().retained_capacity_bytes,0);assert_eq!(step.progress().released_bytes,0);assert!(step.progress().fits(issued));turns+=1;assert!(turns<2*key.len()+2*candidates.len()+16);
 }
 if cut.is_none(){assert!(cursor.ready());assert_eq!(cursor.found(),expected);}
 assert_eq!(key.as_ptr(),key_pointer);for(index,candidate)in candidates.iter().enumerate(){assert_eq!(candidate.as_ptr(),pointers[index]);}assert_eq!(crate::observe_retirement_allocations(||{let _=cursor;}).1,(0,0));
}
/// 🔑️ Portable opaque UTF8 identities settle only after exact equality under original byte grants.
#[test]
fn exact_key_lookup_matches_portable_opaque_identity_without_heap_work(){
 let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../🔑️lookup/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){let key=row["key"].as_str().unwrap();let candidates:Vec<String>=row["candidates"].as_array().unwrap().iter().map(|value|value.as_str().unwrap().to_owned()).collect();for page in[row["pageBytes"].as_u64().unwrap()as usize,1,32768]{lookup(key,&candidates,page,row["expected"].as_u64().map(|value|value as usize),None);}}
 eprintln!("[DEBUG] eight exact opaque UTF8 identities pass at three original page widths, denied item/copy/depth unchanged, empty key/missing/prefix/case/Unicode exact, native constructor/advance/drop zero heap");
}
/// 📦️ Long common prefixes remain resumable without copied source text or partial-key authority.
#[test]
fn exact_key_lookup_long_prefixes_and_cancellation_preserve_originals(){
 let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../🔑️lookup/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let row=&fixture["wide"];let prefix="p".repeat(row["prefixLength"].as_u64().unwrap()as usize);let key=format!("{prefix}{}",row["wanted"].as_str().unwrap());let candidates:Vec<String>=row["suffixes"].as_array().unwrap().iter().map(|value|format!("{prefix}{}",value.as_str().unwrap())).collect();
 for cut in std::iter::once(None).chain(row["cancelCuts"].as_array().unwrap().iter().map(|value|Some(value.as_u64().unwrap()as usize))){lookup(&key,&candidates,row["pageBytes"].as_u64().unwrap()as usize,row["expected"].as_u64().map(|value|value as usize),cut);}
 let mut cursor=ExactKeyCursor::new();let issued=grant(ExactKeyCursor::fixed_copy_bytes()+2);cursor.advance(b"same",Some(b"same"),issued).unwrap();assert!(cursor.demand(b"same",Some(b"xx"),4096).is_err());
 eprintln!("[DEBUG] actual65537-byte common prefix settles exact second original key under paged grants and preserves source/cursor custody at five cancellation frontiers; changed partial candidate refuses");
}
