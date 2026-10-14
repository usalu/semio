//! 🕰️ Absolute caller time remains independent of the timer wheel's wake resolution.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalDeadlineError{MissingClock,Expired}
pub fn remaining_original_deadline_us(now:Option<u64>,deadline_us:u64)->Result<u64,OriginalDeadlineError>{
    let now=now.ok_or(OriginalDeadlineError::MissingClock)?;
    deadline_us.checked_sub(now).filter(|remaining|*remaining!=0).ok_or(OriginalDeadlineError::Expired)
}
