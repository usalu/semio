//! 🧾️ A birth ledger records the capacity an owner's eager backings were born with, so their later release settles against the ledger that recorded the birth and never against the stock of the caller that closes them.
use std::cell::Cell;

/// 🧾️ Capacity born under one ledger and the part of it already settled by releases; a release never settles beyond what was born.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct OriginalBirthLedger{born:usize,settled:usize}
impl OriginalBirthLedger{
    /// 🍼️ Records capacity born under this ledger.
    pub fn record(&mut self,bytes:usize){self.born=self.born.saturating_add(bytes)}
    /// 🧮️ Capacity born and not yet settled.
    pub fn outstanding(&self)->usize{self.born-self.settled}
    /// 🍼️ Capacity ever recorded as born.
    pub fn born(&self)->usize{self.born}
    /// ⚖️ Capacity already settled by releases.
    pub fn settled(&self)->usize{self.settled}
    /// ⚖️ Settles a release against the outstanding births and returns the settled part; the rest stays with the releasing caller.
    pub fn settle(&mut self,released:usize)->usize{let settled=released.min(self.outstanding());self.settled+=settled;settled}
}

thread_local!{static AMBIENT:Cell<OriginalBirthLedger>=const{Cell::new(OriginalBirthLedger{born:0,settled:0})}}

/// 🍼️ Records eager capacity born on this thread, whose owners have no caller context to charge at birth.
pub fn record_original_birth(bytes:usize){AMBIENT.with(|ledger|{let mut value=ledger.get();value.record(bytes);ledger.set(value)})}
/// ⚖️ Settles a release against the births this thread recorded and returns the settled part.
pub fn settle_original_birth(released:usize)->usize{AMBIENT.with(|ledger|{let mut value=ledger.get();let settled=value.settle(released);ledger.set(value);settled})}
/// 🔭️ Reads the ledger of the births this thread recorded.
pub fn original_birth_ledger()->OriginalBirthLedger{AMBIENT.with(Cell::get)}
