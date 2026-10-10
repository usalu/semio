//! 📤️ Borrowed Host checkpoint framing retains original allocation and failure custody.
use semio_framework_value::{ValueError,ValueRefusalKind};
use super::input::{JsonInputWriter,JsonBackingOwner};
use crate::os_vcs::io::binary::entity_identity::control::{OriginalOperationHostIo,OriginalOperationReceiptProjection};

/// 🧾️ Converges bounded decimal framing against a readonly projection, then writes the actual admitted receipt.
pub(crate) fn encode_projected_checkpoint_frame(scope:&mut OriginalOperationHostIo<'_, '_>,metadata:&mut dyn FnMut(&OriginalOperationReceiptProjection<'_>,&mut semio_framework_value::NativeEncodeControl<'_>,Option<&mut Vec<u8>>)->Result<usize,ValueError>,actor:&mut dyn FnMut(&mut dyn FnMut(&[u8])->Result<(),ValueError>)->Result<(),ValueError>)->Result<Vec<u8>,ValueError>{
    scope.control().begin_stage(0)?;
    let mut actor_count=0usize;
    actor(&mut |bytes|{actor_count=actor_count.checked_add(bytes.len()).filter(|count|*count<=isize::MAX as usize).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"projected actor extent exceeds address space"))?;scope.control().advance(bytes.len())})?;
    let cursor_bytes=std::mem::size_of::<JsonBackingOwner>();
    let mut frame_count=20usize.checked_add(actor_count).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"projected frame extent overflow"))?;
    for _ in 0..usize::MAX.ilog10()+2{
        let charge=cursor_bytes.checked_add(frame_count).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"projected native admission overflow"))?;
        let metadata_count=scope.with_projected_host_charge(charge,|projection,control|{control.begin_stage(0)?;metadata(projection,control,None)})?;
        let metadata_length=u32::try_from(metadata_count).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"projected metadata exceeds framing"))?;
        let next=20usize.checked_add(actor_count).and_then(|count|count.checked_add(metadata_count)).filter(|count|*count<=isize::MAX as usize).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"projected frame exceeds address space"))?;
        if next<frame_count{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"projected metadata demand decreased"));}
        if next!=frame_count{frame_count=next;continue;}
        return scope.with_projected_host_charge(charge,|projection,control|{
            let expected=control.owned_bytes().checked_add(charge).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"projected source ledger overflow"))?;
            control.begin_stage(frame_count)?;
            control.with_retirement_owner(cursor_bytes,|control|{
                let mut owner=Box::new(JsonBackingOwner{bytes:std::mem::ManuallyDrop::new(None)});
                let result=(||{
                    owner.bytes.replace(control.allocate_vec(frame_count)?);
                    if control.owned_bytes()!=expected{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"projected frame differs from actual source admission"));}
                    let output=owner.bytes.as_mut().unwrap();
                    for bytes in [b"SMOWNH01".as_slice(),metadata_length.to_le_bytes().as_slice(),(actor_count as u64).to_le_bytes().as_slice()]{control.advance(bytes.len())?;output.extend_from_slice(bytes);}
                    let written=metadata(projection,control,Some(&mut *output))?;
                    if written!=metadata_count||output.len()!=20+metadata_count{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"projected metadata changed after admission"));}
                    let mut written=0usize;
                    actor(&mut |bytes|{written=written.checked_add(bytes.len()).filter(|count|*count<=actor_count).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"projected actor changed after admission"))?;control.advance(bytes.len())?;output.extend_from_slice(bytes);Ok(())})?;
                    if written!=actor_count||output.len()!=frame_count{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"projected actor changed after admission"));}
                    Ok(owner.bytes.take().unwrap())
                })();
                (result,Some(owner as Box<dyn semio_framework_value::ErasedSnapshotRetirement>))
            })
        });
    }
    Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"projected checkpoint decimal demand did not converge"))
}
/// 📤️ Measures borrowed metadata and actor state before one original buffer admission.
pub(crate) fn encode_checkpoint_frame<I:serde::Serialize>(metadata:&I,actor:&mut dyn FnMut(&mut dyn FnMut(&[u8])->Result<(),ValueError>)->Result<(),ValueError>,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<Vec<u8>,ValueError>{
    control.scoped_stage(|control|{
        control.begin_stage(0)?;
        let mut measure=JsonInputWriter{control,output:None,count:0};super::input::encode_borrowed(&mut measure,metadata)?;let metadata_count=measure.count;let metadata_length=u32::try_from(metadata_count).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"host checkpoint metadata extent exceeds framing"))?;
        let mut actor_count=0usize;actor(&mut |bytes|{actor_count=actor_count.checked_add(bytes.len()).filter(|count|*count<=isize::MAX as usize).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"host checkpoint actor extent exceeds address space"))?;control.advance(bytes.len())})?;
        let count=20usize.checked_add(metadata_count).and_then(|count|count.checked_add(actor_count)).filter(|count|*count<=isize::MAX as usize).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"host checkpoint frame extent exceeds address space"))?;control.begin_stage(count)?;
        control.with_retirement_owner(std::mem::size_of::<JsonBackingOwner>(),|control|{
            let mut owner=Box::new(JsonBackingOwner{bytes:std::mem::ManuallyDrop::new(None)});
            let result=(||{
                owner.bytes.replace(control.allocate_vec(count)?);let output=owner.bytes.as_mut().unwrap();
                for bytes in [b"SMOWNH01".as_slice(),metadata_length.to_le_bytes().as_slice(),(actor_count as u64).to_le_bytes().as_slice()]{control.advance(bytes.len())?;output.extend_from_slice(bytes);}
                let mut writer=JsonInputWriter{control,output:Some(output),count:0};super::input::encode_borrowed(&mut writer,metadata)?;if writer.count!=metadata_count{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"host checkpoint metadata changed after measurement"));}
                let mut written=0usize;actor(&mut |bytes|{written=written.checked_add(bytes.len()).filter(|written|*written<=actor_count).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"host checkpoint actor changed after measurement"))?;control.advance(bytes.len())?;output.extend_from_slice(bytes);Ok(())})?;
                if written!=actor_count||output.len()!=count{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"host checkpoint actor changed after measurement"));}Ok(owner.bytes.take().unwrap())
            })();(result,Some(owner as Box<dyn semio_framework_value::ErasedSnapshotRetirement>))
        })
    })
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod checkpoint_tests;
