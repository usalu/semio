//! 📄️ Fixed scalar buffers and explicit typed page metadata reserve for owned writers.
use semio_framework_value_persistent::list::PagedList;
pub const PAGE:usize=3072;
pub struct Fragment{pub bytes:[u8;4096],pub len:usize,pub at:usize}
impl Default for Fragment{fn default()->Self{Self{bytes:[0;4096],len:0,at:0}}}
impl std::fmt::Write for Fragment{fn write_str(&mut self,value:&str)->std::fmt::Result{self.write_bytes(value.as_bytes())}}
impl Fragment{
 pub fn reset(&mut self){self.len=0;self.at=0;}
 pub fn write_bytes(&mut self,value:&[u8])->std::fmt::Result{let end=self.len.checked_add(value.len()).ok_or(std::fmt::Error)?;if end>self.bytes.len(){return Err(std::fmt::Error);}self.bytes[self.len..end].copy_from_slice(value);self.len=end;Ok(())}
}
pub fn reserve<T>(values:&mut PagedList<T,{usize::MAX}>)->Result<bool,String>{if values.has_reserved_slot(){return Ok(true);}let bytes=values.next_allocation_bytes().map_err(|error|error.to_string())?;values.reserve_one_cold(bytes).map_err(|error|semio_framework_value::ValueError::from(error.refusal()).into_message())?;Ok(false)}

#[cfg(any(test, feature = "testkit"))]
pub fn physical_close<T: semio_framework_value::retirement::RetireOwned>(value: T) {
    use semio_framework_job::OriginalRetirementPump;
    use semio_framework_value::{RetainedCloneGrant, RetainedWorkPoll, retirement::controlled::ControlledRetirement};
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;

    let mut owner = ControlledRetirement::retain(value);
    for _ in 0..2_000_000 {
        if owner.terminal_is_empty() { break; }
        let copy = owner.next_copy_byte_demand().unwrap();
        let release = owner.next_release_byte_demand().unwrap();
        let body = if copy > 0 { copy } else { release };
        let demand = owner.next_demand(body).unwrap();
        let grant = RetainedCloneGrant {
            maximum_items: 1,
            maximum_copy_bytes: demand.copy_bytes,
            maximum_capacity_bytes: demand.capacity_bytes,
            maximum_release_bytes: demand.release_bytes,
            maximum_depth: demand.depth,
        };
        let mut run = |grant: RetainedCloneGrant| {
            let mut turn = OriginalRetirementPump::new().turn(1 << 24, u64::MAX, || Some(0), grant).unwrap();
            let (poll, heap) = observe(|| turn.with_effect(|effect| owner.step(body, effect)).unwrap().unwrap().0);
            let progress = match poll {
                RetainedWorkPoll::Complete(turn) => turn.result.unwrap().progress(),
                RetainedWorkPoll::Pending { .. } => Default::default(),
            };
            assert!(!heap.overflowed);
            assert!(progress.fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
            progress
        };
        assert_eq!(run(RetainedCloneGrant { maximum_items: 0, ..grant }), Default::default());
        run(grant);
    }
    assert!(owner.terminal_is_empty());
    let (_, heap) = observe(|| drop(owner));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
}
