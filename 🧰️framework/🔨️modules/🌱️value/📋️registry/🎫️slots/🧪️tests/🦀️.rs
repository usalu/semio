use super::*;

/// 🧪️ The portable admission corpus preserves exact keys and fixed slots through stale releases.
#[test]
fn fixed_key_slots_match_portable_admission_and_stale_release(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 let (mut owner,heap)=crate::observe_retirement_allocations(FixedKeySlots::<u64,3>::new);
 assert_eq!(heap,(0,0));
 let mut leases=std::collections::BTreeMap::new();
 let mut oracle=std::collections::BTreeMap::new();
 assert_eq!(law["capacity"],3);
 for action in law["actions"].as_array().unwrap(){
  let key=action["key"].as_u64().unwrap();
  match action["kind"].as_str().unwrap(){
   "claim"=>{let(slot,heap)=crate::observe_retirement_allocations(||owner.claim(key));assert_eq!(heap,(0,0));assert_eq!(slot.map(|slot|slot.index as u64),action["index"].as_u64());if let Some(slot)=slot{assert_eq!(slot.generation.to_string(),action["generation"].as_str().unwrap());leases.insert((key,slot.generation),slot);oracle.insert(key,slot.index);}},
   "lookup"=>{let(slot,heap)=crate::observe_retirement_allocations(||owner.lookup(key));assert_eq!(heap,(0,0));assert_eq!(slot.map(|slot|slot.index as u64),action["index"].as_u64());assert_eq!(slot.map(|slot|slot.index),oracle.get(&key).copied());},
   "release"=>{let generation=action["generation"].as_str().unwrap().parse::<u64>().unwrap();let slot=*leases.get(&(key,generation)).or_else(||leases.get(&(33,generation))).unwrap();let(accepted,heap)=crate::observe_retirement_allocations(||owner.release(key,slot));assert_eq!(heap,(0,0));assert_eq!(accepted,action["accepted"].as_bool().unwrap());if accepted{oracle.remove(&key);}},
   _=>panic!("unknown fixed slot action"),
  }
  assert_eq!(owner.len(),oracle.len());
 }
 assert!(owner.is_empty());
 let(mut a,mut b)=(FixedKeySlots::<u64,1>::new(),FixedKeySlots::<u64,1>::new());
 let own=a.claim(1).unwrap();let foreign=b.claim(1).unwrap();
 assert!(!a.release(1,foreign));assert_eq!(a.lookup(1),Some(own));assert!(a.release(1,own));assert!(b.release(1,foreign));
 let mut exhausted=FixedKeySlots::<u64,1>::new();exhausted.generations[0]=u64::MAX;assert!(exhausted.claim(1).is_none());
 assert!(FixedKeySlots::<u64,0>::new().claim(1).is_none());
 println!("[DEBUG] {} exact fixed-slot fixture transitions preserve foreign and stale ownership; exhausted generations refuse; original registry construction/claim/lookup/release allocate and release zero heap bytes",law["actions"].as_array().unwrap().len());
}
