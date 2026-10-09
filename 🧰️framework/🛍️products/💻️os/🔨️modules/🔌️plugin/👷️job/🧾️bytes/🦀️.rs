//! 👷️ Original reserved payload work and whole backing release consume distinct currencies.
use semio_framework_job::{InteractiveJobCloseStep,Operation};
use semio_framework_value::{RetirementDemand,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};

pub(crate) fn demands(raw:&Vec<u8>,operation:&Option<Operation>)->RetirementDemand{
 RetirementDemand{copy_bytes:if !raw.is_empty(){1}else if raw.capacity()!=0{0}else if operation.is_some(){std::mem::size_of::<Operation>()}else{0},capacity_bytes:0,release_bytes:if raw.is_empty(){raw.capacity()}else{0},depth:usize::from(!raw.is_empty()||raw.capacity()!=0||operation.is_some())}
}

pub(crate) fn close(raw:&mut Vec<u8>,operation:&mut Option<Operation>,grant:RetainedCloneGrant)->InteractiveJobCloseStep{
 let empty=RetainedCloneProgress::default();
 let demand=demands(raw,operation);
 if demand==RetirementDemand::default(){return InteractiveJobCloseStep::Complete{progress:empty}}
 if grant.maximum_items==0{return InteractiveJobCloseStep::Pending{progress:empty}}
 if grant.maximum_depth<demand.depth{return InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::DepthLimit,progress:Default::default()}}
 if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_release_bytes<demand.release_bytes{return InteractiveJobCloseStep::Pending{progress:empty}}
 if !raw.is_empty(){let copied_bytes=raw.len().min(grant.maximum_copy_bytes);raw.truncate(raw.len()-copied_bytes);return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes,..empty}}}
 if raw.capacity()!=0{let released_bytes=raw.capacity();drop(std::mem::take(raw));return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes,..empty}}}
 operation.take();
 InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}}
}
