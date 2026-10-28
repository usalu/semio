//! 🧪️ The real issuer and opaque native box preserve the original grant and child-before-box closure.
use super::*;
use semio_framework_value::retained_clone::RetainedCloneSource;
struct Probe{closed:bool,expected:RetainedCloneGrant}
impl EmbeddedChildAdmissionCursorV1 for Probe{
 fn next_demand(&self,_:ChildAdmissionSource<'_>,_:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:128,depth:2,..Default::default()})}
 fn advance(&mut self,_:ChildAdmissionSource<'_>,g:RetainedCloneGrant)->Result<ChildAdmissionAdvance,ValueError>{assert_eq!(g.maximum_copy_bytes,self.expected.maximum_copy_bytes);assert_eq!(g.maximum_capacity_bytes,self.expected.maximum_capacity_bytes);assert_eq!(g.maximum_release_bytes,self.expected.maximum_release_bytes);assert_eq!(g.maximum_depth,self.expected.maximum_depth);Ok(ChildAdmissionAdvance{progress:RetainedCloneProgress{copied_items:1,copied_bytes:128,..Default::default()},done:false,error:None,canvas:None})}
 fn begin_close(&mut self){}
 fn close_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:128,depth:2,..Default::default()})}
 fn close_step(&mut self,_:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.closed=true;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:128,..Default::default()}))}
 fn terminal_is_empty(&self)->bool{self.closed}
}
#[test]
fn original_remainder_and_paired_opaque_native_birth_are_exact(){let fixture:serde_json::Value=serde_json::from_str(include_str!("🧪️.json")).unwrap();assert_eq!(fixture["cases"].as_array().unwrap().len(),6);let g=RetainedCloneGrant{maximum_items:8,maximum_copy_bytes:8192,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:16};let(mut original,_)=RetainedCloneSource::admit_owned(0usize,(),g).unwrap_or_else(|_|panic!("actual issuer"));let expected=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-EmbeddedChildAdmission::OVERHEAD,maximum_depth:g.maximum_depth-1,..g};let(mut cursor,birth)=EmbeddedChildAdmission::admit(Probe{closed:false,expected},g).unwrap_or_else(|_|panic!("opaque birth"));let source=erase_child_admission_source(original.borrow(),3);assert_eq!(cursor.advance(source,Default::default()).unwrap().progress,Default::default());assert!(cursor.advance(source,g).unwrap().progress.fits(g));cursor.begin_close();assert_eq!(cursor.close_step(g).unwrap().progress().released_bytes,0);assert!(!cursor.terminal_is_empty());assert_eq!(cursor.close_step(RetainedCloneGrant{maximum_release_bytes:0,..g}).unwrap().progress(),Default::default());assert_eq!(cursor.close_step(g).unwrap().progress().released_bytes,birth.retained_capacity_bytes);assert!(cursor.terminal_is_empty());while !original.terminal_is_empty(){original.close_step(g).unwrap();}}
