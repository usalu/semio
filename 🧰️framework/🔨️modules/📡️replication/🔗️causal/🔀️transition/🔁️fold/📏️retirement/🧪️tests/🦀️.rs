#[test]
fn bounded_history_fold_physical_retirement_preserves_whole_queued_owners_and_terminal_frames(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for extent in law["extents"].as_array().unwrap(){let extent=extent.as_u64().unwrap()as usize;let mut job=HistoryFoldJob::<()>::new(|_|std::future::pending());let mut original=Vec::with_capacity(extent);if extent!=0{original.push(37u8);}assert_eq!(original.capacity(),extent);assert_eq!(original.len(),extent.min(law["logicalItems"].as_u64().unwrap()as usize));job.control.retire(original);let expected=job.control.0.retirements.lock().front().unwrap().next_close_byte_demand();assert_eq!(job.next_close_byte_demand(),expected,"fold must publish the actual queued owner demand");let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.request_cancel());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let completed=job.completed();
        for _ in 0..100000{if job.terminal_is_empty(){break;}let demand=job.next_close_byte_demand();assert!(demand<=law["maximumAdmission"].as_u64().unwrap()as usize);if demand>0{for denied in [0,demand-1]{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(1,denied).unwrap());assert!(matches!(step,SnapshotRetirementStep::Pending{released_items:0,released_bytes:0}|SnapshotRetirementStep::Blocked));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(job.next_close_byte_demand(),demand);}}
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(1,demand.max(7)).unwrap());assert!(heap.requested_bytes<=demand);let reported=match step{SnapshotRetirementStep::Pending{released_bytes,..}=>released_bytes,SnapshotRetirementStep::Complete|SnapshotRetirementStep::Blocked=>0};assert_eq!(heap.released_bytes,reported);assert_eq!(job.completed(),completed);}
        assert!(job.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));println!("[DEBUG] fold extent={extent} queued original field/cursor/queue/future/waker/control frames queried whole, cancelled without work, terminalDrop0heap");
    }
}

#[test]
fn bounded_history_fold_cancellation_preserves_tracked_future_and_error_backings(){
    fn drain(job:&mut HistoryFoldJob<'_,()>){
        let completed=job.completed();
        let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.request_cancel());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        for _ in 0..100000{if job.terminal_is_empty(){break;}let demand=job.next_close_byte_demand();assert!(demand<=262144);if demand>0{for denied in [0,demand-1]{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(1,denied).unwrap());assert!(matches!(step,SnapshotRetirementStep::Pending{released_items:0,released_bytes:0}|SnapshotRetirementStep::Blocked));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(job.next_close_byte_demand(),demand);}}
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(1,demand.max(7)).unwrap());assert!(heap.requested_bytes<=demand);let reported=match step{SnapshotRetirementStep::Pending{released_bytes,..}=>released_bytes,_=>0};assert_eq!(heap.released_bytes,reported);assert_eq!(job.completed(),completed);
        }assert!(job.terminal_is_empty());
    }
    for extent in [1usize,8194,65536,262144]{
        let original=String::from_utf8(vec![37u8;extent]).unwrap();let pointer=original.as_ptr()as usize;
        let mut job=HistoryFoldJob::new(move|control|async move{let original=control.track(original);assert_eq!(original.as_ptr()as usize,pointer);std::future::pending::<()>().await;drop(original);Ok(())});
        assert!(matches!(job.step(1,1,&mut||false).unwrap(),HistoryFoldJobStep::Pending{released_bytes:0,..}));drain(&mut job);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        let detail=String::from_utf8(vec![37u8;extent]).unwrap();let mut job=HistoryFoldJob::<()>::new(move|_|std::future::ready(Err(crate::ProtocolError::Io(detail))));assert!(matches!(job.step(1,1,&mut||false).unwrap(),HistoryFoldJobStep::Pending{released_bytes:0,..}));drain(&mut job);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));println!("[DEBUG] tracked future and stored error original extent={extent} retain every undergrant, fund wrapper birth independently, release exact frame/backing, terminalDrop0heap");
    }
}
