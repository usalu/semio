//! 🎟️ A loaned scope that already holds the quoted item and copy body keeps its claim: the quote clamps work, capacity and release to the stock on hand instead of billing a top-up row inside the callback.
use semio_framework_job_return_budget::{OriginalReturnEscrowAmounts as Amounts,OriginalReturnOrigin,OriginalReturnScopePlan};
use semio_framework_value_control_contract::{OriginalSourceTurnIdentity,RetainedCloneGrant,RetainedCloneProgress};
#[derive(Clone,Copy)]
struct Origin;
impl OriginalReturnOrigin for Origin{
 fn same_origin(&self,_:&Self)->bool{true}
 fn recipient(&self)->usize{0}
 fn source_identity(&self)->usize{0}
 fn source_turn(&self)->Option<OriginalSourceTurnIdentity>{None}
 fn granted(&self)->RetainedCloneGrant{Default::default()}
 fn fuel(&self)->u64{0}
 fn spent(&self)->RetainedCloneProgress{Default::default()}
 fn work(&self)->u128{0}
 fn succeeded(self,_:RetainedCloneGrant,_:u64)->Self{self}
}
fn grant(items:usize,copy:usize,capacity:usize,release:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:8}}
fn held(items:u64,copy:u64,work:u64,release:u64,capacity:u64)->Amounts{Amounts{items,copy_bytes:copy,work,release_bytes:release,capacity_bytes:capacity}}
fn plan(existing:Amounts,quantum:RetainedCloneGrant,quantum_work:u64,loaned:bool)->Option<OriginalReturnScopePlan>{
 OriginalReturnScopePlan::quote::<Origin>(grant(64,1<<20,1<<16,1<<16),quantum,10_000,quantum_work,existing,loaned,0,false,false,false,RetainedCloneProgress::default(),0).unwrap()
}
#[test]
fn a_loaned_scope_clamps_work_capacity_and_release_to_its_stock_instead_of_topping_up(){
 let existing=held(50,100_000,4_967,1_900,2_000);
 let plan=plan(existing,grant(1,100,2_048,2_048),5_000,true).expect("the held scope serves the quote");
 assert!(plan.ready);
 assert_eq!((plan.setup,plan.setup_work),(RetainedCloneProgress::default(),0),"no top-up row is billed");
 assert_eq!(plan.wanted,existing);
 assert_eq!((plan.quantum.maximum_capacity_bytes,plan.quantum.maximum_release_bytes),(2_000,1_900));
 assert!(plan.quantum_work<5_000&&plan.quantum_work<=existing.work);
}
#[test]
fn a_loaned_scope_whose_stock_covers_the_quote_runs_it_whole() {
 let existing=held(50,100_000,9_000,4_096,4_096);
 let plan=plan(existing,grant(1,100,2_048,2_048),5_000,true).expect("the held scope serves the quote");
 assert_eq!((plan.setup,plan.setup_work),(RetainedCloneProgress::default(),0));
 assert_eq!((plan.quantum.maximum_capacity_bytes,plan.quantum.maximum_release_bytes,plan.quantum_work),(2_048,2_048,5_000));
}
#[test]
fn an_unclaimed_scope_still_pays_its_setup() {
 let plan=plan(Amounts::default(),grant(1,100,2_048,2_048),5_000,false).expect("fresh stock funds the first claim");
 assert_eq!(plan.setup.copied_items,2);
 assert!(plan.setup_work>0);
}
