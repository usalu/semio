//! 🔢️ Scalar cursor of the retained clone contract: the scalar cursor, its cursor implementation and the scalar retained clone macro with its invocation for the primitive types. See [`super`].

use super::*;

pub struct ScalarCursor<T> {
    pub(super) value: Option<T>,
    pub(super) source: Option<RetainedCloneBinding>,
    pub(super) spent: bool,
    pub(super) closing: bool,
    pub(super) close: RetainedCloneClose,
}

impl<T> Default for ScalarCursor<T> {
    fn default() -> Self {
        Self { value: None, source: None, spent: false, closing: false, close: RetainedCloneClose::default() }
    }
}

impl<T> RetainedCloneCursor<T> for ScalarCursor<T>
where
    T: RetainedClone + Copy,
{
    fn output_ready(&self)->bool{!self.closing&&!self.spent&&self.value.is_some()}
    fn advance_demands(&self, source:RetainedCloneRef<'_,T>, _body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
        if self.closing||self.spent{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"scalar clone cannot quote a closing or spent owner"));}
        if maximum_work_units==0{return Ok(Default::default())}Ok(if self.source.is_none(){RetirementDemand{copy_bytes:source.binding_copy_bytes().max(size_of::<T>()),depth:1,..Default::default()}}else if self.value.is_none(){RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}else{Default::default()})
    }
    fn advance_work_demand(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.advance_demands(source,body,maximum_work_units)?;Ok(u64::from(maximum_work_units>0&&(self.source.is_none()||self.value.is_none())))}
    fn advance(&mut self, source: RetainedCloneRef<'_, T>, body: usize, maximum_work_units:u64, effect:&mut crate::RetainedAllocationEffect<'_>) ->crate::OriginalRetirementOutcome{let grant=effect.grant();let maximum_work_units=maximum_work_units.min(effect.maximum_work_units());let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
        if self.closing {
            return Err((ValueError::literal(ValueRefusalKind::InvariantViolated, "retained scalar clone cursor is closing")).into());
        }
        if self.spent {
            return Err((ValueError::literal(ValueRefusalKind::InvariantViolated, "retained scalar clone cursor is spent")).into());
        }
        if maximum_work_units==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        if self.value.is_none()&&!(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<T>(),..Default::default()}).fits(grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(progress)={let turn=source.bind(&mut self.source,grant);work+=turn.work;turn.result}?{return Ok(RetainedCloneStep::Progress(progress));}
        if self.value.is_some() {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<T>(), retained_capacity_bytes: 0, released_bytes: 0 };
        if !progress.fits(grant) {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        self.value = Some(*source.get());work+=1;
        Ok(RetainedCloneStep::Complete(progress))
    })();RetainedWorkOutcome{result,work}}

    fn take_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(self.output_ready()))}
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Ok(0)}self.close.next_owner_work::<T>(self.value.is_some(),&self.source,body)}
    fn take_demand(&self)->Result<RetirementDemand,ValueError>{crate::retained_clone::retained_clone_cursor_take_demand::<T,bool>()}
    fn take<'reply>(&mut self,output:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{crate::retained_clone::take_retained_clone_cursor_output!(self.output_ready(),self.value,output,self.spent,true,effect,reply,T,bool)}

    
    fn close_started(&self)->bool{self.closing}
    fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(!self.closing))}
    fn begin_close_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{let flags=usize::from(!self.closing);Ok(crate::RetirementDemand{copy_bytes:flags*std::mem::size_of::<bool>(),depth:usize::from(flags!=0),..Default::default()})}
fn begin_close_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<crate::RetainedCloneProgress,crate::ValueError>{crate::retained_clone::start_retained_close(&mut self.closing,effect)}

    

    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Ok(Default::default());}self.close.next_owner_demands::<T>(self.value.is_some(),&self.source,body)}

    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let grant=effect.grant();let mut work=0;let result=(||{
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "clone cursor must begin close before granted retirement").into()); }
        if !retained_clone_grant_admits(grant,self.close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if !self.close.is_empty() { let turn=self.close.step_granted(effect);work+=turn.work;return turn.result; }
        {let turn=self.close.begin_granted(&mut self.value,effect);work+=turn.work;if let Some(step)=turn.result?{return Ok(step)}}
        {let turn=close_retained_binding(&mut self.source,effect);work+=turn.work;turn.result}
    })();RetainedWorkOutcome{result,work}}

    fn next_close_depth_demand(&self)->Result<usize,ValueError>{if !self.closing {return Ok(0);} self.close.next_owner_depth_with_binding(self.value.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_owner_capacity_with_binding::<T>(self.value.is_some(), maximum_release_bytes,&self.source)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_release_with_binding(&self.source)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.value.is_none() && self.close.is_empty() && self.source.is_none()
    }
}

macro_rules! retained_clone_scalar {
    ($($type:ty),+ $(,)?) => {$ (
        impl RetainedClone for $type {
            type Cursor = ScalarCursor<Self>;
            fn retained_clone_cursor() -> Self::Cursor { ScalarCursor::default() }
            const BITWISE: bool = true;
            fn bitwise_array<const N: usize>(source: &[Self; N]) -> Option<[Self; N]> { Some(*source) }
            fn bitwise_extend(target: &mut Vec<Self>, source: &[Self]) -> bool { target.extend_from_slice(source); true }
        }
    )+ };
}

retained_clone_scalar!((), bool, char, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
