//! 🧪️ Portable Python hashlib fixture and serde JSON independently validate original borrowed hashing.
use super::*;
#[test]
fn original_borrowed_record_matches_portable_oracle(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let source=fixture["source"].as_str().unwrap().to_owned();let domain=fixture["domain"].as_str().unwrap().as_bytes();let prefix=[0;32];
 assert_eq!(serde_json::to_string(&source).unwrap(),fixture["canonical"].as_str().unwrap());
 let mut digest=ArtifactCanonicalRecordDigest::default();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:64};
 let denied=RetainedCloneGrant{maximum_items:0,..grant};let turn=digest.advance(ArtifactCanonicalTreeSource::view(&source),domain,&prefix,denied);assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());
 for _ in 0..10000{let turn=digest.advance(ArtifactCanonicalTreeSource::view(&source),domain,&prefix,grant);assert!(turn.work<=1);turn.result.unwrap();if digest.digest().is_some(){break}}
 let expected:Vec<u8>=fixture["digest"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect();assert_eq!(digest.digest().unwrap().as_slice(),expected);
}

#[test]
fn batched_record_digest_matches_portable_oracle_over_the_size_grid(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️grid.json")).unwrap();let domain=fixture["domain"].as_str().unwrap().as_bytes();let prefix:Vec<u8>=fixture["prefix"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();
 let alphabet:Vec<char>=fixture["alphabet"].as_array().unwrap().iter().map(|text|text.as_str().unwrap().chars().next().unwrap()).collect();
 for row in fixture["rows"].as_array().unwrap(){
  let length=row["length"].as_u64().unwrap()as usize;let source:String=(0..length).map(|at|alphabet[at%alphabet.len()]).collect();
  assert_eq!(serde_json::to_string(&source).unwrap().len(),row["canonicalBytes"].as_u64().unwrap()as usize);
  let expected:Vec<u8>=row["digest"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();
  for items in [1,2,3,64,65,1000]{
   let grant=RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:1<<20,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:64};
   let mut digest=ArtifactCanonicalRecordDigest::default();let mut turns=0;
   while digest.digest().is_none(){let turn=digest.advance(ArtifactCanonicalTreeSource::view(&source),domain,&prefix,grant);assert!(turn.work<=1);turn.result.unwrap();turns+=1;assert!(turns<100000,"no progress at length {length} items {items}")}
   assert_eq!(digest.digest().unwrap().as_slice(),expected,"length {length} items {items}");
  }
 }
}
