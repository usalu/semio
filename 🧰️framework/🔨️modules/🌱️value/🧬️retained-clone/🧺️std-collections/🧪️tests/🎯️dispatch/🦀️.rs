//! 🎯️ native sorted entries generic selected close dispatch through actual source lifecycle.
use super::*;
use crate::{RetirementDemand,ValueError,observe_retirement_allocations};
use crate::retained_clone::RetainedCloneSource;
#[path="../../../🧬️contract/🧪️tests/♻️close/🎯️dispatch/🦀️.rs"]
mod contract;
pub(super) use contract::record;
#[test]
fn standard_entries_generic_close_dispatch_selects_one_body_correlated_event(){contract::verify(||NativeSortedMap::<u64,()>::default(),NativeCollectionCursor::<NativeSortedMap<u64,()>>::close_demands,|c|Some(c as *const _ as usize));}
