//! 🧊️ Cold native setup uses the actual issued storage and its unique destructive owner.
use semio_framework_value_allocation_return::original::{SharedOriginalStorage,SharedOriginalRelease};
#[test]
fn cold_storage_issues_the_same_original_alias_family(){
 let law:serde_json::Value=serde_json::from_str(include_str!("cold.json")).unwrap();let value=law["value"].as_u64().unwrap();let (storage,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||SharedOriginalStorage::from_cold(value));let extent=SharedOriginalStorage::<u64>::allocation_bytes();assert_eq!(heap,(extent,0));let pointer=storage.identity();assert_eq!(*storage.get(),value);assert_eq!(storage.aliases(),law["counts"][0].as_u64().unwrap()as usize);
 let(alias,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||storage.alias());assert_eq!(heap,(0,0));assert!(storage.same_original(&alias));assert_eq!(alias.identity(),pointer);assert_eq!(alias.aliases(),law["counts"][1].as_u64().unwrap()as usize);
 let(released,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||storage.retire());assert_eq!(heap,(0,0));assert!(matches!(released,SharedOriginalRelease::Alias));assert_eq!(alias.aliases(),law["counts"][2].as_u64().unwrap()as usize);
 let(original,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||alias.retire());assert_eq!(heap,(0,0));let SharedOriginalRelease::Original(mut original)=original else{panic!("issued aliases omitted their original owner")};assert_eq!(original.identity(),pointer);assert!(original.physical_is_unique());let(output,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||original.take_value());assert_eq!(heap,(0,extent));assert_eq!(output,Some(value));assert!(original.terminal_is_empty());
}

#[test]
fn issued_native_election_uses_the_same_finite_receipt_and_resident_output(){
 use semio_framework_value_testing_effect_bank::{EffectBank,Wallet,RetainedWorkPoll};
 use semio_framework_value_control_contract::{RetainedCloneGrant,RetainedCloneProgress};
 use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
 let law:serde_json::Value=serde_json::from_str(include_str!("cold.json")).unwrap();
 let mut first=SharedOriginalStorage::from_cold(law["value"].as_u64().unwrap());let mut second=first.alias();let pointer=first.identity();let extent=SharedOriginalStorage::<u64>::allocation_bytes();let mut elected=None;let mut output=None;
 let stock=RetainedCloneGrant{maximum_items:32,maximum_copy_bytes:8192,maximum_capacity_bytes:0,maximum_release_bytes:4096,maximum_depth:8};let mut bank=EffectBank::new(Wallet::new(stock,64));
 let demand=first.retire_into_demand().unwrap();let grant=bank.loan(1,demand.copy_bytes-1);
 let((poll,marker),heap)=observe(||bank.run(grant,4,|effect|RetainedWorkPoll::Complete(first.elect_native_into(&mut elected,effect))));
 let RetainedWorkPoll::Complete(turn)=poll else{panic!("closed native election became pending")};assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(marker,None);assert_eq!(heap,(0,0));assert_eq!(first.identity(),pointer);assert!(elected.is_none());assert_eq!(bank.take_admitted(),None);bank.post(Default::default(),grant,heap);
 for(index,storage)in [&mut first,&mut second].into_iter().enumerate(){
  let demand=storage.retire_into_demand().unwrap();let grant=bank.loan(1,demand.copy_bytes);let((poll,marker),heap)=observe(||bank.run(grant,4,|effect|RetainedWorkPoll::Complete(storage.elect_native_into(&mut elected,effect))));
  let RetainedWorkPoll::Complete(turn)=poll else{panic!("closed native election became pending")};assert_eq!(turn.work,law["nativeWork"][index].as_u64().unwrap()as u128);let progress=turn.result.unwrap().progress();assert!(progress.copied_bytes<=demand.copy_bytes);assert_eq!(marker,None);assert_eq!(heap,(0,0));assert!(storage.terminal_is_empty());assert_eq!(elected.is_some(),index==1);assert_eq!(bank.take_admitted(),Some(true));bank.post(progress,grant,heap);
 }
 let owner=elected.as_mut().unwrap();assert_eq!(owner.identity(),pointer);let demand=owner.take_value_demand().unwrap();let grant=bank.loan(1,demand.copy_bytes);
 let((poll,marker),heap)=observe(||bank.run(grant,3,|effect|RetainedWorkPoll::Complete(owner.take_value_step(&mut output,effect))));
 let RetainedWorkPoll::Complete(turn)=poll else{panic!("closed native handback became pending")};assert_eq!(turn.work,0);assert!(turn.result.unwrap().is_none());assert!(output.is_none());assert_eq!(owner.identity(),pointer);assert_eq!(marker,None);assert_eq!(heap,(0,0));assert_eq!(bank.take_admitted(),None);bank.post(Default::default(),grant,heap);
 let((poll,marker),heap)=observe(||bank.run(grant,4,|effect|RetainedWorkPoll::Complete(owner.take_value_step(&mut output,effect))));
 let RetainedWorkPoll::Complete(turn)=poll else{panic!("closed native handback became pending")};assert_eq!(turn.work,law["nativeWork"][2].as_u64().unwrap()as u128);let progress:RetainedCloneProgress=turn.result.unwrap().unwrap();assert_eq!(heap,(0,extent));assert_eq!(progress.released_bytes,extent);assert_eq!(marker,None);assert_eq!(output,Some(law["value"].as_u64().unwrap()));assert!(owner.terminal_is_empty());assert_eq!(bank.take_admitted(),Some(true));bank.post(progress,grant,heap);
}
