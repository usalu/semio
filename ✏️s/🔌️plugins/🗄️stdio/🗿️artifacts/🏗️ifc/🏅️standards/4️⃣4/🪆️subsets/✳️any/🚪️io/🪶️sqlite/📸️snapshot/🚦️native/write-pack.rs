//! 📦️ Original IFC Pack publication borrows graph payloads through funded canonical locators and one final wire owner.
use super::{Frame, IfcSnapshot, STDIO_IFC_DOCUMENT_SCHEMA, pack_index::Index};
use semio_framework_dsl_record::BorrowedDslRecord;
use semio_os_kernel::{NativeSnapshotEncodeOwner, OriginalPackWriting};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ValueError, retained_clone::retained_clone_grant_admits};
const WRITER_ACTION_WORK: u64 = 1 << 16;
use std::mem::size_of;
use semio_framework_dsl_record::receiving::{NativeSnapshotFailure,receiving_turn};
use semio_framework_value::RetainedWorkOutcome;
fn birth<T>(slot:&mut Option<T>,owner:&mut NativeSnapshotEncodeOwner<'_,'_>,marker:&mut semio_framework_io_control::NativeSnapshotEffectProgress,unwind_failure:&mut Option<NativeSnapshotFailure>,construct:impl FnOnce()->Result<T,ValueError>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
    let demand=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<T>(),maximum_depth:1,..Default::default()};
    let mut construct=Some(construct);
    let turn=owner.effect(demand,marker,unwind_failure,&mut |performed,physical|{*performed=true;let value=construct.take().unwrap()()?;*slot=Some(value);*physical=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<T>(),..Default::default()};Ok(())});
    let mut turn=turn;if turn.result.is_ok(){if let Err(error)=marker.reset_completed(){turn.result=Err(error.into())}}
    turn
}
impl IfcSnapshot {
    /// 📨️ Emits the shared literal IFC native Pack without a generic record or copied source payloads.
    pub fn encode_exchange_pack(&self,output:&mut Option<Vec<u8>>,owner:&mut NativeSnapshotEncodeOwner<'_,'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
        owner.receive_nested::<Index,Vec<u8>,NativeSnapshotFailure>(output,&mut |index,produced,marker,unwind_failure,original|receiving_turn(|meter|{
            meter.take(birth(index,original,marker,unwind_failure,||Ok(Index::default())))?;original.native().begin_stage(0)?;
            meter.take(index.as_mut().unwrap().build(self,original))?;
            meter.take(original.receive_nested::<OriginalPackWriting,Vec<u8>,NativeSnapshotFailure>(produced,&mut |writer,produced,marker,unwind_failure,original|receiving_turn(|meter|{
                let maximum=original.native().maximum_bytes();meter.take(birth(writer,original,marker,unwind_failure,||OriginalPackWriting::new(STDIO_IFC_DOCUMENT_SCHEMA,maximum)))?;
                meter.take(original.drive_cursor::<OriginalPackWriting,Vec<u8>,NativeSnapshotFailure>(writer,produced,&mut |writer,produced,_,_,native,wallet|{
                    let body=wallet.remaining_grant();
                    let demand=if writer.ready(){RetirementDemand{depth:1,..Default::default()}}else{match writer.demand(body.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{work:0,result:Err(error.into())}}};
                    if !retained_clone_grant_admits(body,demand){return RetainedWorkOutcome{work:0,result:Ok(())}}
                    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1)};
                    match native.with_effect(grant,WRITER_ACTION_WORK,|native,effect|{
                        let result=if writer.ready(){writer.take(effect).map(|result|result.map_or(Default::default(),|(wire,progress)|{*produced=Some(wire);progress}))}else{writer.advance(&index.as_ref().unwrap().source(self),Frame::RECORD,native,effect)};
                        (result,effect.performed_work())
                    }){
                        Ok(Some((result,work)))=>RetainedWorkOutcome{work,result:result.and_then(|progress|wallet.record_progress(progress)).map_err(NativeSnapshotFailure::from)},
                        Ok(None)=>RetainedWorkOutcome{work:0,result:Ok(())},
                        Err(error)=>RetainedWorkOutcome{work:0,result:Err(error.into())},
                    }
                }))
            })))
        }))
    }
}
