//! 🎯️ ordered map generic selected close dispatch through actual source lifecycle.
use super::*;
use crate::{RetirementDemand,ValueError,observe_retirement_allocations};
use crate::retained_clone::RetainedCloneSource;
#[path="../../../../🧬️contract/🧪️tests/♻️close/🎯️dispatch/🦀️.rs"]
mod contract;
pub(super) use contract::record;
#[test]
fn ordered_map_generic_close_dispatch_selects_one_body_correlated_event(){contract::verify(||[(1u64,())].into_iter().collect::<RetainedOrderedMap<u64,()>>(),RetainedOrderedMapCloneCursor::<u64,()>::close_demands);}
