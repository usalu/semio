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

/// 🧮️ Nets one streaming owner's capacity births against its own releases, so the stock it posts is the peak it holds at once and never the sum of everything it ever held.
///
/// `outstanding` is what the owner holds now, `posted` the peak already charged to the caller's stock and `owed` the births absorbed under that peak that no release has yet offset.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct OriginalNettedCapacity{outstanding:usize,posted:usize,owed:usize}
impl OriginalNettedCapacity{
    /// 🧮️ Capacity the owner holds right now.
    pub fn outstanding(&self)->usize{self.outstanding}
    /// 🏔️ Peak capacity already charged to the caller's stock.
    pub fn posted(&self)->usize{self.posted}
    /// 🧾️ Absorbed births that later releases still have to offset.
    pub fn owed(&self)->usize{self.owed}
    /// ⚖️ Takes one turn's raw births and releases and returns the `(capacity,release)` that stay chargeable: births under the posted peak are absorbed, and releases first offset what was absorbed, so a full close still releases at least what was posted.
    pub fn net(&mut self,born:usize,released:usize)->(usize,usize){
        let held=self.outstanding.saturating_add(born);
        let settled=released.min(held);
        self.outstanding=held-settled;
        let peak=self.posted.max(self.outstanding);
        let capacity=peak-self.posted;
        self.posted=peak;
        let debt=self.owed.saturating_add(born-capacity);
        let offset=released.min(debt);
        self.owed=debt-offset;
        (capacity,released-offset)
    }
}

#[cfg(test)]
mod netted_capacity_laws{
    use super::*;
    fn next(seed:&mut u64)->u64{*seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);*seed>>33}
    /// ⚖️ LAW: over any run that finally releases everything it was born with, the posted capacity equals the true peak held at once, the posted release covers it, and no turn posts more than it raw-spent.
    #[test]
    fn the_posted_capacity_is_the_peak_held_and_the_posted_release_covers_it(){
        for run in 0..64u64{
            let mut seed=run+1;let mut ledger=OriginalNettedCapacity::default();
            let (mut held,mut peak,mut born_total,mut cap_total,mut rel_total)=(0usize,0usize,0usize,0usize,0usize);
            for _ in 0..2000{
                let born=(next(&mut seed)%4096)as usize;let released=(next(&mut seed)as usize%4096).min(held+born);
                held=held+born-released;peak=peak.max(held);born_total+=born;
                let(capacity,release)=ledger.net(born,released);
                assert!(capacity<=born&&release<=released);
                cap_total+=capacity;rel_total+=release;
            }
            let(capacity,release)=ledger.net(0,held);cap_total+=capacity;rel_total+=release;
            assert_eq!(cap_total,peak);assert_eq!(ledger.posted(),peak);assert_eq!(ledger.outstanding(),0);
            assert!(rel_total>=cap_total,"run {run}: released {rel_total} posted {cap_total}");
            assert!(cap_total<=born_total);
        }
    }
    /// ⚖️ LAW: a steady one-owner-at-a-time stream posts one owner's capacity once, whatever its length.
    #[test]
    fn a_steady_stream_posts_one_owner_once(){
        let mut ledger=OriginalNettedCapacity::default();let mut posted=0usize;
        for _ in 0..100_000{let(c,_)=ledger.net(1000,0);posted+=c;let(c,_)=ledger.net(0,1000);posted+=c;}
        assert_eq!(posted,1000);
    }
}
