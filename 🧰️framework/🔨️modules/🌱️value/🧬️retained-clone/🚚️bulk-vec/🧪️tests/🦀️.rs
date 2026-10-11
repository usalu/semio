//! 🚚️ Bitwise vector clones move whole pages per turn: equal to the `Clone` oracle, O(bytes/page) turns, exact capacity accounting and leak-free cancellation at every turn.
use super::*;
use crate::value::observe_retirement_allocations as observe;
use std::{fmt::Debug, ops::Deref};

const CLOSE_COPY_BYTES: usize = RETAINED_CLONE_BULK_PAGE_BYTES;

struct Held<T: RetainedClone>(RetainedCloneSource<Vec<T>>);

impl<T: RetainedClone> Held<T> {
    fn new(owner: Vec<T>) -> Self {
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: RetainedCloneSource::<Vec<T>>::constructor_copy_bytes(), maximum_capacity_bytes: RetainedCloneSource::<Vec<T>>::owned_constructor_capacity_bytes::<()>(), maximum_depth: 1, ..Default::default() };
        Self(RetainedCloneSource::admit_owned(owner, (), grant).unwrap_or_else(|_| panic!("vector source admission")).0)
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
            assert!(self.0.close_step(grant).unwrap().progress().fits(grant));
        }
        panic!("vector source did not close");
    }
}

fn law() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap() }

fn number(value: &serde_json::Value, name: &str) -> usize { value[name].as_u64().unwrap() as usize }

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
        assert!(cursor.close_step(grant).unwrap().progress().fits(grant));
    }
    panic!("vector clone close did not terminate");
}

struct Turn { progress: RetainedCloneProgress, heap: (usize, usize), grant: RetainedCloneGrant }

fn turn_grant<T: RetainedClone>(cursor: &VecCursor<T>, source: &Held<T>, planned: usize, copy: usize) -> RetainedCloneGrant {
    let bound = cursor.source.is_some();
    RetainedCloneGrant {
        maximum_items: 1,
        maximum_copy_bytes: if bound { copy } else { copy.max(source.borrow().binding_copy_bytes()) },
        maximum_capacity_bytes: if bound && cursor.phase == 0 { planned } else { 0 },
        maximum_release_bytes: 0,
        maximum_depth: 8,
    }
}

fn advance_one<T: RetainedClone>(cursor: &mut VecCursor<T>, source: &Held<T>, planned: usize, copy: usize) -> (RetainedCloneStep, Turn) {
    let grant = turn_grant(cursor, source, planned, copy);
    let (result, heap) = observe(|| cursor.advance(source.borrow(), grant));
    let step = result.unwrap();
    assert!(step.progress().fits(grant));
    (step, Turn { progress: step.progress(), heap, grant })
}

fn drive<T: RetainedClone>(source: &Held<T>, copy: usize) -> (Vec<T>, Vec<Turn>, VecCursor<T>) {
    let planned = source.borrow().get().len() * size_of::<T>();
    let mut cursor = VecCursor::<T>::default();
    let mut turns = Vec::new();
    for _ in 0..1_000_000 {
        let (step, turn) = advance_one(&mut cursor, source, planned, copy);
        turns.push(turn);
        if matches!(step, RetainedCloneStep::Complete(_)) { return (cursor.take().expect("completed vector owner"), turns, cursor); }
    }
    panic!("vector clone did not terminate");
}

fn clone_equals<T: RetainedClone + Clone + PartialEq + Debug>(original: Vec<T>) -> usize {
    let oracle = original.clone();
    let source = Held::new(original);
    let (output, turns, mut cursor) = drive(&source, 1 << 20);
    assert_eq!(output, oracle);
    assert_eq!(source.borrow().get(), &oracle);
    drop(output);
    close_all(&mut cursor);
    turns.len()
}

#[test]
fn bulk_vector_clone_equals_the_clone_oracle_for_every_bitwise_element_shape() {
    let law = law();
    for count in law["sampleCounts"].as_array().unwrap() {
        let count = count.as_u64().unwrap() as usize;
        clone_equals(samples(count));
    }
    clone_equals((0..70_000u32).map(|index| index.wrapping_mul(2_654_435_761)).collect::<Vec<u32>>());
    clone_equals((0..9_000u64).map(|index| index.wrapping_mul(0x9E37_79B9_7F4A_7C15)).collect::<Vec<u64>>());
    clone_equals((0..300_000usize).map(|index| (index % 251) as u8).collect::<Vec<u8>>());
    clone_equals((0..4_000i128).map(|index| -index * 0x1_0000_0000_0001).collect::<Vec<i128>>());
    clone_equals((0..5_000).map(|index| f64::from(index) * 0.25 - 7.0).collect::<Vec<f64>>());
    clone_equals((0..20_000).map(|index| index % 3 == 0).collect::<Vec<bool>>());
    clone_equals((0..9_000u32).filter_map(|index| char::from_u32(index + 0x400)).collect::<Vec<char>>());
    clone_equals((0..30_000u32).map(|index| [index as u8, (index >> 8) as u8, (index >> 16) as u8, 7]).collect::<Vec<[u8; 4]>>());
    clone_equals((0..9_000u16).map(|index| [[index, index ^ 1, index ^ 2], [index ^ 3, index ^ 4, index ^ 5]]).collect::<Vec<[[u16; 3]; 2]>>());
    assert_eq!(clone_equals(vec![(); 5_000]), 3);
    assert_eq!(clone_equals(Vec::<u16>::new()), 3);
}

#[test]
fn non_bitwise_vector_clones_keep_the_per_element_path() {
    assert!(!String::BITWISE && !<Vec<u8>>::BITWISE && !<Option<u8>>::BITWISE);
    let strings: Vec<String> = (0..40).map(|index| format!("row-{index}")).collect();
    let turns = clone_equals_wide(strings);
    assert!(turns > 40 * 3, "elements clone one by one: {turns}");
    clone_equals_wide((0..16).map(|index| (0..index * 300).map(|value| value as u8).collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>());
    clone_equals_wide((0..64u32).map(|index| (index % 4 != 0).then_some(index)).collect::<Vec<Option<u32>>>());
}

fn clone_equals_wide<T: RetainedClone + Clone + PartialEq + Debug>(original: Vec<T>) -> usize {
    let oracle = original.clone();
    let source = Held::new(original);
    let planned = oracle.len() * size_of::<T>();
    let mut cursor = VecCursor::<T>::default();
    let mut turns = 0usize;
    let output = loop {
        turns += 1;
        assert!(turns < 1_000_000, "vector clone did not terminate");
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1 << 16, maximum_capacity_bytes: if cursor.phase == 0 && cursor.source.is_some() { planned } else { 1 << 16 }, maximum_release_bytes: 1 << 16, maximum_depth: 64 };
        let step = cursor.advance(source.borrow(), grant).unwrap();
        assert!(step.progress().fits(grant));
        if matches!(step, RetainedCloneStep::Complete(_)) { break cursor.take().unwrap(); }
    };
    assert_eq!(output, oracle);
    drop(output);
    close_all(&mut cursor);
    turns
}

#[test]
fn bulk_vector_clone_turn_count_is_bytes_over_page() {
    let law = law();
    let (page, width, overhead, empty) = (number(&law, "pageBytes"), number(&law, "elementBytes"), number(&law, "overheadTurns"), number(&law, "emptyCompletionTurns"));
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
            assert_eq!(turns.len(), overhead + chunks.max(empty), "count {count} copy {copy}");
            for turn in turns.iter().skip(overhead) { assert!(turn.progress.copied_bytes <= copy.min(page)); assert_eq!(turn.progress.copied_items, 1); }
            drop(output);
            close_all(&mut cursor);
        }
    }
    let four_mebibytes = samples(2 * 1024 * 1024);
    assert_eq!(four_mebibytes.len() * width, 4 * 1024 * 1024);
    let source = Held::new(four_mebibytes);
    let (output, turns, mut cursor) = drive(&source, 1 << 20);
    assert_eq!(turns.len(), number(&law, "maximumTurnsForFourMebibytes"));
    assert!(turns.len() * 1000 < output.len(), "[DEBUG] 4 MiB vec clone took {} turns for {} elements", turns.len(), output.len());
    eprintln!("[DEBUG] bulk vec clone 4 MiB Vec<u16>: {} turns", turns.len());
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
    let chunks = &turns[2..];
    assert_eq!(chunks.len(), planned.div_ceil(copy.min(RETAINED_CLONE_BULK_PAGE_BYTES) / 2 * 2));
    for turn in chunks {
        assert_eq!(turn.grant.maximum_capacity_bytes, 0);
        assert_eq!(turn.heap, (0, 0), "chunk copies never allocate");
        assert_eq!(turn.progress.retained_capacity_bytes, 0);
        assert_eq!(turn.progress.released_bytes, 0);
        assert_eq!(turn.progress.copied_items, 1);
    }
    assert_eq!(chunks.iter().map(|turn| turn.progress.copied_bytes).sum::<usize>(), planned);
    assert_eq!(turns.iter().map(|turn| turn.progress.retained_capacity_bytes).sum::<usize>(), planned);
    assert_eq!(output, samples(count));
    drop(output);
    close_all(&mut cursor);

    let mut cursor = VecCursor::<u16>::default();
    let bind = turn_grant(&cursor, &source, planned, copy);
    assert_eq!(cursor.advance(source.borrow(), bind).unwrap().progress().copied_items, 1);
    let admitted = turn_grant(&cursor, &source, planned, copy);
    for denied in [
        RetainedCloneGrant { maximum_capacity_bytes: planned - 1, ..admitted },
        RetainedCloneGrant { maximum_items: 0, ..admitted },
    ] {
        let (result, heap) = observe(|| cursor.advance(source.borrow(), denied));
        assert_eq!(result.unwrap().progress(), RetainedCloneProgress::default());
        assert_eq!(heap, (0, 0));
        assert_eq!(cursor.phase, 0);
    }
    assert_eq!(cursor.advance(source.borrow(), admitted).unwrap().progress().retained_capacity_bytes, planned);
    let copying = turn_grant(&cursor, &source, planned, copy);
    for denied in [
        RetainedCloneGrant { maximum_copy_bytes: size_of::<u16>() - 1, ..copying },
        RetainedCloneGrant { maximum_items: 0, ..copying },
        RetainedCloneGrant { maximum_depth: 0, ..copying },
    ] {
        let (result, heap) = observe(|| cursor.advance(source.borrow(), denied));
        assert_eq!(result.unwrap().progress(), RetainedCloneProgress::default());
        assert_eq!(heap, (0, 0));
        assert_eq!(cursor.index, 0);
    }
    let one = RetainedCloneGrant { maximum_copy_bytes: size_of::<u16>(), ..copying };
    assert_eq!(cursor.advance(source.borrow(), one).unwrap().progress(), RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<u16>(), ..Default::default() });
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
                let step = cursor.advance(source.borrow(), grant).unwrap();
                assert!(step.progress().fits(grant));
                if matches!(step, RetainedCloneStep::Complete(_)) { drop(cursor.take()); break; }
            }
            close_all(&mut cursor);
        });
        assert_eq!(allocated, freed, "cancel after {stop} of {total} turns must release every allocation");
        assert_eq!(source.borrow().get(), &samples(count), "the original source stays intact");
        if stop >= 2 { assert!(allocated >= planned, "the reservation was made before cancel at turn {stop}"); }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, crate::RetainedClone, crate::RetireOwned)]
#[retained_clone(bitwise)]
struct Texel { red: u32, green: u32, blue: u32, alpha: u32, reserved: u32 }

#[derive(Clone, Copy, Debug, Default, PartialEq, crate::RetainedClone, crate::RetireOwned)]
struct PlainTexel { red: u32, green: u32 }

#[test]
fn derived_bitwise_structs_clone_in_pages_and_plain_structs_stay_elementwise() {
    assert!(Texel::BITWISE && <[Texel; 3]>::BITWISE && !PlainTexel::BITWISE);
    let texels: Vec<Texel> = (0..5_000u32).map(|index| Texel { red: index, green: index ^ 7, blue: index.wrapping_mul(3), alpha: 255, reserved: 0 }).collect();
    let page_elements = RETAINED_CLONE_BULK_PAGE_BYTES / size_of::<Texel>();
    let source = Held::new(texels.clone());
    let (output, turns, mut cursor) = drive(&source, 1 << 20);
    assert_eq!(output, texels);
    assert_eq!(turns.len(), 2 + texels.len().div_ceil(page_elements));
    assert_eq!(turns.iter().map(|turn| turn.progress.copied_bytes).sum::<usize>(), source.borrow().binding_copy_bytes() + texels.len() * size_of::<Texel>());
    drop(output);
    close_all(&mut cursor);
    let plain: Vec<PlainTexel> = (0..40u32).map(|index| PlainTexel { red: index, green: index + 1 }).collect();
    assert!(clone_equals_wide(plain) > 40 * 3);
}

#[test]
fn bulk_run_elements_obeys_every_grant_axis() {
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 1 };
    assert_eq!(bulk_run_elements(grant, 2), RETAINED_CLONE_BULK_PAGE_BYTES / 2);
    assert_eq!(bulk_run_elements(RetainedCloneGrant { maximum_copy_bytes: 1001, ..grant }, 2), 500);
    assert_eq!(bulk_run_elements(RetainedCloneGrant { maximum_copy_bytes: 1, ..grant }, 2), 0);
    assert_eq!(bulk_run_elements(RetainedCloneGrant { maximum_items: 0, ..grant }, 2), 0);
    assert_eq!(bulk_run_elements(grant, 0), usize::MAX);
    assert_eq!(bulk_run_progress(500, 2), RetainedCloneProgress { copied_items: 1, copied_bytes: 1000, ..Default::default() });
}
