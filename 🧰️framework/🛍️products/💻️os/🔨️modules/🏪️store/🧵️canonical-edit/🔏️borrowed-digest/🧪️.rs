//! 🧪️ Portable Python hashlib fixture and serde JSON independently validate original borrowed hashing.
use super::*;
#[test]
fn original_borrowed_record_matches_portable_oracle(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let source=fixture["source"].as_str().unwrap().to_owned();let domain=fixture["domain"].as_str().unwrap().as_bytes();let prefix=[0;32];
 assert_eq!(serde_json::to_string(&source).unwrap(),fixture["canonical"].as_str().unwrap());
 let mut digest=ArtifactCanonicalRecordDigest::default();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:64};
 let denied=RetainedCloneGrant{maximum_items:0,..grant};assert_eq!(digest.advance(ArtifactCanonicalTreeSource::view(&source),domain,&prefix,denied).unwrap().progress(),Default::default());
 for _ in 0..10000{digest.advance(ArtifactCanonicalTreeSource::view(&source),domain,&prefix,grant).unwrap();if digest.digest().is_some(){break}}
 let expected:Vec<u8>=fixture["digest"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect();assert_eq!(digest.digest().unwrap().as_slice(),expected);
}
