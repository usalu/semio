/// 🧰️ Original interaction owner installation shared by the query laws and the facade dispatch laws; the includer provides `Wallet`, `InteractionStore` and `interaction_store_owners`.
#[doc(hidden)]
pub fn original_receiving_recipient() -> Wallet {
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📨️authority.json")).unwrap();
    Wallet::new(serde_json::from_value(law["wholeOperationGrant"].clone()).unwrap())
}

#[doc(hidden)]
pub fn install_original_interaction_owners(store:&mut InteractionStore,recipient:&mut Wallet){
    let(result,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||interaction_store_owners(recipient.remaining_grant()));
    let(mut owners,progress)=match result{Ok(received)=>received,Err(error)=>panic!("original interaction catalog refused: {}",error.error)};
    recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
    while !owners.constructor_is_complete(){
        let(result,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owners.admit_constructor(recipient.remaining_grant()));let progress=result.unwrap_or_else(|(error,_)|panic!("original interaction catalog ticket refused: {error}"));recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert_ne!(progress,Default::default());
    }
    store.install_document_store_owners_exact(owners).unwrap_or_else(|(error,_)|panic!("original interaction catalog installation refused: {error}"));
}
