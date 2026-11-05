//! 📦️ Original IFC Pack publication borrows graph payloads through funded canonical locators and one final wire owner.
use super::{Frame, IfcSnapshot, STDIO_IFC_DOCUMENT_SCHEMA, pack_index::Index};
use semio_framework_dsl_record::BorrowedDslRecord;
use semio_framework_os_kernel::{NativeSnapshotEncodeOwner, OriginalPackWriting};
use semio_framework_value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
use std::mem::size_of;
use semio_framework_dsl_record::receiving::{NativeSnapshotFailure,receiving_turn};
use semio_framework_value::RetainedWorkOutcome;
fn birth<T>(owner: &mut NativeSnapshotEncodeOwner<'_, '_>) -> Result<(), ValueError> {
    let g = owner.remaining_grant();
    if g.maximum_items == 0 || g.maximum_copy_bytes < size_of::<T>() || g.maximum_depth == 0 {
        return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "original IFC Pack publication header exceeds caller grant"));
    }
    owner.native().checkpoint()?;
    owner.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<T>(), ..Default::default() })
}
impl IfcSnapshot {
    /// 📨️ Emits the shared literal IFC native Pack without a generic record or copied source payloads.
    pub fn encode_exchange_pack(&self,owner:&mut NativeSnapshotEncodeOwner<'_,'_>)->RetainedWorkOutcome<Vec<u8>,NativeSnapshotFailure>{
        owner.receive_nested::<Index,Vec<u8>,NativeSnapshotFailure>(|index,original|receiving_turn(|meter|{
            birth::<Index>(original)?;meter.performed();*index=Some(Index::default());original.native().begin_stage(0)?;
            meter.take(index.as_mut().unwrap().build(self,original))?;
            meter.take(original.receive_nested::<OriginalPackWriting,Vec<u8>,NativeSnapshotFailure>(|writer,original|receiving_turn(|meter|{
                birth::<OriginalPackWriting>(original)?;meter.performed();
                *writer=Some(OriginalPackWriting::new(STDIO_IFC_DOCUMENT_SCHEMA,original.native().maximum_bytes())?);
                meter.take(original.drive_cursor::<OriginalPackWriting,Vec<u8>,ValueError>(writer,|writer,native,grant|{
                    let result=if writer.ready(){writer.take(grant).map(|r|r.map_or((None,Default::default()),|(wire,p)|(Some(wire),p)))}else{writer.advance(&index.as_ref().unwrap().source(self),Frame::RECORD,native,grant).map(|p|(None,p))};
                    RetainedWorkOutcome{result:result.map_err(|error|{let progress=error.retained_progress();error.with_retained_progress(progress)}),work:writer.last_turn_work()}
                }))
            })))
        }))
    }
}
