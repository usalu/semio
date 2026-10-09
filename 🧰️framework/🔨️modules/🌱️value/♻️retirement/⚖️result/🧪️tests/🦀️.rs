use super::*;
#[test]
fn original_result_variants_preserve_native_payload_and_all_axis_receipts(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 let g=&law["grant"];let grant=RetainedCloneGrant{maximum_items:g["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:g["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:g["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:g["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:g["maximumDepth"].as_u64().unwrap()as usize};
 for variant in law["variants"].as_array().unwrap(){for repeat in law["repeats"].as_array().unwrap(){
  let n=repeat.as_u64().unwrap()as usize;let ok=variant.as_str().unwrap()=="Ok";
  let(source,born)=crate::value::observe_retirement_allocations(||{let text="original🧩".repeat(n);if ok{Ok(text)}else{Err(text)}});let source_pointer=match &source{Ok(v)|Err(v)=>v.as_ptr()};let mut direct=OriginalResult{original:ManuallyDrop::new(Some(source)),active:ManuallyDrop::new(None)};let(mut direct_births,mut direct_releases,mut direct_turns)=(born.0,born.1,0);
  while !direct.terminal_is_empty(){direct_turns+=1;assert!(direct_turns<100000);if let Some(original)=direct.original.as_ref(){assert_eq!(match original{Ok(v)|Err(v)=>v.as_ptr()},source_pointer);assert_eq!(original.is_ok(),ok);}if let Some(active)=direct.active.as_ref(){match active{ActiveResult::Ok(v)=>{assert!(ok);if let Some(v)=v.original(){assert_eq!(v.as_ptr(),source_pointer);}},ActiveResult::Err(v)=>{assert!(!ok);if let Some(v)=v.original(){assert_eq!(v.as_ptr(),source_pointer);}}}}
   let(step,heap)=crate::value::observe_retirement_allocations(||direct.close_step(grant));let progress=match step{RetirementStep::Progress(p)=>p,RetirementStep::Complete=>Default::default(),_=>panic!("original Result direct source failed")};assert!(progress.fits(grant));assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),heap);direct_births+=heap.0;direct_releases+=heap.1;
  }let(_,heap)=crate::value::observe_retirement_allocations(||drop(direct));assert_eq!(heap,(0,0));assert_eq!(direct_births,direct_releases);
  let(original,born)=crate::value::observe_retirement_allocations(||{let text="original🧩".repeat(n);if ok{Ok(text)}else{Err(text)}});let pointer=match &original{Ok(v)|Err(v)=>v.as_ptr()};
  let oracle=serde_json::to_value(&original).unwrap();assert_eq!(oracle[variant.as_str().unwrap()],"original🧩".repeat(n));
  let(mut owner,heap)=crate::value::observe_retirement_allocations(||ControlledRetirement::new(original).unwrap());assert_eq!(heap,(0,0));
  let(mut births,mut releases,mut turns)=(born.0,born.1,0);
  while !owner.terminal_is_empty(){
   turns+=1;assert!(turns<100000);let((copy,capacity,release,depth),heap)=crate::value::observe_retirement_allocations(||(owner.next_copy_byte_demand().unwrap(),owner.next_capacity_byte_demand(grant.maximum_copy_bytes).unwrap(),owner.next_release_byte_demand().unwrap(),owner.next_depth_demand().unwrap()));assert_eq!(heap,(0,0));assert!(copy<=grant.maximum_copy_bytes&&capacity<=grant.maximum_capacity_bytes&&release<=grant.maximum_release_bytes&&depth<=grant.maximum_depth);
   for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:copy.saturating_sub(1),..grant},RetainedCloneGrant{maximum_capacity_bytes:capacity.saturating_sub(1),..grant},RetainedCloneGrant{maximum_release_bytes:release.saturating_sub(1),..grant},RetainedCloneGrant{maximum_depth:depth.saturating_sub(1),..grant}].into_iter().enumerate(){
    if denied.0==1&&copy==0||denied.0==2&&capacity==0||denied.0==3&&release==0||denied.0==4&&depth==0{continue;}
    let(step,heap)=crate::value::observe_retirement_allocations(||owner.step(denied.1));assert_eq!(heap,(0,0));match step{Ok(step)=>assert_eq!(step.progress(),RetainedCloneProgress::default()),Err(e)=>assert_eq!(e.retained_progress(),RetainedCloneProgress::default())};
    if let Some(original)=owner.original(){assert_eq!(match original{Ok(v)|Err(v)=>v.as_ptr()},pointer);assert_eq!(original.is_ok(),ok);}
   }
   let transfer=owner.original().is_some()&&capacity==size_of::<OriginalResult<String,String>>();if transfer{assert_eq!(copy,size_of::<Result<String,String>>());}
   let(step,heap)=crate::value::observe_retirement_allocations(||owner.step(grant).unwrap());if transfer{assert_eq!(step.progress().copied_bytes,size_of::<Result<String,String>>());}assert!(step.progress().fits(grant));assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),heap);assert_eq!(owner.step_progress(),step.progress());births+=heap.0;releases+=heap.1;
  }
  let(_,heap)=crate::value::observe_retirement_allocations(||drop(owner));assert_eq!(heap,(0,0));assert_eq!(births,releases);println!("[DEBUG] original Result {} repeat{n} pointer{pointer:?} turns{turns} birth{births} release{releases}, independent axis denials0/0 and terminalDrop0",variant.as_str().unwrap());
 }}
}
