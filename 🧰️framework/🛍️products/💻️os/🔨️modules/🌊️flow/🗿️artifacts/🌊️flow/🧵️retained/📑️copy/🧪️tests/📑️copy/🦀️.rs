//! 🧪️ Selected typed Flow copies preserve canonical payloads and exact rooted ownership.

use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

//#region 🧪️RootAuthority
fn normal<R:RetireOwned+Send+Sync+'static,T:Copy>(cursor:&mut CopyCursor<R,T>,items:usize,body:usize)->Result<Option<usize>,ValueError>{
    let d=cursor.next_normal_demands(body)?;
    cursor.advance(RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:body,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth})
}
#[derive(Debug)]
struct Root { host_snapshot: Option<FlowHostSnapshot>, drops: Arc<AtomicUsize> }
impl Drop for Root { fn drop(&mut self) { assert!(self.host_snapshot.is_none()); self.drops.fetch_add(1, Ordering::SeqCst); } }
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct RootFactory;
impl semio_framework_value::retirement::RetireOwned for Root {
    fn retirement(mut self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {self.host_snapshot.take().unwrap().retirement()}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(self.host_snapshot.as_ref().unwrap())}
    fn controlled_retirement_supported()->bool {true}
}
impl SnapshotRetirementFactory<Root> for RootFactory {
    fn retirement_birth_bytes(&self,_:&Arc<Root>)->usize {semio_framework_value::retirement::arc_custody::shared_retirement_birth_bytes::<Root>()}
    fn retire(&self,root:Arc<Root>,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Arc<Root>)> {semio_framework_value::retirement::arc_custody::admit_shared_retirement(root,grant,false)}
}
fn paid<R:RetireOwned+Send+Sync+'static,T:Copy>(cursor:&CopyCursor<R,T>,copy:usize)->RetainedCloneGrant {let d=cursor.next_close_demands(copy).unwrap();RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}

fn source() -> (Arc<Root>, Arc<AtomicUsize>) {
    let fixture = semio_framework_pack_json::parse(include_str!("../../../🧪️tests/🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let native=semio_framework_value::FromValue::guard_decoded(semio_framework_pack_json::to_dsl_value(fixture.get("hostSnapshot").unwrap()));let mut receive=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(16*1024*1024,&mut receive);
    let host_snapshot: FlowHostSnapshot = semio_framework_value::FromValue::from_value_controlled(native.get(),&mut control).unwrap();
    let drops = Arc::new(AtomicUsize::new(0));
    (Arc::new(Root { host_snapshot: Some(host_snapshot), drops: drops.clone() }), drops)
}
fn close<R: RetireOwned + Send + Sync + 'static, T: Copy>(cursor: &mut CopyCursor<R, T>, grant: usize) {
    cursor.begin_close();
    for _ in 0..200_000 {
        let grant=paid(cursor,grant);let step=cursor.close_step(grant).unwrap();assert!(step.progress().fits(grant));if matches!(step,RetainedCloneStep::Complete(_)){break;}assert_ne!(step.progress().copied_items,0,"exact selected-copy close grant stalled");
    }
    assert!(cursor.terminal_is_empty());
}
//#endregion 🧪️RootAuthority

//#region 🧪️CanonicalCopy
#[test]
fn flow_selected_copy_matches_serde_and_shares_unchanged_ordered_roots() {
    let vectors = semio_framework_pack_json::parse(include_str!("../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for grant in [64, 4096] {
        for case in vectors.get("cases").and_then(semio_framework_pack_json::Value::as_array).unwrap() {
            let (root, drops) = source();
            let before = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(root.host_snapshot.as_ref().unwrap()));
            let source_pointer = Arc::as_ptr(&root);
            let kind = case.get("kind").and_then(semio_framework_pack_json::Value::as_str).unwrap();
            let index = case.get("index").and_then(semio_framework_pack_json::Value::as_u64).unwrap() as usize;
            macro_rules! run {
                ($type:ty, $project:expr) => {{
                    let mut cursor = CopyCursor::<Root, $type>::new(root, index, $project, Arc::new(RootFactory),[1;32]);
                    assert_eq!(normal(&mut cursor,0,grant).unwrap(), None);
                    let atomic=cursor.next_normal_demands(0).unwrap().copy_bytes;assert_eq!(normal(&mut cursor,1,0).unwrap(),if atomic==0{Some(0)}else{None});
                    for _ in 0..200_000 {
                        assert_eq!(Arc::as_ptr(cursor.owned.source.as_ref().unwrap()), source_pointer);
                        let bytes = normal(&mut cursor,1,grant).unwrap().unwrap();
                        assert!(bytes <= grant);
                        if cursor.complete() { break; }
                    }
                    assert!(cursor.complete());
                    let copied = cursor.take().unwrap();
                    assert_eq!(semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&copied)), *before.pointer(case.get("pointer").and_then(semio_framework_pack_json::Value::as_str).unwrap()).unwrap());
                    assert_eq!(semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(cursor.owned.source.as_ref().unwrap().host_snapshot.as_ref().unwrap())), before);
                    copied.retire(&mut cursor.owned.retirement);
                    std::thread::spawn(move || { close(&mut cursor, grant); }).join().unwrap();
                }};
            }
            match kind {
                "widget" => run!(Widget, |root, index| root.host_snapshot.as_ref()?.widgets.get(index)),
                "synapse" => run!(SynapseSpec, |root, index| root.host_snapshot.as_ref()?.synapses.get(index)),
                "hostSnapshot" => {
                    let mut cursor = FlowHostSnapshotCopy::new(root, index, |root, index| (index == 0).then_some(root.host_snapshot.as_ref()?), Arc::new(RootFactory),[1;32]);
                    while !cursor.complete() { assert!(normal(&mut cursor.cursor,1,grant).unwrap().unwrap() <= grant); }
                    let copied = cursor.take().unwrap();
                    let original = cursor.cursor.owned.source.as_ref().unwrap().host_snapshot.as_ref().unwrap();
                    assert_eq!(semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&copied)), before);
                    for ((left_key, left), (right_key, right)) in copied.layout.iter().zip(original.layout.iter()) {
                        assert!(std::ptr::eq(left_key, right_key) && std::ptr::eq(left, right));
                    }
                    copied.retire(&mut cursor.cursor.owned.retirement);
                    std::thread::spawn(move || close(&mut cursor.cursor, grant)).join().unwrap();
                }
                _ => panic!("unknown shared copy fixture kind"),
            }
            assert_eq!(drops.load(Ordering::SeqCst), 1);
        }
    }
}

#[test]
fn flow_selected_copy_cancellation_and_invalid_projection_preserve_root_until_close() {
    for polls in [0, 1, 4, 25, 4097] {
        let (root, drops) = source();
        let source_address=Arc::as_ptr(&root);
        let mut cursor = FlowHostSnapshotCopy::new(root, 0, |root, _| root.host_snapshot.as_ref(), Arc::new(RootFactory),[1;32]);
        for _ in 0..polls { normal(&mut cursor.cursor,1,64).unwrap(); }
        cursor.begin_close();
        assert!(!cursor.complete());
        assert!(cursor.take().is_none());
        let grant=paid(&cursor.cursor,1);assert_eq!(cursor.close_step(RetainedCloneGrant {maximum_items:0,..grant}).unwrap().progress(),RetainedCloneProgress::default());
        assert_eq!(Arc::as_ptr(cursor.cursor.owned.source.as_ref().unwrap()),source_address);
        std::thread::spawn(move || close(&mut cursor.cursor, 1)).join().unwrap();
        assert_eq!(drops.load(Ordering::SeqCst), 1);
    }
    let (root, drops) = source();
    let mut cursor = FlowWidgetCopy::new(root, usize::MAX, |root, index| root.host_snapshot.as_ref()?.widgets.get(index), Arc::new(RootFactory),[1;32]);
    assert!(normal(&mut cursor.cursor,1,1).is_err());
    assert!(normal(&mut cursor.cursor,1,1).unwrap().is_none());
    assert_eq!(drops.load(Ordering::SeqCst), 0);
    close(&mut cursor.cursor, 1);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}

#[test]
fn flow_selected_copy_nonterminal_drop_is_guarded_without_destroying_root() {
    let (root, drops) = source();
    let mut cursor = FlowWidgetCopy::new(root, 0, |root, index| root.host_snapshot.as_ref()?.widgets.get(index), Arc::new(RootFactory),[1;32]);
    normal(&mut cursor.cursor,1,64).unwrap();
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(cursor))).is_err());
    assert_eq!(drops.load(Ordering::SeqCst), 0);
    assert!(std::thread::spawn(|| {
        let (root, _) = source();
        let _cursor = FlowHostSnapshotCopy::new(root, 0, |root, _| root.host_snapshot.as_ref(), Arc::new(RootFactory),[1;32]);
        panic!("primary selected Flow fault");
    }).join().is_err());
}

#[test]
fn flow_selected_copy_rejects_actual_root_overgrant_and_keeps_owner_for_recovery() {
    struct Adversary {inner:Box<dyn ErasedSnapshotRetirement>,fault:bool}
    impl ErasedSnapshotRetirement for Adversary {
        fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {if self.fault{self.fault=false;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:grant.maximum_items+1,copied_bytes:grant.maximum_copy_bytes+1,..Default::default()}));}self.inner.close_step(grant)}
        fn terminal_is_empty(&self)->bool {self.inner.terminal_is_empty()}
        fn next_copy_byte_demand(&self)->Result<usize,ValueError> {self.inner.next_copy_byte_demand()}
        fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {self.inner.next_capacity_byte_demand(copy)}
        fn next_release_byte_demand(&self)->Result<usize,ValueError> {self.inner.next_release_byte_demand()}
        fn next_depth_demand(&self)->Result<usize,ValueError> {self.inner.next_depth_demand()}
    }
    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct Factory;
    impl SnapshotRetirementFactory<Root> for Factory {
        fn retirement_birth_bytes(&self,root:&Arc<Root>)->usize {RootFactory.retirement_birth_bytes(root)+size_of::<Adversary>()}
        fn retire(&self,root:Arc<Root>,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Arc<Root>)> {let bytes=self.retirement_birth_bytes(&root);if grant.maximum_capacity_bytes<bytes{return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"adversarial root frame requires admission"),root));}let (inner,mut progress)=RootFactory.retire(root,grant)?;progress.retained_capacity_bytes+=size_of::<Adversary>();Ok((Box::new(Adversary {inner,fault:true}),progress))}
    }
    let (root,drops)=source();let mut cursor=FlowWidgetCopy::new(root,0,|root,index|root.host_snapshot.as_ref()?.widgets.get(index),Arc::new(Factory),[1;32]);cursor.begin_close();let grant=paid(&cursor.cursor,1);cursor.close_step(grant).unwrap();let grant=paid(&cursor.cursor,1);assert_eq!(cursor.close_step(grant).unwrap_err().kind,ValueRefusalKind::InvariantViolated);assert!(!cursor.terminal_is_empty());assert_eq!(drops.load(Ordering::SeqCst),0);close(&mut cursor.cursor,1);assert_eq!(drops.load(Ordering::SeqCst),1);
}

#[test]
fn flow_selected_copy_allocation_admission_is_separate_and_never_reallocates_payload_pages() {
    let(root,drops)=source();let pointer=Arc::as_ptr(&root);let mut denied=FlowWidgetCopy::new(root,0,|root,index|root.host_snapshot.as_ref()?.widgets.get(index),Arc::new(RootFactory),[1;32]);let demand=denied.next_normal_demands(64).unwrap();assert!(demand.capacity_bytes>0);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:demand.capacity_bytes-1,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let(result,born,freed)=crate::retained_laws::field_tests::observe(||denied.cursor.advance(grant));assert_eq!(result.unwrap(),None);assert_eq!((born,freed),(0,0));assert_eq!(Arc::as_ptr(denied.cursor.owned.source.as_ref().unwrap()),pointer);assert_eq!(denied.allocations().reserved_bytes(),0);close(&mut denied.cursor,1);assert_eq!(drops.load(Ordering::SeqCst),1);
    let (root, drops) = source();
    let mut cursor = FlowHostSnapshotCopy::new(root, 0, |root, _| root.host_snapshot.as_ref(), Arc::new(RootFactory),[1;32]);
    assert_eq!(cursor.allocations().reservation_count(), 0);
    let mut maximum_reservation = std::time::Duration::ZERO;
    while !cursor.complete() {
        let previous = cursor.allocations().reservation_count();
        let started = std::time::Instant::now();
        let copied = normal(&mut cursor.cursor,1,4096).unwrap().unwrap();
        let elapsed = started.elapsed();
        if cursor.allocations().reservation_count() != previous {
            assert_eq!(copied, 0, "uninitialized reservation must not copy source bytes");
            maximum_reservation = maximum_reservation.max(elapsed);
        }
    }
    assert!(cursor.allocations().reserved_bytes() > 0);
    assert!(maximum_reservation < std::time::Duration::from_secs(1), "an admitted reservation is one allocation, not a copy");
    close(&mut cursor.cursor, 4096);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
    let (root, _) = source();
    let source = Rooted { pointer: &root.host_snapshot.as_ref().unwrap().schema as *const String };
    let mut task = TextTask { source, bytes: Vec::new(), reserved: false };
    let mut admission = FlowCopyAllocations::new();
    assert!(matches!(task.advance(1, &mut admission), Advance::Bytes(0)));
    let address = task.bytes.as_ptr();
    while task.bytes.len() < task.source.get().len() {
        assert!(matches!(task.advance(1, &mut admission), Advance::Bytes(1)));
        assert_eq!(address, task.bytes.as_ptr());
    }
    let mut retirement = Retirement::default();
    Box::new(task).retire(&mut retirement);
    retirement.retire_cold();
    let grant=RetainedCloneGrant::one_capacity_turn(RootFactory.retirement_birth_bytes(&root),1);let (mut root_retirement,_)=RootFactory.retire(root,grant).unwrap_or_else(|(error,_)|panic!("test root retirement admission refused: {error}"));
    while !root_retirement.terminal_is_empty(){let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:root_retirement.next_capacity_byte_demand(4096).unwrap(),maximum_release_bytes:root_retirement.next_release_byte_demand().unwrap(),maximum_depth:root_retirement.next_depth_demand().unwrap()};root_retirement.close_step(grant).unwrap();}
}
#[test]
fn flow_selected_copy_close_refuses_subexact_capacity_and_release_without_false_progress() {
    let (root,drops)=source();let mut cursor=FlowWidgetCopy::new(root,0,|root,index|root.host_snapshot.as_ref()?.widgets.get(index),Arc::new(RootFactory),[1;32]);cursor.begin_close();let mut saw_capacity=false;let mut saw_release=false;let mut total=RetainedCloneProgress::default();
    for _ in 0..200_000 {
        if cursor.terminal_is_empty(){break;}
        let grant=paid(&cursor.cursor,1);
        if grant.maximum_capacity_bytes>0 {saw_capacity=true;assert_eq!(cursor.close_step(RetainedCloneGrant {maximum_capacity_bytes:grant.maximum_capacity_bytes-1,..grant}).unwrap().progress(),RetainedCloneProgress::default());}
        if grant.maximum_release_bytes>0 {saw_release=true;assert_eq!(cursor.close_step(RetainedCloneGrant {maximum_release_bytes:grant.maximum_release_bytes-1,..grant}).unwrap().progress(),RetainedCloneProgress::default());}
        let progress=cursor.close_step(grant).unwrap().progress();assert!(progress.fits(grant));assert_ne!(progress.copied_items,0);total=total.checked_add(progress).unwrap();
    }
    assert!(cursor.terminal_is_empty()&&saw_capacity&&saw_release);assert_eq!(drops.load(Ordering::SeqCst),1);
}
//#endregion 🧪️CanonicalCopy

#[test]
fn flow_selected_copy_preserves_original_factory_through_erased_handoff(){
 fn observe<T>(operation:impl FnOnce()->T)->(T,(usize,usize)){let(value,born,freed)=crate::retained_laws::field_tests::observe(operation);(value,(born,freed))}
 let text=include_str!("../🧫️fixtures/🔣️.json");let fixture:serde_json::Value=serde_json::from_str(text).unwrap();
 for copy in fixture["factoryCustody"]["copyGrants"].as_array().unwrap(){
  let copy=copy.as_u64().unwrap()as usize;let(root,drops)=source();let factory:Arc<dyn SnapshotRetirementFactory<Root>>=Arc::new(RootFactory);let address=Arc::as_ptr(&factory)as *const();let mut cursor=CopyCursor::<Root,Widget>::new(root,0,|root,index|root.host_snapshot.as_ref()?.widgets.get(index),factory,[1;32]);cursor.begin_close();let mut handoff=false;
  for _ in 0..1_048_576{
   if cursor.terminal_is_empty(){break;}
   let grant=paid(&cursor,copy);let(stage,heap)=observe(||(cursor.next_capacity_byte_demand(copy).unwrap(),cursor.next_release_byte_demand().unwrap(),cursor.next_depth_demand().unwrap()));assert_eq!((heap.0,heap.1),(0,0));assert_eq!(stage,(grant.maximum_capacity_bytes,grant.maximum_release_bytes,grant.maximum_depth));
   let(step,heap)=observe(||cursor.close_step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.0,heap.1),(0,0));
   let before=cursor.owned.root_retirement.as_ref().map(|factory|Arc::as_ptr(factory)as *const());let(step,heap)=observe(||cursor.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.0,heap.1),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
   if before==Some(address)&&cursor.owned.factory_source.is_some(){let held=Arc::as_ptr(cursor.owned.factory_source.as_ref().unwrap())as *const();assert_eq!(held,address);assert!(fixture["factoryCustody"]["sameOriginalSource"].as_bool().unwrap());assert_eq!(heap.0,fixture["factoryCustody"]["handoffCapacityBytes"].as_u64().unwrap()as usize);assert_eq!(heap.1,fixture["factoryCustody"]["handoffReleaseBytes"].as_u64().unwrap()as usize);handoff=true;}
  }
  assert!(handoff);assert!(cursor.terminal_is_empty());assert_eq!(drops.load(Ordering::SeqCst),1);let(_,heap)=observe(||drop(cursor));assert_eq!((heap.0,heap.1),(0,fixture["factoryCustody"]["terminalDropBytes"].as_u64().unwrap()as usize));
 }
}

struct RefusedFactory(std::sync::atomic::AtomicBool);
struct RefusedTicket{original:Option<Arc<RefusedFactory>>,prepared:bool}
fn refused_factory_extent()->usize{std::alloc::Layout::new::<[usize;2]>().extend(std::alloc::Layout::new::<RefusedFactory>()).unwrap().0.pad_to_align().size()}
impl semio_framework_value::FactoryRetirement for RefusedFactory{
 fn factory_retirement_birth_bytes(&self)->usize{size_of::<RefusedTicket>()}
 fn factory_retirement_copy_byte_demand(&self)->usize{0}
 fn factory_retirement_depth_demand(&self)->usize{1}
 fn preborn_factory_retirement(self:Arc<Self>,grant:RetainedCloneGrant)->Result<(Box<dyn semio_framework_value::FactoryRetirementTicket>,RetainedCloneProgress),semio_framework_value::FactoryRetirementAdmissionError>{
  use semio_framework_value::FactoryRetirementAdmissionError;
  if grant.maximum_items==0||grant.maximum_capacity_bytes<size_of::<RefusedTicket>()||grant.maximum_depth==0{return Err(FactoryRetirementAdmissionError{error:ValueError::literal(ValueRefusalKind::OwnershipLimit,"original refused factory requires its exact constructor"),original:Some(self),ticket:None,progress:Default::default()})}
  let refused=!self.0.swap(true,Ordering::SeqCst);let original=refused.then(||self.clone() as Arc<dyn semio_framework_value::FactoryRetirement>);let ticket=Box::new(RefusedTicket{original:Some(self),prepared:false});let progress=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:size_of::<RefusedTicket>(),..Default::default()};
  if refused{Err(FactoryRetirementAdmissionError{error:ValueError::literal(ValueRefusalKind::AllocationFailed,"original constructor retained its partial ticket").with_retained_progress(progress),original,ticket:Some(ticket),progress})}else{Ok((ticket,progress))}
 }
}
impl SnapshotRetirementFactory<Root> for RefusedFactory{
 fn retirement_birth_bytes(&self,_:&Arc<Root>)->usize{semio_framework_value::retirement::arc_custody::shared_retirement_birth_bytes::<Root>()}
 fn retire(&self,root:Arc<Root>,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Arc<Root>)>{semio_framework_value::retirement::arc_custody::admit_shared_retirement(root,grant,false)}
}
impl semio_framework_value::FactoryRetirementTicket for RefusedTicket{
 fn preparation_demands(&self,_:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{Ok(semio_framework_value::RetirementDemand{depth:usize::from(!self.prepared),..Default::default()})}
 fn prepare_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.prepared{return Ok(RetainedCloneStep::Complete(Default::default()))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original partial ticket preparation needs depth"))}self.prepared=true;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))}
 fn preparation_is_complete(&self)->bool{self.prepared}
}
impl ErasedSnapshotRetirement for RefusedTicket{
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original partial ticket close needs depth"))}
  if !self.prepared{let step=semio_framework_value::FactoryRetirementTicket::prepare_step(self,grant)?;return Ok(RetainedCloneStep::Progress(step.progress()))}
  let released_bytes=self.next_release_byte_demand()?;if grant.maximum_release_bytes<released_bytes{return Ok(RetainedCloneStep::Progress(Default::default()))}drop(self.original.take());Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()}))
 }
 fn terminal_is_empty(&self)->bool{self.original.is_none()}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(if self.prepared&&self.original.as_ref().is_some_and(|original|Arc::strong_count(original)==1){refused_factory_extent()}else{0})}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(!self.terminal_is_empty()))}
}

/// 🧾️ A genuine failed constructor leaves its same original alias and born partial ticket in the caller.
#[test]
fn flow_selected_copy_retains_actual_failed_factory_ticket_and_receipt(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for copy in fixture["factoryCustody"]["copyGrants"].as_array().unwrap(){
  let copy=copy.as_u64().unwrap()as usize;let(root,drops)=source();let factory:Arc<dyn SnapshotRetirementFactory<Root>>=Arc::new(RefusedFactory(std::sync::atomic::AtomicBool::new(false)));let address=Arc::as_ptr(&factory)as*const();let mut cursor=CopyCursor::<Root,Widget>::new(root,0,|root,index|root.host_snapshot.as_ref()?.widgets.get(index),factory,[1;32]);cursor.begin_close();let mut refused=false;
  for turn in 0..1_048_576{
   if cursor.terminal_is_empty(){break}assert!(turn<1_048_575,"original failed factory ticket must close");let grant=paid(&cursor,copy);let(result,born,freed)=crate::retained_laws::field_tests::observe(||cursor.close_step(grant));let progress=match result{Ok(step)=>step.progress(),Err(error)=>{assert!(!refused);refused=true;assert_eq!(error.kind,ValueRefusalKind::AllocationFailed);assert_eq!(Arc::as_ptr(cursor.owned.factory_source.as_ref().unwrap())as*const(),address);assert!(cursor.owned.factory_close.is_some());assert!(fixture["factoryCustody"]["partialTicketPreserved"].as_bool().unwrap());assert!(fixture["factoryCustody"]["refusedOriginalPreserved"].as_bool().unwrap());error.retained_progress()}};assert!(progress.fits(grant));assert_eq!((born,freed),(progress.retained_capacity_bytes,progress.released_bytes));
  }
  assert!(refused&&cursor.terminal_is_empty());assert_eq!(drops.load(Ordering::SeqCst),1);let(_,born,freed)=crate::retained_laws::field_tests::observe(||drop(cursor));assert_eq!((born,freed),(0,0));
 }
}

#[test]
fn flow_selected_copy_normal_frames_and_cancel_match_physical_receipts(){
    struct Heap{requested_bytes:usize,released_bytes:usize}
    fn observe<T>(operation:impl FnOnce()->T)->(T,Heap){let(value,requested_bytes,released_bytes)=crate::retained_laws::field_tests::observe(operation);(value,Heap{requested_bytes,released_bytes})}
    for body in [1,3,64]{
        let(root,drops)=source();let pointer=Arc::as_ptr(&root);
        let mut cursor=FlowHostSnapshotCopy::new(root,0,|root,_|root.host_snapshot.as_ref(),Arc::new(RootFactory),[1;32]);
        let(mut caller,_)=semio_framework_job::StepContextOwner::new(semio_framework_job::OperationId(17),semio_framework_job::Generation(9),RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:semio_framework_job::StepContextOwner::birth_bytes(),maximum_depth:1,..Default::default()}).unwrap();
        let mut sequence=0;
        for _ in 0..200000{
            let d=cursor.next_normal_demands(body).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
            for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),(d.copy_bytes>0&&cursor.next_normal_demands(d.copy_bytes-1).unwrap().copy_bytes>d.copy_bytes-1).then(||RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..grant}),(d.capacity_bytes>0).then(||RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..grant}),(d.release_bytes>0).then(||RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..grant}),Some(RetainedCloneGrant{maximum_depth:0,..grant})].into_iter().flatten(){
                let mut physical=Default::default();let mut cx=caller.context(semio_framework_job::StepBudget::new(1,u64::MAX,denied),semio_framework_job::root_cancel_token(),||Some(1),&mut sequence,&mut physical).unwrap();
                let(result,heap)=observe(||cursor.advance([1;32],&mut cx));assert!(result.unwrap().is_none());assert_eq!(cx.retained_progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(Arc::as_ptr(cursor.cursor.owned.source.as_ref().unwrap()),pointer);
            }
            let mut physical=Default::default();let mut cx=caller.context(semio_framework_job::StepBudget::new(1,u64::MAX,grant),semio_framework_job::root_cancel_token(),||Some(1),&mut sequence,&mut physical).unwrap();
            let(result,heap)=observe(||cursor.advance([1;32],&mut cx));let result=result.unwrap();let p=cx.retained_progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));if d.copy_bytes>body{assert!(result.is_none());assert_eq!(p,Default::default());assert_eq!(cx.fuel_remaining(),1);break}assert!(result.is_some());assert_eq!(cx.fuel_remaining(),0);if cursor.complete(){break}
        }
        assert_eq!(cursor.complete(),body>=64);cursor.begin_close();
        for _ in 0..200000{if cursor.terminal_is_empty(){break}let grant=paid(&cursor.cursor,body);let(result,heap)=observe(||cursor.close_step(grant));let p=result.unwrap().progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));}
        assert!(cursor.terminal_is_empty());assert_eq!(drops.load(Ordering::SeqCst),1);assert_eq!(observe(||drop(cursor)).1.released_bytes,0);
        caller.close_step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:semio_framework_job::StepContextOwner::birth_bytes(),maximum_depth:1,..Default::default()});assert!(caller.terminal_is_empty());drop(caller);
    }
}

#[test]
fn flow_original_snapshot_intake_contract_keeps_actual_source_through_close(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📥️intake.json")).unwrap();
 for case in corpus["cases"].as_array().unwrap(){
  let(root,drops)=source();let source_address=Arc::as_ptr(&root);
  let mut cursor=FlowHostSnapshotCopy::new(root,0,|root,_|root.host_snapshot.as_ref(),Arc::new(RootFactory),[1;32]);
  for _ in 0..case["cancelAfter"].as_u64().unwrap(){normal(&mut cursor.cursor,1,64).unwrap();}
  let same_source=Arc::as_ptr(cursor.cursor.owned.source.as_ref().unwrap())==source_address;
  let intake:&mut dyn semio_framework_job::OriginalSnapshotIntake<FlowHostSnapshot>=&mut cursor;
  intake.begin_close();let denied_take=intake.take().is_some();
  let body=case["copyBytes"].as_u64().unwrap()as usize;
  for _ in 0..200_000{
   if intake.terminal_is_empty(){break;}
   let demand=intake.next_close_demands(body).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
   let step=intake.close_step(grant).unwrap();assert!(step.progress().fits(grant));assert_ne!(step.progress().copied_items,0);
  }
  let terminal=intake.terminal_is_empty();let demand=intake.next_close_demands(body).unwrap();
  let concrete_body=demand.release_bytes==size_of::<FlowHostSnapshotCopy<Root>>()&&demand.depth==1;
  let actual=serde_json::json!({"sameSourceAllocation":same_source,"takeAfterClose":denied_take,"finalRootDrops":drops.load(Ordering::SeqCst),"terminalEmpty":terminal,"terminalReleaseIsConcreteBody":concrete_body});
  assert_eq!(actual,case["expected"]);
 }
}
