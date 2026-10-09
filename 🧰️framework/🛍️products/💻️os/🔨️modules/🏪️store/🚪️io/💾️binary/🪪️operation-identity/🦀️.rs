//! ⚙️ Controlled operation-source commitments with deferred original cursor retirement.
use super::{ArtifactPreparedOperationCursor,ArtifactPreparedOperationSource,ErasedSnapshotRetirement,ARTIFACT_CANONICAL_JSON_DEPTH};
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneStep,RetainedCloneProgress}};
use crate::os_vcs::io::binary::entity_identity::NativeIdentityPreimage;

pub(super) struct OperationIdentityCursor{cursor:ArtifactPreparedOperationCursor,terminal:bool}
impl OperationIdentityCursor{
    pub(super) fn new()->Self{Self{cursor:ArtifactPreparedOperationCursor::default(),terminal:false}}
    pub(super) fn commit(&mut self,source:ArtifactPreparedOperationSource<'_>,stamp:(u64,u64,u64),control:&mut NativeEncodeControl<'_>)->Result<(String,[u8;32]),ValueError>{
        let mut identity=NativeIdentityPreimage::new("mutation",control)?;let mut payload=semio_framework_hash::Hasher::new();let mut output=[0;64];
        loop{
            control.checkpoint()?;
            let capacity=self.cursor.next_capacity_byte_demand()?;control.charge(capacity)?;
            let step=self.cursor.advance(source,&mut output,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:capacity,maximum_release_bytes:0,maximum_depth:ARTIFACT_CANONICAL_JSON_DEPTH}).map_err(|error|error.reason)?;
            control.advance(step.processed_items)?;
            identity.write(&output[..step.written_bytes],control)?;payload.update(&output[..step.written_bytes]);
            if step.complete{break;}
            if step.processed_items==0&&step.written_bytes==0&&step.retained_capacity_bytes==0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"operation identity source exhausted its declared progress"));}
        }
        for part in[stamp.0,stamp.1,stamp.2]{identity.write(&part.to_le_bytes(),control)?;}
        let id=identity.finish(control)?;control.begin_stage(1)?;control.advance(1)?;Ok((id,*payload.finalize().as_bytes()))
    }
}
impl ErasedSnapshotRetirement for OperationIdentityCursor{
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let step=self.cursor.close(grant).map_err(|error|error.reason)?;self.terminal=step.complete;let progress=RetainedCloneProgress{copied_items:step.processed_items,copied_bytes:step.copied_bytes,retained_capacity_bytes:step.retained_capacity_bytes,released_bytes:step.released_bytes};Ok(if step.complete{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_capacity_byte_demand(&self,_copy:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.cursor.next_close_byte_demand()}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(ARTIFACT_CANONICAL_JSON_DEPTH)}
    fn terminal_is_empty(&self)->bool{self.terminal}
}
