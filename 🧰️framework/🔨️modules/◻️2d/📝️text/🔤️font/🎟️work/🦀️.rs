//! 🎟️ Font producers expose the canonical domain-neutral original work owner.
use semio_framework_value::RetainedAllocationEffect;
use semio_framework_value_persistent::list::PagedList;
pub use semio_framework_value_persistent::work::RetainedWorkLedger;
/// 🎫️ Reserves one slot on the original ledger and degrades a refusal to the producer failure text.
pub fn slot<T>(work:&mut RetainedWorkLedger,values:&mut PagedList<T,{usize::MAX}>,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,&'static str>{work.slot(values,effect).map_err(|_|"Font work slot refused")}
