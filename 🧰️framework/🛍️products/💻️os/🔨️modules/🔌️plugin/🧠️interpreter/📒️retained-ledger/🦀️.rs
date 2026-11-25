//! 📒️ Always-on per-instance, per-category totals of what the paid interpreter posted to the retained copy ledger, so a refusal can name its contributors.
use std::cell::Cell;
use std::sync::atomic::{AtomicU64,Ordering};
/// 📒️ Who posted a retained item: the category a ledger line is booked under.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum RetainedCategory{FrameBirth,OperandStack,PoolBirth,MemoryGrow,DataInstantiate,HostArguments,HostResults,CheckpointImage,Retirement,Other}
const NAMES:[&str;10]=["frame births","operand-stack growth","pool births","memory.grow","data/instantiate","host import arguments","host import results","checkpoint/image","retirement","other"];
const COUNT:usize=NAMES.len();
/// 📒️ One interpreter instance's items and copy bytes per category; constant-time plain counters.
#[derive(Debug,Default)]
pub struct RetainedLedger{items:[AtomicU64;COUNT],copy_bytes:[AtomicU64;COUNT]}
impl RetainedLedger{
 /// 📒️ Books one retained item and its copy bytes under a category.
 pub fn add(&self,category:RetainedCategory,copy:usize){self.items[category as usize].fetch_add(1,Ordering::Relaxed);self.copy_bytes[category as usize].fetch_add(u64::try_from(copy).unwrap_or(u64::MAX),Ordering::Relaxed);}
 /// 📒️ Reads one category as items and copy bytes.
 pub fn read(&self,category:RetainedCategory)->(u64,u64){(self.items[category as usize].load(Ordering::Relaxed),self.copy_bytes[category as usize].load(Ordering::Relaxed))}
}
impl RetainedLedger{
 /// 📒️ Copies the counters as items then copy bytes per category; no formatting, no allocation.
 pub fn snapshot(&self)->[u64;2*COUNT]{let mut snapshot=[0;2*COUNT];for index in 0..COUNT{snapshot[index]=self.items[index].load(Ordering::Relaxed);snapshot[COUNT+index]=self.copy_bytes[index].load(Ordering::Relaxed);}snapshot}
}
/// 📒️ Formats a snapshot as one line naming every nonzero category; called only when a refusal is produced.
pub fn format_retained_snapshot(snapshot:&[u64;2*COUNT])->String{let mut text=String::from("interpreter retained ledger (items/copy bytes)");let(mut items,mut copy)=(0,0);for index in 0..COUNT{let(i,c)=(snapshot[index],snapshot[COUNT+index]);items+=i;copy+=c;if i!=0||c!=0{text.push_str(&format!("; {}={i}/{c}",NAMES[index]));}}text.push_str(&format!("; total={items}/{copy}"));text}
impl std::fmt::Display for RetainedLedger{fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{formatter.write_str(&format_retained_snapshot(&self.snapshot()))}}
thread_local!{static CURRENT:Cell<*const RetainedLedger>=const{Cell::new(std::ptr::null())};static BOOKED:Cell<(u64,u64)>=const{Cell::new((0,0))};}
/// 📒️ Items and copy bytes booked on this thread since it started, whichever instance they belonged to.
pub fn thread_booked()->(u64,u64){BOOKED.with(Cell::get)}
/// 📒️ Directs this thread's bookings to one instance's ledger until dropped, then restores the previous one; the instance owns the ledger inline and never moves while its step runs.
pub struct LedgerScope{previous:*const RetainedLedger}
impl LedgerScope{/// 📒️ Enters an instance's ledger.
 pub fn enter(ledger:&RetainedLedger)->Self{Self{previous:CURRENT.with(|current|current.replace(std::ptr::from_ref(ledger)))}}}
impl Drop for LedgerScope{fn drop(&mut self){CURRENT.with(|current|current.set(self.previous));}}
/// 📒️ Books one retained item under a category of the instance whose step is running on this thread.
pub fn tally(category:RetainedCategory,copy:usize){BOOKED.with(|booked|{let(items,bytes)=booked.get();booked.set((items+1,bytes+u64::try_from(copy).unwrap_or(u64::MAX)))});CURRENT.with(|current|{let ledger=current.get();if !ledger.is_null(){unsafe{&*ledger}.add(category,copy)}})}
