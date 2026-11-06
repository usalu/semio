//! 🚚️ Bitwise vector clones move whole pages per turn: equal to the `Clone` oracle, O(bytes/page) turns, exact capacity accounting and leak-free cancellation at every turn.
use semio_framework_test_fixtures::fixture_text;
use super::*;
use crate::observe_retirement_allocations as observe;
use std::{fmt::Debug, ops::Deref};

const CLOSE_COPY_BYTES: usize = RETAINED_CLONE_BULK_PAGE_BYTES;
fn assert_actual_step(outcome:crate::RetainedWorkOutcome)->RetainedCloneStep{let work=outcome.work;let step=outcome.result.unwrap_or_else(|error|panic!("retained work {work} failed: {error:?}"));assert!(step.progress()==RetainedCloneProgress::default()||work>0,"retained effects have no actual work");step}


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
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: self.0.next_close_capacity_byte_demand(copy).unwrap(), maximum_release_bytes: self.0.next_close_release_byte_demand().unwrap(), maximum_depth: self.0.next_close_depth_demand().unwrap().max(1) };
            assert!(assert_actual_step(self.0.close_step(grant)).progress().fits(grant));
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
    cursor.begin_close();
    for _ in 0..1_000_000 {
        if cursor.terminal_is_empty() { return; }
        let copy = cursor.next_close_copy_byte_demand().unwrap().max(CLOSE_COPY_BYTES);
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: cursor.next_close_capacity_byte_demand(copy).unwrap(), maximum_release_bytes: cursor.next_close_release_byte_demand().unwrap(), maximum_depth: cursor.next_close_depth_demand().unwrap().max(1) };
        let quote=cursor.close_work_demand(copy).unwrap();let turn=cursor.close_step(grant);assert!(turn.work<=quote);assert!(assert_actual_step(turn).progress().fits(grant));
    }
    panic!("vector clone close did not terminate");
}

struct Turn { work:u64, progress: RetainedCloneProgress, heap: (usize, usize), grant: RetainedCloneGrant }

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
    let quoted=cursor.advance_work_demand(source.borrow(),copy).unwrap();
    let (result, heap) = observe(|| cursor.advance(source.borrow(), copy, grant));
    assert!(result.work<=quoted);
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
        if matches!(step, RetainedCloneStep::Complete(_)) { let d=cursor.take_demand().unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let quoted=cursor.take_work_demand().unwrap();let taken=cursor.take(g);assert_eq!(taken.work,quoted);assert_eq!(taken.work,1);let(value,p)=taken.result.unwrap().unwrap();assert!(p.fits(g));assert_eq!(p.copied_bytes,0);return (value,turns,cursor); }
    }
    panic!("vector clone did not terminate");
}









#[test]
fn bulk_vector_work_quote_refuses_a_different_original_without_effects() {
    let original=Held::new(samples(8));let foreign=Held::new(samples(8));let mut cursor=VecCursor::<u16>::default();
    advance_one(&mut cursor,&original,16,4096);
    let before=cursor.advance_demands(original.borrow(),4096).unwrap();
    let (quote,heap)=observe(||cursor.advance_work_demand(foreign.borrow(),4096));
    assert_eq!(quote.unwrap_err().kind,ValueRefusalKind::InvariantViolated);assert_eq!(heap,(0,0));
    assert_eq!(cursor.advance_demands(original.borrow(),4096).unwrap(),before);
    close_all(&mut cursor);
}
#[test]
fn nested_string_vector_quotes_each_child_and_closes_every_prefix(){
 fn close_output<T:RetireOwned>(value:T){let mut original=Some(value);let mut close=RetainedCloneClose::default();for _ in 0..10000{if original.is_none()&&close.is_empty(){return}let d=close.next_owner_demands::<T>(original.is_some(),&None,4096).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096.max(d.copy_bytes),maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)};let quote=close.next_owner_work::<T>(original.is_some(),&None,4096).unwrap();let turn=if close.is_empty(){let result=close.begin_granted(&mut original,g);RetainedWorkOutcome{result:result.result.map(|step|step.unwrap_or(RetainedCloneStep::Complete(Default::default()))),work:result.work}}else{close.step_granted(g)};assert!(turn.work<=quote);turn.result.unwrap();}panic!("original cloned output did not close")}
 let expected=vec![String::from("alpha"),String::from("βeta"),String::new()];
 for cutoff in 0..64{let source=Held::new(expected.clone());let mut cursor=VecCursor::<String>::default();let mut complete=false;
  for _ in 0..cutoff{let(step,_)=advance_one(&mut cursor,&source,expected.len()*size_of::<String>(),4096);if matches!(step,RetainedCloneStep::Complete(_)){complete=true;break}}
  if complete{let quote=cursor.take_work_demand().unwrap();let turn=cursor.take(RetainedCloneGrant{maximum_items:1,maximum_depth:8,..Default::default()});assert_eq!(turn.work,quote);let output=turn.result.unwrap().unwrap().0;assert_eq!(output,expected);close_output(output);}
  close_all(&mut cursor);
 }
}
#[test]
fn bulk_vector_clone_turn_count_is_bytes_over_page() {
    let law = law();
    let (page, width, overhead) = (number(&law, "pageBytes"), number(&law, "elementBytes"), number(&law, "overheadTurns"));
    assert_eq!(page, RETAINED_CLONE_BULK_PAGE_BYTES);
    assert_eq!(width, size_of::<u16>());
    for count in law["sampleCounts"].as_array().unwrap() {
        let count = count.as_u64().unwrap() as usize;
        for copy in law["copyGrants"].as_array().unwrap() {
            let copy = copy.as_u64().unwrap() as usize;
            let elements = copy.min(page) / width;
            let chunks = count.div_ceil(elements);
            if chunks > 40_000 { continue; }
            let source = Held::new(samples(count));
            let (output, turns, mut cursor) = drive(&source, copy);
            assert_eq!(output, samples(count));
            assert_eq!(turns.len(), overhead + chunks, "count {count} copy {copy}");
            for turn in &turns[2..turns.len()-1] { assert!(turn.work>0);assert!(turn.progress.copied_bytes <= copy.min(page)); assert_eq!(turn.progress.copied_items, 1); }
            drop(output);
            close_all(&mut cursor);
        }
    }
    let four_mebibytes = samples(2 * 1024 * 1024);
    assert_eq!(four_mebibytes.len() * width, 4 * 1024 * 1024);
    let source = Held::new(four_mebibytes);
    let (output, turns, mut cursor) = drive(&source, 1 << 20);
    assert_eq!(turns.len(), number(&law, "maximumTurnsForFourMebibytes"));
    assert!(turns.len() * 1000 < output.len(), "4 MiB vec clone took {} turns for {} elements", turns.len(), output.len());
    drop(output);
    close_all(&mut cursor);
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
    assert_eq!(assert_actual_step(cursor.advance(source.borrow(),copy,bind)).progress().copied_items, 1);
    let admitted = turn_grant(&cursor, &source, planned, copy);
    for denied in [
        RetainedCloneGrant { maximum_capacity_bytes: planned - 1, ..admitted },
        RetainedCloneGrant { maximum_items: 0, ..admitted },
    ] {
        let (result, heap) = observe(|| cursor.advance(source.borrow(), copy, denied));
        assert_eq!(result.work,0);assert_eq!(result.result.unwrap().progress(),RetainedCloneProgress::default());
        assert_eq!(heap, (0, 0));
        assert_eq!(cursor.phase, 0);
    }
    assert_eq!(assert_actual_step(cursor.advance(source.borrow(),copy,admitted)).progress().retained_capacity_bytes, planned);
    let copying = turn_grant(&cursor, &source, planned, copy);
    for denied in [
        RetainedCloneGrant { maximum_copy_bytes: size_of::<u16>() - 1, ..copying },
        RetainedCloneGrant { maximum_items: 0, ..copying },
        RetainedCloneGrant { maximum_depth: 0, ..copying },
    ] {
        let (result, heap) = observe(|| cursor.advance(source.borrow(), copy, denied));
        assert_eq!(result.work,0);assert_eq!(result.result.unwrap().progress(),RetainedCloneProgress::default());
        assert_eq!(heap, (0, 0));
        assert_eq!(cursor.index, 0);
    }
    let one = RetainedCloneGrant { maximum_copy_bytes: size_of::<u16>(), ..copying };
    assert_eq!(assert_actual_step(cursor.advance(source.borrow(),size_of::<u16>(),one)).progress(), RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<u16>(), ..Default::default() });
    assert_eq!(cursor.index, 1);
    close_all(&mut cursor);
}

#[test]
fn bulk_vector_clone_cancels_at_every_turn_and_releases_everything() {
    let count = 40_000usize;
    let source = Held::new(samples(count));
    let planned = count * size_of::<u16>();
    let copy = 1 << 12;
    let total = {
        let (output, turns, mut cursor) = drive(&source, copy);
        drop(output);
        close_all(&mut cursor);
        turns.len()
    };
    assert!(total > 8);
    for stop in 0..=total {
        let (_, (allocated, freed)) = observe(|| {
            let mut cursor = VecCursor::<u16>::default();
            for _ in 0..stop {
                let grant = turn_grant(&cursor, &source, planned, copy);
                let step = assert_actual_step(cursor.advance(source.borrow(),copy,grant));
                assert!(step.progress().fits(grant));
                if matches!(step, RetainedCloneStep::Complete(_)) { let d=cursor.take_demand().unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let taken=cursor.take(g);assert_eq!(taken.work,1);let(value,p)=taken.result.unwrap().unwrap();assert!(p.fits(g));assert_eq!(p.copied_bytes,0);drop(value);break; }
            }
            close_all(&mut cursor);
        });
        assert_eq!(allocated, freed, "cancel after {stop} of {total} turns must release every allocation");
        assert_eq!(source.borrow().get(), &samples(count), "the original source stays intact");
        if stop >= 2 { assert!(allocated >= planned, "the reservation was made before cancel at turn {stop}"); }
    }
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
            let demand=cursor.advance_demands(source.borrow(),RETAINED_CLONE_BULK_PAGE_BYTES).unwrap();
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
            let before=cursor.index;
            let(step,heap)=observe(||assert_actual_step(cursor.advance(source.borrow(),RETAINED_CLONE_BULK_PAGE_BYTES,grant)));
            assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
            if cursor.index>=before { assert!(cursor.index-before<=RETAINED_CLONE_BULK_PAGE_ITEMS);rows+=cursor.index-before; }
            if matches!(step,RetainedCloneStep::Complete(_)){let d=cursor.take_demand().unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let taken=cursor.take(g);assert_eq!(taken.work,1);let(value,p)=taken.result.unwrap().unwrap();assert!(p.fits(g));assert_eq!(p.copied_bytes,0);output=Some(value);break;}
        }
        if let Some(output)=output { assert_eq!(output,oracle);assert_eq!(rows,N); }
        cursor.begin_close();
        for turn in 0..200_000 {
            if cursor.terminal_is_empty(){break;}
            let copy=cursor.next_close_copy_byte_demand().unwrap();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:cursor.next_close_release_byte_demand().unwrap(),maximum_depth:cursor.next_close_depth_demand().unwrap()};
            let(step,heap)=observe(||assert_actual_step(cursor.close_step(grant)));assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(step.progress()!=Default::default() || cursor.terminal_is_empty());assert!(turn<199_999);
        }
        assert!(cursor.terminal_is_empty());assert_eq!(observe(||drop(cursor)).1,(0,0));assert_eq!(source.borrow().get(),&oracle);
    }
}


