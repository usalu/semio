//! ♻️ Native original alias batches conserve exact paused custody and every heap currency.
use super::*;
use semio_framework_value::{paged::PagedUtf8,ArtifactOwnedValueRetirementFactory,retirement::{OwnedValueRetirementFactory,shared::FactorySharedRetirement}};
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
#[test]
fn tool_original_alias_batch_preserves_original_admission_and_exact_heap_cleanup(){
    type P=PagedUtf8<{usize::MAX}>;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let root=Arc::new(P::from("original🧩".repeat(4096)));
    let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<P>>=Arc::new(OwnedValueRetirementFactory::<P>::default());
    for count in fixture["counts"].as_array().unwrap(){for pause in fixture["pauses"].as_array().unwrap(){
        let count=count.as_u64().unwrap()as usize;let pause=pause.as_u64().unwrap()as usize;
        let mut original=Some(Vec::with_capacity(if count==0{8192}else{count}));
        for _ in 0..count{original.as_mut().unwrap().push(Arc::clone(&root));}
        let capacity=original.as_ref().unwrap().capacity();let address=original.as_ref().unwrap().as_ptr();
        let birth=OriginalAliasBatch::<P>::constructor_demand();
        let (refused,heap)=crate::value::observe_retirement_allocations(||OriginalAliasBatch::admit_original(&mut original,RetainedCloneGrant{maximum_copy_bytes:birth.copy_bytes-1,..grant(birth)}));
        assert!(refused.is_err());assert_eq!((heap.0,heap.1),(0,0));assert_eq!(original.as_ref().unwrap().as_ptr(),address);assert_eq!(original.as_ref().unwrap().capacity(),capacity);assert_eq!(Arc::strong_count(&root),count+1);
        let (admitted,heap)=crate::value::observe_retirement_allocations(||OriginalAliasBatch::admit_original(&mut original,grant(birth)));
        let(mut batch,receipt)=admitted.unwrap().unwrap();assert!(original.is_none());assert!(receipt.fits(grant(birth)));assert_eq!((heap.0,heap.1),(0,0));
        let alias_birth=FactorySharedRetirement::<P>::constructor_capacity_bytes();
        let zero=RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0};
        let (step,heap)=crate::value::observe_retirement_allocations(||batch.advance(alias_birth,zero,|alias,g|FactorySharedRetirement::admit_original(alias,&factory,g)).unwrap());
        assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((heap.0,heap.1),(0,0));assert_eq!(Arc::strong_count(&root),count+1);
        for turn in 0..fixture["maximumTurns"].as_u64().unwrap(){
            if batch.terminal_is_empty(){break;}
            let (demand,heap)=crate::value::observe_retirement_allocations(||batch.next_demand(alias_birth).unwrap());assert_eq!((heap.0,heap.1),(0,0));assert!(demand.copy_bytes+demand.capacity_bytes<=4096);
            if turn==pause as u64{assert!(Arc::strong_count(&root)>=1);}
            let g=grant(demand);let(step,heap)=crate::value::observe_retirement_allocations(||batch.advance(alias_birth,g,|alias,g|FactorySharedRetirement::admit_original(alias,&factory,g)).unwrap());
            assert!(step.progress().fits(g));assert_eq!((heap.0,heap.1),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
            if matches!(step,RetainedCloneStep::Complete(_)){assert!(batch.terminal_is_empty());}
        }
        assert!(batch.terminal_is_empty());assert_eq!(Arc::strong_count(&root),1);
        let(_,heap)=crate::value::observe_retirement_allocations(||drop(batch));assert_eq!((heap.0,heap.1),(0,0));
    }}
    println!("[DEBUG] original tool alias batches0/1/65/257 preserve zero/below-grant pointers and exact factory custody, one native alias per turn, empty spare8192 backing, four-axis heap receipts and terminal Drop0heap");
}
#[test]
fn single_original_alias_preserves_root_and_issuer_without_batch_reconstruction(){
 type P=PagedUtf8<{usize::MAX}>;
 let root=Arc::new(P::from("single🧩".repeat(4096)));
 let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<P>>=Arc::new(OwnedValueRetirementFactory::<P>::default());
 let birth=OriginalAliasBatch::<P>::constructor_demand();
 for pause in[0,1,3,17,129]{
  let mut original=Some(Arc::clone(&root));let pointer=Arc::as_ptr(original.as_ref().unwrap());
  let(refused,heap)=crate::value::observe_retirement_allocations(||OriginalAliasBatch::admit_alias_original(&mut original,RetainedCloneGrant{maximum_copy_bytes:birth.copy_bytes-1,..grant(birth)}));assert!(refused.is_err());assert_eq!(heap,(0,0));assert_eq!(Arc::as_ptr(original.as_ref().unwrap()),pointer);
  let(admitted,heap)=crate::value::observe_retirement_allocations(||OriginalAliasBatch::admit_alias_original(&mut original,grant(birth)));let(mut owner,receipt)=admitted.unwrap().unwrap();assert!(original.is_none());assert!(receipt.fits(grant(birth)));assert_eq!(heap,(0,0));assert!(owner.original.is_none());
  for turn in 0..100000{
   if owner.terminal_is_empty(){break;}
   let(d,heap)=crate::value::observe_retirement_allocations(||owner.next_demand(FactorySharedRetirement::<P>::constructor_capacity_bytes()).unwrap());assert_eq!(heap,(0,0));assert!(d.copy_bytes+d.capacity_bytes<=4096);
   if turn==pause{assert!(Arc::strong_count(&root)>=1);}
   let g=grant(d);let(step,heap)=crate::value::observe_retirement_allocations(||owner.advance(FactorySharedRetirement::<P>::constructor_capacity_bytes(),g,|alias,g|FactorySharedRetirement::admit_original(alias,&factory,g)).unwrap());assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
  }
  assert!(owner.terminal_is_empty());assert_eq!(Arc::strong_count(&root),1);let((),heap)=crate::value::observe_retirement_allocations(||drop(owner));assert_eq!(heap,(0,0));
 }
 println!("[DEBUG] single original alias retains root pointer and actual issuer through0/1/3/17/129 pauses, constructor undergrant0heap, no invented Vec, exact four-axis heaps,100000 turns and terminalDrop0");
}
