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
    fn advance_demands(&self, source:RetainedCloneRef<'_,T>, _body:usize)->Result<crate::RetirementDemand,crate::ValueError>{
        if self.closing||self.spent{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"scalar clone cannot quote a closing or spent owner"));}
        Ok(if self.source.is_none(){crate::RetirementDemand{copy_bytes:source.binding_copy_bytes().max(size_of::<T>()),depth:1,..Default::default()}}else if self.value.is_none(){crate::RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}else{Default::default()})
    }
    fn advance(&mut self, source: RetainedCloneRef<'_, T>, body: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained scalar clone cursor is closing"));
        }
        if self.spent {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained scalar clone cursor is spent"));
        }
        if self.value.is_none()&&!(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<T>(),..Default::default()}).fits(grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(progress)=source.bind(&mut self.source,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
        if self.value.is_some() {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<T>(), retained_capacity_bytes: 0, released_bytes: 0 };
        if !progress.fits(grant) {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        self.value = Some(*source.get());
        Ok(RetainedCloneStep::Complete(progress))
    }

    fn take(&mut self) -> Option<T> {
        if !self.output_ready(){return None;}
        let value = self.value.take();
        if value.is_some() {
            self.spent = true;
        }
        value
    }

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    

    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if !self.closing{return Ok(Default::default());}self.close.next_owner_demands::<T>(self.value.is_some(),&self.source,body)}

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "clone cursor must begin close before granted retirement")); }
        if !retained_clone_grant_admits(grant,self.close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.value, grant)? { return Ok(step); }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if !self.closing {return Ok(0);} self.close.next_owner_depth_with_binding(self.value.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_owner_capacity_with_binding::<T>(self.value.is_some(), maximum_release_bytes,&self.source)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
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
