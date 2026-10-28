//! 🧪️ Original native Energy document creation installs genuine semantic authorities and closes all Store owners.
use super::*;
use semio_framework_job::*;
use semio_framework_value::RetainedCloneStep;
use super::initial_source::tests::{caller,call,close_decoder,TURN};
#[test]fn original_energy_document_genesis_installs_the_same_native_publication_and_replay_issuer(){
 use semio_framework_plugin_ownership::original_store_genesis::{OriginalStoreGenesis,OriginalStoreSlot};
 type Birth=OriginalStoreGenesis<EnergyModelGenesis>;
 let policy=RetainedCloneGrant{maximum_copy_bytes:32768,maximum_capacity_bytes:16<<20,maximum_release_bytes:16<<20,..TURN};
 let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();let actor=store::ActorId("native-energy-reader".into());let mut birth=call(&mut caller,&mut decoder,policy,|cx|Birth::admit("native-energy",&actor,cx));let mut output=None;
 for index in 0..1048576{output=call(&mut caller,&mut decoder,policy,|cx|birth.as_mut().unwrap().step(cx));if output.is_some(){break}assert!(index<1048575,"original Energy document genesis retained stage {:?}",birth.as_ref().unwrap().progress());}
 let(address,store)=output.unwrap();assert_eq!(address,"native-energy");assert_eq!(store.envelope().history_shape,store::HistoryShape::Document);assert!(store.parts_ref().owners.one_item_preparation_factory.is_some());assert!(store.parts_ref().owners.replay_preparation_factory.is_some());assert_eq!(std::sync::Arc::as_ptr(store.parts_ref().owners.one_item_preparation_factory.as_ref().unwrap())as*const(),std::sync::Arc::as_ptr(store.parts_ref().owners.replay_preparation_factory.as_ref().unwrap())as*const());assert!(store.owners.snapshot_read_leases.authority_matches(store.cursor.generation,store.cursor.content_revision));
 for index in 0..1048576{if birth.as_ref().unwrap().terminal_is_empty(){break}call(&mut caller,&mut decoder,policy,|cx|cx.with_original_retirement(|cx|birth.as_mut().unwrap().close_step(cx)));assert!(index<1048575);}
 call(&mut caller,&mut decoder,policy,|cx|cx.with_original_retirement(|cx|{let mut d=Birth::boxed_release_demands();d.copy_bytes+=size_of::<String>();d.release_bytes+=address.capacity();close_original_retained_step(cx,d,|_|{birth.take();drop(address);Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))})}));
 let mut store=OriginalStoreSlot::from_original(store);for index in 0..1048576{if store.terminal_is_empty(){break}call(&mut caller,&mut decoder,policy,|cx|cx.with_original_retirement(|cx|store.close_step(cx)));assert!(index<1048575,"original Energy document Store closure retained its native cursor");}
 close_decoder(&mut caller,&mut decoder);caller.retire(policy);
}
