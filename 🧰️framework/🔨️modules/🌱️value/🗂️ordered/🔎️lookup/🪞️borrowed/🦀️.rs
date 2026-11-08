//! 🪞️ Ordered roots lend the actual lower native lookup owner without duplicating its implementation.
use super::*;
pub use semio_framework_value_native_ordered_engine::BorrowedLookupCursor;
impl<V> OrderedMap<V>{
 pub fn lookup_borrowed_constructor_demands(&self)->crate::RetirementDemand{BorrowedLookupCursor::<'_,'_,V>::constructor_demand()}
 pub fn admit_lookup_borrowed_into<'map,'query>(&'map self,query:&'query String,output:&mut Option<BorrowedLookupCursor<'map,'query,V>>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedCloneStep>{BorrowedLookupCursor::admit_into(&self.root,query,output,effect)}
}
