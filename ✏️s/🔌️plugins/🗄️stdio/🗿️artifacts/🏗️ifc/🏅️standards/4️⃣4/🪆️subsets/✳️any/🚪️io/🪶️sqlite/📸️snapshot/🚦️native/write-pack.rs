//! 📦️ Original IFC Pack publication borrows graph payloads through funded canonical locators and one final wire owner.
use super::{Frame, IfcSnapshot, STDIO_IFC_DOCUMENT_SCHEMA, pack_index::Index};
use semio_framework_dsl_record::BorrowedDslRecord;
use semio_os_kernel::{NativeSnapshotEncodeOwner, OriginalPackWriting};
use semio_framework_value::{RetainedCloneProgress, ValueError};
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
                meter.take(original.drive_cursor::<OriginalPackWriting,Vec<u8>,ValueError>(writer,produced,&mut |writer,produced,native,grant|{
                    let result=if writer.ready(){writer.take(grant).map(|r|r.map_or(Default::default(),|(wire,p)|{*produced=Some(wire);p}))}else{writer.advance(&index.as_ref().unwrap().source(self),Frame::RECORD,native,grant)};
                    RetainedWorkOutcome{result:result.map_err(|error|{let progress=error.retained_progress();error.with_retained_progress(progress)}),work:writer.last_turn_work()}
                }))
            })))
        }))
    }
}
