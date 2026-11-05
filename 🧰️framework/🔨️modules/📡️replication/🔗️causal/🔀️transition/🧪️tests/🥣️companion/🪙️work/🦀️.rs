//! 🪙️ Fold work follows actual polls and delegated retirement independently of physical item receipts.
use semio_framework_replication_history::{HistoryFoldJob,HistoryFoldJobStep};
use semio_framework_value::{ErasedSnapshotRetirement,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use std::sync::{Arc,atomic::{AtomicU64,Ordering}};
use semio_framework_value_testing_allocator as _;

struct Validation { performed:Arc<AtomicU64>, failed:bool, terminal:bool }
impl ErasedSnapshotRetirement for Validation {
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome {
        if self.terminal{return RetainedWorkOutcome{work:0,result:Ok(RetainedCloneStep::Complete(Default::default()))}}
        if grant.maximum_items==0||grant.maximum_depth==0{return RetainedWorkOutcome{work:0,result:Ok(RetainedCloneStep::Progress(Default::default()))}}
        let mut work=0;
        for _ in 0..3 {self.performed.fetch_add(1,Ordering::Relaxed);work+=1;}
        self.terminal=true;
        let result=if self.failed{Err(ValueError::literal(ValueRefusalKind::InvalidValue,"fold validation fixture refused after its performed prefix"))}else{Ok(RetainedCloneStep::Progress(Default::default()))};
        RetainedWorkOutcome{work,result}
    }
    fn terminal_is_empty(&self)->bool{self.terminal}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(!self.terminal))}
}
fn finish(job:&mut HistoryFoldJob<'_,()>){
    job.request_cancel();
    for _ in 0..128 {
        if job.terminal_is_empty(){return}
        let grant=job.next_step_grant(1,7).unwrap();
        let (turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(grant));
        let step=turn.result.unwrap();
        assert!(turn.work>0||matches!(step,RetainedCloneStep::Complete(_))&&job.terminal_is_empty());
        let progress=step.progress();
        assert!(progress.fits(grant));
        assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
    }
    panic!("fold did not retire its finite original owners")
}
#[test]
fn fold_work_preserves_delegated_validation_without_physical_items_even_on_failure(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🗂️file/🧪️tests/🪙️work/🧫️fixtures/🔣️.json")).unwrap();
    let expected=law["cases"].as_array().unwrap().iter().find(|row|row["id"]=="fold-delegated-retirement").unwrap()["expected"]["work"].as_u64().unwrap();
    for failed in [false,true] {
        let performed=Arc::new(AtomicU64::new(0));
        let mut job=HistoryFoldJob::<()>::new(|_|std::future::pending());
        job.testkit_control().retirements().push_back(Box::new(Validation{performed:performed.clone(),failed,terminal:false}));
        let grant=job.next_step_grant(1,7).unwrap();
        let (denied,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(RetainedCloneGrant{maximum_items:0,..grant}));
        assert_eq!(denied.work,0);
        assert_eq!(denied.result.unwrap().progress(),RetainedCloneProgress::default());
        assert_eq!((heap.requested_bytes,heap.released_bytes,performed.load(Ordering::Relaxed)),(0,0,0));
        let (turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(grant));
        assert_eq!((turn.work,performed.load(Ordering::Relaxed)),(expected,expected));
        assert_eq!(job.last_turn_work(),expected);
        assert_eq!(turn.result.is_err(),failed);
        let progress=match turn.result{Ok(step)=>step.progress(),Err(error)=>{assert_eq!(error.kind,ValueRefusalKind::InvalidValue);error.retained_progress()}};
        assert_eq!(progress,RetainedCloneProgress::default());
        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        finish(&mut job);
        assert_eq!(performed.load(Ordering::Relaxed),expected);
    }
}
#[test]
fn fold_work_resets_on_denied_calls_and_terminal_probes(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🗂️file/🧪️tests/🪙️work/🧫️fixtures/🔣️.json")).unwrap();
    let rows=law["cases"].as_array().unwrap();
    let work=|id:&str|rows.iter().find(|row|row["id"]==id).unwrap()["expected"]["work"].as_u64().unwrap();
    let mut job=HistoryFoldJob::<()>::new(|_|std::future::pending());
    let grant=job.next_step_grant(1,7).unwrap();
    let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.step(grant,7,&mut||false));
    assert!(matches!(step.unwrap(),HistoryFoldJobStep::Pending{..}));
    assert_eq!(job.last_turn_work(),work("fold-direct-future-poll"));
    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    let (turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(RetainedCloneGrant{maximum_items:0,..grant}));
    assert_eq!(turn.work,work("fold-denied-cleanup"));
    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    finish(&mut job);
    let (turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(RetainedCloneGrant{maximum_items:0,..grant}));
    assert!(matches!(turn.result.unwrap(),RetainedCloneStep::Complete(_)));
    assert_eq!(turn.work,work("fold-terminal-cleanup-probe"));
    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
}

#[test]
fn fold_work_separates_control_teardown_from_handoff_and_closes_intervening_cancellation(){
    for cancel in [false,true] {
        let mut job=HistoryFoldJob::<()>::new(|_|std::future::ready(Ok(())));
        let mut handed_off=false;
        let mut control_teardown_seen=false;
        for _ in 0..16 {
            let grant=job.next_step_grant(1,7).unwrap();
            if grant.maximum_copy_bytes>7 {
                assert!(control_teardown_seen);
                if cancel {job.request_cancel();let grant=job.next_step_grant(1,7).unwrap();let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.step(grant,7,&mut||false));assert!(matches!(step.unwrap(),HistoryFoldJobStep::Pending{..}));assert_eq!(job.last_turn_work(),1);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));finish(&mut job);break}
            }
            let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.step(grant,7,&mut||false));
            assert_eq!(job.last_turn_work(),1);
            let progress=match step.unwrap(){
                HistoryFoldJobStep::Pending{progress,..}=>{if progress.released_bytes>0&&grant.maximum_capacity_bytes==0{control_teardown_seen=true}progress},
                HistoryFoldJobStep::Ready{value:(),progress}=>{assert!(control_teardown_seen);handed_off=true;progress},
                HistoryFoldJobStep::Rejected{..}=>panic!("unit fold rejected"),
            };
            assert_eq!(progress.copied_items,1);
            assert!(progress.fits(grant));
            assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
            if handed_off {break}
        }
        assert!(job.terminal_is_empty());
        assert_eq!(handed_off,!cancel);
    }
}
