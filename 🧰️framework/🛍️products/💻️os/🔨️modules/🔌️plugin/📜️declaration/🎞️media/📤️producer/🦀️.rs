//! 📤️ Original segmented producers retain their exact allocation until output and optional credit commit together.
use super::*;
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::{ValueError,ValueRefusalKind};
use std::sync::atomic::Ordering;

impl ArtifactOutputChunks {
    /// 📏️ Quotes one original vector header and bounded output bookkeeping without copying its payload.
    pub fn original_push_demands()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Vec<u8>>()+5*size_of::<usize>()+size_of::<bool>(),depth:1,..Default::default()}}

    /// 🫴️ Publishes one original allocation only while the actual caller funds both receiving authorities.
    pub fn push_original(&self,source:&mut Option<Vec<u8>>,credit:Option<&ArtifactMediaExportCredit>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        cx.original_source_parent()?;
        let demand=Self::original_push_demands();let grant=cx.retained_grant();
        if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes{return Ok(false)}
        let Ok(mut state)=self.inner.state.try_lock()else{return Ok(false)};
        let _credit_authority=match credit{Some(credit)=>match credit.state.writer.try_lock(){Ok(authority)=>Some(authority),Err(_)=>return Ok(false)},None=>None};
        let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};cx.consume_retained(progress)?;cx.consume_fuel(1);
        let refusal=|kind,message|OriginalTurnFailure::from(ValueError::literal(kind,message).with_retained_progress(progress));
        let Some(chunk)=source.as_ref()else{return Err(refusal(ValueRefusalKind::InvariantViolated,"original segmented producer requires its retained source vector"))};
        if self.inner.sealed.load(Ordering::Acquire)||chunk.is_empty()||chunk.len()>Self::CHUNK_BYTES{return Err(refusal(ValueRefusalKind::InvalidValue,"original segmented source is sealed, empty or oversized"))}
        let Some(next)=self.inner.bytes.load(Ordering::Acquire).checked_add(chunk.len()).filter(|next|*next<=self.inner.maximum)else{return Err(refusal(ValueRefusalKind::OwnershipLimit,"original segmented source exceeds its exact byte bound"))};
        let Some(next_remaining)=self.inner.bytes_remaining.load(Ordering::Acquire).checked_add(chunk.len()).filter(|remaining|*remaining<=next)else{return Err(refusal(ValueRefusalKind::InvariantViolated,"original segmented outstanding byte count is inconsistent"))};
        if !state.chunks.can_push(){return Err(refusal(ValueRefusalKind::OwnershipLimit,"original segmented receiving slot or allocation was not admitted"))}
        let next_credit=match credit{Some(credit)=>{
            if credit.exceeded(){return Err(refusal(ValueRefusalKind::OwnershipLimit,"original media output credit was already exceeded"))}
            let Some(next)=credit.bytes().checked_add(chunk.len()).filter(|next|*next<=credit.maximum)else{return Err(refusal(ValueRefusalKind::OwnershipLimit,"original media output exceeds its exact credit bound"))};Some(next)
        },None=>None};
        if let Err(original)=state.chunks.push(source.take().expect("validated original segmented source")){*source=Some(original);return Err(refusal(ValueRefusalKind::InvariantViolated,"original segmented receiving authority refused its admitted slot"))}
        self.inner.remaining.store(state.chunks.len(),Ordering::Release);self.inner.bytes.store(next,Ordering::Release);
        self.inner.bytes_remaining.store(next_remaining,Ordering::Release);
        if let(Some(credit),Some(next))=(credit,next_credit){credit.state.bytes.store(next,Ordering::SeqCst)}
        Ok(true)
    }
    /// 📐️ Quotes one bounded original seal validation and tag transition.
    pub fn original_seal_demands()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<usize>()+size_of::<bool>(),depth:1,..Default::default()}}

    /// 🔏️ Seals the retained queue only after its genuine caller funds and acquires its receiving authority.
    pub fn seal_original(&self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        cx.original_source_parent()?;let demand=Self::original_seal_demands();let grant=cx.retained_grant();
        if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes{return Ok(false)}
        let Ok(_state)=self.inner.state.try_lock()else{return Ok(false)};
        let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};cx.consume_retained(progress)?;cx.consume_fuel(1);
        if self.inner.sealed.load(Ordering::Acquire){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original segmented output cannot be sealed twice").with_retained_progress(progress).into())}
        self.inner.sealed.store(true,Ordering::Release);Ok(true)
    }

}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
