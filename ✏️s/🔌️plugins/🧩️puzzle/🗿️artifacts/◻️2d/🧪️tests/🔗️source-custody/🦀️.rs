//! 🔗️ Native fixtures admit real public source custody and observe its exact final lifecycle.
use semio_framework_value::retained_clone::{RetainedCloneBorrowAuthority, RetainedCloneGrant, RetainedCloneSource};
use semio_framework_trace::observe_heap_allocations_on_this_thread;

pub(crate) fn admit() -> RetainedCloneBorrowAuthority {
    let capacity=RetainedCloneBorrowAuthority::constructor_capacity_bytes::<()>();
    assert!(capacity<=4096);
    assert!(capacity>0);
    let (refused,heap)=observe_heap_allocations_on_this_thread(||RetainedCloneBorrowAuthority::admit((),RetainedCloneGrant::one_capacity_turn(capacity-1,1)));
    assert!(refused.is_err());
    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    let grant=RetainedCloneGrant::one_capacity_turn(capacity,1);
    let (result,heap)=observe_heap_allocations_on_this_thread(||RetainedCloneBorrowAuthority::admit((),grant));
    let (source,progress)=result.unwrap();
    assert!(progress.fits(grant));
    assert_eq!(heap.requested_bytes,progress.retained_capacity_bytes);
    assert_eq!(heap.released_bytes,0);
    assert!(!heap.overflowed);
    source
}

pub(crate) fn close(source:&mut RetainedCloneBorrowAuthority) {
    for _ in 0..100000 {
        if source.terminal_is_empty(){return;}
        let copy=source.next_close_copy_byte_demand().unwrap();
        let capacity=source.next_close_capacity_byte_demand(copy).unwrap();
        let release=source.next_close_release_byte_demand().unwrap();
        let depth=source.next_close_depth_demand().unwrap();
        assert!(copy+capacity+release<=4096);
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};
        let (zero,heap)=observe_heap_allocations_on_this_thread(||source.close_step(RetainedCloneGrant::default()).unwrap());
        assert_eq!(zero.progress(),Default::default());
        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        let (step,heap)=observe_heap_allocations_on_this_thread(||source.close_step(grant).unwrap());
        let progress=step.progress();
        assert!(progress.fits(grant));
        assert_eq!(heap.requested_bytes,progress.retained_capacity_bytes);
        assert_eq!(heap.released_bytes,progress.released_bytes);
        assert!(!heap.overflowed);
    }
    panic!("original source custody did not close under its pure demands");
}

pub(crate) fn admit_source<T:semio_framework_value::retirement::RetireOwned+Sync>(original:std::sync::Arc<T>) -> RetainedCloneSource<T> {
    let capacity=RetainedCloneSource::<T>::constructor_capacity_bytes::<()>();
    assert!(capacity>0&&capacity<=4096);
    let (refused,heap)=observe_heap_allocations_on_this_thread(||RetainedCloneSource::admit(std::sync::Arc::clone(&original),(),RetainedCloneGrant::one_capacity_turn(capacity-1,1)));
    assert!(refused.is_err());
    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    drop(refused);
    let grant=RetainedCloneGrant::one_capacity_turn(capacity,1);
    let (result,heap)=observe_heap_allocations_on_this_thread(||RetainedCloneSource::admit(original,(),grant));
    let (source,progress)=match result {Ok(value)=>value,Err((error,_,_))=>panic!("{error:?}")};
    assert!(progress.fits(grant));
    assert_eq!(heap.requested_bytes,progress.retained_capacity_bytes);
    assert_eq!(heap.released_bytes,0);
    assert!(!heap.overflowed);
    source
}

pub(crate) fn close_source<T:semio_framework_value::retirement::RetireOwned+Sync>(source:&mut RetainedCloneSource<T>) {
    for _ in 0..100000 {
        if source.terminal_is_empty(){return;}
        let copy=source.next_close_copy_byte_demand().unwrap();
        let capacity=source.next_close_capacity_byte_demand(copy).unwrap();
        let release=source.next_close_release_byte_demand().unwrap();
        let depth=source.next_close_depth_demand().unwrap();
        assert!(copy+capacity+release<=4096);
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};
        let (zero,heap)=observe_heap_allocations_on_this_thread(||source.close_step(RetainedCloneGrant::default()).unwrap());
        assert_eq!(zero.progress(),Default::default());
        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        let (step,heap)=observe_heap_allocations_on_this_thread(||source.close_step(grant).unwrap());
        let progress=step.progress();
        assert!(progress.fits(grant));
        assert_eq!(heap.requested_bytes,progress.retained_capacity_bytes);
        assert_eq!(heap.released_bytes,progress.released_bytes);
        assert!(!heap.overflowed);
    }
    panic!("original snapshot source custody did not close under its pure demands");
}

pub(crate) fn close_cursor<C>(cursor:&mut C,maximum_turns:usize,demands:fn(&C)->(usize,usize,usize,usize),step:fn(&mut C,RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError>,terminal:fn(&C)->bool) {
    for _ in 0..maximum_turns {
        if terminal(cursor){return;}
        let ((copy,capacity,release,depth),heap)=observe_heap_allocations_on_this_thread(||demands(cursor));
        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        assert!(!heap.overflowed);assert!(copy+capacity+release<=4096);
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};
        let (zero,heap)=observe_heap_allocations_on_this_thread(||step(cursor,Default::default()).unwrap());
        assert_eq!(zero.progress(),Default::default());
        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        for axis in 0..4 {
            let mut below=grant;
            let demand=match axis {0=>&mut below.maximum_copy_bytes,1=>&mut below.maximum_capacity_bytes,2=>&mut below.maximum_release_bytes,_=>&mut below.maximum_depth};
            if *demand==0 {continue;}
            *demand-=1;
            let (result,heap)=observe_heap_allocations_on_this_thread(||step(cursor,below));
            assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            assert!(!heap.overflowed);
            match result {Ok(step)=>assert_eq!(step.progress(),Default::default()),Err(error)=>assert!(axis==3&&error.kind==semio_framework_value::ValueRefusalKind::DepthLimit)}
            assert_eq!(demands(cursor),(copy,capacity,release,depth));
        }
        let (result,heap)=observe_heap_allocations_on_this_thread(||step(cursor,grant));
        let progress=result.unwrap().progress();
        assert!(progress.fits(grant));
        assert!(!heap.overflowed);
        assert!(heap.requested_bytes<=progress.retained_capacity_bytes);
        assert!(heap.released_bytes<=progress.released_bytes);
    }
    panic!("native domain controller did not close within its original turn limit");
}
