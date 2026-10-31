//! 🔢️ Bounded ordering of the retained ordered map: the grant, progress and step records, the bounded ordering trait and cursors for scalars and strings. See [`super`].

use super::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BoundedOrdGrant {
    pub maximum_items: usize,
    pub maximum_bytes: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BoundedOrdProgress {
    pub compared_items: usize,
    pub compared_bytes: usize,
}

impl BoundedOrdProgress {
    pub fn fits(self, grant: BoundedOrdGrant) -> bool {
        self.compared_items <= grant.maximum_items && self.compared_bytes <= grant.maximum_bytes
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoundedOrdStep {
    Authority(RetainedCloneProgress),
    Progress(BoundedOrdProgress),
    Complete { ordering: Ordering, progress: BoundedOrdProgress },
}

pub trait BoundedOrd: Ord + Send + Sync + Sized + 'static {
    type Cursor: BoundedOrdCursor<Self>;
    fn bounded_ord_birth_demand()->crate::RetirementDemand;
    fn admit_bounded_ord_cursor(grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(Self::Cursor,RetainedCloneProgress)>>;
}

pub trait BoundedOrdCursor<T: BoundedOrd>: Send {
        fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>;
fn advance_retirement_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>;
    fn begin_close(&mut self)->bool;
    fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>;
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>;
    fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>;
    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>;
    fn terminal_is_empty(&self)->bool;
    fn close_step(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedCloneStep,crate::ValueError>;
    fn compare(&mut self, left: RetainedCloneRef<'_, T>, right: RetainedCloneRef<'_, T>, grant: BoundedOrdGrant, retirement:RetainedCloneGrant) -> crate::RetainedWorkOutcome<BoundedOrdStep, crate::ValueError>;
}

pub struct ScalarBoundedOrdCursor<T> {
    pub(super) closing:bool,
    pub(super) complete: Option<Ordering>,
    pub(super) left: Option<RetainedCloneBinding>,
    pub(super) right: Option<RetainedCloneBinding>,
    pub(super) marker: std::marker::PhantomData<fn() -> T>,
}

impl<T> Default for ScalarBoundedOrdCursor<T> {
    fn default() -> Self {
        Self { closing:false, complete: None, left: None, right: None, marker: std::marker::PhantomData }
    }
}

impl<T: BoundedOrd + Copy> BoundedOrdCursor<T> for ScalarBoundedOrdCursor<T> {
 fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{RetainedCloneBinding::close_demand(if self.left.is_some(){&self.left}else{&self.right},body)}
fn advance_retirement_demands(&self,_body:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(if self.left.is_none()||self.right.is_none(){crate::RetirementDemand{copy_bytes:RetainedCloneBinding::alias_copy_bytes(),depth:1,..Default::default()}}else{Default::default()})}
fn begin_close(&mut self)->bool {let started=!self.closing;self.closing=true;started}
fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::copy_demand(if self.left.is_some(){&self.left}else{&self.right})}
fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{RetainedCloneBinding::capacity_demand(if self.left.is_some(){&self.left}else{&self.right},body)}
fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::release_demand(if self.left.is_some(){&self.left}else{&self.right})}
fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::depth_demand(if self.left.is_some(){&self.left}else{&self.right})}
fn terminal_is_empty(&self)->bool{self.left.is_none()&&self.right.is_none()}
fn close_step(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedCloneStep,crate::ValueError>{let mut work=0;let result=(||{if !self.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"comparator must begin close"));}let step={let turn=close_retained_binding(if self.left.is_some(){&mut self.left}else{&mut self.right},grant);work+=turn.work;turn.result}?;Ok(if self.left.is_none()&&self.right.is_none(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})})();crate::RetainedWorkOutcome{result,work}}
    fn compare(&mut self, left: RetainedCloneRef<'_, T>, right: RetainedCloneRef<'_, T>, grant: BoundedOrdGrant, retirement:RetainedCloneGrant) -> crate::RetainedWorkOutcome<BoundedOrdStep, crate::ValueError> {let mut work=0;let result=(||{
        if self.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"comparator is closing"));}
        if self.left.is_none()&&grant.maximum_items==0{return Ok(BoundedOrdStep::Progress(Default::default()));}

        if let Some(progress)={let turn=left.bind(&mut self.left,retirement);work+=turn.work;turn.result}?{return Ok(BoundedOrdStep::Authority(progress));}
        if let Some(progress)={let turn=right.bind(&mut self.right,retirement);work+=turn.work;turn.result}?{return Ok(BoundedOrdStep::Authority(progress));}
        if let Some(ordering)=self.complete{return Ok(BoundedOrdStep::Complete{ordering,progress:Default::default()});}
        let progress = BoundedOrdProgress { compared_items: 1, compared_bytes: size_of::<T>() };
        if !progress.fits(grant) {
            return Ok(BoundedOrdStep::Progress(BoundedOrdProgress::default()));
        }
        work+=1;let ordering = left.get().cmp(right.get());
        self.complete = Some(ordering);
        Ok(BoundedOrdStep::Complete { ordering, progress })
    })();crate::RetainedWorkOutcome{result,work}}
}

macro_rules! bounded_ord_scalar {
    ($($type:ty),+ $(,)?) => {$ (
        impl BoundedOrd for $type {
            type Cursor = ScalarBoundedOrdCursor<Self>;
            fn bounded_ord_birth_demand()->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:2*size_of::<Self::Cursor>(),depth:1,..Default::default()}}
    fn admit_bounded_ord_cursor(grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(Self::Cursor,RetainedCloneProgress)>>{let d=Self::bounded_ord_birth_demand();if !retained_clone_grant_admits(grant,d){return crate::RetainedWorkOutcome{result:Ok(None),work:0}}crate::RetainedWorkOutcome{result:Ok(Some((ScalarBoundedOrdCursor::default(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))),work:1}}
        }
    )+ };
}

bounded_ord_scalar!(bool, char, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

#[derive(Default)]
pub struct StringBoundedOrdCursor {
    pub(super) initialized:bool,
    pub(super) closing:bool,
    pub(super) left: Option<RetainedCloneBinding>,
    pub(super) right: Option<RetainedCloneBinding>,
    offset: usize,
    pub(super) complete: Option<Ordering>,
}

impl BoundedOrdCursor<String> for StringBoundedOrdCursor {
 fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{RetainedCloneBinding::close_demand(if self.left.is_some(){&self.left}else{&self.right},body)}
fn advance_retirement_demands(&self,_body:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(if self.left.is_none()||self.right.is_none(){crate::RetirementDemand{copy_bytes:RetainedCloneBinding::alias_copy_bytes(),depth:1,..Default::default()}}else{Default::default()})}
fn begin_close(&mut self)->bool {let started=!self.closing;self.closing=true;started}
fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::copy_demand(if self.left.is_some(){&self.left}else{&self.right})}
fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{RetainedCloneBinding::capacity_demand(if self.left.is_some(){&self.left}else{&self.right},body)}
fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::release_demand(if self.left.is_some(){&self.left}else{&self.right})}
fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::depth_demand(if self.left.is_some(){&self.left}else{&self.right})}
fn terminal_is_empty(&self)->bool{self.left.is_none()&&self.right.is_none()}
fn close_step(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedCloneStep,crate::ValueError>{let mut work=0;let result=(||{if !self.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"comparator must begin close"));}let step={let turn=close_retained_binding(if self.left.is_some(){&mut self.left}else{&mut self.right},grant);work+=turn.work;turn.result}?;Ok(if self.left.is_none()&&self.right.is_none(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})})();crate::RetainedWorkOutcome{result,work}}
    fn compare(&mut self, left: RetainedCloneRef<'_, String>, right: RetainedCloneRef<'_, String>, grant: BoundedOrdGrant, retirement:RetainedCloneGrant) -> crate::RetainedWorkOutcome<BoundedOrdStep, crate::ValueError> {let mut work=0;let result=(||{
        if self.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"comparator is closing"));}
        if self.left.is_none()&&grant.maximum_items==0{return Ok(BoundedOrdStep::Progress(Default::default()));}

        let first = !self.initialized;
        if first&&grant.maximum_items==0{return Ok(BoundedOrdStep::Progress(Default::default()));}
        if let Some(progress)={let turn=left.bind(&mut self.left,retirement);work+=turn.work;turn.result}?{return Ok(BoundedOrdStep::Authority(progress));}
        if let Some(progress)={let turn=right.bind(&mut self.right,retirement);work+=turn.work;turn.result}?{return Ok(BoundedOrdStep::Authority(progress));}
        if let Some(ordering)=self.complete{return Ok(BoundedOrdStep::Complete{ordering,progress:Default::default()});}
        if first {work+=1;self.initialized=true;
            if grant.maximum_items == 0 {
                return Ok(BoundedOrdStep::Progress(BoundedOrdProgress::default()));
            }
            return Ok(BoundedOrdStep::Progress(BoundedOrdProgress { compared_items: 1, compared_bytes: 0 }));
        }
        let left = left.get().as_bytes();
        let right = right.get().as_bytes();
        let common = left.len().min(right.len());
        if self.offset == common {
            if grant.maximum_items == 0 {
                return Ok(BoundedOrdStep::Progress(BoundedOrdProgress::default()));
            }
            work+=1;let ordering = left.len().cmp(&right.len());
            self.complete = Some(ordering);
            return Ok(BoundedOrdStep::Complete { ordering, progress: BoundedOrdProgress { compared_items: 1, compared_bytes: 0 } });
        }
        if grant.maximum_bytes == 0 {
            return Ok(BoundedOrdStep::Progress(BoundedOrdProgress::default()));
        }
        let end = common.min(self.offset.saturating_add(grant.maximum_bytes));
        let start = self.offset;
        while self.offset < end {
            work+=1;let ordering = left[self.offset].cmp(&right[self.offset]);
            self.offset += 1;
            if ordering != Ordering::Equal {
                self.complete = Some(ordering);
                return Ok(BoundedOrdStep::Complete { ordering, progress: BoundedOrdProgress { compared_items: 0, compared_bytes: self.offset - start } });
            }
        }
        Ok(BoundedOrdStep::Progress(BoundedOrdProgress { compared_items: 0, compared_bytes: self.offset - start }))
    })();crate::RetainedWorkOutcome{result,work}}
}

impl BoundedOrd for String {
    type Cursor = StringBoundedOrdCursor;
    fn bounded_ord_birth_demand()->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:2*size_of::<Self::Cursor>(),depth:1,..Default::default()}}
    fn admit_bounded_ord_cursor(grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(Self::Cursor,RetainedCloneProgress)>>{let d=Self::bounded_ord_birth_demand();if !retained_clone_grant_admits(grant,d){return crate::RetainedWorkOutcome{result:Ok(None),work:0}}crate::RetainedWorkOutcome{result:Ok(Some((StringBoundedOrdCursor::default(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))),work:1}}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod actual_work_tests;
