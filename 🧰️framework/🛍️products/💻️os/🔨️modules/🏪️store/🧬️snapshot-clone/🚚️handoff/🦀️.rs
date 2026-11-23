//! 🚚️ Store-owned bounded close handoff for retained-clone cursors.

use semio_framework_value::{
    ErasedSnapshotRetirement, OriginalRetirementFailure, ReplyStash, RetirementDemand, RetirementPollExit, ValueError,
    retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneStep, admit_retained_clone_close},
};
use std::mem::ManuallyDrop;
use super::{frame_release_demand, release_frame};

pub(super) struct RetainedCloneCursorHandoff<T: RetainedClone> {
    cursor: ManuallyDrop<Option<Box<T::Cursor>>>,
    close_reply: ReplyStash<OriginalRetirementFailure>,
}

impl<T: RetainedClone> RetainedCloneCursorHandoff<T> {
    pub(super) fn new(cursor: Box<T::Cursor>) -> Self {
        Self { cursor: ManuallyDrop::new(Some(cursor)), close_reply: ReplyStash::default() }
    }
    pub fn cursor(&self) -> Result<&T::Cursor, ValueError> {
        self.cursor.as_deref().ok_or_else(|| ValueError::invariant("retained clone cursor handoff lost its close owner"))
    }
}

impl<T: RetainedClone> ErasedSnapshotRetirement for RetainedCloneCursorHandoff<T> {
    fn native_header_release(&self)->Option<semio_framework_value::RetirementHeaderRelease>{self.terminal_is_empty().then(||unsafe{semio_framework_value::RetirementHeaderRelease::native(self,0,0)})}
    fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{
        let Some(cursor)=self.cursor.as_ref()else{return Ok(0)};
        if cursor.terminal_is_empty()&&!self.close_reply.pending(){return Ok(1)}
        if !cursor.close_started(){return cursor.begin_close_work_demand()}
        cursor.close_work_demand(body)
    }
    fn next_demand(&self, body:usize)->Result<RetirementDemand,ValueError>{
        let Some(cursor)=self.cursor.as_ref()else{return Ok(Default::default())};
        if cursor.terminal_is_empty()&&!self.close_reply.pending(){return Ok(frame_release_demand::<T::Cursor>())}
        if !cursor.close_started(){return cursor.begin_close_demand()}
        cursor.close_demands(body)
    }
    fn close_step(&mut self, body: usize, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> semio_framework_value::OriginalRetirementPoll { let result = (|| -> Result<RetainedCloneStep, semio_framework_value::RetirementPollExit> { let grant = effect.grant();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        if self.cursor()?.terminal_is_empty()&&!self.close_reply.pending(){
            return Ok(match release_frame(&mut self.cursor,effect)?{
                Some(progress)=>RetainedCloneStep::Complete(progress),
                None=>RetainedCloneStep::Progress(Default::default()),
            });
        }
        let cursor=self.cursor.as_deref_mut().ok_or_else(||ValueError::invariant("retained clone cursor handoff lost its close owner"))?;
        if !cursor.close_started(){return Ok(RetainedCloneStep::Progress(cursor.begin_close_paid(effect).result?));}
        let nested=&mut effect.nested(grant);
        let answer=self.close_reply.run(|reply,recover|if recover{cursor.recover_close_into(body,nested,reply)}else{cursor.close_into(body,nested,reply)}.map(drop))?;
        let step=answer.ok_or(RetirementPollExit::Pending(0))??;
        let step = admit_retained_clone_close(grant, step, cursor.terminal_is_empty(), "retained clone cursor handoff")?;
        Ok(RetainedCloneStep::Progress(step.progress()))
    })(); semio_framework_value::RetirementPollExit::settle(effect, result) }

    fn terminal_is_empty(&self) -> bool {
        self.cursor.is_none()
    }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.next_demand(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.next_demand(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.next_demand(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.next_demand(0)?.depth) }
}

impl<T: RetainedClone> Drop for RetainedCloneCursorHandoff<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "retained clone cursor handoff reached Drop before terminal-empty ownership");
        if self.terminal_is_empty() {
            unsafe { ManuallyDrop::drop(&mut self.cursor) };
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
