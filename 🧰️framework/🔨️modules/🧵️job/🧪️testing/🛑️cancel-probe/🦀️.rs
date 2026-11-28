//! 🛑️ One original caller that steps a real job under a cancelled or live source and reports only the kind of each turn, so every job proves "cancelled before the first step ⇒ Cancelled, no work" and "cancelled mid-run ⇒ Cancelled at the next step" the same way. See [`InteractiveJob::step`] and [`StepContext::cancelled_turn`].
use super::*;
use semio_framework_value_retained_clone::RetainedCloneGrant;

const SOURCE:RetainedCloneGrant=RetainedCloneGrant{maximum_items:16_000_000,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:256};
const TURN:RetainedCloneGrant=RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:2<<20,maximum_depth:128};

/// 🔭️ The kind of one observed step: `Pending` is a paid turn without a lent outcome.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum ProbeTurn{Pending,Yield,Cancelled,Other}

/// 🎟️ Owns the caller and its receiving place across every step and the close.
pub struct CancelProbe{caller:Option<OriginalJobFixtureCaller>,origin:OriginalJobReturnOrigin}

impl CancelProbe{
    /// 🏡️ Admits the caller and prepares its receiving place first, because a source cancelled from birth could never prepare it.
    pub fn new(operation:OperationId,generation:Generation)->Self{
        let mut probe=Self{caller:Some(OriginalJobFixtureCaller::admit(operation,generation,SOURCE,||Some(0),u64::MAX,100_000_000)),origin:OriginalJobReturnOrigin::empty()};
        for turn in 0..4096{
            let caller=probe.caller.as_mut().expect("live cancel probe");
            if caller.turn(TURN,|cx|probe.origin.admit(cx)).expect("receiving place preparation"){return probe}
            assert!(turn<4095,"receiving place preparation stalled");
        }
        unreachable!()
    }

    /// 🛑️ Requests cancellation on the caller's root, observed by every later step.
    pub fn cancel(&self){self.caller.as_ref().expect("live cancel probe").cancel().cancel_now()}

    /// ▶️ One production `step` turn; the previous outcome is acknowledged first.
    pub fn step<J:InteractiveJob>(&mut self,job:&mut J,fuel:u64)->ProbeTurn{
        let caller=self.caller.as_mut().expect("live cancel probe");
        for turn in 0..4096{
            if self.origin.outcome().is_none(){break}
            if caller.turn(TURN,|cx|self.origin.acknowledge_outcome(cx)).expect("outcome acknowledgement"){break}
            assert!(turn<4095,"outcome acknowledgement stalled");
        }
        let mut observed=ProbeTurn::Pending;
        caller.turn(TURN,|cx|{
            let mut returned=OriginalJobReturn::Preparing;
            if self.origin.admit(cx)?{
                let receiver=self.origin.receiver()?;
                let output=&mut returned;
                cx.with_original_work_loan(fuel,cx.original_absolute_deadline_us(),move |cx|{let mut verdict=None;*output=drive_step(job,cx,receiver,"job.cancel-probe",InteractiveStage::InteractiveStep,&mut verdict)?;Ok(())})?;
            }
            match returned{
                OriginalJobReturn::Preparing|OriginalJobReturn::Pending=>{}
                OriginalJobReturn::Outcome(reply)=>{
                    observed=match reply.original(){None=>ProbeTurn::Pending,Some(JobOutcomeBorrow::Yield{..})=>ProbeTurn::Yield,Some(JobOutcomeBorrow::Cancelled{..})=>ProbeTurn::Cancelled,Some(_)=>ProbeTurn::Other};
                    let mut input=Some(reply.into_descriptor());
                    assert!(self.origin.receive_outcome(cx,&mut input)?);
                }
                OriginalJobReturn::Continuation(value)=>{let mut input=Some(value);assert!(self.origin.receive_continuation(cx,&mut input)?);panic!("cancel probe step refused: {:?}",self.origin.continuation().expect("received continuation").view().callback)}
            }
            Ok(())
        }).expect("cancel probe step");
        observed
    }

    /// 🔁️ Steps until a turn lends an outcome, at most `turns` times.
    pub fn step_to_outcome<J:InteractiveJob>(&mut self,job:&mut J,fuel:u64,turns:usize)->ProbeTurn{
        for _ in 0..turns{let turn=self.step(job,fuel);if turn!=ProbeTurn::Pending{return turn}}
        panic!("cancel probe job lent no outcome within {turns} turns")
    }

    /// 🪢️ One plain turn on the probe's source for a job's private owners, e.g. a hand-driven close step.
    pub fn turn<R>(&mut self,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{self.caller.as_mut().expect("live cancel probe").turn(TURN,run)}

    /// 🛬️ Returns the receiving place and retires the source of a job the test closed itself.
    pub fn release(&mut self){
        let caller=self.caller.as_mut().expect("live cancel probe");
        for turn in 0..2_000_000{
            if self.origin.terminal_is_empty(){break}
            caller.turn(TURN,|cx|self.origin.close_step(cx)).expect("return origin close");
            assert!(turn<1_999_999,"return origin close stalled");
        }
        assert!(self.origin.terminal_is_empty());
        self.caller.take().expect("probe source retires once").retire(TURN);
    }

    /// 🧹️ Closes the job on the probe's source, then releases the source.
    pub fn close<J:InteractiveJob>(&mut self,job:&mut J){
        let caller=self.caller.as_mut().expect("live cancel probe");
        job.begin_close();
        for turn in 0..2_000_000{
            if job.terminal_is_empty(){break}
            let step=caller.turn(TURN,|cx|job.close_step(cx)).expect("probe job close");
            assert!(!matches!(step,InteractiveJobCloseStep::Blocked),"probe job close blocked");
            assert!(turn<1_999_999,"probe job close stalled");
        }
        assert!(job.terminal_is_empty());
        self.release();
    }
}
