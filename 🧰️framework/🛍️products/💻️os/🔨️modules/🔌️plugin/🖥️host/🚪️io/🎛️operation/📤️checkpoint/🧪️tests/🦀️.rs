use semio_framework_value::{NativeEncodeControl,native_encoding::NativeEncodeProgress,ValueRefusalKind};
#[test]
fn projected_checkpoint_serializes_actual_postcharge_receipt_and_keeps_every_canceled_buffer(){
    use crate::os_vcs::io::binary::entity_identity::control::{EntityIdentityAuthority,OriginalOperationReceiver};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let actor=fixture["actor"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect::<Vec<_>>();
    let g=&fixture["retirement"];let grant=semio_framework_value::RetainedCloneGrant{maximum_items:g["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:g["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:g["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:g["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:g["maximumDepth"].as_u64().unwrap()as usize};
    assert!(usize::MAX.ilog10()+2<=fixture["projectedReceipt"]["maximumDecimalConvergencePasses"].as_u64().unwrap()as u32);
    let mut boundaries=0;
    let mut cancellation=0;
    loop{
        if cancellation>boundaries&&cancellation!=0{break;}
        let calls=std::cell::Cell::new(0usize);let enabled=std::cell::Cell::new(false);
        let mut observer=|_:NativeEncodeProgress|{if enabled.get(){calls.set(calls.get()+1);}!enabled.get()||cancellation==0||calls.get()!=cancellation};
        let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
        let continuation=NativeEncodeControl::new(fixture["originalMaximumBytes"].as_u64().unwrap()as usize,&mut observer).pause().unwrap();
        let continuation=continuation.with_retirement_recipient(&mut recipient).unwrap();
        let mut identity=EntityIdentityAuthority::resume_retirement(continuation,&mut observer);
        identity.encode(|control|control.charge(fixture["projectedReceipt"]["initialCharge"].as_u64().unwrap()as usize)).unwrap();
        let mut receiver=OriginalOperationReceiver::new(&mut identity);receiver.begin().unwrap();receiver.finish(0).unwrap();
        enabled.set(true);
        let (result,allocated,released)=backing(||receiver.with_host_io(|scope|{
            let mut metadata=|projection:&crate::os_vcs::io::binary::entity_identity::control::OriginalOperationReceiptProjection<'_>,control:&mut NativeEncodeControl<'_>,output:Option<&mut Vec<u8>>|{let mut writer=super::super::input::JsonInputWriter{control,output,count:0};super::super::input::encode_borrowed(&mut writer,projection)?;Ok(writer.count)};
            let mut visit=|sink:&mut dyn FnMut(&[u8])->Result<(),semio_framework_value::ValueError>|sink(&actor);
            super::encode_projected_checkpoint_frame(scope,&mut metadata,&mut visit)
        }));
        enabled.set(false);
        assert_eq!(released,0);
        let receipt=receiver.pause();let scalar=serde_json::to_value(&receipt).unwrap();
        assert!(scalar["finished"].as_bool().unwrap());assert_eq!(scalar["received_retirement"].as_u64().unwrap()as usize,allocated);
        let mut returned=match result{
            Ok(bytes)=>{assert_eq!(cancellation,0);boundaries=calls.get();let length=u32::from_le_bytes(bytes[8..12].try_into().unwrap())as usize;assert_eq!(&bytes[..8],fixture["magic"].as_str().unwrap().as_bytes());assert_eq!(serde_json::from_slice::<serde_json::Value>(&bytes[20..20+length]).unwrap(),scalar);assert_eq!(&bytes[20+length..],actor.as_slice());assert_eq!(bytes.len()+std::mem::size_of::<super::super::input::JsonBackingOwner>(),allocated);assert!(grant.maximum_items>0&&grant.maximum_depth>0&&grant.maximum_copy_bytes>=std::mem::size_of::<super::super::input::JsonBackingOwner>());Some(super::super::input::JsonBackingOwner{bytes:std::mem::ManuallyDrop::new(Some(bytes))})}
            Err(error)=>{assert_ne!(cancellation,0);assert_eq!(error.kind,ValueRefusalKind::Canceled);None}
        };
        let mut receiver=OriginalOperationReceiver::resume(receipt,&mut identity).unwrap();receiver.ensure_finished().unwrap();
        receiver.retire_host(|control|{assert_eq!(control.owned_bytes(),fixture["projectedReceipt"]["initialCharge"].as_u64().unwrap()as usize+allocated);while control.has_retirement_owner(){control.close_retirement_recipient(grant)?;}Ok(())}).unwrap();
        if let Some(owner)=returned.as_mut(){use semio_framework_value::ErasedSnapshotRetirement;while !owner.terminal_is_empty(){let capacity=owner.next_capacity_byte_demand(grant.maximum_copy_bytes).unwrap();let (step,physical,release)=backing(||receiver.retire_host(|control|{control.charge(capacity)?;control.checkpoint()?;owner.close_step(grant)}));let progress=step.unwrap().progress();assert!(progress.fits(grant));assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(physical,release));}let(_,physical,release)=backing(||drop(returned.take()));assert_eq!((physical,release),(0,0));}
        let receipt=receiver.pause();let receiver=OriginalOperationReceiver::resume(receipt,&mut identity).unwrap();receiver.ensure_finished().unwrap();drop(receiver.pause());
        drop(identity.pause_retirement().unwrap().into_parts());
        assert!(!recipient.has_owner());
        cancellation+=1;
    }
    println!("[DEBUG] projected checkpoint serializes the actual uniquely retained postcharge receipt; every original cancellation retains physical backing until funded close");
}
fn backing<T>(operation:impl FnOnce()->T)->(T,usize,usize){let(result,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(operation);assert!(!physical.overflowed);(result,physical.requested_bytes,physical.released_bytes)}
#[test]
fn borrowed_host_checkpoint_frame_preserves_exact_bytes_and_original_failure_owner(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let actor=fixture["actor"].as_array().unwrap().iter().map(|x|x.as_u64().unwrap()as u8).collect::<Vec<_>>();
    let metadata=&fixture["metadata"];let json=serde_json::to_vec(metadata).unwrap();let mut expected=fixture["magic"].as_str().unwrap().as_bytes().to_vec();expected.extend_from_slice(&(json.len()as u32).to_le_bytes());expected.extend_from_slice(&(actor.len()as u64).to_le_bytes());expected.extend_from_slice(&json);expected.extend_from_slice(&actor);
    let g=&fixture["retirement"];let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:g["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:g["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:g["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:g["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:g["maximumDepth"].as_u64().unwrap()as usize};
    let observed=std::cell::Cell::new(0);let mut observe=|_:NativeEncodeProgress|{observed.set(observed.get()+1);true};let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let mut control=NativeEncodeControl::new(fixture["originalMaximumBytes"].as_u64().unwrap()as usize,&mut observe);control.install_retirement_recipient(&mut recipient).unwrap();
    let mut visit=|sink:&mut dyn FnMut(&[u8])->Result<(),semio_framework_value::ValueError>|sink(&actor);let (actual,allocated,released)=backing(||super::encode_checkpoint_frame(metadata,&mut visit,&mut control).unwrap());assert_eq!(actual,expected);assert_eq!(allocated,actual.len()+std::mem::size_of::<Option<Vec<u8>>>());assert_eq!(released,0);assert_eq!(control.owned_bytes(),actual.len()+std::mem::size_of::<Option<Vec<u8>>>());let boundaries=observed.get();while control.has_retirement_owner(){control.close_retirement_recipient(grant).unwrap();}drop(control);
    for canceled in 1..=boundaries{let mut calls=0;let mut observe=|_:NativeEncodeProgress|{calls+=1;calls!=canceled};let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let mut control=NativeEncodeControl::new(4096,&mut observe);control.install_retirement_recipient(&mut recipient).unwrap();let mut visit=|sink:&mut dyn FnMut(&[u8])->Result<(),semio_framework_value::ValueError>|sink(&actor);assert_eq!(super::encode_checkpoint_frame(metadata,&mut visit,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);while control.has_retirement_owner(){let step=control.close_retirement_recipient(grant).unwrap();assert!(step.progress().fits(grant));}assert!(!control.has_retirement_owner());}
    let mut observe=|_:NativeEncodeProgress|true;let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let mut control=NativeEncodeControl::new(0,&mut observe);control.install_retirement_recipient(&mut recipient).unwrap();let mut visit=|sink:&mut dyn FnMut(&[u8])->Result<(),semio_framework_value::ValueError>|sink(&actor);assert_eq!(super::encode_checkpoint_frame(metadata,&mut visit,&mut control).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(control.owned_bytes(),0);
}

#[test]
fn borrowed_actual_vm_checkpoint_visit_allocates_nothing(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let module=std::sync::Arc::new(crate::interpreter::CoreModule::parse(&fixture["coreModule"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect::<Vec<_>>()).unwrap());
    let instance=crate::interpreter::CoreInstance::instantiate(module).unwrap();let expected=instance.checkpoint();let mut count=0;
    let (result,allocated,released)=backing(||instance.visit_checkpoint(&mut |bytes|{assert_eq!(&expected[count..count+bytes.len()],bytes);count+=bytes.len();Ok(())}));
    result.unwrap();assert_eq!(count,expected.len());assert_eq!((allocated,released),(0,0));
}
