//! 🔗️ Native fixtures admit real public source custody and observe its exact final lifecycle.
use semio_framework_value::retained_clone::RetainedCloneBorrowAuthority;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneRef;
use semio_framework_value::retained_clone::RetainedCloneSource;
use semio_framework_trace::observe_heap_allocations_on_this_thread;

/// 🔗️ A unit-authority custody that lends any externally owned value under its lease: the alias identity is the value's own address.
/// The lent reference never outlives the borrow of both the custody and the value (the returned ref carries `'a`).
pub(crate) struct Custody(RetainedCloneBorrowAuthority<()>);

impl Custody {
    pub(crate) fn borrow<'a, T: ?Sized + 'static>(&'a self, value: &'a T) -> RetainedCloneRef<'a, T> {
        let lent: &'static T = unsafe { &*(value as *const T) };
        self.0.borrow(0, move |_| lent)
    }
}

impl std::ops::Deref for Custody {
    type Target = RetainedCloneBorrowAuthority<()>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Custody {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn admit() -> Custody {
    let copy=RetainedCloneBorrowAuthority::<()>::constructor_copy_bytes();
    let capacity=RetainedCloneBorrowAuthority::<()>::constructor_capacity_bytes();
    assert!(capacity<=4096);
    assert!(capacity>0);
    let (refused,heap)=observe_heap_allocations_on_this_thread(||RetainedCloneBorrowAuthority::admit((),RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity-1,maximum_release_bytes:0,maximum_depth:1}));
    assert!(refused.is_err());
    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:0,maximum_depth:1};
    let (result,heap)=observe_heap_allocations_on_this_thread(||RetainedCloneBorrowAuthority::admit((),grant));
    let (source,progress)=result.unwrap();
    assert!(progress.fits(grant));
    assert_eq!(heap.requested_bytes,progress.retained_capacity_bytes);
    assert_eq!(heap.released_bytes,0);
    assert!(!heap.overflowed);
    Custody(source)
}

pub fn close(source:&mut Custody) {
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

pub fn admit_source<T:semio_framework_value::retirement::RetireOwned+Sync+Send+'static>(original:std::sync::Arc<T>) -> RetainedCloneSource<T,std::sync::Arc<T>> {
    let capacity=RetainedCloneSource::<T,std::sync::Arc<T>>::borrowed_constructor_capacity_bytes::<std::sync::Arc<T>>();
    let copy=RetainedCloneSource::<T,std::sync::Arc<T>>::constructor_copy_bytes();
    let constructor=|capacity:usize|RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:0,maximum_depth:1};
    assert!(capacity>0&&capacity<=4096);
    let (refused,heap)=observe_heap_allocations_on_this_thread(||RetainedCloneSource::<T,std::sync::Arc<T>>::admit_borrowed(std::sync::Arc::clone(&original),|arc:&std::sync::Arc<T>|arc.as_ref(),constructor(capacity-1)));
    assert!(refused.is_err());
    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    drop(refused);
    let grant=constructor(capacity);
    let (result,heap)=observe_heap_allocations_on_this_thread(||RetainedCloneSource::<T,std::sync::Arc<T>>::admit_borrowed(original,|arc:&std::sync::Arc<T>|arc.as_ref(),grant));
    let (source,progress)=match result {Ok(value)=>value,Err((error,_))=>panic!("{error:?}")};
    assert!(progress.fits(grant));
    assert_eq!(heap.requested_bytes,progress.retained_capacity_bytes);
    assert_eq!(heap.released_bytes,0);
    assert!(!heap.overflowed);
    source
}

pub fn close_source<T:semio_framework_value::retirement::RetireOwned+Sync+Send+'static>(source:&mut RetainedCloneSource<T,std::sync::Arc<T>>) {
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

pub fn close_cursor<C>(cursor:&mut C,maximum_turns:usize,demands:fn(&C)->(usize,usize,usize,usize),step:fn(&mut C,RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError>,terminal:fn(&C)->bool) {
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
