use semio_framework_trace::{Generation,OperationId};
use semio_framework_job_replacement::{ReplacementIdentity,ReplacementStaleness};
fn id(slot:u64,operation:u64,generation:u64)->ReplacementIdentity{ReplacementIdentity::new(slot,OperationId(operation),Generation(generation))}
/// 🪪️ Staleness is decided by the stored identity against the current slot only; a different stepping caller never takes part.
#[test]
fn staleness_is_decided_by_the_stored_identity_against_the_current_slot_only(){
 let stored=id(3,30,7);
 assert_eq!(stored.staleness(id(3,30,7)),ReplacementStaleness::Current);
 assert_eq!(stored.staleness(id(4,30,7)),ReplacementStaleness::Replaced);
 assert_eq!(stored.staleness(id(3,31,7)),ReplacementStaleness::Replaced);
 assert_eq!(stored.staleness(id(3,30,8)),ReplacementStaleness::Advanced);
 assert!(stored.require_current(id(3,30,7),"stale").is_ok());
 assert!(stored.require_current(id(3,30,8),"stale").is_err());
 assert!(!ReplacementIdentity::is_stale(None,None));
 assert!(!ReplacementIdentity::is_stale(None,Some(id(9,9,9))));
 assert!(ReplacementIdentity::is_stale(Some(stored),None));
 assert!(ReplacementIdentity::is_stale(Some(stored),Some(id(3,30,8))));
 assert!(!ReplacementIdentity::is_stale(Some(stored),Some(id(3,30,7))));
}
