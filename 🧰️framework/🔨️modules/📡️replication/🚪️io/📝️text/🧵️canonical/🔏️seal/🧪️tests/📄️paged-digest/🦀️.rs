//! 📄️ Paged canonical sealing yields the exact length and digest of the one-event walk and of an independently framed SHA256.
use protocol::{io::text::canonical::ArtifactCanonicalEditSealCursor,Edit};
use semio_framework_hash::Sha256;
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
use semio_framework_value::{RetirementDemand,retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::RetainedCloneGrant};

fn hex(bytes:[u8;32])->String{bytes.iter().map(|byte|format!("{byte:02x}")).collect()}

fn exact(demand:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}

fn generous()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<16,maximum_capacity_bytes:1<<16,maximum_release_bytes:1<<20,maximum_depth:4096}}

fn seal<M:RetireOwned+ArtifactCanonicalJsonTree>(edit:Edit<M>,paged:bool)->(u64,[u8;32],usize){
 let mut original=Some(Box::new(edit));
 let(mut cursor,_)=ArtifactCanonicalEditSealCursor::admit_original(&mut original,generous()).unwrap().unwrap();
 let mut turns=0;
 while !cursor.is_ready(){
  turns+=1;assert!(turns<10_000_000,"seal did not settle");
  let grant=if paged{generous()}else{exact(cursor.next_demand().unwrap())};
  let step=cursor.advance(grant).unwrap();assert!(step.progress().fits(grant));
 }
 let(length,digest)=(cursor.canonical_length(),cursor.digest().unwrap());
 let(edit,_,_)=cursor.take_edit(generous()).unwrap().unwrap();
 let mut owner=ControlledRetirement::new(edit).unwrap_or_else(|(error,_)|panic!("sealed edit retirement: {error}"));
 for _ in 0..1_000_000{if owner.terminal_is_empty(){break;}owner.step(generous()).unwrap();}
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
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🔣️.json")).unwrap();
 for(index,row)in cases["cases"].as_array().unwrap().iter().enumerate(){
  let decode=||semio_framework_pack_json::from_json_str::<Edit<bool>>(&serde_json::to_string(&row["edit"]).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let(paged_length,paged_digest,paged_turns)=seal(decode(),true);
  let(exact_length,exact_digest,exact_turns)=seal(decode(),false);
  assert_eq!((paged_length,paged_digest),(exact_length,exact_digest));
  assert_eq!(paged_length,fixture["rows"][index]["canonicalLength"].as_u64().unwrap());
  assert_eq!(hex(paged_digest),fixture["rows"][index]["digest"].as_str().unwrap());
  assert!(paged_turns*4<exact_turns,"case {index}: {paged_turns} paged vs {exact_turns} one-event turns");
  println!("[DEBUG] seal fixture case={index} paged={paged_turns} oneEvent={exact_turns} digest {}",hex(paged_digest));
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
  println!("[DEBUG] seal Vec<u16> samples={count} bytes={paged_length} paged={paged_turns} oneEvent={exact_turns}");
 }
}
