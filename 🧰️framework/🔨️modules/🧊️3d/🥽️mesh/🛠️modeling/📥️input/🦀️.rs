//! 📥️ Original typed translation input retains its mesh until the caller funds the defining Work frame.

use super::*;
use protocol::value::{RetirementDemand,ValueError,ValueRefusalKind};

/// 🧳️ Inline cold custody of the caller's exact original source and operation parameters.
pub struct MeshTranslationInput {
    source:Option<HalfedgeMesh>,
    delta:Vec3,
    cancelled:bool,
}

impl HalfedgeMesh {
    /// 🛫️ Moves the same original mesh into a cold typed input without creating a Work frame.
    pub fn translation_input_owned(self,delta:Vec3)->MeshTranslationInput {MeshTranslationInput {source:Some(self),delta,cancelled:false}}
}

impl MeshTranslationInput {
    /// 👁️ Borrows the original source while its cold input owns the graph.
    pub fn source(&self)->Option<&HalfedgeMesh> {self.source.as_ref()}
    /// ⛔️ Retains cold source custody when cancellation changes only its control state.
    pub fn cancel(&mut self) {self.cancelled=true;}
    /// 🚦️ Reports the original input's cancellation control without consuming its source.
    pub fn is_cancelled(&self)->bool {self.cancelled}
    /// 🕳️ Reports whether original source custody has already moved to the admitted job.
    pub fn terminal_is_empty(&self)->bool {self.source.is_none()}
    /// 🧮️ Quotes the defining original Transform frame and its structural handoff.
    pub fn start_demands(&self)->Option<RetirementDemand> {
        (self.source.is_some()&&!self.cancelled).then_some(RetirementDemand {capacity_bytes:std::mem::size_of::<Transform>(),depth:2,..Default::default()})
    }
    /// 🛬️ Starts one original Work frame under the same caller authority and preserves denied or faulted input.
    pub fn start(&mut self,grant:RetainedCloneGrant,receipt:&mut RetainedCloneProgress)->Result<Option<MeshModelingJob>,ValueError> {
        *receipt=Default::default();
        if grant.maximum_items==0||self.cancelled||self.source.is_none() {return Ok(None);}
        if self.delta.0.iter().any(|value|!value.is_finite()) {return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original translation input requires finite offset"));}
        let demand=self.start_demands().unwrap();
        if grant.maximum_depth<demand.depth {return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original translation Work frame exceeds admitted depth"));}
        if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes {return Ok(None);}
        let source=self.source.take().unwrap();
        let job=source.transform_owner([[1.0,0.0,0.0],[0.0,1.0,0.0],[0.0,0.0,1.0]],self.delta.0,[[1.0,0.0,0.0],[0.0,1.0,0.0],[0.0,0.0,1.0]],false,false);
        *receipt=RetainedCloneProgress {copied_items:1,retained_capacity_bytes:demand.capacity_bytes,..Default::default()};Ok(Some(job))
    }
    /// 🎟️ Quotes the actual boxed retirement ticket needed for the retained cold input.
    pub fn retirement_birth_bytes(&self)->usize {protocol::value::retirement::owned_retirement_birth_bytes::<Self>()}
    /// 🧹️ Moves the same cold source to paid typed retirement or returns the unchanged input on denial.
    pub fn into_retirement(self,grant:RetainedCloneGrant)->Result<(Box<dyn protocol::value::ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Self)> {protocol::value::retirement::admit_owned_retirement(self,grant)}
}

impl protocol::value::retirement::RetireOwned for MeshTranslationInput {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::retirement::deferred(self.source)}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(protocol::value::retirement::deferred_birth_bytes_for(&self.source))}
    fn controlled_retirement_supported()->bool {true}
}
