//! 🚚️ Bitwise vector clones move whole pages per turn: equal to the `Clone` oracle, O(bytes/page) turns, exact capacity accounting and leak-free cancellation at every turn.
use semio_framework_test_fixtures::fixture_text;
use super::*;
use crate::observe_retirement_allocations as observe;
use std::{fmt::Debug, ops::Deref};

const CLOSE_COPY_BYTES: usize = RETAINED_CLONE_BULK_PAGE_BYTES;
fn assert_actual_step<E:Debug>(outcome:crate::RetainedWorkOutcome<RetainedCloneStep,E>)->RetainedCloneStep{let work=outcome.work;let step=outcome.result.unwrap_or_else(|error|panic!("retained work {work} failed: {error:?}"));assert!(step.progress()==RetainedCloneProgress::default()||work>0,"retained effects have no actual work");step}


struct Held<T: RetainedClone>(FixtureSource<Vec<T>>);

impl<T: RetainedClone> Held<T> {
    fn new(owner: Vec<T>) -> Self {
        Self(RetainedCloneSource::from_owner(owner))
    }
}

impl<T: RetainedClone> Deref for Held<T> {
    type Target = RetainedCloneSource<Vec<T>>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T: RetainedClone> Drop for Held<T> {
    fn drop(&mut self) {
        if std::thread::panicking() { return; }
        for _ in 0..1_000_000 {
            if self.0.terminal_is_empty() { return; }
            let copy = self.0.next_close_copy_byte_demand().unwrap().max(CLOSE_COPY_BYTES);
            let d = self.0.next_close_demand(copy).unwrap();
            let grant = RetainedCloneGrant { maximum_items: copy, maximum_copy_bytes: copy.max(d.copy_bytes), maximum_capacity_bytes: d.capacity_bytes, maximum_release_bytes: d.release_bytes, maximum_depth: d.depth.max(1) };
            assert!(assert_actual_step(effect::settle(grant,|e|self.0.close_step(e))).progress().fits(grant));
        }
        panic!("vector source did not close");
    }
}

fn law() -> serde_json::Value { serde_json::from_str(fixture_text!("../../../🚚️bulk-vec/🧪️tests/🧫️fixtures/🔣️.json")).unwrap() }

fn number(value: &serde_json::Value, name: &str) -> usize { value[name].as_u64().unwrap() as usize }
fn ownership_copy()->usize{let law:serde_json::Value=serde_json::from_str(fixture_text!("../../📜️clone-and-cursor-traits/🧪️tests/take.json")).unwrap();number(&law,"ownershipCopyBytes")}

fn samples(count: usize) -> Vec<u16> {
    let law = law();
    let (multiplier, offset, modulo) = (number(&law["sample"], "multiplier"), number(&law["sample"], "offset"), number(&law["sample"], "modulo"));
    (0..count).map(|index| ((index * multiplier + offset) % modulo) as u16).collect()
}

fn close_all<T: RetainedClone>(cursor: &mut VecCursor<T>) {
    effect::begin_close(cursor);
    for _ in 0..1_000_000 {
        if cursor.terminal_is_empty() { return; }
        let copy = cursor.next_close_copy_byte_demand().unwrap().max(CLOSE_COPY_BYTES);
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: cursor.next_close_capacity_byte_demand(copy).unwrap(), maximum_release_bytes: cursor.next_close_release_byte_demand().unwrap(), maximum_depth: cursor.next_close_depth_demand().unwrap().max(1) };
        let quote=cursor.close_work_demand(copy).unwrap();let turn=effect::close_step(cursor,grant);assert!(turn.work<=u128::from(quote));assert!(assert_actual_step(turn).progress().fits(grant));
    }
    panic!("vector clone close did not terminate");
}

struct Turn { work:u128, progress: RetainedCloneProgress, heap: (usize, usize), grant: RetainedCloneGrant }

fn turn_grant<T: RetainedClone>(cursor: &VecCursor<T>, source: &Held<T>, planned: usize, copy: usize) -> RetainedCloneGrant {
    let bound = cursor.source.is_some();
    RetainedCloneGrant {
        maximum_items: 1,
        maximum_copy_bytes: if bound && cursor.index == source.borrow().get().len() && cursor.phase != 0 { copy.max(size_of::<Vec<T>>()) } else if bound { copy } else { copy.max(source.borrow().binding_copy_bytes()) },
        maximum_capacity_bytes: if bound && cursor.phase == 0 { planned } else { 0 },
        maximum_release_bytes: 0,
        maximum_depth: 8,
    }
}

fn advance_one<T: RetainedClone>(cursor: &mut VecCursor<T>, source: &Held<T>, planned: usize, copy: usize) -> (RetainedCloneStep, Turn) {
    let grant = turn_grant(cursor, source, planned, copy);
    let quoted=cursor.advance_work_demand(source.borrow(),copy,effect::CPU).unwrap();
    let (result, heap) = observe(|| effect::advance(cursor,source.borrow(), copy,effect::CPU, grant));
    assert!(result.work<=u128::from(quoted),"actual work {} exceeds its quote {quoted} (phase {})",result.work,cursor.phase);
    let work=result.work;let step=assert_actual_step(result);
    assert!(step.progress().fits(grant));
    (step, Turn { work,progress: step.progress(), heap, grant })
}

fn drive<T: RetainedClone>(source: &Held<T>, copy: usize) -> (Vec<T>, Vec<Turn>, VecCursor<T>) {
    let planned = source.borrow().get().len() * size_of::<T>();
    let mut cursor = VecCursor::<T>::default();
    let mut turns = Vec::new();
    for _ in 0..1_000_000 {
        let (step, turn) = advance_one(&mut cursor, source, planned, copy);
        turns.push(turn);
        if matches!(step, RetainedCloneStep::Complete(_)) { let d=cursor.take_demand().unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let quoted=cursor.take_work_demand().unwrap();let taken=effect::take(&mut cursor,g);assert_eq!(taken.work,u128::from(quoted));assert_eq!(taken.work,1);let(value,p)=taken.result.unwrap().unwrap();assert!(p.fits(g));assert_eq!(p.copied_bytes,d.copy_bytes);return (value,turns,cursor); }
    }
    panic!("vector clone did not terminate");
}









#[test]
fn bulk_vector_clone_capacity_accounting_is_exact() {
    let count = 100_003usize;
    let planned = count * size_of::<u16>();
    let source = Held::new(samples(count));
    let copy = 10_000usize;
    let (output, turns, mut cursor) = drive(&source, copy);
    assert_eq!(output.capacity() * size_of::<u16>(), planned);
    assert_eq!(turns[0].progress, RetainedCloneProgress { copied_items: 1, copied_bytes: source.borrow().binding_copy_bytes(), ..Default::default() });
    assert_eq!(turns[0].heap, (0, 0));
    assert_eq!(turns[1].progress, RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: planned, ..Default::default() });
    assert_eq!(turns[1].heap, (planned, 0), "one reservation for the whole capacity");
    assert_eq!(turns[1].grant.maximum_capacity_bytes, planned);
    let chunks = &turns[2..turns.len()-1];
    assert_eq!(chunks.len(), planned.div_ceil(copy.min(RETAINED_CLONE_BULK_PAGE_BYTES) / 2 * 2));
    for turn in chunks {
        assert_eq!(turn.grant.maximum_capacity_bytes, 0);
        assert_eq!(turn.heap, (0, 0), "chunk copies never allocate");
        assert_eq!(turn.progress.retained_capacity_bytes, 0);
        assert_eq!(turn.progress.released_bytes, 0);
        assert_eq!(turn.progress.copied_items, 1);
    }
    assert_eq!(size_of::<Vec<u16>>(),number(&law(),"ownerHeaderBytes"));assert_eq!(turns.last().unwrap().progress.copied_bytes,ownership_copy());
    assert_eq!(chunks.iter().map(|turn| turn.progress.copied_bytes).sum::<usize>(), planned);
    assert_eq!(turns.iter().map(|turn| turn.progress.retained_capacity_bytes).sum::<usize>(), planned);
    assert_eq!(output, samples(count));
    drop(output);
    close_all(&mut cursor);

    let mut cursor = VecCursor::<u16>::default();
    let bind = turn_grant(&cursor, &source, planned, copy);
    assert_eq!(assert_actual_step(effect::advance(&mut cursor,source.borrow(),copy,effect::CPU,bind)).progress().copied_items, 1);
    let admitted = turn_grant(&cursor, &source, planned, copy);
    for denied in [
        RetainedCloneGrant { maximum_capacity_bytes: planned - 1, ..admitted },
        RetainedCloneGrant { maximum_items: 0, ..admitted },
    ] {
        let (result, heap) = observe(|| effect::advance(&mut cursor,source.borrow(), copy,effect::CPU, denied));
        assert_eq!(result.work,0);assert_eq!(result.result.unwrap().progress(),RetainedCloneProgress::default());
        assert_eq!(heap, (0, 0));
        assert_eq!(cursor.phase, 0);
    }
    assert_eq!(assert_actual_step(effect::advance(&mut cursor,source.borrow(),copy,effect::CPU,admitted)).progress().retained_capacity_bytes, planned);
    let copying = turn_grant(&cursor, &source, planned, copy);
    for denied in [
        RetainedCloneGrant { maximum_copy_bytes: size_of::<u16>() - 1, ..copying },
        RetainedCloneGrant { maximum_items: 0, ..copying },
        RetainedCloneGrant { maximum_depth: 0, ..copying },
    ] {
        let (result, heap) = observe(|| effect::advance(&mut cursor,source.borrow(), copy,effect::CPU, denied));
        assert_eq!(result.work,0);assert_eq!(result.result.unwrap().progress(),RetainedCloneProgress::default());
        assert_eq!(heap, (0, 0));
        assert_eq!(cursor.index, 0);
    }
    let one = RetainedCloneGrant { maximum_copy_bytes: size_of::<u16>(), ..copying };
    assert_eq!(assert_actual_step(effect::advance(&mut cursor,source.borrow(),size_of::<u16>(),effect::CPU,one)).progress(), RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<u16>(), ..Default::default() });
    assert_eq!(cursor.index, 1);
    close_all(&mut cursor);
}

#[test]
fn bulk_array_zero_sized_work_is_bounded_and_every_prefix_closes() {
    const N:usize=8193;
    assert_eq!(N,number(&law(),"largeZeroSizedArrayCount"));
    let original=[();N];let oracle=original.clone();
    let source=RetainedCloneSource::from_owner(original);
    let expected=2+N.div_ceil(RETAINED_CLONE_BULK_PAGE_ITEMS);
    for cut in 0..=expected+1 {
        let mut cursor=ArrayCursor::<(),N>::default();let mut output=None;let mut rows=0;
        for turn in 0..expected {
            if turn==cut { break; }
            let demand=cursor.advance_demands(source.borrow(),RETAINED_CLONE_BULK_PAGE_BYTES,effect::CPU).unwrap();
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
            let before=cursor.index;
            let(step,heap)=observe(||assert_actual_step(effect::advance(&mut cursor,source.borrow(),RETAINED_CLONE_BULK_PAGE_BYTES,effect::CPU,grant)));
            assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
            if cursor.index>=before { assert!(cursor.index-before<=RETAINED_CLONE_BULK_PAGE_ITEMS);rows+=cursor.index-before; }
            if matches!(step,RetainedCloneStep::Complete(_)){let d=cursor.take_demand().unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let taken=effect::take(&mut cursor,g);assert_eq!(taken.work,1);let(value,p)=taken.result.unwrap().unwrap();assert!(p.fits(g));assert_eq!(p.copied_bytes,d.copy_bytes);output=Some(value);break;}
        }
        if let Some(output)=output { assert_eq!(output,oracle);assert_eq!(rows,N); }
        effect::begin_close(&mut cursor);
        for turn in 0..200_000 {
            if cursor.terminal_is_empty(){break;}
            let copy=cursor.next_close_copy_byte_demand().unwrap();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:cursor.next_close_release_byte_demand().unwrap(),maximum_depth:cursor.next_close_depth_demand().unwrap()};
            let(step,heap)=observe(||assert_actual_step(effect::close_step(&mut cursor,grant)));assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(step.progress()!=Default::default() || cursor.terminal_is_empty());assert!(turn<199_999);
        }
        assert!(cursor.terminal_is_empty());assert_eq!(observe(||drop(cursor)).1,(0,0));assert_eq!(source.borrow().get(),&oracle);
    }
}


