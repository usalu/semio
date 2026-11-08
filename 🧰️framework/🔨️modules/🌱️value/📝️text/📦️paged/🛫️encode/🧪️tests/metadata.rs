//! 📏️ Actual resident text frame and producer metadata are bounded by the original copy grant.
use semio_framework_value_paged_text::{paged_text::{PagedTextEncodeProducer,PagedTextProducerLoan},OriginalEffectAuthority,RetainedCloneGrant};
use semio_framework_value_native_encoding::NativeEncodeOwnedProducerFrame;
use std::mem::size_of;
#[test]
fn resident_text_metadata_fits_the_original_copy_turn(){
 type Producer=PagedTextEncodeProducer<'static,'static,str,16384>;
 type Loan=PagedTextProducerLoan<'static,'static,'static,str,16384>;
 type Frame=NativeEncodeOwnedProducerFrame<Loan>;
 let fixture:serde_json::Value=serde_json::from_str(include_str!("metadata.json")).unwrap();let limit=fixture["perTurnCopy"].as_u64().unwrap() as usize;
 let _frame=size_of::<Frame>();let _authority=size_of::<Option<OriginalEffectAuthority<Loan>>>();let _producer=size_of::<Producer>();let producer_transfer=OriginalEffectAuthority::<Loan>::producer_transfer_demand().unwrap().copy_bytes;let success_header=size_of::<Option<Result<semio_framework_value_native_encoding::NativeEncodeProducerOutcome,semio_framework_value_native_encoding::NativeEncodeProducerFailure>>>();let request_header=size_of::<Option<(RetainedCloneGrant,RetainedCloneGrant,u64,Option<usize>)>>();let loan_return=producer_transfer+success_header+request_header;
 assert!(loan_return<=limit,"actual resident loan handoff exceeds original copy grant");
}
