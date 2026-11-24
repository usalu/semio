//! 📄️ Paged canonical sealing yields the exact length and digest of the one-event walk and of an independently framed SHA256.
use protocol::{io::text::canonical::ArtifactCanonicalEditSealCursor,Edit};
use semio_framework_hash::Sha256;
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
use semio_framework_value::{RetirementDemand,retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::{RetainedCloneGrant,RetainedCloneSource}};
use semio_framework_value_clone_source_testing::{admit_owned,drive::drive};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
const STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:4194304,maximum_copy_bytes:16777216,maximum_capacity_bytes:16777216,maximum_release_bytes:33554432,maximum_depth:256};
const CPU:u64=1048576;

fn hex(bytes:[u8;32])->String{bytes.iter().map(|byte|format!("{byte:02x}")).collect()}

fn exact(demand:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}

fn generous()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,..STOCK}}

fn stage<M:RetireOwned+Sync+'static>(owner:Box<Edit<M>>)->RetainedCloneSource<Box<Edit<M>>>{let mut bank=EffectBank::new(Wallet::new(STOCK,CPU));let admitted=admit_owned(&mut bank,owner,(),STOCK).result;let source=match admitted{Ok((source,_))=>source,Err(_)=>panic!("staged edit source admission")};source}

fn seal<M:RetireOwned+Sync+'static+ArtifactCanonicalJsonTree>(edit:Edit<M>,paged:bool)->(u64,[u8;32],usize){seal_with(edit,paged,1)}

fn seal_with<M:RetireOwned+Sync+'static+ArtifactCanonicalJsonTree>(edit:Edit<M>,paged:bool,work_factor:u64)->(u64,[u8;32],usize){
 let mut source=Some(stage(Box::new(edit)));
 let(mut cursor,_)=ArtifactCanonicalEditSealCursor::admit_source(&mut source,generous()).unwrap().unwrap();
 let mut turns=0;
 while !cursor.is_ready(){
  turns+=1;assert!(turns<10_000_000,"seal did not settle");
  let grant=if paged{generous()}else{exact(cursor.next_demand().unwrap())};
  let quote=cursor.next_work_demand().unwrap();assert_ne!(quote,0,"an unsettled seal turn quotes the work of its own transition");let units=quote*work_factor;let step=drive(grant,units,|effect|cursor.advance(effect)).result.unwrap_or_else(|_|panic!("seal turn refused"));assert!(step.progress().fits(grant));
 }
 let(length,digest)=(cursor.canonical_length(),cursor.digest().unwrap());
 let(edit,_,_)=cursor.take_edit(generous()).unwrap().unwrap();
 let mut owner=ControlledRetirement::new(edit).unwrap_or_else(|(error,_)|panic!("sealed edit retirement: {error}"));
 let body=generous().maximum_copy_bytes;for _ in 0..1_000_000{if owner.terminal_is_empty(){break;}let units=owner.next_computational_work(body).unwrap();let step=drive(generous(),units,|effect|owner.step(body,effect)).result.unwrap_or_else(|_|panic!("sealed edit retirement turn refused"));assert!(step.progress().fits(generous()));}
 assert!(owner.terminal_is_empty());assert!(cursor.terminal_is_empty());
 (length,digest,turns)
}

fn framed_digest(id:&str,canonical:&[u8])->[u8;32]{
 let mut hash=Sha256::new();
 hash.update(b"semio.artifact.cursor.v2");hash.update(&4u64.to_be_bytes());hash.update(b"edit");hash.update(&(id.len()as u64).to_be_bytes());hash.update(id.as_bytes());hash.update(&(canonical.len()as u64).to_be_bytes());hash.update(canonical);
 hash.finalize()
}

#[test]
fn paged_seal_digests_equal_the_one_event_seal_and_the_independent_framed_oracle(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for(index,row)in cases["cases"].as_array().unwrap().iter().enumerate(){
  let decode=||semio_framework_pack_json::from_json_str::<Edit<bool>>(&serde_json::to_string(&row["edit"]).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let(paged_length,paged_digest,paged_turns)=seal(decode(),true);
  let(exact_length,exact_digest,exact_turns)=seal(decode(),false);
  assert_eq!((paged_length,paged_digest),(exact_length,exact_digest));
  assert_eq!(paged_length,fixture["rows"][index]["canonicalLength"].as_u64().unwrap());
  assert_eq!(hex(paged_digest),fixture["rows"][index]["digest"].as_str().unwrap());
  assert!(paged_turns*4<exact_turns,"case {index}: {paged_turns} paged vs {exact_turns} one-event turns");
 }
 for count in [0usize,1,9,700,3000]{
  let samples:Vec<u16>=(0..count).map(|index|((index*40503+12345)%65536)as u16).collect();
  let inverse:Vec<u16>=samples.iter().rev().copied().collect();
  let json=serde_json::json!({"id":"heavy","forwards":[samples],"inverse":[inverse],"sequenceNumber":3,"startedAt":"now","line":null});
  let decode=||semio_framework_pack_json::from_json_str::<Edit<Vec<u16>>>(&json.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let canonical=format!("{{\"id\":\"heavy\",\"forwards\":[{}],\"inverse\":[{}],\"startedAt\":\"now\",\"line\":null}}",serde_json::to_string(&json["forwards"][0]).unwrap(),serde_json::to_string(&json["inverse"][0]).unwrap());
  let(paged_length,paged_digest,paged_turns)=seal(decode(),true);
  let(exact_length,exact_digest,exact_turns)=seal(decode(),false);
  assert_eq!((paged_length,paged_digest),(exact_length,exact_digest),"count {count}");
  assert_eq!(paged_length,canonical.len()as u64);
  assert_eq!(paged_digest,framed_digest("heavy",canonical.as_bytes()),"count {count}: independent framed digest");
  if count>=700{assert!(paged_turns*40<exact_turns,"count {count}: {paged_turns} paged vs {exact_turns} one-event turns");}
 }
}

/// 🪜️ A turn funded with many work units per step stops at its first page allocation and resumes on the next turn; it never reserves a second page inside one effect.
#[test]
fn a_wide_work_turn_seals_the_same_digest_as_a_one_step_turn(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📄️wide-work.json")).unwrap();
 for count in fixture["counts"].as_array().unwrap().iter().map(|count|count.as_u64().unwrap() as usize){
  let samples:Vec<u16>=(0..count).map(|index|((index*40503+12345)%65536)as u16).collect();
  let json=serde_json::json!({"id":"wide","forwards":[samples],"inverse":[samples],"sequenceNumber":3,"startedAt":"now","line":null});
  let decode=||semio_framework_pack_json::from_json_str::<Edit<Vec<u16>>>(&json.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let(one_length,one_digest,_)=seal_with(decode(),true,1);
  let(wide_length,wide_digest,_)=seal_with(decode(),true,fixture["workFactor"].as_u64().unwrap());
  assert_eq!((wide_length,wide_digest),(one_length,one_digest),"count {count}");
 }
}
