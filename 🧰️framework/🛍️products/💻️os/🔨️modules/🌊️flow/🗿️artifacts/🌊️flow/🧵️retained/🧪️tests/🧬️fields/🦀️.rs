//! 🧪️ Every Flow field retains logical work and actual allocation custody under independent grants.

use crate::FlowHostSnapshot;
use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::RetainedCloneGrant};

thread_local! {static OBSERVATION:std::cell::Cell<Option<(usize,usize)>>=const {std::cell::Cell::new(None)};}
pub(super) fn observe<T>(operation:impl FnOnce()->T)->(T,usize,usize) {OBSERVATION.with(|state|assert!(state.replace(Some((0,0))).is_none()));let value=operation();let (born,freed)=OBSERVATION.with(|state|state.replace(None).unwrap());(value,born,freed)}
struct ObservedSystem;
unsafe impl std::alloc::GlobalAlloc for ObservedSystem {
    unsafe fn alloc(&self,layout:std::alloc::Layout)->*mut u8 {let pointer=unsafe {std::alloc::GlobalAlloc::alloc(&std::alloc::System,layout)};if !pointer.is_null(){let _=OBSERVATION.try_with(|state|if let Some((born,freed))=state.get(){state.set(Some((born+layout.size(),freed)));});}pointer}
    unsafe fn dealloc(&self,pointer:*mut u8,layout:std::alloc::Layout) {let _=OBSERVATION.try_with(|state|if let Some((born,freed))=state.get(){state.set(Some((born,freed+layout.size())));});unsafe {std::alloc::GlobalAlloc::dealloc(&std::alloc::System,pointer,layout)};}
}
#[global_allocator]
static OBSERVED_SYSTEM:ObservedSystem=ObservedSystem;

#[test]
fn flow_all_widget_fields_close_under_independent_work_birth_release_and_depth() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧬️fields/🔣️.json")).unwrap();
    let expected=fixture["expectedCopyBytes"].as_u64().unwrap()as usize;
    assert_eq!(fixture["ownedStrings"].as_array().unwrap().iter().map(|text|text.as_str().unwrap().len()).sum::<usize>(),expected);
    for copy in fixture["copyGrants"].as_array().unwrap() {
        let copy=copy.as_u64().unwrap()as usize;
        let wire=semio_framework_pack_json::parse(&serde_json::to_string(&fixture["snapshot"]).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let (source,born,freed)=observe(||semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&wire)).unwrap());
        let source:FlowHostSnapshot=source;
        let serialized=semio_framework_value::ToValue::to_value(&source);
        assert_eq!(semio_framework_pack_json::from_dsl_value(&serialized),wire);
        let original=born-freed;
        let mut owner=ControlledRetirement::new(source).map_err(|(error,_)|error).unwrap();
        let (mut allocated,mut released,mut copied)=(0,0,0);
        for turn in 0..1_000_000 {
            if owner.terminal_is_empty(){break;}
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
            let (zero,a,r)=observe(||owner.step(RetainedCloneGrant {maximum_items:0,..grant}).unwrap());assert_eq!(zero.progress().copied_items,0);assert_eq!((a,r),(0,0));
            if grant.maximum_capacity_bytes!=0 {let (denied,a,r)=observe(||owner.step(RetainedCloneGrant {maximum_capacity_bytes:grant.maximum_capacity_bytes-1,..grant}).unwrap());assert_eq!(denied.progress().copied_items,0);assert_eq!((a,r),(0,0));}
            if grant.maximum_release_bytes!=0 && owner.next_copy_byte_demand().unwrap()==0 {let (denied,a,r)=observe(||owner.step(RetainedCloneGrant {maximum_release_bytes:grant.maximum_release_bytes-1,..grant}).unwrap());assert_eq!(denied.progress().copied_items,0);assert_eq!((a,r),(0,0));}
            let (step,a,r)=observe(||owner.step(grant).unwrap());let progress=step.progress();assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(a,r));assert!(progress.copied_items<=1&&progress.copied_bytes<=copy);allocated+=a;released+=r;copied+=progress.copied_bytes;assert!(progress.copied_items!=0,"exact Flow field grant blocked on turn {turn}");
        }
        assert!(owner.terminal_is_empty());assert_eq!(copied,expected);assert_eq!(released,original+allocated);
        eprintln!("[DEBUG] Actual Flow fields copy={copy} work={copied} original={original} births={allocated} physical={released}");
    }
}
