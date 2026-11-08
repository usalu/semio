//! 📥️ Test-only original source admission drives every staged birth turn under its own exact quote.
use super::*;
use crate::retirement::RetireOwned;

pub(crate) struct Admitted<T:RetireOwned+Sync>{pub(crate) source:RetainedCloneSource<T>,pub(crate) progress:RetainedCloneProgress,pub(crate) work:u128,pub(crate) heap:(usize,usize)}

pub(crate) fn admit_owned<T:RetireOwned+Sync>(value:T)->Admitted<T>{
 let mut admission=RetainedCloneSource::admission_owned(value,());let mut source=None;let mut progress=RetainedCloneProgress::default();let mut work=0u128;let mut heap=(0,0);
 for _ in 0..10000{
  let d=admission.next_demand().unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
  let(turn,physical)=crate::observe_retirement_allocations(||if admission.phase()=="ready"{admission.take_output(&mut source,g)}else{{let mut actual=None;crate::retained_clone::effect::with_effect(g,crate::retained_clone::effect::CPU,&mut actual,|e|admission.advance(e)).0}});
  work+=turn.work;let p=turn.result.unwrap().progress();assert!(p.fits(g));assert_eq!(physical,(p.retained_capacity_bytes,p.released_bytes));
  progress.copied_items+=p.copied_items;progress.copied_bytes+=p.copied_bytes;progress.retained_capacity_bytes+=p.retained_capacity_bytes;progress.released_bytes+=p.released_bytes;heap.0+=physical.0;heap.1+=physical.1;
  if let Some(source)=source{assert!(admission.terminal_is_empty());return Admitted{source,progress,work,heap}}
 }
 panic!("funded test source admission did not settle");
}
