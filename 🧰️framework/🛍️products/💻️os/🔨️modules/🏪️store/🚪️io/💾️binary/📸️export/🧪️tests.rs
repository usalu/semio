//! 🧪️ Canonical output, unchanged original fields and every physical receipt survive native export and cancellation.
use super::*;
use semio_framework_dsl_record::{BorrowedDslRecord,native_encoding::FieldProjectionSource};
use store::os_store::serialization::{OriginalSnapshotPackExport as Export,OriginalSnapshotPackView as View};
#[derive(semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(layout="lines")]
struct Record{title:String,enabled:bool}
fn view(record:&Record)->View<'_>{View{source:record,spec:<Record as BorrowedDslRecord>::RECORD,envelope:"test.original-export"}}
fn funded(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.max(4096),maximum_capacity_bytes:d.capacity_bytes.max(4096),maximum_release_bytes:d.release_bytes.max(4096),maximum_depth:d.depth.max(256),maximum_items:1}}
fn denied(d:RetirementDemand)->Vec<RetainedCloneGrant>{(0..5).filter_map(|axis|{let mut g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};match axis{0=>g.maximum_items=0,1 if d.copy_bytes>0=>g.maximum_copy_bytes-=1,2 if d.capacity_bytes>0=>g.maximum_capacity_bytes-=1,3 if d.release_bytes>0=>g.maximum_release_bytes-=1,4 if d.depth>0=>g.maximum_depth-=1,_=>return None}Some(g)}).collect()}
#[test]
fn original_snapshot_pack_export_preserves_source_and_canonical_bytes_through_cancellation(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let expected=Record{title:law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap()as usize),enabled:true};let body=store::pack_rt::encode_document(&Record::__dsl_spec(),&expected.__dsl_to_record(),&Default::default()).unwrap();let envelope=store::semio_format::SemioEnvelope::from_envelope_id(law["envelope"].as_str().unwrap(),store::semio_format::Component::Pack,1).unwrap();let reference=store::semio_format::wrap_binary(&envelope,&body);
 for cut in law["cancelPrefixes"].as_array().unwrap().iter().map(|n|Some(n.as_u64().unwrap()as usize)).chain([None]){let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();let mut record = None;
 let mut construction = semio_framework_job::OriginalConstructionAdmission::default();
 let bytes = expected.title.len();
 let demand = OriginalConstructionDemand { items: 1, copy_bytes: bytes + size_of::<Record>(), capacity_bytes: bytes, release_bytes: 0, depth: 1 };
 turn(&mut caller, &mut decoder, RetainedCloneGrant { maximum_items: 64, ..funded(demand.physical()) }, |cx| {
     let ready = semio_framework_job::admit_original_construction_settled(cx, &mut construction, &mut record, demand, 1, |_, _| {
         let record = Record { title: expected.title.clone(), enabled: true };
         let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: bytes, ..Default::default() };
         semio_framework_value::RetainedWorkOutcome { result: Ok((record, progress)), work: 1 }
     })?;
     assert!(ready);
     Ok(())
 });let record=record.unwrap();let pointer=record.title.as_ptr();let identity=record.projection_identity();let mut owner=None;let birth=Export::capture_demands();for g in denied(birth){turn(&mut caller,&mut decoder,g,|cx|{assert!(!Export::admit(view(&record),17,usize::MAX,cx,&mut owner)?);Ok(())});assert!(owner.is_none());}turn(&mut caller,&mut decoder,funded(birth),|cx|{assert!(Export::admit(view(&record),17,usize::MAX,cx,&mut owner)?);Ok(())});let mut foreign=OriginalJobDecodeOwner::empty();turn(&mut caller,&mut foreign,funded(birth),|cx|{assert!(owner.as_mut().unwrap().advance(view(&record),17,cx).is_err());Ok(())});turn(&mut caller,&mut decoder,funded(birth),|cx|{assert!(owner.as_mut().unwrap().advance(view(&record),18,cx).is_err());Ok(())});
 for index in 0..4000000{if cut==Some(index)||owner.as_ref().unwrap().ready(){break}let d=owner.as_ref().unwrap().advance_demands().unwrap();for g in denied(d){turn(&mut caller,&mut decoder,g,|cx|{assert!(!owner.as_mut().unwrap().advance(view(&record),17,cx)?);Ok(())});assert_eq!(owner.as_ref().unwrap().advance_demands().unwrap(),d);}turn(&mut caller,&mut decoder,funded(d),|cx|owner.as_mut().unwrap().advance(view(&record),17,cx));assert!(index<3999999,"native snapshot export must complete");}assert_eq!((record.title.as_ptr(),record.projection_identity()),(pointer,identity));if cut.is_none(){assert!(owner.as_ref().unwrap().ready());let mut output=None;for g in denied(Export::take_demands()){turn(&mut caller,&mut decoder,g,|cx|{assert!(!owner.as_mut().unwrap().take(cx,&mut output)?);Ok(())});assert!(output.is_none()&&owner.as_ref().unwrap().ready());}turn(&mut caller,&mut decoder,funded(birth),|cx|{assert!(owner.as_mut().unwrap().take(cx,&mut output)?);Ok(())});let output=output.unwrap();assert_eq!(output.bytes().to_vec_owner(),reference);assert_eq!(output.digest(),*semio_framework_hash::hash(&reference).as_bytes());if let Some(path)=std::env::var_os("ENERGY_EXPORT_ORACLE"){std::fs::write(path,output.bytes().to_vec_owner()).unwrap();}
    let original_page = output.bytes().retained_bytes().contiguous_at(0).unwrap().as_ptr();
    let initial = Record { title: expected.title.clone(), enabled: expected.enabled };
    let demand = semio_os_kernel::os_vcs::io::binary::genesis::AdmittedArtifactGenesis::<Record>::verified_pack_birth_demand().unwrap();
    let refused = RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..funded(Default::default()) };
    let Err((_, initial, output)) = output.admit_snapshot(initial, refused) else { panic!("inadequate genesis capacity was accepted") };
    assert_eq!(output.bytes().retained_bytes().contiguous_at(0).unwrap().as_ptr(), original_page);
    let constructed = OriginalConstructionDemand { items: 1, copy_bytes: size_of::<Record>() + std::mem::size_of_val(&output), capacity_bytes: demand.capacity_bytes, release_bytes: 0, depth: demand.depth };
    let mut source = Some((initial, output));
    let mut admitted = None;
    let mut construction = semio_framework_job::OriginalConstructionAdmission::default();
    turn(&mut caller, &mut decoder, RetainedCloneGrant { maximum_items: 64, ..funded(constructed.physical()) }, |cx| {
        let grant = cx.retained_grant();
        let ready = semio_framework_job::admit_original_construction_settled(cx, &mut construction, &mut admitted, constructed, 1, |_, _| {
            let (snapshot, output) = source.take().unwrap();
            let result = match output.admit_snapshot(snapshot, grant) {
                Ok((genesis, mut progress)) => { progress.copied_bytes = constructed.copy_bytes; Ok((genesis, progress)) }
                Err((error, snapshot, output)) => { source = Some((snapshot, output)); Err(error) }
            };
            semio_framework_value::RetainedWorkOutcome { result, work: 1 }
        })?;
        assert!(ready);
        Ok(())
    });
    let admitted = admitted.expect("funded genesis admission");
    assert_eq!(admitted.facts().snapshot().title, expected.title);
    assert_eq!(admitted.facts().digest(), *semio_framework_hash::hash(&reference).as_bytes());
    let pack = admitted.share_pack();
    let semio_os_kernel::os_vcs::io::binary::genesis::ArtifactGenesisPack::Paged(bytes) = pack.as_ref() else { panic!("genesis flattened the original Pack pages") };
    assert_eq!(bytes.retained_bytes().contiguous_at(0).unwrap().as_ptr(), original_page);
    assert_eq!(bytes.to_vec_owner(), reference);
    drop(pack);
    close_typed(admitted, &mut caller, &mut decoder);
}else{caller.cancel().cancel_now();for index in 0..4000000{if owner.as_ref().unwrap().terminal_is_empty(){break}let d=owner.as_ref().unwrap().close_demands().unwrap();for g in denied(d){turn(&mut caller,&mut decoder,g,|cx|cx.with_original_retirement(|cx|{assert!(!owner.as_mut().unwrap().close(cx)?);Ok(())}));assert_eq!(owner.as_ref().unwrap().close_demands().unwrap(),d);}turn(&mut caller,&mut decoder,funded(d),|cx|cx.with_original_retirement(|cx|owner.as_mut().unwrap().close(cx)));assert!(index<3999999);}}
 let d=Export::boxed_release_demands();turn(&mut caller,&mut decoder,funded(d),|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|_|{owner=None;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))})?;Ok(())}));let d=RetirementDemand{copy_bytes:size_of::<Record>(),release_bytes:record.title.capacity(),depth:1,..Default::default()};turn(&mut caller,&mut decoder,funded(d),|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|_|{drop(record);Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))})?;Ok(())}));while !decoder.terminal_is_empty(){caller.turn(funded(Default::default()),|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(4096)?;decoder.close_on_source(cx)?;Ok(())})).unwrap();}caller.retire(funded(Default::default()));}
}
