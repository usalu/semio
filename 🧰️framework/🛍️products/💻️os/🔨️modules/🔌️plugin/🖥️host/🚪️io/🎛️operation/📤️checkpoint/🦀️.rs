//! 📤️ Borrowed Host checkpoint framing retains original allocation and failure custody.
use semio_framework_value::{ValueError,ValueRefusalKind};
use super::input::JsonInputWriter;
struct CheckpointFrameOwner{bytes:std::mem::ManuallyDrop<Option<Vec<u8>>>}
impl semio_framework_value::ErasedSnapshotRetirement for CheckpointFrameOwner{
    fn close_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,ValueError>{use semio_framework_value::retained_clone::{RetainedCloneStep,RetainedCloneProgress};let empty=RetainedCloneProgress::default();if self.bytes.is_none(){return Ok(RetainedCloneStep::Complete(empty));}let release=self.bytes.as_ref().unwrap().capacity();let copy=std::mem::size_of::<Option<Vec<u8>>>();if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_release_bytes<release{return Ok(RetainedCloneStep::Progress(empty));}if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"host checkpoint release requires original depth"));}drop(self.bytes.take());Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:copy,released_bytes:release,..empty}))}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(if self.bytes.is_some(){std::mem::size_of::<Option<Vec<u8>>>()}else{0})}
    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.bytes.as_ref().map_or(0,Vec::capacity))}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(self.bytes.is_some()))}
    fn terminal_is_empty(&self)->bool{self.bytes.is_none()}
}
impl Drop for CheckpointFrameOwner{fn drop(&mut self){assert!(std::thread::panicking()||self.bytes.is_none(),"host checkpoint original frame reached Drop before admitted release");if self.bytes.is_none(){unsafe{std::mem::ManuallyDrop::drop(&mut self.bytes);}}}}

/// 📤️ Measures borrowed metadata and actor state before one original buffer admission.
pub(crate) fn encode_checkpoint_frame<I:serde::Serialize>(metadata:&I,actor:&mut dyn FnMut(&mut dyn FnMut(&[u8])->Result<(),ValueError>)->Result<(),ValueError>,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<Vec<u8>,ValueError>{
    control.scoped_stage(|control|{
        control.begin_stage(0)?;
        let mut measure=JsonInputWriter{control,output:None,count:0};super::input::encode_borrowed(&mut measure,metadata)?;let metadata_count=measure.count;let metadata_length=u32::try_from(metadata_count).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"host checkpoint metadata extent exceeds framing"))?;
        let mut actor_count=0usize;actor(&mut |bytes|{actor_count=actor_count.checked_add(bytes.len()).filter(|count|*count<=isize::MAX as usize).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"host checkpoint actor extent exceeds address space"))?;control.advance(bytes.len())})?;
        let count=20usize.checked_add(metadata_count).and_then(|count|count.checked_add(actor_count)).filter(|count|*count<=isize::MAX as usize).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"host checkpoint frame extent exceeds address space"))?;control.begin_stage(count)?;
        control.with_retirement_owner(std::mem::size_of::<CheckpointFrameOwner>(),|control|{
            let mut owner=Box::new(CheckpointFrameOwner{bytes:std::mem::ManuallyDrop::new(None)});
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
