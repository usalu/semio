//! 📦️ Original IFC Pack publication borrows graph payloads through funded canonical locators and one final wire owner.
use super::{Frame, IfcSnapshot, STDIO_IFC_DOCUMENT_SCHEMA, pack_index::Index};
use semio_framework_dsl_record::BorrowedDslRecord;
use semio_framework_os_kernel::{NativeSnapshotEncodeOwner, OriginalPackWriting};
use semio_framework_value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
use std::mem::size_of;
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
    pub fn encode_exchange_pack(&self, owner: &mut NativeSnapshotEncodeOwner<'_, '_>) -> Result<Vec<u8>, ValueError> {
        owner.receive_nested::<Index, Vec<u8>>(|index, original| {
            birth::<Index>(original)?;
            *index = Some(Index::default());
            original.native().begin_stage(0)?;
            index.as_mut().unwrap().build(self, original)?;
            original.receive_nested::<OriginalPackWriting, Vec<u8>>(|writer, original| {
                birth::<OriginalPackWriting>(original)?;
                *writer = Some(OriginalPackWriting::new(STDIO_IFC_DOCUMENT_SCHEMA, original.native().maximum_bytes())?);
                original.drive_cursor(writer, |writer, native, grant| {
                    let result = if writer.ready() {
                        writer.take(grant).map(|r| r.map_or((None, Default::default()), |(wire, p)| (Some(wire), p)))
                    } else {
                        writer.advance(&index.as_ref().unwrap().source(self), Frame::RECORD, native, grant).map(|p| (None, p))
                    };
                    match result {
                        Ok((wire, p)) => (Ok(wire), p),
                        Err(e) => {
                            let p = e.retained_progress();
                            (Err(e), p)
                        }
                    }
                })
            })
        })
    }
}
